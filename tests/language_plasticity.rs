use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageObservationProfile, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPSILON_FLOOR: f64 = 1.0e-12;
const EPSILON_R: f64 = 0.10;

const PATTERN_A: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::B,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
];

const PATTERN_B: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::D,
    SurfaceSymbol::C,
    SurfaceSymbol::B,
];

fn counts(pattern: &[SurfaceSymbol], repeats: usize) -> [usize; 4] {
    let mut out = [0; 4];
    for _ in 0..repeats {
        for &symbol in pattern {
            out[symbol.index()] += 1;
        }
    }
    out
}

fn adjacent_pairs(pattern: &[SurfaceSymbol], repeats: usize) -> Vec<[SurfaceSymbol; 2]> {
    let mut sequence = Vec::with_capacity(pattern.len() * repeats);
    for _ in 0..repeats {
        sequence.extend_from_slice(pattern);
    }
    sequence.windows(2).map(|pair| [pair[0], pair[1]]).collect()
}

#[test]
fn cf_lm_001_surface_mapping_is_deterministic_one_hot() {
    for symbol in SurfaceSymbol::ALL {
        let one_hot = symbol.one_hot();
        assert_eq!(one_hot.iter().filter(|&&value| value == 1.0).count(), 1);
        assert_eq!(one_hot.iter().filter(|&&value| value == 0.0).count(), 3);
        assert_eq!(one_hot[symbol.index()], 1.0);
    }
}

#[test]
fn cf_lm_001_histories_have_exactly_matched_symbol_counts() {
    assert_eq!(counts(&PATTERN_A, 32), [32, 32, 32, 32]);
    assert_eq!(counts(&PATTERN_B, 32), [32, 32, 32, 32]);
}

#[test]
fn cf_lm_001_frozen_probes_are_fresh_relative_to_both_histories() {
    let profile = LanguageObservationProfile::cf_lm_001();
    let a_pairs = adjacent_pairs(&PATTERN_A, 32);
    let b_pairs = adjacent_pairs(&PATTERN_B, 32);

    for probe in profile.probes {
        assert!(!a_pairs.contains(&probe), "probe {probe:?} occurs in H_A");
        assert!(!b_pairs.contains(&probe), "probe {probe:?} occurs in H_B");
    }
}

#[test]
fn cf_lm_001_ordered_exposure_creates_persistent_relational_difference() {
    let model = CohfieldLanguageModelV1::default();
    let initial = LanguageState::initial();
    let a = model.expose(&initial, &PATTERN_A, 32).unwrap();
    let b = model.expose(&initial, &PATTERN_B, 32).unwrap();

    let distance = CohfieldLanguageModelV1::psi_frobenius_distance(&a, &b);
    assert!(distance > 0.0);
    assert!((distance - 2.6118385827).abs() < 1.0e-9);
    assert_eq!(a.theta, [1.0; 4]);
    assert_eq!(b.theta, [1.0; 4]);
}

#[test]
fn cf_lm_001_pre_intervention_response_exceeds_frozen_threshold() {
    let model = CohfieldLanguageModelV1::default();
    let profile = LanguageObservationProfile::cf_lm_001();
    let initial = LanguageState::initial();
    let a = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_A, 32).unwrap());
    let b = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_B, 32).unwrap());

    assert_eq!(a.x, b.x);
    assert_eq!(a.theta, b.theta);

    let ra = model.observe(&a, &profile).unwrap();
    let rb = model.observe(&b, &profile).unwrap();
    assert_eq!(ra.flattened().len(), 96);
    assert_eq!(rb.flattened().len(), 96);

    let distance = CohfieldLanguageModelV1::response_distance(&ra, &rb).unwrap();
    assert!(distance > EPSILON_R, "measured D_R={distance}");
    assert!((distance - 0.2867803345).abs() < 1.0e-9);
}

#[test]
fn cf_lm_001_direct_psi_replacement_collapses_response_difference() {
    let model = CohfieldLanguageModelV1::default();
    let profile = LanguageObservationProfile::cf_lm_001();
    let initial = LanguageState::initial();
    let a = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_A, 32).unwrap());
    let b = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_B, 32).unwrap());

    let mut intervened = a.clone();
    intervened.psi = b.psi;

    assert_eq!(intervened.x, b.x);
    assert_eq!(intervened.theta, b.theta);

    let ri = model.observe(&intervened, &profile).unwrap();
    let rb = model.observe(&b, &profile).unwrap();
    let distance = CohfieldLanguageModelV1::response_distance(&ri, &rb).unwrap();
    assert!(distance <= EPSILON_FLOOR, "post-intervention D_R={distance}");
}

#[test]
fn cf_lm_001_identical_history_control_is_at_repeat_floor() {
    let model = CohfieldLanguageModelV1::default();
    let profile = LanguageObservationProfile::cf_lm_001();
    let initial = LanguageState::initial();
    let a1 = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_A, 32).unwrap());
    let a2 = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_A, 32).unwrap());

    assert_eq!(a1.psi, a2.psi);
    let r1 = model.observe(&a1, &profile).unwrap();
    let r2 = model.observe(&a2, &profile).unwrap();
    let distance = CohfieldLanguageModelV1::response_distance(&r1, &r2).unwrap();
    assert!(distance <= EPSILON_FLOOR);
}

#[test]
fn cf_lm_001_no_adaptation_control_collapses_history_effect() {
    let model = CohfieldLanguageModelV1::without_adaptation();
    let profile = LanguageObservationProfile::cf_lm_001();
    let initial = LanguageState::initial();
    let a = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_A, 32).unwrap());
    let b = LanguageState::equalized_from(&model.expose(&initial, &PATTERN_B, 32).unwrap());

    assert_eq!(a.psi, [[0.0; 4]; 4]);
    assert_eq!(b.psi, [[0.0; 4]; 4]);

    let ra = model.observe(&a, &profile).unwrap();
    let rb = model.observe(&b, &profile).unwrap();
    let distance = CohfieldLanguageModelV1::response_distance(&ra, &rb).unwrap();
    assert!(distance <= EPSILON_FLOOR, "no-adaptation D_R={distance}");
}

#[test]
fn cf_lm_001_state_roles_remain_distinct_and_equalizable() {
    let model = CohfieldLanguageModelV1::default();
    let initial = LanguageState::initial();
    let exposed = model.expose(&initial, &PATTERN_A, 32).unwrap();
    let roles = model.roles(&exposed);

    assert_ne!(roles.fast, [0.0; 4]);
    assert_eq!(roles.local_condition, [1.0; 4]);
    assert_ne!(roles.relational_configuration, [[0.0; 4]; 4]);

    let equalized = LanguageState::equalized_from(&exposed);
    assert_eq!(equalized.x, [0.0; 4]);
    assert_eq!(equalized.theta, [1.0; 4]);
    assert_eq!(equalized.psi, exposed.psi);
}
