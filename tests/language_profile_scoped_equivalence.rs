use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::profiles::language_v2::{InternalEquivalenceProfile, LanguageStateV2};
use cohfield_lm::profiles::language_v3::{
    CohfieldLanguageModelV3, ConsequenceEquivalenceAssessment, LanguageExperienceV3,
    LanguageStateV3,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_TRANSFER: f64 = 0.01;
const REGRESSION_TOL: f64 = 1.0e-9;
const EPISODES: usize = 8;

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

fn profile_ab() -> InternalEquivalenceProfile {
    InternalEquivalenceProfile {
        continuation_steps: 4,
        projection: [SurfaceSymbol::A, SurfaceSymbol::B],
        epsilon: EPS_FLOOR,
    }
}

fn profile_bc() -> InternalEquivalenceProfile {
    InternalEquivalenceProfile {
        continuation_steps: 4,
        projection: [SurfaceSymbol::B, SurfaceSymbol::C],
        epsilon: EPS_FLOOR,
    }
}

fn source_v2() -> LanguageStateV2 {
    let v1 = CohfieldLanguageModelV1::default();
    let learned_c = v1
        .expose(&LanguageState::initial(), &H_C, 64)
        .expect("frozen C-route exposure must be valid");
    let learned_d = v1
        .expose(&LanguageState::initial(), &H_D, 64)
        .expect("frozen D-route exposure must be valid");

    let mut combined = LanguageState::initial();
    combined.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()] =
        learned_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()];
    combined.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] =
        learned_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()];
    combined.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()] =
        learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()];
    combined.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] =
        learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()];

    LanguageStateV2::from_v1(&combined)
}

fn source_v3() -> LanguageStateV3 {
    LanguageStateV3::from_v2_without_assessments(&source_v2())
}

fn assess(
    model: &CohfieldLanguageModelV3,
    state: &LanguageStateV3,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV3 {
    model
        .adapt(
            state,
            &LanguageExperienceV3::AssessConsequenceEquivalence(profile),
        )
        .expect("frozen assessment must be valid")
}

fn teach_c_to_a(model: &CohfieldLanguageModelV3, state: &LanguageStateV3) -> LanguageStateV3 {
    let mut next = state.clone();
    for _ in 0..EPISODES {
        next = model
            .expose(&next, &[SurfaceSymbol::C, SurfaceSymbol::A], 1)
            .expect("isolated C->A episode must be valid");
    }
    next
}

fn probe_d(model: &CohfieldLanguageModelV3, state: &LanguageStateV3) -> Vec<[f64; 4]> {
    let mut local = LanguageStateV3::equalized_from(state);
    local = model
        .evolve(&local, &LanguageInput::symbol(SurfaceSymbol::D), 1.0)
        .expect("D probe drive must be valid");
    let mut out = vec![local.x];
    for _ in 0..4 {
        local = model
            .evolve(&local, &LanguageInput::zero(), 1.0)
            .expect("probe continuation must be valid");
        out.push(local.x);
    }
    out
}

fn active_pairs(state: &LanguageStateV3) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for left in 0..SurfaceSymbol::ALL.len() {
        for right in (left + 1)..SurfaceSymbol::ALL.len() {
            if state.relational.active_consequence_equivalence[left][right] {
                pairs.push((left, right));
            }
        }
    }
    pairs
}

fn epoch_records(state: &LanguageStateV3, epoch: u64) -> Vec<ConsequenceEquivalenceAssessment> {
    state
        .relational
        .assessment_history
        .iter()
        .filter(|record| record.epoch == epoch)
        .cloned()
        .collect()
}

#[test]
fn cf_lm_010_v2_to_v3_migration_preserves_substrate_and_starts_unassessed() {
    let v2 = source_v2();
    let v3 = LanguageStateV3::from_v2_without_assessments(&v2);

    assert_eq!(v3.x, v2.x);
    assert_eq!(v3.theta, v2.theta);
    assert_eq!(v3.relational.sequential, v2.relational.sequential);
    assert_eq!(v3.relational.active_profile, None);
    assert!(active_pairs(&v3).is_empty());
    assert!(v3.relational.assessment_history.is_empty());
}

#[test]
fn cf_lm_010_p_ab_assessment_activates_only_cd_and_appends_six_records() {
    let model = CohfieldLanguageModelV3::default();
    let assessed = assess(&model, &source_v3(), profile_ab());

    assert_eq!(assessed.relational.active_profile, Some(profile_ab()));
    assert_eq!(
        active_pairs(&assessed),
        vec![(SurfaceSymbol::C.index(), SurfaceSymbol::D.index())]
    );
    assert_eq!(assessed.relational.assessment_history.len(), 6);
    assert!(epoch_records(&assessed, 1)
        .iter()
        .all(|record| record.profile == profile_ab()));
}

#[test]
fn cf_lm_010_p_ab_distances_match_preregistered_geometry() {
    let model = CohfieldLanguageModelV3::default();
    let assessed = assess(&model, &source_v3(), profile_ab());
    let records = epoch_records(&assessed, 1);
    let expected = [
        0.808_461_499_583_201_6,
        0.589_184_158_804_122_9,
        0.589_184_158_804_122_9,
        0.522_975_282_118_704_5,
        0.522_975_282_118_704_5,
        0.0,
    ];

    assert_eq!(records.len(), expected.len());
    for (record, expected_distance) in records.iter().zip(expected) {
        assert!((record.measured_distance - expected_distance).abs() < REGRESSION_TOL);
    }
}

#[test]
fn cf_lm_010_p_bc_revision_deactivates_cd_and_preserves_prior_history() {
    let model = CohfieldLanguageModelV3::default();
    let first = assess(&model, &source_v3(), profile_ab());
    let frozen_first = epoch_records(&first, 1);
    let revised = assess(&model, &first, profile_bc());

    assert_eq!(revised.relational.active_profile, Some(profile_bc()));
    assert!(active_pairs(&revised).is_empty());
    assert_eq!(revised.relational.assessment_history.len(), 12);
    assert_eq!(epoch_records(&revised, 1), frozen_first);
    assert_eq!(epoch_records(&revised, 2).len(), 6);
}

#[test]
fn cf_lm_010_p_bc_witness_ignores_prior_active_equivalence_and_matches_regression() {
    let model = CohfieldLanguageModelV3::default();
    let first = assess(&model, &source_v3(), profile_ab());
    assert_eq!(
        active_pairs(&first),
        vec![(SurfaceSymbol::C.index(), SurfaceSymbol::D.index())]
    );

    let revised = assess(&model, &first, profile_bc());
    let records = epoch_records(&revised, 2);
    let expected = [
        0.589_778_690_146_824_3,
        0.536_905_529_967_305_5,
        0.203_388_495_320_425_08,
        0.778_788_134_351_788_1,
        0.522_975_282_118_704_5,
        0.577_068_291_019_355_9,
    ];

    assert_eq!(records.len(), expected.len());
    for (record, expected_distance) in records.iter().zip(expected) {
        assert!(!record.equivalent);
        assert!((record.measured_distance - expected_distance).abs() < REGRESSION_TOL);
    }
}

#[test]
fn cf_lm_010_p_ab_active_relation_preserves_verified_transfer() {
    let model = CohfieldLanguageModelV3::default();
    let assessed = assess(&model, &source_v3(), profile_ab());
    let trained = teach_c_to_a(&model, &assessed);
    let response = probe_d(&model, &trained);

    assert!(response[2][SurfaceSymbol::A.index()] > EPS_TRANSFER);
    assert!(
        (response[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs() < REGRESSION_TOL
    );
    assert!(
        (trained.relational.sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()]
            - 0.557_984_402_843_442_6)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        trained.relational.sequential[SurfaceSymbol::D.index()][SurfaceSymbol::A.index()].abs()
            <= EPS_FLOOR
    );
}

#[test]
fn cf_lm_010_p_bc_revision_collapses_transfer_without_erasing_c_to_a_learning() {
    let model = CohfieldLanguageModelV3::default();
    let first = assess(&model, &source_v3(), profile_ab());
    let revised = assess(&model, &first, profile_bc());
    let trained = teach_c_to_a(&model, &revised);
    let response = probe_d(&model, &trained);

    assert!(response[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert!(
        (trained.relational.sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()]
            - 0.557_984_402_843_442_6)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        trained.relational.sequential[SurfaceSymbol::D.index()][SurfaceSymbol::A.index()].abs()
            <= EPS_FLOOR
    );
    assert_eq!(trained.relational.assessment_history.len(), 12);
}

#[test]
fn cf_lm_010_reacquiring_p_ab_restores_transfer_and_preserves_all_assessment_epochs() {
    let model = CohfieldLanguageModelV3::default();
    let first = assess(&model, &source_v3(), profile_ab());
    let second = assess(&model, &first, profile_bc());
    let third = assess(&model, &second, profile_ab());

    assert_eq!(third.relational.assessment_history.len(), 18);
    assert_eq!(third.relational.active_profile, Some(profile_ab()));
    assert_eq!(
        active_pairs(&third),
        vec![(SurfaceSymbol::C.index(), SurfaceSymbol::D.index())]
    );
    assert_eq!(epoch_records(&third, 1).len(), 6);
    assert_eq!(epoch_records(&third, 2).len(), 6);
    assert_eq!(epoch_records(&third, 3).len(), 6);

    let trained = teach_c_to_a(&model, &third);
    let response = probe_d(&model, &trained);
    assert!(
        (response[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs() < REGRESSION_TOL
    );
}

#[test]
fn cf_lm_010_assessment_changes_only_relation_memory_not_sequential_substrate() {
    let model = CohfieldLanguageModelV3::default();
    let source = source_v3();
    let first = assess(&model, &source, profile_ab());
    let second = assess(&model, &first, profile_bc());

    assert_eq!(first.x, source.x);
    assert_eq!(first.theta, source.theta);
    assert_eq!(first.relational.sequential, source.relational.sequential);
    assert_eq!(second.x, source.x);
    assert_eq!(second.theta, source.theta);
    assert_eq!(second.relational.sequential, source.relational.sequential);
}

#[test]
fn cf_lm_010_assessment_revision_and_transfer_are_deterministic() {
    let model = CohfieldLanguageModelV3::default();

    let run = || {
        let first = assess(&model, &source_v3(), profile_ab());
        let second = assess(&model, &first, profile_bc());
        let third = assess(&model, &second, profile_ab());
        let trained = teach_c_to_a(&model, &third);
        (third, probe_d(&model, &trained))
    };

    let left = run();
    let right = run();
    assert_eq!(left.0, right.0);
    assert_eq!(left.1, right.1);
}
