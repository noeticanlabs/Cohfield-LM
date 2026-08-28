use cohfield_lm::profiles::language::{CohfieldLanguageModelV1, LanguageState, SurfaceSymbol};
use cohfield_lm::teacher_bridge_v002::{CfLmTeacherBridgeV002, TeacherCurriculumV002};

const EPS: f64 = 1.0e-12;
const HELD_OUT_FLOOR: f64 = 1.0e-4;

fn train_default() -> LanguageState {
    let model = CohfieldLanguageModelV1::default();
    CfLmTeacherBridgeV002
        .train(
            &model,
            &LanguageState::initial(),
            &TeacherCurriculumV002::llm_authored_branching(),
        )
        .expect("frozen LLM-authored branching curriculum must train")
}

#[test]
fn withheld_two_hop_combinations_compose_after_teacher_removal() {
    let model = CohfieldLanguageModelV1::default();
    let bridge = CfLmTeacherBridgeV002;
    let trained = train_default();

    // Taught local relations exist.
    assert!(trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::B.index()] > 0.0);
    assert!(trained.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] > 0.0);
    assert!(trained.psi[SurfaceSymbol::B.index()][SurfaceSymbol::D.index()] > 0.0);

    // Withheld combinations were never stored as direct relations.
    assert!(trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()].abs() <= EPS);
    assert!(trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()].abs() <= EPS);
    // The v0.01 edge C->D is deliberately absent from the v0.02 curriculum.
    assert!(trained.psi[SurfaceSymbol::C.index()][SurfaceSymbol::D.index()].abs() <= EPS);

    let probe = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 2)
        .expect("teacher-off probe must run");
    let c = probe
        .activation(2, SurfaceSymbol::C)
        .expect("step two must exist");
    let d = probe
        .activation(2, SurfaceSymbol::D)
        .expect("step two must exist");
    assert!(c > HELD_OUT_FLOOR, "withheld A->C activation was {c}");
    assert!(d > HELD_OUT_FLOOR, "withheld A->D activation was {d}");
}

#[test]
fn structurally_underivable_pairs_stay_silent() {
    let model = CohfieldLanguageModelV1::default();
    let bridge = CfLmTeacherBridgeV002;
    let trained = train_default();

    // No taught episode ever targets A, so nothing can derive into A.
    for source in [SurfaceSymbol::B, SurfaceSymbol::C, SurfaceSymbol::D] {
        assert!(
            trained.psi[source.index()][SurfaceSymbol::A.index()].abs() <= EPS,
            "an underivable relation into A was learned from {source:?}"
        );
    }

    // C has no outgoing taught relation and no derivable continuation.
    for target in [SurfaceSymbol::A, SurfaceSymbol::B, SurfaceSymbol::D] {
        assert!(
            trained.psi[SurfaceSymbol::C.index()][target.index()].abs() <= EPS,
            "an underivable relation out of C to {target:?} was learned"
        );
    }

    let probe = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::C, 2)
        .expect("teacher-off probe from C must run");
    for step in 0..=2 {
        for symbol in [SurfaceSymbol::A, SurfaceSymbol::B, SurfaceSymbol::D] {
            let activation = probe.activation(step, symbol).expect("step must exist");
            assert!(
                activation.abs() <= EPS,
                "underivable {symbol:?} activation was {activation} at step {step}"
            );
        }
    }
}

#[test]
fn no_adaptation_control_cannot_form_withheld_combinations() {
    let model = CohfieldLanguageModelV1::without_adaptation();
    let bridge = CfLmTeacherBridgeV002;
    let trained = bridge
        .train(
            &model,
            &LanguageState::initial(),
            &TeacherCurriculumV002::llm_authored_branching(),
        )
        .expect("control curriculum must execute");
    let probe = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 2)
        .expect("teacher-off control probe must run");
    assert!(probe.activation(2, SurfaceSymbol::C).unwrap().abs() <= EPS);
    assert!(probe.activation(2, SurfaceSymbol::D).unwrap().abs() <= EPS);
}

#[test]
fn teacher_off_probe_does_not_mutate_persistent_learning() {
    let model = CohfieldLanguageModelV1::default();
    let bridge = CfLmTeacherBridgeV002;
    let trained = train_default();
    let before = trained.psi;
    let _ = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 2)
        .expect("teacher-off probe must run");
    assert_eq!(trained.psi, before);
}

#[test]
fn frozen_v002_curriculum_and_teacher_off_trajectory_regressions() {
    const REGRESSION_TOL: f64 = 1.0e-9;
    let model = CohfieldLanguageModelV1::default();
    let bridge = CfLmTeacherBridgeV002;
    let trained = train_default();

    assert!(
        (trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::B.index()] - 0.646_105_948_108_114_1)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        (trained.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] - 0.672_746_718_146_724_4)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        (trained.psi[SurfaceSymbol::B.index()][SurfaceSymbol::D.index()] - 0.700_485_962_251_899_6)
            .abs()
            < REGRESSION_TOL
    );
    assert!(trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()].abs() <= EPS);
    assert!(trained.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()].abs() <= EPS);

    let probe = bridge
        .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 2)
        .expect("teacher-off probe must run");
    let step_two = probe.trajectory[2];
    assert!(
        (step_two[SurfaceSymbol::C.index()] - 0.008_693_313_123_296_232).abs() < REGRESSION_TOL
    );
    assert!(
        (step_two[SurfaceSymbol::D.index()] - 0.009_051_762_935_543_765).abs() < REGRESSION_TOL
    );
}

#[test]
fn double_dissociation_ablation_localizes_each_withheld_combination() {
    type RunResult = (LanguageState, Vec<[f64; 4]>, Vec<[f64; 4]>, Vec<[f64; 4]>);

    fn run() -> RunResult {
        let model = CohfieldLanguageModelV1::default();
        let bridge = CfLmTeacherBridgeV002;
        let trained = train_default();
        let intact = bridge
            .probe_teacher_off(&model, &trained, SurfaceSymbol::A, 2)
            .expect("intact probe must run");

        // Ablate only the C-route edge.
        let mut c_ablated_state = trained.clone();
        c_ablated_state.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] = 0.0;
        let c_ablated = bridge
            .probe_teacher_off(&model, &c_ablated_state, SurfaceSymbol::A, 2)
            .expect("C-ablated probe must run");

        // Ablate only the D-route edge on a fresh copy.
        let mut d_ablated_state = trained.clone();
        d_ablated_state.psi[SurfaceSymbol::B.index()][SurfaceSymbol::D.index()] = 0.0;
        let d_ablated = bridge
            .probe_teacher_off(&model, &d_ablated_state, SurfaceSymbol::A, 2)
            .expect("D-ablated probe must run");

        (
            trained,
            intact.trajectory,
            c_ablated.trajectory,
            d_ablated.trajectory,
        )
    }

    fn activation(trajectory: &[[f64; 4]], symbol: SurfaceSymbol) -> f64 {
        trajectory[2][symbol.index()]
    }

    let first = run();
    let second = run();
    assert_eq!(first, second, "probe outcomes must be deterministic");

    let intact_c = activation(&first.1, SurfaceSymbol::C);
    let intact_d = activation(&first.1, SurfaceSymbol::D);
    let c_ablated_c = activation(&first.2, SurfaceSymbol::C);
    let c_ablated_d = activation(&first.2, SurfaceSymbol::D);
    let d_ablated_c = activation(&first.3, SurfaceSymbol::C);
    let d_ablated_d = activation(&first.3, SurfaceSymbol::D);

    // Sanity: the trained B->C and B->D edges are distinct and both nonzero.
    assert!(
        first.0.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] > 0.0,
        "B->C relation must be learned"
    );
    assert!(
        first.0.psi[SurfaceSymbol::B.index()][SurfaceSymbol::D.index()] > 0.0,
        "B->D relation must be learned"
    );
    assert!(intact_c > HELD_OUT_FLOOR);
    assert!(intact_d > HELD_OUT_FLOOR);
    // Ablating B->C collapses the withheld A->C reading but not A->D.
    assert!(c_ablated_c.abs() <= EPS);
    assert!(c_ablated_d > HELD_OUT_FLOOR);
    // Ablating B->D collapses the withheld A->D reading but not A->C.
    assert!(d_ablated_c > HELD_OUT_FLOOR);
    assert!(d_ablated_d.abs() <= EPS);
}
