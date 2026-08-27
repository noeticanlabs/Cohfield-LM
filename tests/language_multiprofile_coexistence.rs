use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::profiles::language_v2::{InternalEquivalenceProfile, LanguageStateV2};
use cohfield_lm::profiles::language_v3::{
    CohfieldLanguageModelV3, LanguageExperienceV3, LanguageStateV3,
};
use cohfield_lm::profiles::language_v4::{
    CohfieldLanguageModelV4, LanguageErrorV4, LanguageExperienceV4, LanguageStateV4,
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

fn p_ab() -> InternalEquivalenceProfile {
    InternalEquivalenceProfile {
        continuation_steps: 4,
        projection: [SurfaceSymbol::A, SurfaceSymbol::B],
        epsilon: EPS_FLOOR,
    }
}

fn p_bc() -> InternalEquivalenceProfile {
    InternalEquivalenceProfile {
        continuation_steps: 4,
        projection: [SurfaceSymbol::B, SurfaceSymbol::C],
        epsilon: EPS_FLOOR,
    }
}

fn p_ac() -> InternalEquivalenceProfile {
    InternalEquivalenceProfile {
        continuation_steps: 4,
        projection: [SurfaceSymbol::A, SurfaceSymbol::C],
        epsilon: EPS_FLOOR,
    }
}

fn source_v3() -> LanguageStateV3 {
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

    LanguageStateV3::from_v2_without_assessments(&LanguageStateV2::from_v1(&combined))
}

fn source_v4(model: &CohfieldLanguageModelV4) -> LanguageStateV4 {
    model
        .migrate_from_v3(&source_v3())
        .expect("unassessed V3 source must migrate")
}

fn assess(
    model: &CohfieldLanguageModelV4,
    state: &LanguageStateV4,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV4 {
    model
        .adapt(
            state,
            &LanguageExperienceV4::AssessConsequenceEquivalence(profile),
        )
        .expect("frozen profile assessment must be valid")
}

fn select(
    model: &CohfieldLanguageModelV4,
    state: &LanguageStateV4,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV4 {
    model
        .adapt(
            state,
            &LanguageExperienceV4::SelectConsequenceProfile(profile),
        )
        .expect("assessed profile selection must be valid")
}

fn assessed_both(model: &CohfieldLanguageModelV4) -> LanguageStateV4 {
    let source = source_v4(model);
    let after_ab = assess(model, &source, p_ab());
    assess(model, &after_ab, p_bc())
}

fn teach_c_to_a(model: &CohfieldLanguageModelV4, state: &LanguageStateV4) -> LanguageStateV4 {
    let mut next = state.clone();
    for _ in 0..EPISODES {
        next = model
            .expose(&next, &[SurfaceSymbol::C, SurfaceSymbol::A], 1)
            .expect("isolated C->A teaching episode must be valid");
    }
    next
}

fn probe(
    model: &CohfieldLanguageModelV4,
    state: &LanguageStateV4,
    symbol: SurfaceSymbol,
    continuation_steps: usize,
) -> Vec<[f64; 4]> {
    let mut local = LanguageStateV4::equalized_from(state);
    local = model
        .evolve(&local, &LanguageInput::symbol(symbol), 1.0)
        .expect("probe drive must be valid");
    let mut out = vec![local.x];
    for _ in 0..continuation_steps {
        local = model
            .evolve(&local, &LanguageInput::zero(), 1.0)
            .expect("probe continuation must be valid");
        out.push(local.x);
    }
    out
}

fn nontrivial_pairs(matrix: [[bool; 4]; 4]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for (left, row) in matrix.iter().enumerate() {
        for (right, &active) in row.iter().enumerate().skip(left + 1) {
            if active {
                pairs.push((left, right));
            }
        }
    }
    pairs
}

fn c_d_distance_for_epoch(state: &LanguageStateV4, epoch: u64) -> f64 {
    state
        .relational
        .assessment_history
        .iter()
        .find(|record| {
            record.epoch == epoch
                && record.left == SurfaceSymbol::C
                && record.right == SurfaceSymbol::D
        })
        .expect("C/D assessment record must exist")
        .measured_distance
}

#[test]
fn cf_lm_011_v3_to_v4_migration_preserves_substrate_history_and_selection() {
    let v3_model = CohfieldLanguageModelV3::default();
    let source = source_v3();
    let after_ab = v3_model
        .adapt(
            &source,
            &LanguageExperienceV3::AssessConsequenceEquivalence(p_ab()),
        )
        .expect("V3 P_AB assessment must be valid");
    let after_bc = v3_model
        .adapt(
            &after_ab,
            &LanguageExperienceV3::AssessConsequenceEquivalence(p_bc()),
        )
        .expect("V3 P_BC assessment must be valid");
    let v3 = v3_model
        .adapt(
            &after_bc,
            &LanguageExperienceV3::AssessConsequenceEquivalence(p_ab()),
        )
        .expect("V3 P_AB reacquisition must be valid");

    let v4_model = CohfieldLanguageModelV4::default();
    let v4 = v4_model
        .migrate_from_v3(&v3)
        .expect("conforming V3 state must migrate");

    assert_eq!(v4.x, v3.x);
    assert_eq!(v4.theta, v3.theta);
    assert_eq!(v4.relational.sequential, v3.relational.sequential);
    assert_eq!(
        v4.relational.assessment_history,
        v3.relational.assessment_history
    );
    assert_eq!(v4.relational.selected_profile, v3.relational.active_profile);
    assert_eq!(
        v4_model.selected_equivalence(&v4).unwrap(),
        v3.relational.active_consequence_equivalence
    );
}

#[test]
fn cf_lm_011_two_profile_assessments_coexist_without_implicit_selection() {
    let model = CohfieldLanguageModelV4::default();
    let source = source_v4(&model);
    let assessed = assessed_both(&model);

    assert_eq!(assessed.relational.assessment_history.len(), 12);
    assert_eq!(assessed.relational.selected_profile, None);
    assert_eq!(assessed.x, source.x);
    assert_eq!(assessed.theta, source.theta);
    assert_eq!(assessed.relational.sequential, source.relational.sequential);
    assert!(assessed.relational.assessment_history[..6]
        .iter()
        .all(|record| record.epoch == 1 && record.profile == p_ab()));
    assert!(assessed.relational.assessment_history[6..]
        .iter()
        .all(|record| record.epoch == 2 && record.profile == p_bc()));
}

#[test]
fn cf_lm_011_stored_profile_views_match_frozen_incompatible_dispositions() {
    let model = CohfieldLanguageModelV4::default();
    let assessed = assessed_both(&model);

    let ab = model
        .equivalence_for_profile(&assessed, p_ab())
        .unwrap()
        .expect("P_AB must be assessed");
    let bc = model
        .equivalence_for_profile(&assessed, p_bc())
        .unwrap()
        .expect("P_BC must be assessed");

    assert_eq!(
        nontrivial_pairs(ab),
        vec![(SurfaceSymbol::C.index(), SurfaceSymbol::D.index())]
    );
    assert!(nontrivial_pairs(bc).is_empty());
    assert!(c_d_distance_for_epoch(&assessed, 1).abs() <= EPS_FLOOR);
    assert!(
        (c_d_distance_for_epoch(&assessed, 2) - 0.577_068_291_019_355_9).abs() < REGRESSION_TOL
    );
}

#[test]
fn cf_lm_011_selecting_p_ab_enables_frozen_internal_transfer() {
    let model = CohfieldLanguageModelV4::default();
    let selected = select(&model, &assessed_both(&model), p_ab());
    let trained = teach_c_to_a(&model, &selected);
    let response = probe(&model, &trained, SurfaceSymbol::D, 4);

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
fn cf_lm_011_switching_to_p_bc_collapses_transfer_without_reassessment_or_learning_loss() {
    let model = CohfieldLanguageModelV4::default();
    let selected_ab = select(&model, &assessed_both(&model), p_ab());
    let trained = teach_c_to_a(&model, &selected_ab);
    let history = trained.relational.assessment_history.clone();
    let learned = trained.relational.sequential;

    let selected_bc = select(&model, &trained, p_bc());
    let response = probe(&model, &selected_bc, SurfaceSymbol::D, 4);

    assert!(response[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert_eq!(selected_bc.relational.assessment_history, history);
    assert_eq!(selected_bc.relational.sequential, learned);
    assert_eq!(selected_bc.relational.selected_profile, Some(p_bc()));
}

#[test]
fn cf_lm_011_switching_back_to_p_ab_restores_identical_transfer_without_reassessment() {
    let model = CohfieldLanguageModelV4::default();
    let selected_ab = select(&model, &assessed_both(&model), p_ab());
    let trained = teach_c_to_a(&model, &selected_ab);
    let first = probe(&model, &trained, SurfaceSymbol::D, 4);
    let selected_bc = select(&model, &trained, p_bc());
    let selected_ab_again = select(&model, &selected_bc, p_ab());
    let restored = probe(&model, &selected_ab_again, SurfaceSymbol::D, 4);

    assert_eq!(first, restored);
    assert_eq!(selected_ab_again.relational.assessment_history.len(), 12);
    assert_eq!(selected_ab_again.relational.selected_profile, Some(p_ab()));
    assert!(
        (restored[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs() < REGRESSION_TOL
    );
}

#[test]
fn cf_lm_011_profile_selection_changes_only_selected_profile() {
    let model = CohfieldLanguageModelV4::default();
    let assessed = assessed_both(&model);
    let selected_ab = select(&model, &assessed, p_ab());
    let selected_bc = select(&model, &selected_ab, p_bc());

    assert_eq!(selected_bc.x, selected_ab.x);
    assert_eq!(selected_bc.theta, selected_ab.theta);
    assert_eq!(
        selected_bc.relational.sequential,
        selected_ab.relational.sequential
    );
    assert_eq!(
        selected_bc.relational.assessment_history,
        selected_ab.relational.assessment_history
    );
    assert_ne!(
        selected_bc.relational.selected_profile,
        selected_ab.relational.selected_profile
    );
}

#[test]
fn cf_lm_011_unassessed_profile_selection_fails_closed() {
    let model = CohfieldLanguageModelV4::default();
    let assessed = assessed_both(&model);
    let before = assessed.clone();
    let result = model.adapt(
        &assessed,
        &LanguageExperienceV4::SelectConsequenceProfile(p_ac()),
    );

    assert_eq!(result, Err(LanguageErrorV4::ProfileNotAssessed));
    assert_eq!(assessed, before);
}

#[test]
fn cf_lm_011_assessment_witness_ignores_current_selection_and_preserves_it() {
    let model = CohfieldLanguageModelV4::default();
    let source = source_v4(&model);
    let after_ab = assess(&model, &source, p_ab());
    let selected_ab = select(&model, &after_ab, p_ab());
    let after_bc = assess(&model, &selected_ab, p_bc());

    assert_eq!(after_bc.relational.selected_profile, Some(p_ab()));
    assert_eq!(after_bc.relational.assessment_history.len(), 12);
    assert!(
        (c_d_distance_for_epoch(&after_bc, 2) - 0.577_068_291_019_355_9).abs() < REGRESSION_TOL
    );
    assert_eq!(
        nontrivial_pairs(model.selected_equivalence(&after_bc).unwrap()),
        vec![(SurfaceSymbol::C.index(), SurfaceSymbol::D.index())]
    );
}

#[test]
fn cf_lm_011_full_assess_select_train_switch_cycle_is_deterministic() {
    type DeterminismRun = (LanguageStateV4, Vec<[f64; 4]>, Vec<[f64; 4]>, Vec<[f64; 4]>);

    fn run() -> DeterminismRun {
        let model = CohfieldLanguageModelV4::default();
        let selected_ab = select(&model, &assessed_both(&model), p_ab());
        let trained = teach_c_to_a(&model, &selected_ab);
        let first = probe(&model, &trained, SurfaceSymbol::D, 4);
        let selected_bc = select(&model, &trained, p_bc());
        let middle = probe(&model, &selected_bc, SurfaceSymbol::D, 4);
        let selected_ab_again = select(&model, &selected_bc, p_ab());
        let final_response = probe(&model, &selected_ab_again, SurfaceSymbol::D, 4);
        (selected_ab_again, first, middle, final_response)
    }

    assert_eq!(run(), run());
}
