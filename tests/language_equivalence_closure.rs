use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_DISTINCT: f64 = 0.70;
const EPS_RICH: f64 = 0.13;
const EPS_BREAK: f64 = 0.045;
const REGRESSION_TOL: f64 = 1.0e-9;
const HOST_WEIGHTS: [f64; 3] = [0.5, 1.0, 2.0];

const H_C: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::C,
    SurfaceSymbol::B,
    SurfaceSymbol::D,
];

const H_D: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::D,
    SurfaceSymbol::B,
    SurfaceSymbol::C,
];

const H_LOOP: [SurfaceSymbol; 2] = [SurfaceSymbol::D, SurfaceSymbol::D];

fn exposed(
    model: &CohfieldLanguageModelV1,
    pattern: &[SurfaceSymbol],
    repeats: usize,
) -> LanguageState {
    model
        .expose(&LanguageState::initial(), pattern, repeats)
        .expect("frozen exposure must be valid")
}

fn learned_sources(
    model: &CohfieldLanguageModelV1,
) -> (LanguageState, LanguageState, LanguageState) {
    (
        exposed(model, &H_C, 64),
        exposed(model, &H_D, 64),
        exposed(model, &H_LOOP, 64),
    )
}

fn carrier(
    learned_c: &LanguageState,
    learned_d: &LanguageState,
    learned_loop: &LanguageState,
) -> [LanguageState; 3] {
    let mut core_c = LanguageState::initial();
    core_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()] =
        learned_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()];
    core_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] =
        learned_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()];

    let mut core_d = LanguageState::initial();
    core_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()] =
        learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()];
    core_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] =
        learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()];

    let mut core_l = core_c.clone();
    core_l.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()] =
        learned_loop.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()];

    [core_c, core_d, core_l]
}

fn host(core: &LanguageState, weight: f64) -> LanguageState {
    let mut state = core.clone();
    state.psi[SurfaceSymbol::B.index()][SurfaceSymbol::A.index()] = weight;
    state
}

fn run_context(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    context: SurfaceSymbol,
) -> Vec<[f64; 4]> {
    let mut current = model
        .evolve(state, &LanguageInput::symbol(context), 1.0)
        .expect("context step must be valid");
    let mut records = vec![current.x];

    for _ in 0..4 {
        current = model
            .evolve(&current, &LanguageInput::zero(), 1.0)
            .expect("zero continuation must be valid");
        records.push(current.x);
    }

    records
}

fn projected_response(model: &CohfieldLanguageModelV1, state: &LanguageState) -> Vec<f64> {
    let mut out = Vec::with_capacity(20);
    for context in [SurfaceSymbol::A, SurfaceSymbol::B] {
        for x in run_context(model, state, context) {
            out.push(x[SurfaceSymbol::A.index()]);
            out.push(x[SurfaceSymbol::B.index()]);
        }
    }
    out
}

fn rich_response(model: &CohfieldLanguageModelV1, state: &LanguageState) -> Vec<f64> {
    let mut out = Vec::with_capacity(80);
    for context in SurfaceSymbol::ALL {
        for x in run_context(model, state, context) {
            out.extend_from_slice(&x);
        }
    }
    out
}

fn euclidean(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}

fn projected_distance(
    model: &CohfieldLanguageModelV1,
    left: &LanguageState,
    right: &LanguageState,
    weight: f64,
) -> f64 {
    let left_host = host(left, weight);
    let right_host = host(right, weight);
    euclidean(
        &projected_response(model, &left_host),
        &projected_response(model, &right_host),
    )
}

fn related(model: &CohfieldLanguageModelV1, left: &LanguageState, right: &LanguageState) -> bool {
    HOST_WEIGHTS
        .iter()
        .all(|&weight| projected_distance(model, left, right, weight) <= EPS_FLOOR)
}

fn broken_route(core_c: &LanguageState) -> LanguageState {
    let mut broken = core_c.clone();
    broken.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] = 0.0;
    broken
}

#[test]
fn cf_lm_006_carrier_uses_only_frozen_learned_route_and_loop_weights() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let [core_c, core_d, core_l] = carrier(&learned_c, &learned_d, &learned_loop);

    assert_eq!(
        core_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()],
        learned_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()]
    );
    assert_eq!(
        core_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()],
        learned_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()]
    );
    assert_eq!(
        core_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()],
        learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()]
    );
    assert_eq!(
        core_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()],
        learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()]
    );
    assert_eq!(
        core_l.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()],
        learned_loop.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()]
    );

    assert_eq!(
        core_c.psi.iter().flatten().filter(|&&v| v != 0.0).count(),
        2
    );
    assert_eq!(
        core_d.psi.iter().flatten().filter(|&&v| v != 0.0).count(),
        2
    );
    assert_eq!(
        core_l.psi.iter().flatten().filter(|&&v| v != 0.0).count(),
        3
    );
}

#[test]
fn cf_lm_006_all_carrier_members_remain_exact_different() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let carrier = carrier(&learned_c, &learned_d, &learned_loop);

    for (i, left_core) in carrier.iter().enumerate() {
        for (j, right_core) in carrier.iter().enumerate().skip(i + 1) {
            let left = host(left_core, 1.0);
            let right = host(right_core, 1.0);
            let distance = CohfieldLanguageModelV1::psi_frobenius_distance(&left, &right);
            assert!(
                distance > EPS_DISTINCT,
                "pair ({i},{j}) distance {distance}"
            );
            assert_ne!(left.psi, right.psi);
        }
    }
}

#[test]
fn cf_lm_006_relation_is_reflexive_on_frozen_carrier() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let carrier = carrier(&learned_c, &learned_d, &learned_loop);

    for state in &carrier {
        assert!(related(&model, state, state));
    }
}

#[test]
fn cf_lm_006_relation_is_symmetric_on_all_distinct_carrier_pairs() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let carrier = carrier(&learned_c, &learned_d, &learned_loop);

    for (i, left) in carrier.iter().enumerate() {
        for right in carrier.iter().skip(i + 1) {
            assert!(related(&model, left, right));
            assert!(related(&model, right, left));
        }
    }
}

#[test]
fn cf_lm_006_relation_is_nontrivially_transitive() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let [core_c, core_d, core_l] = carrier(&learned_c, &learned_d, &learned_loop);

    assert!(related(&model, &core_c, &core_d));
    assert!(related(&model, &core_d, &core_l));
    assert!(related(&model, &core_c, &core_l));

    assert_ne!(core_c.psi, core_d.psi);
    assert_ne!(core_d.psi, core_l.psi);
    assert_ne!(core_c.psi, core_l.psi);
}

#[test]
fn cf_lm_006_pairwise_relation_holds_separately_under_every_host_composition() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let carrier = carrier(&learned_c, &learned_d, &learned_loop);

    for weight in HOST_WEIGHTS {
        for (i, left) in carrier.iter().enumerate() {
            for (j, right) in carrier.iter().enumerate().skip(i + 1) {
                let distance = projected_distance(&model, left, right, weight);
                assert!(
                    distance <= EPS_FLOOR,
                    "host {weight} pair ({i},{j}) projected distance {distance}"
                );
            }
        }
    }
}

#[test]
fn cf_lm_006_rich_observer_distinguishes_every_exact_different_pair() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let carrier = carrier(&learned_c, &learned_d, &learned_loop);

    for (i, left_core) in carrier.iter().enumerate() {
        for (j, right_core) in carrier.iter().enumerate().skip(i + 1) {
            let left = host(left_core, 1.0);
            let right = host(right_core, 1.0);
            let distance = euclidean(
                &rich_response(&model, &left),
                &rich_response(&model, &right),
            );
            assert!(
                distance > EPS_RICH,
                "pair ({i},{j}) rich distance {distance}"
            );
        }
    }
}

#[test]
fn cf_lm_006_broken_route_is_outside_equivalence_class() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let carrier = carrier(&learned_c, &learned_d, &learned_loop);
    let broken = broken_route(&carrier[0]);

    for (index, state) in carrier.iter().enumerate() {
        assert!(!related(&model, &broken, state));
        let distance = projected_distance(&model, &broken, state, 1.0);
        assert!(
            distance > EPS_BREAK,
            "broken-to-carrier {index} distance {distance}"
        );
    }
}

#[test]
fn cf_lm_006_carrier_and_relation_evaluation_are_deterministic_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let left_sources = learned_sources(&model);
    let right_sources = learned_sources(&model);
    let left = carrier(&left_sources.0, &left_sources.1, &left_sources.2);
    let right = carrier(&right_sources.0, &right_sources.1, &right_sources.2);

    for (left_state, right_state) in left.iter().zip(right.iter()) {
        assert_eq!(left_state.psi, right_state.psi);
        assert!(related(&model, left_state, right_state));
    }
}

#[test]
fn cf_lm_006_matches_preregistered_preimplementation_cross_check() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d, learned_loop) = learned_sources(&model);
    let [core_c, core_d, core_l] = carrier(&learned_c, &learned_d, &learned_loop);

    let loop_weight = learned_loop.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()];
    assert!((loop_weight - 3.692_552_048_108_993).abs() < REGRESSION_TOL);

    let host_c = host(&core_c, 1.0);
    let host_d = host(&core_d, 1.0);
    let host_l = host(&core_l, 1.0);

    let psi_cd = CohfieldLanguageModelV1::psi_frobenius_distance(&host_c, &host_d);
    let psi_cl = CohfieldLanguageModelV1::psi_frobenius_distance(&host_c, &host_l);
    let psi_dl = CohfieldLanguageModelV1::psi_frobenius_distance(&host_d, &host_l);

    assert!((psi_cd - 1.988_348_028_216_815).abs() < REGRESSION_TOL);
    assert!((psi_cl - 3.692_552_048_108_993).abs() < REGRESSION_TOL);
    assert!((psi_dl - 4.193_860_811_866_271).abs() < REGRESSION_TOL);

    let rich_cd = euclidean(
        &rich_response(&model, &host_c),
        &rich_response(&model, &host_d),
    );
    let rich_cl = euclidean(
        &rich_response(&model, &host_c),
        &rich_response(&model, &host_l),
    );
    let rich_dl = euclidean(
        &rich_response(&model, &host_d),
        &rich_response(&model, &host_l),
    );

    assert!((rich_cd - 0.346_932_252_345_079_8).abs() < REGRESSION_TOL);
    assert!((rich_cl - 1.627_068_432_104_466_4).abs() < REGRESSION_TOL);
    assert!((rich_dl - 1.656_077_327_943_506).abs() < REGRESSION_TOL);

    let broken = broken_route(&core_c);
    let broken_distance = projected_distance(&model, &broken, &core_c, 1.0);
    assert!((broken_distance - 0.048_012_141_014_796_256).abs() < REGRESSION_TOL);
}
