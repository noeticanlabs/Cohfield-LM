use cohfield_lm::teacher_bridge_v003::{run as run_v3, Mechanism as V3Mechanism, S};
use cohfield_lm::teacher_bridge_v004::{run, V4Curriculum, V4Mechanism};

const EPS: f64 = 1.0e-12;
const POSITIVE_FLOOR: f64 = 1.0e-6;
const REGRESSION_TOL: f64 = 1.0e-9;

#[test]
fn matched_arms_share_training_state_and_withhold_b3_c3() {
    let curriculum = V4Curriculum::llm_authored();
    let plain = run(V4Mechanism::Plain, &curriculum);
    let binding = run(V4Mechanism::StructuralBinding, &curriculum);

    assert_eq!(plain.state, binding.state);
    assert!(plain.state.base.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
    assert!(plain.state.binding_gain > POSITIVE_FLOOR);
}

#[test]
fn v004_plain_runtime_matches_frozen_v003_plain_dynamics() {
    let curriculum = V4Curriculum::llm_authored();
    let v3 = run_v3(V3Mechanism::Plain, &curriculum);
    let v4 = run(V4Mechanism::Plain, &curriculum);

    assert_eq!(v3.state, v4.state.base);
    let p3 = v3.model.probe_teacher_off(&v3.state, S::B3, 3);
    let p4 = v4.model.probe_teacher_off(&v4.state, S::B3, 3);
    assert_eq!(p3.trajectory, p4.trajectory);
}

#[test]
fn plain_composition_cannot_cross_withheld_b3_c3_relation() {
    let curriculum = V4Curriculum::llm_authored();
    let r = run(V4Mechanism::Plain, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S::B3, 3);

    assert!(r.state.base.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
    for symbol in [S::C1, S::C2, S::C3] {
        for step in 0..=3 {
            assert!(probe.activation(step, symbol).unwrap().abs() <= EPS);
        }
    }
}

#[test]
fn structural_binding_selects_c3_without_storing_b3_c3() {
    let curriculum = V4Curriculum::llm_authored();
    let r = run(V4Mechanism::StructuralBinding, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S::B3, 3);

    assert!(r.state.base.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
    assert!(probe.activation(1, S::C3).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(2, S::C3).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(3, S::C3).unwrap() > POSITIVE_FLOOR);

    for symbol in [S::C1, S::C2] {
        for step in 0..=3 {
            assert!(
                probe.activation(step, symbol).unwrap().abs() <= EPS,
                "binding leaked into {symbol:?} at step {step}"
            );
        }
    }
}

#[test]
fn binding_gain_ablation_collapses_transfer_without_touching_psi() {
    let curriculum = V4Curriculum::llm_authored();
    let r = run(V4Mechanism::StructuralBinding, &curriculum);
    let before_psi = r.state.base.psi;
    let before_affinity = r.state.slot_affinity;

    let mut ablated = r.state.clone();
    ablated.binding_gain = 0.0;
    let probe = r.model.probe_teacher_off(&ablated, S::B3, 3);

    assert!(probe.activation(3, S::C3).unwrap().abs() <= EPS);
    assert_eq!(ablated.base.psi, before_psi);
    assert_eq!(ablated.slot_affinity, before_affinity);
}

#[test]
fn third_slot_affinity_ablation_collapses_only_structural_route() {
    let curriculum = V4Curriculum::llm_authored();
    let r = run(V4Mechanism::StructuralBinding, &curriculum);
    let before_psi = r.state.base.psi;
    let before_gain = r.state.binding_gain;

    let mut ablated = r.state.clone();
    ablated.slot_affinity[S::B3.index()][S::C3.index()] = 0.0;
    let probe = r.model.probe_teacher_off(&ablated, S::B3, 3);

    assert!(probe.activation(3, S::C3).unwrap().abs() <= EPS);
    assert_eq!(ablated.base.psi, before_psi);
    assert_eq!(ablated.binding_gain, before_gain);
}

#[test]
fn swapping_third_slot_affinity_moves_response_to_wrong_target() {
    let curriculum = V4Curriculum::llm_authored();
    let r = run(V4Mechanism::StructuralBinding, &curriculum);
    let mut swapped = r.state.clone();

    let c2 = swapped.slot_affinity[S::B3.index()][S::C2.index()];
    let c3 = swapped.slot_affinity[S::B3.index()][S::C3.index()];
    swapped.slot_affinity[S::B3.index()][S::C2.index()] = c3;
    swapped.slot_affinity[S::B3.index()][S::C3.index()] = c2;

    let probe = r.model.probe_teacher_off(&swapped, S::B3, 2);
    assert!(probe.activation(1, S::C2).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(1, S::C3).unwrap().abs() <= EPS);
}

#[test]
fn anchors_without_visible_b_to_c_examples_do_not_create_a_schema() {
    let curriculum = V4Curriculum::anchors_only();
    let r = run(V4Mechanism::StructuralBinding, &curriculum);

    assert!((r.state.slot_affinity[S::B3.index()][S::C3.index()] - 1.0).abs() <= EPS);
    assert!(r.state.binding_gain.abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S::B3, 2);
    assert!(probe.activation(2, S::C3).unwrap().abs() <= EPS);
}

#[test]
fn visible_schema_without_third_anchor_cannot_identify_c3() {
    let curriculum = V4Curriculum::without_third_target_anchor();
    let r = run(V4Mechanism::StructuralBinding, &curriculum);

    assert!(r.state.binding_gain > POSITIVE_FLOOR);
    assert!(r.state.slot_affinity[S::B3.index()][S::C3.index()].abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S::B3, 2);
    assert!(probe.activation(2, S::C3).unwrap().abs() <= EPS);
}

#[test]
fn teacher_off_probe_is_nonmutating_and_deterministic() {
    let curriculum = V4Curriculum::llm_authored();
    for mechanism in [V4Mechanism::Plain, V4Mechanism::StructuralBinding] {
        let a = run(mechanism, &curriculum);
        let b = run(mechanism, &curriculum);
        let before = a.state.clone();
        let pa = a.model.probe_teacher_off(&a.state, S::B3, 3);
        let pb = b.model.probe_teacher_off(&b.state, S::B3, 3);
        assert_eq!(a.state, before);
        assert_eq!(a.state, b.state);
        assert_eq!(pa, pb);
    }
}

#[test]
fn frozen_exact_v004_diagnostics_are_stable() {
    let curriculum = V4Curriculum::llm_authored();
    let r = run(V4Mechanism::StructuralBinding, &curriculum);

    assert!(
        (r.state.base.psi[S::A3.index()][S::B3.index()] - 0.236_659_250_689_321_5).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (r.state.base.psi[S::A3.index()][S::C3.index()] - 0.267_157_290_960_156_67).abs()
            <= REGRESSION_TOL
    );
    assert!(r.state.base.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
    assert!((r.state.binding_gain - 0.283_907_866_679_987_8).abs() <= REGRESSION_TOL);
    assert!(
        (r.state.slot_affinity[S::B1.index()][S::C1.index()] - 0.663_088_975_707_497_4).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (r.state.slot_affinity[S::B2.index()][S::C2.index()] - 0.663_088_975_707_497_5).abs()
            <= REGRESSION_TOL
    );
    assert!((r.state.slot_affinity[S::B3.index()][S::C3.index()] - 1.0).abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S::B3, 3);
    assert!((probe.activation(1, S::C3).unwrap() - 0.028_390_786_667_998_782).abs() <= REGRESSION_TOL);
    assert!((probe.activation(2, S::C3).unwrap() - 0.028_390_786_667_998_782).abs() <= REGRESSION_TOL);
    assert!((probe.activation(3, S::C3).unwrap() - 0.021_293_090_000_999_087).abs() <= REGRESSION_TOL);
}
