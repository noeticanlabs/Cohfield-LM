use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::profiles::language_v2::{InternalEquivalenceProfile, LanguageStateV2};
use cohfield_lm::profiles::language_v3::LanguageStateV3;
use cohfield_lm::profiles::language_v4::{CohfieldLanguageModelV4, LanguageStateV4};
use cohfield_lm::profiles::language_v5::{CohfieldLanguageModelV5, LanguageStateV5};
use cohfield_lm::profiles::language_v6::{
    CohfieldLanguageModelV6, LanguageExperienceV6, LanguageStateV6,
};
use cohfield_lm::profiles::language_v7::{
    CohfieldLanguageModelV7, LanguageErrorV7, LanguageExperienceV7, LanguageStateV7,
    OutcomeApplicabilityDistanceV7, OutcomePredictionErrorV7,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
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

const T_C1: [SurfaceSymbol; 4] = [
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
];
const T_C2: [SurfaceSymbol; 4] = [
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
];
const T_A1: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::D,
];
const T_A2: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
];

const K_C: [SurfaceSymbol; 4] = [
    SurfaceSymbol::B,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
];
const K_A: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::B,
    SurfaceSymbol::D,
];

const Y_TRANSFER: [f64; 5] = [
    0.0,
    0.0,
    0.011_159_688_056_868_854,
    0.016_739_532_085_303_28,
    0.017_363_331_386_570_834,
];
const Y_ZERO: [f64; 5] = [0.0; 5];
const Y_MID: [f64; 5] = [
    0.0,
    0.0,
    0.005_579_844_028_434_427,
    0.008_369_766_042_651_64,
    0.008_681_665_693_285_417,
];
const Y_FAR: [f64; 5] = [1.0; 5];

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

fn source_v6() -> LanguageStateV6 {
    let v4_model = CohfieldLanguageModelV4::default();
    let v4: LanguageStateV4 = v4_model
        .migrate_from_v3(&source_v3())
        .expect("unassessed V3 source must migrate to V4");
    let v5: LanguageStateV5 = CohfieldLanguageModelV5::default()
        .migrate_from_v4(&v4)
        .expect("unassessed V4 source must migrate to V5");
    CohfieldLanguageModelV6::default()
        .migrate_from_v5(&v5)
        .expect("unassessed V5 source must migrate to V6")
}

fn source_v7(model: &CohfieldLanguageModelV7) -> LanguageStateV7 {
    model
        .migrate_from_v6(&source_v6())
        .expect("unassessed V6 source must migrate to V7")
}

fn assess(
    model: &CohfieldLanguageModelV7,
    state: &LanguageStateV7,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV7 {
    model
        .adapt(
            state,
            &LanguageExperienceV7::AssessConsequenceEquivalence(profile),
        )
        .expect("frozen profile assessment must be valid")
}

fn assessed_both(model: &CohfieldLanguageModelV7) -> LanguageStateV7 {
    let source = source_v7(model);
    let after_ab = assess(model, &source, p_ab());
    assess(model, &after_ab, p_bc())
}

fn recognize(
    model: &CohfieldLanguageModelV7,
    state: &LanguageStateV7,
    cue: &[SurfaceSymbol],
) -> LanguageStateV7 {
    model
        .adapt(state, &LanguageExperienceV7::RecognizeContext(cue.to_vec()))
        .expect("frozen context recognition must be valid")
}

fn teach_c_to_a(model: &CohfieldLanguageModelV7, state: &LanguageStateV7) -> LanguageStateV7 {
    let mut next = state.clone();
    for _ in 0..EPISODES {
        next = model
            .expose(&next, &[SurfaceSymbol::C, SurfaceSymbol::A], 1)
            .expect("isolated C->A teaching episode must be valid");
    }
    next
}

fn record_outcome(
    model: &CohfieldLanguageModelV7,
    state: &LanguageStateV7,
    observed: [f64; 5],
) -> LanguageStateV7 {
    model
        .adapt(
            state,
            &LanguageExperienceV7::RecordObservedConsequence(observed),
        )
        .expect("frozen observed consequence must yield a unique supported profile")
}

fn trained_substrate(model: &CohfieldLanguageModelV7) -> LanguageStateV7 {
    teach_c_to_a(model, &assessed_both(model))
}

fn train_outcome_applicability(model: &CohfieldLanguageModelV7) -> LanguageStateV7 {
    let trained = trained_substrate(model);
    let t1 = recognize(model, &trained, &T_C1);
    let t1 = record_outcome(model, &t1, Y_TRANSFER);
    let t2 = recognize(model, &t1, &T_C2);
    let t2 = record_outcome(model, &t2, Y_TRANSFER);
    let t3 = recognize(model, &t2, &T_A1);
    let t3 = record_outcome(model, &t3, Y_ZERO);
    let t4 = recognize(model, &t3, &T_A2);
    record_outcome(model, &t4, Y_ZERO)
}

fn infer_outcome(model: &CohfieldLanguageModelV7, state: &LanguageStateV7) -> LanguageStateV7 {
    model
        .adapt(
            state,
            &LanguageExperienceV7::InferConsequenceProfileFromOutcomeApplicability,
        )
        .expect("held-out consequence-grounded applicability inference must be valid")
}

fn probe(
    model: &CohfieldLanguageModelV7,
    state: &LanguageStateV7,
    symbol: SurfaceSymbol,
    continuation_steps: usize,
) -> Vec<[f64; 4]> {
    let mut local = LanguageStateV7::equalized_from(state);
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

fn euclidean5(left: [f64; 5], right: [f64; 5]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}

fn error_for(errors: &[OutcomePredictionErrorV7], profile: InternalEquivalenceProfile) -> f64 {
    errors
        .iter()
        .find(|entry| entry.profile == profile)
        .expect("profile prediction error must exist")
        .error
}

fn distance_for(
    distances: &[OutcomeApplicabilityDistanceV7],
    profile: InternalEquivalenceProfile,
) -> f64 {
    distances
        .iter()
        .find(|entry| entry.profile == profile)
        .expect("profile applicability distance must exist")
        .distance
}

fn old_projection_score(profile: InternalEquivalenceProfile, activity: [f64; 4]) -> f64 {
    profile
        .projection
        .iter()
        .map(|symbol| activity[symbol.index()])
        .sum()
}

#[test]
fn cf_lm_014_v6_to_v7_migration_preserves_parent_state_and_starts_empty_outcome_history() {
    let v6_model = CohfieldLanguageModelV6::default();
    let source = source_v6();
    let after_ab = v6_model
        .adapt(
            &source,
            &LanguageExperienceV6::AssessConsequenceEquivalence(p_ab()),
        )
        .expect("V6 P_AB assessment must be valid");
    let recognized = v6_model
        .adapt(
            &after_ab,
            &LanguageExperienceV6::RecognizeContext(T_C1.to_vec()),
        )
        .expect("V6 context recognition must be valid");
    let v6 = v6_model
        .adapt(
            &recognized,
            &LanguageExperienceV6::RecordContextApplicability(p_ab()),
        )
        .expect("V6 supervised applicability record must be valid");

    let v7 = CohfieldLanguageModelV7::default()
        .migrate_from_v6(&v6)
        .expect("conforming V6 State must migrate");

    assert_eq!(v7.x, v6.x);
    assert_eq!(v7.theta, v6.theta);
    assert_eq!(v7.relational.sequential, v6.relational.sequential);
    assert_eq!(
        v7.relational.selected_profile,
        v6.relational.selected_profile
    );
    assert_eq!(
        v7.relational.assessment_history,
        v6.relational.assessment_history
    );
    assert_eq!(
        v7.relational.current_context_epoch,
        v6.relational.current_context_epoch
    );
    assert_eq!(v7.relational.context_history, v6.relational.context_history);
    assert_eq!(
        v7.relational.projection_selection_history,
        v6.relational.projection_selection_history
    );
    assert_eq!(
        v7.relational.applicability_history,
        v6.relational.applicability_history
    );
    assert_eq!(
        v7.relational.learned_selection_history,
        v6.relational.learned_selection_history
    );
    assert!(v7.relational.outcome_applicability_history.is_empty());
    assert!(v7.relational.outcome_selection_history.is_empty());
}

#[test]
fn cf_lm_014_counterfactual_predictions_match_frozen_outcomes_without_mutating_actual_state() {
    let model = CohfieldLanguageModelV7::default();
    let trained = trained_substrate(&model);
    let before = trained.clone();

    let predicted_ab = model
        .predicted_consequence_signature(&trained, p_ab())
        .expect("P_AB counterfactual must be available");
    let predicted_bc = model
        .predicted_consequence_signature(&trained, p_bc())
        .expect("P_BC counterfactual must be available");

    for (actual, expected) in predicted_ab.iter().zip(Y_TRANSFER.iter()) {
        assert!((actual - expected).abs() < REGRESSION_TOL);
    }
    assert!(predicted_bc.iter().all(|value| value.abs() <= EPS_FLOOR));
    assert!(
        (euclidean5(predicted_ab, predicted_bc) - 0.026_575_098_283_946_105).abs() < REGRESSION_TOL
    );
    assert_eq!(trained, before);
}

#[test]
fn cf_lm_014_unlabeled_observed_consequences_infer_frozen_profiles_without_runtime_selection() {
    let model = CohfieldLanguageModelV7::default();
    let trained = trained_substrate(&model);
    let learned = train_outcome_applicability(&model);

    assert_eq!(learned.relational.outcome_applicability_history.len(), 4);
    assert_eq!(learned.relational.selected_profile, None);
    assert_eq!(learned.relational.sequential, trained.relational.sequential);
    assert_eq!(
        learned.relational.assessment_history,
        trained.relational.assessment_history
    );
    assert_eq!(
        learned
            .relational
            .outcome_applicability_history
            .iter()
            .map(|record| record.inferred_profile)
            .collect::<Vec<_>>(),
        vec![p_ab(), p_ab(), p_bc(), p_bc()]
    );

    for record in &learned.relational.outcome_applicability_history[..2] {
        assert!(error_for(&record.candidate_errors, p_ab()) <= EPS_FLOOR);
        assert!(
            (error_for(&record.candidate_errors, p_bc()) - 0.026_575_098_283_946_105).abs()
                < REGRESSION_TOL
        );
    }
    for record in &learned.relational.outcome_applicability_history[2..] {
        assert!(error_for(&record.candidate_errors, p_bc()) <= EPS_FLOOR);
        assert!(
            (error_for(&record.candidate_errors, p_ab()) - 0.026_575_098_283_946_105).abs()
                < REGRESSION_TOL
        );
    }
}

#[test]
fn cf_lm_014_outcome_derived_context_prototypes_match_preregistered_values() {
    let model = CohfieldLanguageModelV7::default();
    let learned = train_outcome_applicability(&model);
    let prototypes = model
        .outcome_applicability_prototypes(&learned)
        .expect("outcome-derived prototypes must be available");

    let ab = prototypes
        .iter()
        .find(|entry| entry.profile == p_ab())
        .expect("P_AB prototype must exist");
    let bc = prototypes
        .iter()
        .find(|entry| entry.profile == p_bc())
        .expect("P_BC prototype must exist");

    assert_eq!(ab.activity, [0.0, 0.0, 0.625, 0.375]);
    assert_eq!(bc.activity, [0.625, 0.0, 0.0, 0.375]);
}

#[test]
fn cf_lm_014_heldout_contexts_generalize_from_outcomes_and_invert_old_projection_heuristic() {
    let model = CohfieldLanguageModelV7::default();
    let learned = train_outcome_applicability(&model);

    let recognized_c = recognize(&model, &learned, &K_C);
    let activity_c = recognized_c
        .relational
        .context_history
        .last()
        .expect("K_C context must exist")
        .activity;
    let inferred_c = infer_outcome(&model, &recognized_c);
    let distances_c = &inferred_c
        .relational
        .outcome_selection_history
        .last()
        .expect("K_C selection must exist")
        .candidate_distances;
    assert_eq!(inferred_c.relational.selected_profile, Some(p_ab()));
    assert!((distance_for(distances_c, p_ab()) - 0.306_186_217_847_897_24).abs() < REGRESSION_TOL);
    assert!((distance_for(distances_c, p_bc()) - 0.847_791_247_890_658_5).abs() < REGRESSION_TOL);
    assert!(old_projection_score(p_ab(), activity_c) < old_projection_score(p_bc(), activity_c));

    let recognized_a = recognize(&model, &learned, &K_A);
    let activity_a = recognized_a
        .relational
        .context_history
        .last()
        .expect("K_A context must exist")
        .activity;
    let inferred_a = infer_outcome(&model, &recognized_a);
    let distances_a = &inferred_a
        .relational
        .outcome_selection_history
        .last()
        .expect("K_A selection must exist")
        .candidate_distances;
    assert_eq!(inferred_a.relational.selected_profile, Some(p_bc()));
    assert!((distance_for(distances_a, p_ab()) - 0.847_791_247_890_658_5).abs() < REGRESSION_TOL);
    assert!((distance_for(distances_a, p_bc()) - 0.306_186_217_847_897_24).abs() < REGRESSION_TOL);
    assert!(old_projection_score(p_bc(), activity_a) < old_projection_score(p_ab(), activity_a));
}

#[test]
fn cf_lm_014_midpoint_observed_consequence_fails_ambiguous_without_successor_state() {
    let model = CohfieldLanguageModelV7::default();
    let trained = trained_substrate(&model);
    let recognized = recognize(&model, &trained, &T_C1);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV7::RecordObservedConsequence(Y_MID),
    );

    assert_eq!(result, Err(LanguageErrorV7::AmbiguousOutcome));
    assert_eq!(recognized, before);
    assert!((euclidean5(Y_TRANSFER, Y_MID) - 0.013_287_549_141_973_052).abs() < REGRESSION_TOL);
    assert!((euclidean5(Y_ZERO, Y_MID) - 0.013_287_549_141_973_052).abs() < REGRESSION_TOL);
}

#[test]
fn cf_lm_014_far_observed_consequence_fails_unsupported_without_successor_state() {
    let model = CohfieldLanguageModelV7::default();
    let trained = trained_substrate(&model);
    let recognized = recognize(&model, &trained, &T_A1);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV7::RecordObservedConsequence(Y_FAR),
    );

    assert_eq!(result, Err(LanguageErrorV7::UnsupportedOutcome));
    assert_eq!(recognized, before);
}

#[test]
fn cf_lm_014_heldout_inference_without_outcome_applicability_experience_fails_closed() {
    let model = CohfieldLanguageModelV7::default();
    let trained = trained_substrate(&model);
    let recognized = recognize(&model, &trained, &K_C);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV7::InferConsequenceProfileFromOutcomeApplicability,
    );

    assert_eq!(
        result,
        Err(LanguageErrorV7::NoOutcomeApplicabilityExperience)
    );
    assert_eq!(recognized, before);
}

#[test]
fn cf_lm_014_outcome_grounded_applicability_controls_transfer_and_restores_it_without_relearning() {
    let model = CohfieldLanguageModelV7::default();
    let learned = train_outcome_applicability(&model);
    let sequential = learned.relational.sequential;
    let assessments = learned.relational.assessment_history.clone();
    let outcomes = learned.relational.outcome_applicability_history.clone();

    assert!(
        (sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()] - 0.557_984_402_843_442_6)
            .abs()
            < REGRESSION_TOL
    );
    assert!(sequential[SurfaceSymbol::D.index()][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);

    let k_c = infer_outcome(&model, &recognize(&model, &learned, &K_C));
    let first = probe(&model, &k_c, SurfaceSymbol::D, 4);
    assert!(
        (first[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs() < REGRESSION_TOL
    );

    let k_a = infer_outcome(&model, &recognize(&model, &k_c, &K_A));
    let middle = probe(&model, &k_a, SurfaceSymbol::D, 4);
    assert!(middle[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);

    let k_c_again = infer_outcome(&model, &recognize(&model, &k_a, &K_C));
    let restored = probe(&model, &k_c_again, SurfaceSymbol::D, 4);
    assert_eq!(first, restored);

    assert_eq!(k_c_again.relational.sequential, sequential);
    assert_eq!(k_c_again.relational.assessment_history, assessments);
    assert_eq!(k_c_again.relational.outcome_applicability_history, outcomes);
    assert!(k_c_again.relational.applicability_history.is_empty());
    assert!(k_c_again.relational.learned_selection_history.is_empty());
    assert!(k_c_again.relational.projection_selection_history.is_empty());
}

#[test]
fn cf_lm_014_consequence_grounded_provenance_and_full_cycle_are_deterministic() {
    type RunResult = (LanguageStateV7, Vec<[f64; 4]>, Vec<[f64; 4]>, Vec<[f64; 4]>);

    fn run() -> RunResult {
        let model = CohfieldLanguageModelV7::default();
        let learned = train_outcome_applicability(&model);
        let first_state = infer_outcome(&model, &recognize(&model, &learned, &K_C));
        let first = probe(&model, &first_state, SurfaceSymbol::D, 4);
        let second_state = infer_outcome(&model, &recognize(&model, &first_state, &K_A));
        let middle = probe(&model, &second_state, SurfaceSymbol::D, 4);
        let final_state = infer_outcome(&model, &recognize(&model, &second_state, &K_C));
        let final_response = probe(&model, &final_state, SurfaceSymbol::D, 4);
        (final_state, first, middle, final_response)
    }

    let first = run();
    let second = run();
    assert_eq!(first, second);

    let state = &first.0;
    assert_eq!(state.relational.outcome_applicability_history.len(), 4);
    assert_eq!(state.relational.outcome_selection_history.len(), 3);
    assert!(state
        .relational
        .outcome_applicability_history
        .iter()
        .all(|record| record.candidate_errors.len() == 2));
    assert!(state
        .relational
        .outcome_selection_history
        .iter()
        .all(|record| record.candidate_distances.len() == 2));
    assert!(state.relational.applicability_history.is_empty());
    assert!(state.relational.learned_selection_history.is_empty());
    assert!(state.relational.projection_selection_history.is_empty());
    assert_eq!(first.1, first.3);
    assert!(first.2[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
}
