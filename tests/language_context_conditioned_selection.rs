use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::profiles::language_v2::{InternalEquivalenceProfile, LanguageStateV2};
use cohfield_lm::profiles::language_v3::LanguageStateV3;
use cohfield_lm::profiles::language_v4::{
    CohfieldLanguageModelV4, LanguageExperienceV4, LanguageStateV4,
};
use cohfield_lm::profiles::language_v5::{
    CohfieldLanguageModelV5, LanguageErrorV5, LanguageExperienceV5, LanguageStateV5,
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
const K_AB: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::B,
    SurfaceSymbol::D,
];
const K_BC: [SurfaceSymbol; 4] = [
    SurfaceSymbol::B,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
];
const K_TIE: [SurfaceSymbol; 2] = [SurfaceSymbol::B, SurfaceSymbol::D];
const K_NONE: [SurfaceSymbol; 2] = [SurfaceSymbol::D, SurfaceSymbol::D];

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

fn source_v4() -> LanguageStateV4 {
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

    let v3 = LanguageStateV3::from_v2_without_assessments(&LanguageStateV2::from_v1(&combined));
    CohfieldLanguageModelV4::default()
        .migrate_from_v3(&v3)
        .expect("unassessed V3 source must migrate to V4")
}

fn source_v5(model: &CohfieldLanguageModelV5) -> LanguageStateV5 {
    model
        .migrate_from_v4(&source_v4())
        .expect("frozen V4 source must migrate")
}

fn assess(
    model: &CohfieldLanguageModelV5,
    state: &LanguageStateV5,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV5 {
    model
        .adapt(
            state,
            &LanguageExperienceV5::AssessConsequenceEquivalence(profile),
        )
        .expect("frozen profile assessment must be valid")
}

fn assessed_both(model: &CohfieldLanguageModelV5) -> LanguageStateV5 {
    let source = source_v5(model);
    let after_ab = assess(model, &source, p_ab());
    assess(model, &after_ab, p_bc())
}

fn recognize(
    model: &CohfieldLanguageModelV5,
    state: &LanguageStateV5,
    cue: &[SurfaceSymbol],
) -> LanguageStateV5 {
    model
        .adapt(
            state,
            &LanguageExperienceV5::RecognizeContext(cue.to_vec()),
        )
        .expect("frozen context recognition must be valid")
}

fn infer(model: &CohfieldLanguageModelV5, state: &LanguageStateV5) -> LanguageStateV5 {
    model
        .adapt(
            state,
            &LanguageExperienceV5::InferConsequenceProfileFromContext,
        )
        .expect("frozen context inference must be valid")
}

fn teach_c_to_a(model: &CohfieldLanguageModelV5, state: &LanguageStateV5) -> LanguageStateV5 {
    let mut next = state.clone();
    for _ in 0..EPISODES {
        next = model
            .expose(&next, &[SurfaceSymbol::C, SurfaceSymbol::A], 1)
            .expect("isolated C->A teaching episode must be valid");
    }
    next
}

fn probe(
    model: &CohfieldLanguageModelV5,
    state: &LanguageStateV5,
    symbol: SurfaceSymbol,
    continuation_steps: usize,
) -> Vec<[f64; 4]> {
    let mut local = LanguageStateV5::equalized_from(state);
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

fn contextualize(
    model: &CohfieldLanguageModelV5,
    state: &LanguageStateV5,
    cue: &[SurfaceSymbol],
) -> LanguageStateV5 {
    infer(model, &recognize(model, state, cue))
}

#[test]
fn cf_lm_012_v4_to_v5_migration_preserves_parent_state_and_starts_empty_context_history() {
    let v4_model = CohfieldLanguageModelV4::default();
    let source = source_v4();
    let after_ab = v4_model
        .adapt(
            &source,
            &LanguageExperienceV4::AssessConsequenceEquivalence(p_ab()),
        )
        .expect("V4 P_AB assessment must be valid");
    let after_bc = v4_model
        .adapt(
            &after_ab,
            &LanguageExperienceV4::AssessConsequenceEquivalence(p_bc()),
        )
        .expect("V4 P_BC assessment must be valid");
    let selected = v4_model
        .adapt(
            &after_bc,
            &LanguageExperienceV4::SelectConsequenceProfile(p_ab()),
        )
        .expect("V4 P_AB selection must be valid");

    let v5_model = CohfieldLanguageModelV5::default();
    let v5 = v5_model
        .migrate_from_v4(&selected)
        .expect("conforming V4 State must migrate");

    assert_eq!(v5.x, selected.x);
    assert_eq!(v5.theta, selected.theta);
    assert_eq!(v5.relational.sequential, selected.relational.sequential);
    assert_eq!(
        v5.relational.assessment_history,
        selected.relational.assessment_history
    );
    assert_eq!(v5.relational.selected_profile, selected.relational.selected_profile);
    assert_eq!(v5.relational.current_context_epoch, None);
    assert!(v5.relational.context_history.is_empty());
    assert!(v5.relational.selection_history.is_empty());
}

#[test]
fn cf_lm_012_k_ab_recognition_produces_exact_signature_without_selecting_profile() {
    let model = CohfieldLanguageModelV5::default();
    let assessed = assessed_both(&model);
    let recognized = recognize(&model, &assessed, &K_AB);

    assert_eq!(recognized.relational.selected_profile, None);
    assert_eq!(recognized.relational.context_history.len(), 1);
    assert_eq!(recognized.relational.current_context_epoch, Some(1));
    assert_eq!(recognized.relational.context_history[0].cue, K_AB);
    assert_eq!(
        recognized.relational.context_history[0].activity,
        [0.50, 0.25, 0.00, 0.25]
    );
    assert_eq!(recognized.relational.assessment_history, assessed.relational.assessment_history);
    assert_eq!(recognized.relational.sequential, assessed.relational.sequential);
}

#[test]
fn cf_lm_012_k_ab_inference_scores_all_profiles_and_selects_p_ab_without_profile_input() {
    let model = CohfieldLanguageModelV5::default();
    let recognized = recognize(&model, &assessed_both(&model), &K_AB);
    let inferred = infer(&model, &recognized);

    assert_eq!(inferred.relational.selected_profile, Some(p_ab()));
    assert_eq!(inferred.relational.selection_history.len(), 1);
    let record = &inferred.relational.selection_history[0];
    assert_eq!(record.context_epoch, 1);
    assert_eq!(record.selected_profile, p_ab());
    assert_eq!(record.candidate_scores.len(), 2);
    assert_eq!(record.candidate_scores[0].profile, p_ab());
    assert_eq!(record.candidate_scores[0].score, 0.75);
    assert_eq!(record.candidate_scores[1].profile, p_bc());
    assert_eq!(record.candidate_scores[1].score, 0.25);
}

#[test]
fn cf_lm_012_k_bc_inference_selects_p_bc_from_the_same_generic_rule() {
    let model = CohfieldLanguageModelV5::default();
    let recognized = recognize(&model, &assessed_both(&model), &K_BC);
    let inferred = infer(&model, &recognized);

    assert_eq!(recognized.relational.context_history[0].activity, [0.00, 0.25, 0.50, 0.25]);
    assert_eq!(inferred.relational.selected_profile, Some(p_bc()));
    let scores = &inferred.relational.selection_history[0].candidate_scores;
    assert_eq!(scores[0].profile, p_ab());
    assert_eq!(scores[0].score, 0.25);
    assert_eq!(scores[1].profile, p_bc());
    assert_eq!(scores[1].score, 0.75);
}

#[test]
fn cf_lm_012_tied_context_fails_closed_without_inference_state_mutation() {
    let model = CohfieldLanguageModelV5::default();
    let recognized = recognize(&model, &assessed_both(&model), &K_TIE);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV5::InferConsequenceProfileFromContext,
    );

    assert_eq!(result, Err(LanguageErrorV5::AmbiguousContext));
    assert_eq!(recognized, before);
}

#[test]
fn cf_lm_012_unsupported_context_fails_closed_without_inference_state_mutation() {
    let model = CohfieldLanguageModelV5::default();
    let recognized = recognize(&model, &assessed_both(&model), &K_NONE);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV5::InferConsequenceProfileFromContext,
    );

    assert_eq!(result, Err(LanguageErrorV5::UnsupportedContext));
    assert_eq!(recognized, before);
}

#[test]
fn cf_lm_012_inferred_p_ab_enables_frozen_transfer_after_unselected_teaching() {
    let model = CohfieldLanguageModelV5::default();
    let assessed = assessed_both(&model);
    let trained = teach_c_to_a(&model, &assessed);
    let contextualized = contextualize(&model, &trained, &K_AB);
    let response = probe(&model, &contextualized, SurfaceSymbol::D, 4);

    assert_eq!(contextualized.relational.selected_profile, Some(p_ab()));
    assert!(response[2][SurfaceSymbol::A.index()] > EPS_TRANSFER);
    assert!(
        (response[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs()
            < REGRESSION_TOL
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
fn cf_lm_012_inferred_p_bc_collapses_transfer_without_reassessment_or_learning_loss() {
    let model = CohfieldLanguageModelV5::default();
    let trained = teach_c_to_a(&model, &assessed_both(&model));
    let history = trained.relational.assessment_history.clone();
    let sequential = trained.relational.sequential;
    let contextualized = contextualize(&model, &trained, &K_BC);
    let response = probe(&model, &contextualized, SurfaceSymbol::D, 4);

    assert_eq!(contextualized.relational.selected_profile, Some(p_bc()));
    assert!(response[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert_eq!(contextualized.relational.assessment_history, history);
    assert_eq!(contextualized.relational.sequential, sequential);
}

#[test]
fn cf_lm_012_context_cycle_restores_identical_transfer_without_reassessment_or_relearning() {
    let model = CohfieldLanguageModelV5::default();
    let trained = teach_c_to_a(&model, &assessed_both(&model));
    let assessment_history = trained.relational.assessment_history.clone();
    let sequential = trained.relational.sequential;

    let ab = contextualize(&model, &trained, &K_AB);
    let first = probe(&model, &ab, SurfaceSymbol::D, 4);
    let bc = contextualize(&model, &ab, &K_BC);
    let middle = probe(&model, &bc, SurfaceSymbol::D, 4);
    let ab_again = contextualize(&model, &bc, &K_AB);
    let restored = probe(&model, &ab_again, SurfaceSymbol::D, 4);

    assert_eq!(first, restored);
    assert!(middle[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert_eq!(ab_again.relational.assessment_history, assessment_history);
    assert_eq!(ab_again.relational.sequential, sequential);
    assert_eq!(ab_again.relational.context_history.len(), 3);
    assert_eq!(ab_again.relational.selection_history.len(), 3);
    assert_eq!(ab_again.relational.selected_profile, Some(p_ab()));
}

#[test]
fn cf_lm_012_context_provenance_and_full_cycle_are_deterministic() {
    type Run = (LanguageStateV5, Vec<[f64; 4]>, Vec<[f64; 4]>, Vec<[f64; 4]>);

    fn run() -> Run {
        let model = CohfieldLanguageModelV5::default();
        let trained = teach_c_to_a(&model, &assessed_both(&model));
        let ab = contextualize(&model, &trained, &K_AB);
        let first = probe(&model, &ab, SurfaceSymbol::D, 4);
        let bc = contextualize(&model, &ab, &K_BC);
        let middle = probe(&model, &bc, SurfaceSymbol::D, 4);
        let ab_again = contextualize(&model, &bc, &K_AB);
        let final_response = probe(&model, &ab_again, SurfaceSymbol::D, 4);
        (ab_again, first, middle, final_response)
    }

    let (state, first, middle, restored) = run();
    assert_eq!((state.clone(), first.clone(), middle.clone(), restored.clone()), run());
    assert_eq!(first, restored);
    assert!(middle[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert_eq!(state.relational.context_history[0].cue, K_AB);
    assert_eq!(state.relational.context_history[1].cue, K_BC);
    assert_eq!(state.relational.context_history[2].cue, K_AB);
    assert_eq!(state.relational.selection_history[0].selected_profile, p_ab());
    assert_eq!(state.relational.selection_history[1].selected_profile, p_bc());
    assert_eq!(state.relational.selection_history[2].selected_profile, p_ab());
}
