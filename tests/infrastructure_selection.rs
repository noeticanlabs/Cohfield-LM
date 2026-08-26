use cohfield_lm::profiles::infrastructure_selection::{
    decay_relational_configuration, frobenius_norm, retention_ratio, AffineForgettingProfile,
    RetentionError,
};

const PSI_HARMFUL: [[f64; 3]; 3] = [
    [1.655, 0.635, 0.682],
    [0.635, 0.248, 0.265],
    [0.682, 0.265, 0.283],
];

const PSI_USEFUL: [[f64; 3]; 3] = [
    [0.364, 0.364, 0.729],
    [0.364, 0.364, 0.729],
    [0.729, 0.729, 1.458],
];

#[test]
fn v007_reported_selective_functional_ordering_is_preserved_as_evidence() {
    let baseline = 0.186150;
    let useful = 0.221769;
    let neutral = 0.189324;
    let harmful = 0.160680;

    assert!(useful > neutral);
    assert!(neutral > harmful);
    assert!(useful > baseline);
    assert!(harmful < baseline);
}

#[test]
fn reported_useful_and_harmful_configuration_norms_match_reconstructed_states() {
    let useful_norm = frobenius_norm(&PSI_USEFUL).unwrap();
    let harmful_norm = frobenius_norm(&PSI_HARMFUL).unwrap();

    assert!((useful_norm - 2.1867).abs() < 0.0010);
    assert!((harmful_norm - 2.1810).abs() < 0.0010);
}

#[test]
fn v008_fixed_forgetting_baseline_matches_reported_retention() {
    let fixed = retention_ratio(0.035, 30).unwrap();
    assert!((fixed - 0.343415).abs() < 1.0e-6);
}

#[test]
fn v008_reported_endogenous_persistence_modulation_preserves_the_failure() {
    let fixed = retention_ratio(0.035, 30).unwrap();
    let useful = 0.372346;
    let neutral = 0.375176;
    let harmful = 0.374891;

    assert!(useful > fixed);
    assert!(neutral > fixed);
    assert!(harmful > fixed);

    // The preregistered usefulness-aligned ordering failed: neutral > harmful > useful.
    assert!(neutral > harmful);
    assert!(harmful > useful);
}

#[test]
fn v009_binary_survival_record_is_only_partially_discriminative() {
    let useful_survivals = 18;
    let neutral_survivals = 16;
    let harmful_survivals = 16;

    assert!(useful_survivals > neutral_survivals);
    assert_eq!(neutral_survivals, harmful_survivals);
}

#[test]
fn v010_recovery_margin_record_restores_three_way_endogenous_ordering() {
    let q_useful = 0.411920;
    let q_neutral = 0.407113;
    let q_harmful = 0.389890;

    assert!(q_useful > q_neutral);
    assert!(q_neutral > q_harmful);
    assert!(((q_useful - q_neutral) - 0.004807).abs() < 1.0e-12);
}

#[test]
fn v010_affine_forgetting_reconstruction_matches_reported_retention() {
    let q_useful = 0.411920;
    let q_neutral = 0.407113;
    let q_harmful = 0.389890;

    // The reported 30-step retention values are exactly consistent, to rounding,
    // with an affine map assigning rho=0.02 to the highest score and rho=0.05
    // to the lowest score. This is a reconstruction of the reported retention
    // mapping, not a universal CF-ACP law.
    let profile = AffineForgettingProfile {
        score_min: q_harmful,
        score_max: q_useful,
        forgetting_at_min: 0.05,
        forgetting_at_max: 0.02,
    };

    let useful = profile.retention_ratio(q_useful, 30).unwrap();
    let neutral = profile.retention_ratio(q_neutral, 30).unwrap();
    let harmful = profile.retention_ratio(q_harmful, 30).unwrap();

    assert!((useful - 0.5455).abs() < 5.0e-5);
    assert!((neutral - 0.4461).abs() < 5.0e-5);
    assert!((harmful - 0.2146).abs() < 5.0e-5);
    assert!(useful > neutral);
    assert!(neutral > harmful);
}

#[test]
fn uniform_decay_scales_relational_configuration_norm_by_retention_ratio() {
    let rate = 0.035;
    let steps = 30;
    let ratio = retention_ratio(rate, steps).unwrap();
    let before = frobenius_norm(&PSI_USEFUL).unwrap();
    let after_matrix = decay_relational_configuration(&PSI_USEFUL, rate, steps).unwrap();
    let after = frobenius_norm(&after_matrix).unwrap();

    assert!((after / before - ratio).abs() < 1.0e-12);
}

#[test]
fn retention_profile_fails_closed_outside_declared_score_domain() {
    let profile = AffineForgettingProfile {
        score_min: 0.389890,
        score_max: 0.411920,
        forgetting_at_min: 0.05,
        forgetting_at_max: 0.02,
    };

    assert_eq!(
        profile.forgetting_rate(0.5),
        Err(RetentionError::ScoreOutsideProfile)
    );
}
