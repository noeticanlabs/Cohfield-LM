use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::profiles::language_v2::{InternalEquivalenceProfile, LanguageStateV2};
use cohfield_lm::profiles::language_v3::LanguageStateV3;
use cohfield_lm::profiles::language_v4::{CohfieldLanguageModelV4, LanguageStateV4};
use cohfield_lm::profiles::language_v5::{CohfieldLanguageModelV5, LanguageStateV5};
use cohfield_lm::profiles::language_v6::{CohfieldLanguageModelV6, LanguageStateV6};
use cohfield_lm::profiles::language_v7::{
    CohfieldLanguageModelV7, LanguageExperienceV7, LanguageStateV7,
};
use cohfield_lm::profiles::language_v8::{
    CohfieldLanguageModelV8, DerivedAbstractionIdentityV8, LanguageErrorV8, LanguageExperienceV8,
    LanguageStateV8,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const REGRESSION_TOL: f64 = 1.0e-9;
const ADAPT_EVENTS: usize = 8;

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

fn source_v7() -> LanguageStateV7 {
    let v4_model = CohfieldLanguageModelV4::default();
    let v4: LanguageStateV4 = v4_model
        .migrate_from_v3(&source_v3())
        .expect("unassessed V3 source must migrate to V4");
    let v5: LanguageStateV5 = CohfieldLanguageModelV5::default()
        .migrate_from_v4(&v4)
        .expect("V4 source must migrate to V5");
    let v6: LanguageStateV6 = CohfieldLanguageModelV6::default()
        .migrate_from_v5(&v5)
        .expect("V5 source must migrate to V6");
    CohfieldLanguageModelV7::default()
        .migrate_from_v6(&v6)
        .expect("V6 source must migrate to V7")
}

fn source_v8(model: &CohfieldLanguageModelV8) -> LanguageStateV8 {
    model
        .migrate_from_v7(&source_v7())
        .expect("V7 source must migrate to V8")
}

fn assess(
    model: &CohfieldLanguageModelV8,
    state: &LanguageStateV8,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV8 {
    model
        .adapt(
            state,
            &LanguageExperienceV8::Parent(LanguageExperienceV7::AssessConsequenceEquivalence(
                profile,
            )),
        )
        .expect("frozen profile assessment must be valid")
}

fn form(
    model: &CohfieldLanguageModelV8,
    state: &LanguageStateV8,
    profile: InternalEquivalenceProfile,
) -> LanguageStateV8 {
    model
        .adapt(
            state,
            &LanguageExperienceV8::FormDerivedAbstractions(profile),
        )
        .expect("frozen derived-abstraction formation must be valid")
}

fn alpha_cd(state: &LanguageStateV8) -> DerivedAbstractionIdentityV8 {
    *state
        .relational
        .derived_abstractions
        .first()
        .expect("C/D abstraction must exist")
}

fn activate(
    model: &CohfieldLanguageModelV8,
    state: &LanguageStateV8,
    identity: DerivedAbstractionIdentityV8,
) -> LanguageStateV8 {
    model
        .adapt(
            state,
            &LanguageExperienceV8::ActivateDerivedAbstraction(identity),
        )
        .expect("known derived abstraction must activate")
}

fn train_c_to_a(model: &CohfieldLanguageModelV8, state: &LanguageStateV8) -> LanguageStateV8 {
    let mut next = state.clone();
    for _ in 0..ADAPT_EVENTS {
        next = model
            .adapt(
                &next,
                &LanguageExperienceV8::Parent(LanguageExperienceV7::Sequential {
                    predecessor: Some(SurfaceSymbol::C),
                    current: SurfaceSymbol::A,
                }),
            )
            .expect("direct C->A adaptation event must be valid");
    }
    next
}

fn probe_a(model: &CohfieldLanguageModelV8, state: &LanguageStateV8) -> Vec<f64> {
    let mut local = LanguageStateV8::equalized_from(state);
    local = model
        .evolve(&local, &LanguageInput::symbol(SurfaceSymbol::D), 1.0)
        .expect("D probe drive must be valid");
    let mut out = vec![local.x[SurfaceSymbol::A.index()]];
    for _ in 0..4 {
        local = model
            .evolve(&local, &LanguageInput::zero(), 1.0)
            .expect("D probe continuation must be valid");
        out.push(local.x[SurfaceSymbol::A.index()]);
    }
    out
}

#[test]
fn cf_lm_015_v7_to_v8_migration_preserves_parent_and_starts_without_abstractions() {
    let model = CohfieldLanguageModelV8::default();
    let v7 = source_v7();
    let v8 = model
        .migrate_from_v7(&v7)
        .expect("conforming V7 state must migrate");

    assert_eq!(v8.x, v7.x);
    assert_eq!(v8.theta, v7.theta);
    assert_eq!(v8.relational.parent, v7.relational);
    assert!(v8.relational.derived_abstractions.is_empty());
    assert!(v8.relational.abstraction_formation_history.is_empty());
    assert!(v8.relational.abstraction_relations.is_empty());
    assert_eq!(v8.relational.active_derived_abstraction, None);
}

#[test]
fn cf_lm_015_p_ab_forms_exactly_one_cd_abstraction_without_substrate_mutation() {
    let model = CohfieldLanguageModelV8::default();
    let assessed = assess(&model, &source_v8(&model), p_ab());
    let formed = form(&model, &assessed, p_ab());

    assert_eq!(formed.x, assessed.x);
    assert_eq!(formed.theta, assessed.theta);
    assert_eq!(formed.relational.parent, assessed.relational.parent);
    assert_eq!(formed.relational.derived_abstractions.len(), 1);
    assert_eq!(
        alpha_cd(&formed),
        DerivedAbstractionIdentityV8 {
            profile: p_ab(),
            members: [false, false, true, true],
        }
    );
    assert_eq!(formed.relational.abstraction_formation_history.len(), 1);
    assert_eq!(
        formed.relational.abstraction_formation_history[0].source_assessment_epoch,
        1
    );
    assert_eq!(formed.relational.parent.selected_profile, None);
}

#[test]
fn cf_lm_015_same_assessment_reformation_is_fully_idempotent() {
    let model = CohfieldLanguageModelV8::default();
    let assessed = assess(&model, &source_v8(&model), p_ab());
    let formed = form(&model, &assessed, p_ab());
    let repeated = form(&model, &formed, p_ab());

    assert_eq!(repeated, formed);
}

#[test]
fn cf_lm_015_reassessment_preserves_semantic_identity_and_appends_provenance() {
    let model = CohfieldLanguageModelV8::default();
    let assessed = assess(&model, &source_v8(&model), p_ab());
    let formed = form(&model, &assessed, p_ab());
    let identity = alpha_cd(&formed);
    let reassessed = assess(&model, &formed, p_ab());
    let reformed = form(&model, &reassessed, p_ab());

    assert_eq!(reformed.relational.derived_abstractions, vec![identity]);
    assert_eq!(reformed.relational.abstraction_formation_history.len(), 2);
    assert_eq!(
        reformed
            .relational
            .abstraction_formation_history
            .iter()
            .map(|record| record.source_assessment_epoch)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert!(reformed
        .relational
        .abstraction_formation_history
        .iter()
        .all(|record| record.abstraction == identity));
}

#[test]
fn cf_lm_015_p_bc_with_no_nontrivial_class_fails_closed() {
    let model = CohfieldLanguageModelV8::default();
    let after_ab = assess(&model, &source_v8(&model), p_ab());
    let after_bc = assess(&model, &after_ab, p_bc());
    let before = after_bc.clone();
    let result = model.adapt(
        &after_bc,
        &LanguageExperienceV8::FormDerivedAbstractions(p_bc()),
    );

    assert_eq!(result, Err(LanguageErrorV8::NoNontrivialAbstraction));
    assert_eq!(after_bc, before);
}

#[test]
fn cf_lm_015_member_experience_learns_relation_to_derived_abstraction() {
    let model = CohfieldLanguageModelV8::default();
    let assessed = assess(&model, &source_v8(&model), p_ab());
    let formed = form(&model, &assessed, p_ab());
    let identity = alpha_cd(&formed);
    let trained = train_c_to_a(&model, &formed);
    let expected = 0.596_947_909_672_857_5;

    assert!(
        (trained.relational.parent.sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()]
            - expected)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        trained.relational.parent.sequential[SurfaceSymbol::D.index()][SurfaceSymbol::A.index()]
            .abs()
            <= EPS_FLOOR
    );
    assert!(
        (model
            .learned_abstraction_relation(&trained, identity, SurfaceSymbol::A)
            .expect("known abstraction relation must be readable")
            - expected)
            .abs()
            < REGRESSION_TOL
    );
    assert_eq!(trained.relational.parent.selected_profile, None);
}

#[test]
fn cf_lm_015_active_derived_abstraction_mediates_frozen_d_to_a_trajectory() {
    let model = CohfieldLanguageModelV8::default();
    let assessed = assess(&model, &source_v8(&model), p_ab());
    let formed = form(&model, &assessed, p_ab());
    let identity = alpha_cd(&formed);
    let trained = train_c_to_a(&model, &formed);
    let active = activate(&model, &trained, identity);
    let trajectory = probe_a(&model, &active);
    // Prediction-model correction (documented in docs/CF-LM-015_IMPLEMENTATION.md):
    // the original preregistered trajectory assumed x_C == 0 after activation, i.e.
    // the derived abstraction behaves as an isolated auxiliary channel
    // D -> α -> A. The full coupled substrate instead feeds A's newly generated
    // activity back into member C via the A->C route (seq[A][C] = 0.9840816505),
    // so x_C > 0 from the second continuation step onward; that fed-back C activity
    // then re-enters A both directly (C->A learned edge) and via the abstraction
    // activation a_alpha = (x_C + x_D)/2. Steps 0-2, where no member feedback has
    // arrived yet, are unchanged; steps 3-4 reflect the full coupled dynamics.
    let expected = [
        0.0,
        0.029_847_395_483_642_875,
        0.029_847_395_483_642_875,
        0.023_578_909_705_625_22,
        0.017_310_423_927_607_566,
    ];

    assert_eq!(active.relational.parent.selected_profile, None);
    for (actual, expected) in trajectory.iter().zip(expected.iter()) {
        assert!((*actual - *expected).abs() < REGRESSION_TOL);
    }
}

#[test]
fn cf_lm_015_same_learning_without_derived_abstraction_does_not_transfer() {
    let model = CohfieldLanguageModelV8::default();
    let assessed = assess(&model, &source_v8(&model), p_ab());
    let trained = train_c_to_a(&model, &assessed);
    let trajectory = probe_a(&model, &trained);

    assert!(trajectory.iter().all(|value| value.abs() <= EPS_FLOOR));
    assert!(trained.relational.abstraction_relations.is_empty());
}

#[test]
fn cf_lm_015_active_abstraction_without_learned_outgoing_relation_does_not_transfer() {
    let model = CohfieldLanguageModelV8::default();
    let assessed = assess(&model, &source_v8(&model), p_ab());
    let formed = form(&model, &assessed, p_ab());
    let active = activate(&model, &formed, alpha_cd(&formed));
    let trajectory = probe_a(&model, &active);

    assert!(trajectory.iter().all(|value| value.abs() <= EPS_FLOOR));
    assert!(active.relational.abstraction_relations.is_empty());
}

#[test]
fn cf_lm_015_surgical_abstraction_relation_ablation_collapses_transfer_and_is_deterministic() {
    type RunResult = (LanguageStateV8, Vec<f64>, Vec<f64>);

    fn run() -> RunResult {
        let model = CohfieldLanguageModelV8::default();
        let assessed = assess(&model, &source_v8(&model), p_ab());
        let formed = form(&model, &assessed, p_ab());
        let identity = alpha_cd(&formed);
        let trained = train_c_to_a(&model, &formed);
        let active = activate(&model, &trained, identity);
        let before = probe_a(&model, &active);

        let mut ablated = active.clone();
        let relation = ablated
            .relational
            .abstraction_relations
            .iter_mut()
            .find(|relation| {
                relation.abstraction == identity && relation.target == SurfaceSymbol::A
            })
            .expect("learned abstraction->A relation must exist");
        relation.weight = 0.0;
        let after = probe_a(&model, &ablated);

        assert_eq!(
            ablated.relational.derived_abstractions,
            active.relational.derived_abstractions
        );
        assert_eq!(
            ablated.relational.abstraction_formation_history,
            active.relational.abstraction_formation_history
        );
        assert_eq!(
            ablated.relational.parent.sequential,
            active.relational.parent.sequential
        );
        assert!(
            (ablated.relational.parent.sequential[SurfaceSymbol::C.index()]
                [SurfaceSymbol::A.index()]
                - 0.596_947_909_672_857_5)
                .abs()
                < REGRESSION_TOL
        );
        assert!(
            ablated.relational.parent.sequential[SurfaceSymbol::D.index()]
                [SurfaceSymbol::A.index()]
            .abs()
                <= EPS_FLOOR
        );
        assert_eq!(ablated.relational.parent.selected_profile, None);
        assert!(after.iter().all(|value| value.abs() <= EPS_FLOOR));

        (ablated, before, after)
    }

    let first = run();
    let second = run();
    assert_eq!(first, second);
    assert!(first.1[1] > 0.025);
}
