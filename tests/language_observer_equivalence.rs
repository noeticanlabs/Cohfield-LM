use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageObservationProfile, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_STATE: f64 = 0.05;
const EPS_DISCRIM: f64 = 0.01;

const H_CD: [SurfaceSymbol; 2] = [SurfaceSymbol::C, SurfaceSymbol::D];
const H_DC: [SurfaceSymbol; 2] = [SurfaceSymbol::D, SurfaceSymbol::C];

fn counts(pattern: &[SurfaceSymbol], repeats: usize) -> [usize; 4] {
    let mut out = [0; 4];
    for _ in 0..repeats {
        for &symbol in pattern {
            out[symbol.index()] += 1;
        }
    }
    out
}

fn exposed(model: &CohfieldLanguageModelV1, pattern: &[SurfaceSymbol]) -> LanguageState {
    model
        .expose(&LanguageState::initial(), pattern, 64)
        .expect("frozen exposure must be valid")
}

fn restricted_observer() -> LanguageObservationProfile {
    LanguageObservationProfile {
        probes: vec![
            [SurfaceSymbol::A, SurfaceSymbol::B],
            [SurfaceSymbol::B, SurfaceSymbol::A],
        ],
        continuation_steps: 4,
    }
}

fn enriched_observer() -> LanguageObservationProfile {
    LanguageObservationProfile {
        probes: vec![
            [SurfaceSymbol::A, SurfaceSymbol::B],
            [SurfaceSymbol::B, SurfaceSymbol::A],
            [SurfaceSymbol::C, SurfaceSymbol::D],
            [SurfaceSymbol::D, SurfaceSymbol::C],
        ],
        continuation_steps: 4,
    }
}

fn observe_distance(
    model: &CohfieldLanguageModelV1,
    left: &LanguageState,
    right: &LanguageState,
    profile: &LanguageObservationProfile,
) -> f64 {
    let left_response = model.observe(left, profile).expect("left observation");
    let right_response = model.observe(right, profile).expect("right observation");
    CohfieldLanguageModelV1::response_distance(&left_response, &right_response)
        .expect("matching nonempty responses")
}

#[test]
fn cf_lm_003_histories_have_exactly_matched_surface_counts() {
    assert_eq!(counts(&H_CD, 64), [0, 0, 64, 64]);
    assert_eq!(counts(&H_DC, 64), [0, 0, 64, 64]);
    assert_eq!(counts(&H_CD, 64), counts(&H_DC, 64));
}

#[test]
fn cf_lm_003_exposure_creates_exact_relational_state_difference() {
    let model = CohfieldLanguageModelV1::default();
    let cd = exposed(&model, &H_CD);
    let dc = exposed(&model, &H_DC);

    let distance = CohfieldLanguageModelV1::psi_frobenius_distance(&cd, &dc);
    assert!(distance > EPS_STATE, "Psi distance {distance}");
    assert_ne!(cd.psi, dc.psi);
}

#[test]
fn cf_lm_003_equalization_preserves_only_relational_difference() {
    let model = CohfieldLanguageModelV1::default();
    let cd = LanguageState::equalized_from(&exposed(&model, &H_CD));
    let dc = LanguageState::equalized_from(&exposed(&model, &H_DC));

    assert_eq!(cd.x, dc.x);
    assert_eq!(cd.x, [0.0; 4]);
    assert_eq!(cd.theta, dc.theta);
    assert_eq!(cd.theta, [1.0; 4]);
    assert_ne!(cd.psi, dc.psi);
}

#[test]
fn cf_lm_003_states_are_equivalent_under_restricted_observer() {
    let model = CohfieldLanguageModelV1::default();
    let cd = LanguageState::equalized_from(&exposed(&model, &H_CD));
    let dc = LanguageState::equalized_from(&exposed(&model, &H_DC));

    let distance = observe_distance(&model, &cd, &dc, &restricted_observer());
    assert!(distance <= EPS_FLOOR, "restricted distance {distance}");
}

#[test]
fn cf_lm_003_enriched_observer_distinguishes_same_state_pair() {
    let model = CohfieldLanguageModelV1::default();
    let cd = LanguageState::equalized_from(&exposed(&model, &H_CD));
    let dc = LanguageState::equalized_from(&exposed(&model, &H_DC));

    let distance = observe_distance(&model, &cd, &dc, &enriched_observer());
    assert!(distance > EPS_DISCRIM, "enriched distance {distance}");
}

#[test]
fn cf_lm_003_observer_enrichment_only_adds_frozen_cd_dc_probes() {
    let restricted = restricted_observer();
    let enriched = enriched_observer();

    assert_eq!(restricted.continuation_steps, enriched.continuation_steps);
    assert_eq!(&enriched.probes[..restricted.probes.len()], &restricted.probes);
    assert_eq!(
        &enriched.probes[restricted.probes.len()..],
        &[
            [SurfaceSymbol::C, SurfaceSymbol::D],
            [SurfaceSymbol::D, SurfaceSymbol::C],
        ]
    );
}

#[test]
fn cf_lm_003_restricted_observer_repeat_is_at_floor() {
    let model = CohfieldLanguageModelV1::default();
    let state = LanguageState::equalized_from(&exposed(&model, &H_CD));
    let profile = restricted_observer();

    let distance = observe_distance(&model, &state, &state.clone(), &profile);
    assert!(distance <= EPS_FLOOR);
}

#[test]
fn cf_lm_003_enriched_observer_repeat_is_at_floor() {
    let model = CohfieldLanguageModelV1::default();
    let state = LanguageState::equalized_from(&exposed(&model, &H_CD));
    let profile = enriched_observer();

    let distance = observe_distance(&model, &state, &state.clone(), &profile);
    assert!(distance <= EPS_FLOOR);
}

#[test]
fn cf_lm_003_matches_preregistered_preimplementation_cross_check() {
    let model = CohfieldLanguageModelV1::default();
    let cd = LanguageState::equalized_from(&exposed(&model, &H_CD));
    let dc = LanguageState::equalized_from(&exposed(&model, &H_DC));

    let psi_distance = CohfieldLanguageModelV1::psi_frobenius_distance(&cd, &dc);
    let restricted_distance = observe_distance(&model, &cd, &dc, &restricted_observer());
    let enriched_distance = observe_distance(&model, &cd, &dc, &enriched_observer());

    assert!((psi_distance - 0.061_531_831_442_227_035).abs() < 1.0e-12);
    assert!((cd.psi[SurfaceSymbol::C.index()][SurfaceSymbol::D.index()] - 1.868_030_811_690_309).abs() < 1.0e-12);
    assert!((cd.psi[SurfaceSymbol::D.index()][SurfaceSymbol::C.index()] - 1.824_521_236_418_682_7).abs() < 1.0e-12);
    assert!((dc.psi[SurfaceSymbol::C.index()][SurfaceSymbol::D.index()] - 1.824_521_236_418_682_7).abs() < 1.0e-12);
    assert!((dc.psi[SurfaceSymbol::D.index()][SurfaceSymbol::C.index()] - 1.868_030_811_690_309).abs() < 1.0e-12);
    assert!(restricted_distance <= EPS_FLOOR);
    assert!((enriched_distance - 0.016_529_790_192_257_32).abs() < 1.0e-12);
}
