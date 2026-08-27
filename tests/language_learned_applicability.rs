use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::profiles::language_v2::{InternalEquivalenceProfile, LanguageStateV2};
use cohfield_lm::profiles::language_v3::LanguageStateV3;
use cohfield_lm::profiles::language_v4::{CohfieldLanguageModelV4, LanguageStateV4};
use cohfield_lm::profiles::language_v5::{
    CohfieldLanguageModelV5, LanguageExperienceV5, LanguageStateV5,
};
use cohfield_lm::profiles::language_v6::{
    CohfieldLanguageModelV6, LanguageErrorV6, LanguageExperienceV6, LanguageStateV6,
    ProfileApplicabilityDistanceV6,
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

const T_AB1: [SurfaceSymbol; 4] = [
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
];
const T_AB2: [SurfaceSymbol; 4] = [
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
];
const T_BC1: [SurfaceSymbol; 4] = [
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::D,
];
const T_BC2: [SurfaceSymbol; 4] = [
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
const K_NONE: [SurfaceSymbol; 2] = [SurfaceSymbol::B, SurfaceSymbol::B];
const K_TIE: [SurfaceSymbol; 16] = [
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::A,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
    SurfaceSymbol::D,
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

fn source_v5() -> LanguageStateV5 {
    let v4_model = CohfieldLanguageModelV4::default();
    let v4: LanguageStateV4 = v4_model
        .migrate_from_v3(&source_v3())
        .expect("unassessed V3 source must migrate to V4");
    CohfieldLanguageModelV5::default()
        .migrate_from_v4(&v4)
        .expect("unassessed V4 source must migrate to V5")
}

fn source_v6(model: &CohfieldLanguageModelV6) -> LanguageStateV6 {
    model
        .migrate_from_v5(&source_v5())
        .expect("unassessed V5 source must migrate to V6")
}

fn assess(
    model: &CohfieldLanguageModelV6,
    state: &LanguageStateV6,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV6 {
    model
        .adapt(
            state,
            &LanguageExperienceV6::AssessConsequenceEquivalence(profile),
        )
        .expect("frozen profile assessment must be valid")
}

fn assessed_both(model: &CohfieldLanguageModelV6) -> LanguageStateV6 {
    let source = source_v6(model);
    let after_ab = assess(model, &source, p_ab());
    assess(model, &after_ab, p_bc())
}

fn recognize(
    model: &CohfieldLanguageModelV6,
    state: &LanguageStateV6,
    cue: &[SurfaceSymbol],
) -> LanguageStateV6 {
    model
        .adapt(state, &LanguageExperienceV6::RecognizeContext(cue.to_vec()))
        .expect("frozen context recognition must be valid")
}

fn record_applicability(
    model: &CohfieldLanguageModelV6,
    state: &LanguageStateV6,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV6 {
    model
        .adapt(
            state,
            &LanguageExperienceV6::RecordContextApplicability(profile),
        )
        .expect("frozen applicability acquisition must be valid")
}

fn train_applicability(model: &CohfieldLanguageModelV6) -> LanguageStateV6 {
    let assessed = assessed_both(model);
    let t1 = recognize(model, &assessed, &T_AB1);
    let t1 = record_applicability(model, &t1, p_ab());
    let t2 = recognize(model, &t1, &T_AB2);
    let t2 = record_applicability(model, &t2, p_ab());
    let t3 = recognize(model, &t2, &T_BC1);
    let t3 = record_applicability(model, &t3, p_bc());
    let t4 = recognize(model, &t3, &T_BC2);
    record_applicability(model, &t4, p_bc())
}

fn infer(model: &CohfieldLanguageModelV6, state: &LanguageStateV6) -> LanguageStateV6 {
    model
        .adapt(
            state,
            &LanguageExperienceV6::InferConsequenceProfileFromLearnedApplicability,
        )
        .expect("held-out learned-applicability inference must be valid")
}

fn teach_c_to_a(model: &CohfieldLanguageModelV6, state: &LanguageStateV6) -> LanguageStateV6 {
    let mut next = state.clone();
    for _ in 0..EPISODES {
        next = model
            .expose(&next, &[SurfaceSymbol::C, SurfaceSymbol::A], 1)
            .expect("isolated C->A teaching episode must be valid");
    }
    next
}

fn probe(
    model: &CohfieldLanguageModelV6,
    state: &LanguageStateV6,
    symbol: SurfaceSymbol,
    continuation_steps: usize,
) -> Vec<[f64; 4]> {
    let mut local = LanguageStateV6::equalized_from(state);
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

fn old_projection_score(profile: InternalEquivalenceProfile, activity: [f64; 4]) -> f64 {
    profile
        .projection
        .iter()
        .map(|symbol| activity[symbol.index()])
        .sum()
}

fn last_distances(state: &LanguageStateV6) -> &[ProfileApplicabilityDistanceV6] {
    &state
        .relational
        .learned_selection_history
        .last()
        .expect("learned selection record must exist")
        .candidate_distances
}

fn distance_for(
    distances: &[ProfileApplicabilityDistanceV6],
    profile: InternalEquivalenceProfile,
) -> f64 {
    distances
        .iter()
        .find(|entry| entry.profile == profile)
        .expect("profile distance must exist")
        .distance
}

#[test]
fn cf_lm_013_v5_to_v6_migration_preserves_parent_state_and_starts_empty_applicability() {
    let v5_model = CohfieldLanguageModelV5::default();
    let source = source_v5();
    let after_ab = v5_model
        .adapt(
            &source,
            &LanguageExperienceV5::AssessConsequenceEquivalence(p_ab()),
        )
        .expect("V5 P_AB assessment must be valid");
    let after_bc = v5_model
        .adapt(
            &after_ab,
            &LanguageExperienceV5::AssessConsequenceEquivalence(p_bc()),
        )
        .expect("V5 P_BC assessment must be valid");
    let recognized = v5_model
        .adapt(
            &after_bc,
            &LanguageExperienceV5::RecognizeContext(K_A.to_vec()),
        )
        .expect("V5 context recognition must be valid");
    let v5 = v5_model
        .adapt(
            &recognized,
            &LanguageExperienceV5::InferConsequenceProfileFromContext,
        )
        .expect("V5 projection inference must be valid");

    let v6 = CohfieldLanguageModelV6::default()
        .migrate_from_v5(&v5)
        .expect("conforming V5 state must migrate");

    assert_eq!(v6.x, v5.x);
    assert_eq!(v6.theta, v5.theta);
    assert_eq!(v6.relational.sequential, v5.relational.sequential);
    assert_eq!(
        v6.relational.selected_profile,
        v5.relational.selected_profile
    );
    assert_eq!(
        v6.relational.assessment_history,
        v5.relational.assessment_history
    );
    assert_eq!(
        v6.relational.current_context_epoch,
        v5.relational.current_context_epoch
    );
    assert_eq!(v6.relational.context_history, v5.relational.context_history);
    assert_eq!(
        v6.relational.projection_selection_history,
        v5.relational.selection_history
    );
    assert!(v6.relational.applicability_history.is_empty());
    assert!(v6.relational.learned_selection_history.is_empty());
}

#[test]
fn cf_lm_013_applicability_acquisition_appends_four_records_without_selecting_or_mutating_substrate(
) {
    let model = CohfieldLanguageModelV6::default();
    let assessed = assessed_both(&model);
    let learned = train_applicability(&model);

    assert_eq!(learned.relational.applicability_history.len(), 4);
    assert_eq!(learned.relational.selected_profile, None);
    assert_eq!(learned.x, assessed.x);
    assert_eq!(learned.theta, assessed.theta);
    assert_eq!(
        learned.relational.sequential,
        assessed.relational.sequential
    );
    assert_eq!(
        learned.relational.assessment_history,
        assessed.relational.assessment_history
    );
    assert_eq!(
        learned
            .relational
            .applicability_history
            .iter()
            .map(|record| record.profile)
            .collect::<Vec<_>>(),
        vec![p_ab(), p_ab(), p_bc(), p_bc()]
    );
    assert_eq!(
        learned
            .relational
            .applicability_history
            .iter()
            .map(|record| record.epoch)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn cf_lm_013_derived_applicability_prototypes_match_preregistered_values() {
    let model = CohfieldLanguageModelV6::default();
    let learned = train_applicability(&model);
    let prototypes = model
        .applicability_prototypes(&learned)
        .expect("learned prototypes must be derivable");

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
fn cf_lm_013_heldout_k_c_selects_p_ab_and_inverts_old_projection_heuristic() {
    let model = CohfieldLanguageModelV6::default();
    let learned = train_applicability(&model);
    let recognized = recognize(&model, &learned, &K_C);
    let activity = recognized
        .relational
        .context_history
        .last()
        .expect("K_C context record must exist")
        .activity;
    let inferred = infer(&model, &recognized);

    assert_eq!(inferred.relational.selected_profile, Some(p_ab()));
    assert!(
        (distance_for(last_distances(&inferred), p_ab()) - 0.306_186_217_847_897_24).abs()
            < REGRESSION_TOL
    );
    assert!(
        (distance_for(last_distances(&inferred), p_bc()) - 0.847_791_247_890_658_5).abs()
            < REGRESSION_TOL
    );
    assert!(old_projection_score(p_ab(), activity) < old_projection_score(p_bc(), activity));
    assert_eq!(old_projection_score(p_ab(), activity), 0.25);
    assert_eq!(old_projection_score(p_bc(), activity), 0.75);
}

#[test]
fn cf_lm_013_heldout_k_a_selects_p_bc_and_inverts_old_projection_heuristic() {
    let model = CohfieldLanguageModelV6::default();
    let learned = train_applicability(&model);
    let recognized = recognize(&model, &learned, &K_A);
    let activity = recognized
        .relational
        .context_history
        .last()
        .expect("K_A context record must exist")
        .activity;
    let inferred = infer(&model, &recognized);

    assert_eq!(inferred.relational.selected_profile, Some(p_bc()));
    assert!(
        (distance_for(last_distances(&inferred), p_ab()) - 0.847_791_247_890_658_5).abs()
            < REGRESSION_TOL
    );
    assert!(
        (distance_for(last_distances(&inferred), p_bc()) - 0.306_186_217_847_897_24).abs()
            < REGRESSION_TOL
    );
    assert!(old_projection_score(p_bc(), activity) < old_projection_score(p_ab(), activity));
    assert_eq!(old_projection_score(p_ab(), activity), 0.75);
    assert_eq!(old_projection_score(p_bc(), activity), 0.25);
}

#[test]
fn cf_lm_013_inference_without_applicability_experience_fails_closed() {
    let model = CohfieldLanguageModelV6::default();
    let recognized = recognize(&model, &assessed_both(&model), &K_C);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV6::InferConsequenceProfileFromLearnedApplicability,
    );

    assert_eq!(result, Err(LanguageErrorV6::NoApplicabilityExperience));
    assert_eq!(recognized, before);
}

#[test]
fn cf_lm_013_ambiguous_midpoint_context_fails_closed() {
    let model = CohfieldLanguageModelV6::default();
    let learned = train_applicability(&model);
    let recognized = recognize(&model, &learned, &K_TIE);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV6::InferConsequenceProfileFromLearnedApplicability,
    );

    assert_eq!(result, Err(LanguageErrorV6::AmbiguousApplicability));
    assert_eq!(recognized, before);
}

#[test]
fn cf_lm_013_unsupported_context_fails_closed() {
    let model = CohfieldLanguageModelV6::default();
    let learned = train_applicability(&model);
    let recognized = recognize(&model, &learned, &K_NONE);
    let before = recognized.clone();
    let result = model.adapt(
        &recognized,
        &LanguageExperienceV6::InferConsequenceProfileFromLearnedApplicability,
    );

    assert_eq!(result, Err(LanguageErrorV6::UnsupportedApplicability));
    assert_eq!(recognized, before);
}

#[test]
fn cf_lm_013_learned_applicability_controls_transfer_and_restores_it_without_relearning() {
    let model = CohfieldLanguageModelV6::default();
    let learned = train_applicability(&model);
    let trained = teach_c_to_a(&model, &learned);

    assert_eq!(trained.relational.selected_profile, None);
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

    let k_c = infer(&model, &recognize(&model, &trained, &K_C));
    let first = probe(&model, &k_c, SurfaceSymbol::D, 4);
    assert!(
        (first[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs() < REGRESSION_TOL
    );

    let k_a = infer(&model, &recognize(&model, &k_c, &K_A));
    let middle = probe(&model, &k_a, SurfaceSymbol::D, 4);
    assert!(middle[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);

    let k_c_again = infer(&model, &recognize(&model, &k_a, &K_C));
    let restored = probe(&model, &k_c_again, SurfaceSymbol::D, 4);
    assert_eq!(first, restored);

    assert_eq!(k_c_again.relational.applicability_history.len(), 4);
    assert_eq!(k_c_again.relational.assessment_history.len(), 12);
    assert_eq!(
        k_c_again.relational.sequential,
        trained.relational.sequential
    );
    assert!(k_c_again.relational.projection_selection_history.is_empty());
}

#[test]
fn cf_lm_013_applicability_provenance_and_full_cycle_are_deterministic() {
    type RunResult = (LanguageStateV6, Vec<[f64; 4]>, Vec<[f64; 4]>, Vec<[f64; 4]>);

    fn run() -> RunResult {
        let model = CohfieldLanguageModelV6::default();
        let learned = train_applicability(&model);
        let trained = teach_c_to_a(&model, &learned);
        let first_state = infer(&model, &recognize(&model, &trained, &K_C));
        let first = probe(&model, &first_state, SurfaceSymbol::D, 4);
        let second_state = infer(&model, &recognize(&model, &first_state, &K_A));
        let middle = probe(&model, &second_state, SurfaceSymbol::D, 4);
        let final_state = infer(&model, &recognize(&model, &second_state, &K_C));
        let final_response = probe(&model, &final_state, SurfaceSymbol::D, 4);
        (final_state, first, middle, final_response)
    }

    let first = run();
    let second = run();
    assert_eq!(first, second);

    let state = &first.0;
    assert_eq!(state.relational.applicability_history.len(), 4);
    assert_eq!(state.relational.learned_selection_history.len(), 3);
    assert!(state
        .relational
        .learned_selection_history
        .iter()
        .all(|record| record.candidate_distances.len() == 2));
    assert!(state.relational.projection_selection_history.is_empty());
    assert_eq!(first.1, first.3);
    assert!(first.2[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
}
