use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageState, SurfaceSymbol,
};
use cohfield_lm::teacher_bridge::{CfLmTeacherBridgeV001, TeacherCurriculumV001};

const EPS: f64 = 1.0e-12;

#[test]
fn llm_authored_local_exposure_composes_after_teacher_removal() {
    let model = CohfieldLanguageModelV1::default();
    let bridge = CfLmTeacherBridgeV001;
    let curriculum = TeacherCurriculumV001::llm_authored();
    let trained = bridge
        .train(&model, &LanguageState::initial(), &curriculum)
        .expect("frozen LLM-authored curriculum must train");

    assert!(trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::B.index()] > 0.0);
    assert!(trained.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] > 0.0);
    assert!(trained.psi[SurfaceSymbol::C.index()][SurfaceSymbol::D.index()] > 0.0);
    assert!(trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()].abs() <= EPS);

    let probe = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 3)
        .expect("teacher-off probe must run");
    let d = probe
        .activation(3, SurfaceSymbol::D)
        .expect("step three must exist");
    assert!(d > 1.0e-4, "held-out three-hop D activation was {d}");
}

#[test]
fn no_adaptation_control_cannot_form_the_held_out_chain() {
    let model = CohfieldLanguageModelV1::without_adaptation();
    let bridge = CfLmTeacherBridgeV001;
    let trained = bridge
        .train(
            &model,
            &LanguageState::initial(),
            &TeacherCurriculumV001::llm_authored(),
        )
        .expect("control curriculum must execute");
    let probe = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 3)
        .expect("teacher-off control probe must run");
    assert!(probe.activation(3, SurfaceSymbol::D).unwrap().abs() <= EPS);
}

#[test]
fn surgical_middle_relation_ablation_collapses_three_hop_transfer() {
    let model = CohfieldLanguageModelV1::default();
    let bridge = CfLmTeacherBridgeV001;
    let mut trained = bridge
        .train(
            &model,
            &LanguageState::initial(),
            &TeacherCurriculumV001::llm_authored(),
        )
        .expect("frozen curriculum must train");
    let before = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 3)
        .expect("pre-ablation probe must run");
    assert!(before.activation(3, SurfaceSymbol::D).unwrap() > 1.0e-4);

    trained.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] = 0.0;
    let after = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 3)
        .expect("post-ablation probe must run");
    assert!(after.activation(3, SurfaceSymbol::D).unwrap().abs() <= EPS);
}

#[test]
fn teacher_off_probe_does_not_mutate_persistent_learning() {
    let model = CohfieldLanguageModelV1::default();
    let bridge = CfLmTeacherBridgeV001;
    let trained = bridge
        .train(
            &model,
            &LanguageState::initial(),
            &TeacherCurriculumV001::llm_authored(),
        )
        .expect("frozen curriculum must train");
    let before = trained.psi;
    let _ = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 3)
        .expect("teacher-off probe must run");
    assert_eq!(trained.psi, before);
}
