use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_SPLIT: f64 = 0.005;
const EPS_ONSET: f64 = 1.0e-4;
const EPS_HOST: f64 = 0.001;
const EPS_DISTINCT: f64 = 0.70;
const REGRESSION_TOL: f64 = 1.0e-9;
const SHORT_HORIZON: usize = 4;
const ONSET_HORIZON: usize = 5;
const LONG_HORIZON: usize = 10;
const CROSS_WEIGHTS: [f64; 3] = [0.5, 1.0, 2.0];
const PAIR_INDICES: [(usize, usize); 3] = [(0, 1), (0, 2), (1, 2)];

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

fn cross_host(core: &LanguageState, weight: f64) -> LanguageState {
    let mut state = core.clone();
    state.psi[SurfaceSymbol::C.index()][SurfaceSymbol::D.index()] = weight;
    state.psi[SurfaceSymbol::D.index()][SurfaceSymbol::C.index()] = weight;
    state
}

fn run_context(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    context: SurfaceSymbol,
    continuation_steps: usize,
) -> Vec<[f64; 4]> {
    let mut current = model
        .evolve(state, &LanguageInput::symbol(context), 1.0)
        .expect("context step must be valid");
    let mut records = vec![current.x];

    for _ in 0..continuation_steps {
        current = model
            .evolve(&current, &LanguageInput::zero(), 1.0)
            .expect("zero continuation must be valid");
        records.push(current.x);
    }

    records
}

fn projected_response(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    continuation_steps: usize,
) -> Vec<f64> {
    let mut out = Vec::with_capacity(4 * (1 + continuation_steps));
    for context in [SurfaceSymbol::A, SurfaceSymbol::B] {
        for x in run_context(model, state, context, continuation_steps) {
            out.push(x[SurfaceSymbol::A.index()]);
            out.push(x[SurfaceSymbol::B.index()]);
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
    continuation_steps: usize,
) -> f64 {
    let left_host = cross_host(left, weight);
    let right_host = cross_host(right, weight);
    euclidean(
        &projected_response(model, &left_host, continuation_steps),
        &projected_response(model, &right_host, continuation_steps),
    )
}

fn ablate_latent_loop(core_l: &LanguageState) -> LanguageState {
    let mut ablated = core_l.clone();
    ablated.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()] = 0.0;
    ablated
}

#[test]
fn cf_lm_007_cross_relay_host_edges_are_unseen_in_all_source_histories() {
    let sources: [(&[SurfaceSymbol], usize); 3] =
        [(&H_C[..], 64), (&H_D[..], 64), (&H_LOOP[..], 64)];

    for (pattern, repeats) in sources {
        assert!(!has_adjacency(
            pattern,
            repeats,
            SurfaceSymbol::C,
            SurfaceSymbol::D
        ));
        assert!(!has_adjacency(
            pattern,
            repeats,
            SurfaceSymbol::D,
            SurfaceSymbol::C
        ));
    }
}

#[test]
fn cf_lm_007_carrier_remains_exact_different_under_common_transfer_host() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let carrier = carrier(&learned.0, &learned.1, &learned.2);
    let hosted = carrier.map(|state| cross_host(&state, 1.0));

    for (left_index, right_index) in PAIR_INDICES {
        let distance = CohfieldLanguageModelV1::psi_frobenius_distance(
            &hosted[left_index],
            &hosted[right_index],
        );
        assert!(
            distance > EPS_DISTINCT,
            "pair ({left_index},{right_index}) state distance {distance}"
        );
        assert_ne!(hosted[left_index].psi, hosted[right_index].psi);
    }
}

#[test]
fn cf_lm_007_short_horizon_transfers_all_pairwise_equivalence() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let carrier = carrier(&learned.0, &learned.1, &learned.2);

    for weight in CROSS_WEIGHTS {
        for (left_index, right_index) in PAIR_INDICES {
            let distance = projected_distance(
                &model,
                &carrier[left_index],
                &carrier[right_index],
                weight,
                SHORT_HORIZON,
            );
            assert!(
                distance <= EPS_FLOOR,
                "host {weight} pair ({left_index},{right_index}) short distance {distance}"
            );
        }
    }
}

#[test]
fn cf_lm_007_long_horizon_preserves_whole_route_pair_equivalence() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let [core_c, core_d, _core_l] = carrier(&learned.0, &learned.1, &learned.2);

    for weight in CROSS_WEIGHTS {
        let distance = projected_distance(&model, &core_c, &core_d, weight, LONG_HORIZON);
        assert!(
            distance <= EPS_FLOOR,
            "host {weight} C/D long distance {distance}"
        );
    }
}

#[test]
fn cf_lm_007_long_horizon_splits_latent_member_from_both_route_members() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let [core_c, core_d, core_l] = carrier(&learned.0, &learned.1, &learned.2);

    for weight in CROSS_WEIGHTS {
        let c_l = projected_distance(&model, &core_c, &core_l, weight, LONG_HORIZON);
        let d_l = projected_distance(&model, &core_d, &core_l, weight, LONG_HORIZON);
        assert!(c_l > EPS_SPLIT, "host {weight} C/L long distance {c_l}");
        assert!(d_l > EPS_SPLIT, "host {weight} D/L long distance {d_l}");
    }
}

#[test]
fn cf_lm_007_latent_split_grows_strictly_with_cross_host_strength() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let [core_c, _core_d, core_l] = carrier(&learned.0, &learned.1, &learned.2);

    let distances = CROSS_WEIGHTS
        .map(|weight| projected_distance(&model, &core_c, &core_l, weight, LONG_HORIZON));

    for window in distances.windows(2) {
        assert!(window[1] > window[0]);
    }
}

#[test]
fn cf_lm_007_latent_effect_is_hidden_through_four_steps_and_visible_at_five() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let [core_c, _core_d, core_l] = carrier(&learned.0, &learned.1, &learned.2);

    for weight in CROSS_WEIGHTS {
        let short = projected_distance(&model, &core_c, &core_l, weight, SHORT_HORIZON);
        let onset = projected_distance(&model, &core_c, &core_l, weight, ONSET_HORIZON);
        assert!(short <= EPS_FLOOR, "host {weight} h4 distance {short}");
        assert!(onset > EPS_ONSET, "host {weight} h5 distance {onset}");
    }
}

#[test]
fn cf_lm_007_direct_latent_loop_ablation_restores_long_horizon_equivalence() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let [core_c, _core_d, core_l] = carrier(&learned.0, &learned.1, &learned.2);
    let ablated = ablate_latent_loop(&core_l);

    assert_eq!(ablated.psi, core_c.psi);

    for weight in CROSS_WEIGHTS {
        let distance = projected_distance(&model, &core_c, &ablated, weight, LONG_HORIZON);
        assert!(
            distance <= EPS_FLOOR,
            "host {weight} ablated C/L long distance {distance}"
        );
    }
}

#[test]
fn cf_lm_007_cross_relay_host_is_nondegenerate_at_long_horizon() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let [core_c, _core_d, _core_l] = carrier(&learned.0, &learned.1, &learned.2);
    let baseline = projected_response(&model, &core_c, LONG_HORIZON);

    for weight in CROSS_WEIGHTS {
        let hosted = cross_host(&core_c, weight);
        let displacement = euclidean(
            &baseline,
            &projected_response(&model, &hosted, LONG_HORIZON),
        );
        assert!(
            displacement > EPS_HOST,
            "host {weight} own-response displacement {displacement}"
        );
    }
}

#[test]
fn cf_lm_007_construction_and_observation_are_deterministic_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let left_sources = learned_sources(&model);
    let right_sources = learned_sources(&model);
    let left = carrier(&left_sources.0, &left_sources.1, &left_sources.2);
    let right = carrier(&right_sources.0, &right_sources.1, &right_sources.2);

    for (left_state, right_state) in left.iter().zip(right.iter()) {
        assert_eq!(left_state.psi, right_state.psi);
        for weight in CROSS_WEIGHTS {
            for horizon in [SHORT_HORIZON, LONG_HORIZON] {
                let distance = projected_distance(&model, left_state, right_state, weight, horizon);
                assert!(distance <= EPS_FLOOR);
            }
        }
    }
}

#[test]
fn cf_lm_007_matches_preregistered_preimplementation_cross_check() {
    let model = CohfieldLanguageModelV1::default();
    let learned = learned_sources(&model);
    let [core_c, core_d, core_l] = carrier(&learned.0, &learned.1, &learned.2);

    let loop_weight = learned.2.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()];
    assert!((loop_weight - 3.692_552_048_108_993).abs() < REGRESSION_TOL);

    let hosted_c = cross_host(&core_c, 1.0);
    let hosted_d = cross_host(&core_d, 1.0);
    let hosted_l = cross_host(&core_l, 1.0);

    let psi_cd = CohfieldLanguageModelV1::psi_frobenius_distance(&hosted_c, &hosted_d);
    let psi_cl = CohfieldLanguageModelV1::psi_frobenius_distance(&hosted_c, &hosted_l);
    let psi_dl = CohfieldLanguageModelV1::psi_frobenius_distance(&hosted_d, &hosted_l);
    assert!((psi_cd - 1.988_348_028_216_815).abs() < REGRESSION_TOL);
    assert!((psi_cl - 3.692_552_048_108_993).abs() < REGRESSION_TOL);
    assert!((psi_dl - 4.193_860_811_866_271).abs() < REGRESSION_TOL);

    let expected_long = [
        0.006_362_262_217_818_672,
        0.026_703_326_585_990_6,
        0.127_836_251_061_514_2,
    ];
    let expected_onset = [
        0.000_145_956_287_756_505_3,
        0.000_583_825_151_026_028_1,
        0.002_335_300_604_104_119_4,
    ];
    let expected_host = [
        0.001_860_225_741_190_769_2,
        0.007_800_361_844_208_359,
        0.037_737_379_795_275_26,
    ];
    let baseline = projected_response(&model, &core_c, LONG_HORIZON);

    for (((weight, expected_long), expected_onset), expected_host) in CROSS_WEIGHTS
        .iter()
        .zip(expected_long.iter())
        .zip(expected_onset.iter())
        .zip(expected_host.iter())
    {
        let long = projected_distance(&model, &core_c, &core_l, *weight, LONG_HORIZON);
        let onset = projected_distance(&model, &core_c, &core_l, *weight, ONSET_HORIZON);
        let hosted = cross_host(&core_c, *weight);
        let host_delta = euclidean(
            &baseline,
            &projected_response(&model, &hosted, LONG_HORIZON),
        );

        assert!((long - expected_long).abs() < REGRESSION_TOL);
        assert!((onset - expected_onset).abs() < REGRESSION_TOL);
        assert!((host_delta - expected_host).abs() < REGRESSION_TOL);
    }
}
