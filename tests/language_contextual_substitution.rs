use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_STATE: f64 = 1.9;
const EPS_BREAK: f64 = 0.045;
const EPS_RICH: f64 = 0.23;
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

#[derive(Clone, Copy)]
enum RouteKind {
    C,
    D,
}

fn counts(pattern: &[SurfaceSymbol], repeats: usize) -> [usize; 4] {
    let mut out = [0; 4];
    for _ in 0..repeats {
        for &symbol in pattern {
            out[symbol.index()] += 1;
        }
    }
    out
}

fn has_adjacency(
    pattern: &[SurfaceSymbol],
    repeats: usize,
    from: SurfaceSymbol,
    to: SurfaceSymbol,
) -> bool {
    let mut previous = None;
    for _ in 0..repeats {
        for &symbol in pattern {
            if previous == Some(from) && symbol == to {
                return true;
            }
            previous = Some(symbol);
        }
    }
    false
}

fn exposed(model: &CohfieldLanguageModelV1, pattern: &[SurfaceSymbol]) -> LanguageState {
    model
        .expose(&LanguageState::initial(), pattern, 64)
        .expect("frozen exposure must be valid")
}

fn learned_pair(model: &CohfieldLanguageModelV1) -> (LanguageState, LanguageState) {
    (exposed(model, &H_C), exposed(model, &H_D))
}

fn host_from_learned(
    learned_c: &LanguageState,
    learned_d: &LanguageState,
    route: RouteKind,
    host_weight: f64,
) -> LanguageState {
    let mut state = LanguageState::initial();

    match route {
        RouteKind::C => {
            state.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()] =
                learned_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()];
            state.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] =
                learned_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()];
        }
        RouteKind::D => {
            state.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()] =
                learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()];
            state.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] =
                learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()];
        }
    }

    state.psi[SurfaceSymbol::B.index()][SurfaceSymbol::A.index()] = host_weight;
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

fn full_response(model: &CohfieldLanguageModelV1, state: &LanguageState) -> Vec<f64> {
    let mut out = Vec::with_capacity(40);
    for context in [SurfaceSymbol::A, SurfaceSymbol::B] {
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

fn full_substitution(c_host: &LanguageState, learned_d: &LanguageState) -> LanguageState {
    let mut state = c_host.clone();
    state.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()] = 0.0;
    state.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] = 0.0;
    state.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()] =
        learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()];
    state.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] =
        learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()];
    state
}

fn first_hop_only_substitution(
    c_host: &LanguageState,
    learned_d: &LanguageState,
) -> LanguageState {
    let mut state = c_host.clone();
    state.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()] = 0.0;
    state.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()] =
        learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()];
    state
}

fn second_hop_only_substitution(
    c_host: &LanguageState,
    learned_d: &LanguageState,
) -> LanguageState {
    let mut state = c_host.clone();
    state.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] = 0.0;
    state.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] =
        learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()];
    state
}

#[test]
fn cf_lm_005_histories_are_matched_and_host_edge_is_unseen() {
    assert_eq!(counts(&H_C, 64), [64, 64, 64, 64]);
    assert_eq!(counts(&H_D, 64), [64, 64, 64, 64]);
    assert_eq!(counts(&H_C, 64), counts(&H_D, 64));

    assert!(!has_adjacency(
        &H_C,
        64,
        SurfaceSymbol::B,
        SurfaceSymbol::A
    ));
    assert!(!has_adjacency(
        &H_D,
        64,
        SurfaceSymbol::B,
        SurfaceSymbol::A
    ));

    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    assert!(learned_c.psi[SurfaceSymbol::B.index()][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert!(learned_d.psi[SurfaceSymbol::B.index()][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
}

#[test]
fn cf_lm_005_route_extraction_uses_only_frozen_learned_route_weights() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, 1.0);
    let host_d = host_from_learned(&learned_c, &learned_d, RouteKind::D, 1.0);

    assert_eq!(
        host_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()],
        learned_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()]
    );
    assert_eq!(
        host_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()],
        learned_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()]
    );
    assert_eq!(
        host_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()],
        learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()]
    );
    assert_eq!(
        host_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()],
        learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()]
    );

    assert_eq!(host_c.psi.iter().flatten().filter(|&&v| v != 0.0).count(), 3);
    assert_eq!(host_d.psi.iter().flatten().filter(|&&v| v != 0.0).count(), 3);
}

#[test]
fn cf_lm_005_host_states_remain_exact_different_across_family() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);

    for host_weight in HOST_WEIGHTS {
        let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, host_weight);
        let host_d = host_from_learned(&learned_c, &learned_d, RouteKind::D, host_weight);
        let distance = CohfieldLanguageModelV1::psi_frobenius_distance(&host_c, &host_d);

        assert!(distance > EPS_STATE, "host {host_weight} state distance {distance}");
        assert_ne!(host_c.psi, host_d.psi);
        assert_eq!(
            host_c.psi[SurfaceSymbol::B.index()][SurfaceSymbol::A.index()],
            host_d.psi[SurfaceSymbol::B.index()][SurfaceSymbol::A.index()]
        );
    }
}

#[test]
fn cf_lm_005_projected_consequence_is_preserved_across_unseen_host_family() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);

    for host_weight in HOST_WEIGHTS {
        let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, host_weight);
        let host_d = host_from_learned(&learned_c, &learned_d, RouteKind::D, host_weight);
        let distance = euclidean(
            &projected_response(&model, &host_c),
            &projected_response(&model, &host_d),
        );

        assert!(distance <= EPS_FLOOR, "host {host_weight} projected distance {distance}");
    }
}

#[test]
fn cf_lm_005_full_route_substitution_is_explicit_and_preserves_consequence() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, 1.0);
    let host_d = host_from_learned(&learned_c, &learned_d, RouteKind::D, 1.0);
    let substituted = full_substitution(&host_c, &learned_d);

    assert_eq!(substituted.x, host_c.x);
    assert_eq!(substituted.theta, host_c.theta);
    assert_eq!(substituted.psi, host_d.psi);
    assert_ne!(substituted.psi, host_c.psi);

    let distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &substituted),
    );
    assert!(distance <= EPS_FLOOR, "full substitution distance {distance}");
}

#[test]
fn cf_lm_005_first_hop_only_substitution_fails_to_preserve_consequence() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, 1.0);
    let hybrid = first_hop_only_substitution(&host_c, &learned_d);

    let distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &hybrid),
    );
    assert!(distance > EPS_BREAK, "first-hop-only distance {distance}");
}

#[test]
fn cf_lm_005_second_hop_only_substitution_fails_to_preserve_consequence() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, 1.0);
    let hybrid = second_hop_only_substitution(&host_c, &learned_d);

    let distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &hybrid),
    );
    assert!(distance > EPS_BREAK, "second-hop-only distance {distance}");
}

#[test]
fn cf_lm_005_route_cut_changes_declared_host_consequence() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, 1.0);
    let mut cut = host_c.clone();
    cut.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] = 0.0;

    let distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &cut),
    );
    assert!(distance > EPS_BREAK, "route-cut distance {distance}");
}

#[test]
fn cf_lm_005_rich_observer_preserves_internal_route_distinction() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, 1.0);
    let host_d = host_from_learned(&learned_c, &learned_d, RouteKind::D, 1.0);

    let distance = euclidean(
        &full_response(&model, &host_c),
        &full_response(&model, &host_d),
    );
    assert!(distance > EPS_RICH, "rich-observer distance {distance}");
}

#[test]
fn cf_lm_005_host_construction_and_observation_are_deterministic_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let (left_c, left_d) = learned_pair(&model);
    let (right_c, right_d) = learned_pair(&model);

    for host_weight in HOST_WEIGHTS {
        let left = host_from_learned(&left_c, &left_d, RouteKind::C, host_weight);
        let right = host_from_learned(&right_c, &right_d, RouteKind::C, host_weight);
        assert_eq!(left.psi, right.psi);

        let distance = euclidean(
            &projected_response(&model, &left),
            &projected_response(&model, &right),
        );
        assert!(distance <= EPS_FLOOR);
    }
}

#[test]
fn cf_lm_005_matches_preregistered_preimplementation_cross_check() {
    let model = CohfieldLanguageModelV1::default();
    let (learned_c, learned_d) = learned_pair(&model);
    let host_c = host_from_learned(&learned_c, &learned_d, RouteKind::C, 1.0);
    let host_d = host_from_learned(&learned_c, &learned_d, RouteKind::D, 1.0);

    assert!(
        (learned_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()]
            - 0.984_081_650_505_525_9)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        (learned_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()]
            - 1.004_164_949_495_434_6)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        (learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()]
            - 0.984_081_650_505_525_9)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        (learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()]
            - 1.004_164_949_495_434_6)
            .abs()
            < REGRESSION_TOL
    );

    let state_distance = CohfieldLanguageModelV1::psi_frobenius_distance(&host_c, &host_d);
    let projected_distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &host_d),
    );
    let full_distance = euclidean(
        &full_response(&model, &host_c),
        &full_response(&model, &host_d),
    );

    let mut cut = host_c.clone();
    cut.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] = 0.0;
    let cut_distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &cut),
    );

    let first_hybrid = first_hop_only_substitution(&host_c, &learned_d);
    let second_hybrid = second_hop_only_substitution(&host_c, &learned_d);
    let first_distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &first_hybrid),
    );
    let second_distance = euclidean(
        &projected_response(&model, &host_c),
        &projected_response(&model, &second_hybrid),
    );

    assert!((state_distance - 1.988_348_028_216_815).abs() < REGRESSION_TOL);
    assert!(projected_distance <= EPS_FLOOR);
    assert!((full_distance - 0.242_670_142_859_152_62).abs() < REGRESSION_TOL);
    assert!((cut_distance - 0.048_012_141_014_796_256).abs() < REGRESSION_TOL);
    assert!((first_distance - 0.048_012_141_014_796_256).abs() < REGRESSION_TOL);
    assert!((second_distance - 0.048_012_141_014_796_256).abs() < REGRESSION_TOL);
}
