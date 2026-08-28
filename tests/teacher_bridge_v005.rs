use cohfield_lm::teacher_bridge_v005::{run, S5, V5Curriculum, V5Mechanism};

const EPS: f64 = 1.0e-12;
const POSITIVE_FLOOR: f64 = 1.0e-6;
const REGRESSION_TOL: f64 = 1.0e-9;

#[test]
fn matched_arms_share_training_state_and_withhold_b3_c3() {
    let curriculum = V5Curriculum::llm_authored();
    let plain = run(V5Mechanism::Plain, &curriculum);
    let binding = run(V5Mechanism::DiscoveredBinding, &curriculum);

    assert_eq!(plain.state, binding.state);
    assert!(plain.state.psi[S5::B3.index()][S5::C3.index()].abs() <= EPS);
}

#[test]
fn role_sets_are_discovered_from_relation_neighborhoods() {
    let curriculum = V5Curriculum::llm_authored();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);

    assert_eq!(r.state.structure.discovered_roles.len(), 2);
    let source = r.state.structure.source_role().unwrap();
    let target = r.state.structure.target_role().unwrap();

    assert_eq!(source.anchor, S5::R1);
    assert_eq!(target.anchor, S5::R2);
    assert_eq!(source.member_count(), 3);
    assert_eq!(target.member_count(), 3);
    for symbol in [S5::B1, S5::B2, S5::B3] {
        assert!(source.contains(symbol));
    }
    for symbol in [S5::C1, S5::C2, S5::C3] {
        assert!(target.contains(symbol));
    }
}

#[test]
fn plain_composition_cannot_cross_withheld_relation() {
    let curriculum = V5Curriculum::llm_authored();
    let r = run(V5Mechanism::Plain, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S5::B3, 3);

    assert!(r.state.psi[S5::B3.index()][S5::C3.index()].abs() <= EPS);
    for symbol in [S5::C1, S5::C2, S5::C3] {
        for step in 0..=3 {
            assert!(probe.activation(step, symbol).unwrap().abs() <= EPS);
        }
    }
}

#[test]
fn discovered_binding_selects_c3_without_direct_edge() {
    let curriculum = V5Curriculum::llm_authored();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S5::B3, 3);

    assert!(r.state.psi[S5::B3.index()][S5::C3.index()].abs() <= EPS);
    for step in 1..=3 {
        assert!(probe.activation(step, S5::C3).unwrap() > POSITIVE_FLOOR);
        assert!(probe.activation(step, S5::C1).unwrap().abs() <= EPS);
        assert!(probe.activation(step, S5::C2).unwrap().abs() <= EPS);
    }
}

#[test]
fn removing_role_anchor_experience_prevents_role_discovery_and_transfer() {
    let curriculum = V5Curriculum::without_role_anchors();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);

    assert!(r.state.structure.discovered_roles.is_empty());
    assert!(r.state.structure.source_role().is_none());
    assert!(r.state.structure.target_role().is_none());
    assert!(r.state.structure.binding_gain.abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S5::B3, 2);
    assert!(probe.activation(2, S5::C3).unwrap().abs() <= EPS);
}

#[test]
fn discovered_roles_without_visible_cross_role_schema_do_not_transfer() {
    let curriculum = V5Curriculum::without_schema_examples();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);

    assert_eq!(r.state.structure.discovered_roles.len(), 2);
    assert!(r.state.structure.source_role().is_none());
    assert!(r.state.structure.target_role().is_none());
    assert!(r.state.structure.binding_gain.abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S5::B3, 2);
    assert!(probe.activation(2, S5::C3).unwrap().abs() <= EPS);
}

#[test]
fn schema_without_third_correspondence_anchor_cannot_select_c3() {
    let curriculum = V5Curriculum::without_third_target_anchor();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);

    assert!(r.state.structure.source_role().is_some());
    assert!(r.state.structure.target_role().is_some());
    assert!(r.state.structure.binding_gain > POSITIVE_FLOOR);
    assert!(
        r.state.structure.slot_affinity[S5::B3.index()][S5::C3.index()].abs() <= EPS
    );

    let probe = r.model.probe_teacher_off(&r.state, S5::B3, 2);
    assert!(probe.activation(2, S5::C3).unwrap().abs() <= EPS);
}

#[test]
fn swapping_only_third_correspondence_moves_transfer_to_c2() {
    let curriculum = V5Curriculum::with_swapped_third_correspondence();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);

    assert!(
        r.state.structure.slot_affinity[S5::B3.index()][S5::C2.index()] > POSITIVE_FLOOR
    );
    assert!(
        r.state.structure.slot_affinity[S5::B3.index()][S5::C3.index()].abs() <= EPS
    );

    let probe = r.model.probe_teacher_off(&r.state, S5::B3, 2);
    assert!(probe.activation(1, S5::C2).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(1, S5::C3).unwrap().abs() <= EPS);
}

#[test]
fn binding_gain_ablation_collapses_transfer_without_touching_psi_or_roles() {
    let curriculum = V5Curriculum::llm_authored();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);
    let before_psi = r.state.psi;
    let before_roles = r.state.structure.discovered_roles.clone();
    let before_affinity = r.state.structure.slot_affinity;

    let mut ablated = r.state.clone();
    ablated.structure.binding_gain = 0.0;
    let probe = r.model.probe_teacher_off(&ablated, S5::B3, 2);

    assert!(probe.activation(2, S5::C3).unwrap().abs() <= EPS);
    assert_eq!(ablated.psi, before_psi);
    assert_eq!(ablated.structure.discovered_roles, before_roles);
    assert_eq!(ablated.structure.slot_affinity, before_affinity);
}

#[test]
fn third_slot_affinity_ablation_collapses_only_correspondence_route() {
    let curriculum = V5Curriculum::llm_authored();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);
    let before_psi = r.state.psi;
    let before_gain = r.state.structure.binding_gain;
    let before_roles = r.state.structure.discovered_roles.clone();

    let mut ablated = r.state.clone();
    ablated.structure.slot_affinity[S5::B3.index()][S5::C3.index()] = 0.0;
    let probe = r.model.probe_teacher_off(&ablated, S5::B3, 2);

    assert!(probe.activation(2, S5::C3).unwrap().abs() <= EPS);
    assert_eq!(ablated.psi, before_psi);
    assert_eq!(ablated.structure.binding_gain, before_gain);
    assert_eq!(ablated.structure.discovered_roles, before_roles);
}

#[test]
fn arbitrary_surface_relabeling_preserves_structural_transfer() {
    let relabeled = V5Curriculum::relabeled();
    let r = run(V5Mechanism::DiscoveredBinding, &relabeled.curriculum);
    let probe = r
        .model
        .probe_teacher_off(&r.state, relabeled.held_out_source, 3);

    assert!(
        r.state.psi[relabeled.held_out_source.index()][relabeled.held_out_target.index()].abs()
            <= EPS
    );
    assert!(probe.activation(1, relabeled.held_out_target).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(3, relabeled.held_out_target).unwrap() > POSITIVE_FLOOR);
}

#[test]
fn runtime_source_contains_no_supplied_family_helpers() {
    let source = include_str!("../src/teacher_bridge_v005.rs");
    for forbidden in ["B_FAMILY", "C_FAMILY", "is_b()", "is_c()"] {
        assert!(
            !source.contains(forbidden),
            "v0.05 runtime still contains forbidden family helper {forbidden}"
        );
    }
}

#[test]
fn teacher_off_probe_is_nonmutating_and_deterministic() {
    let curriculum = V5Curriculum::llm_authored();
    for mechanism in [V5Mechanism::Plain, V5Mechanism::DiscoveredBinding] {
        let a = run(mechanism, &curriculum);
        let b = run(mechanism, &curriculum);
        let before = a.state.clone();
        let pa = a.model.probe_teacher_off(&a.state, S5::B3, 3);
        let pb = b.model.probe_teacher_off(&b.state, S5::B3, 3);
        assert_eq!(a.state, before);
        assert_eq!(a.state, b.state);
        assert_eq!(pa, pb);
    }
}

#[test]
fn frozen_exact_v005_diagnostics_are_stable() {
    let curriculum = V5Curriculum::llm_authored();
    let r = run(V5Mechanism::DiscoveredBinding, &curriculum);

    assert!(
        (r.state.psi[S5::A3.index()][S5::B3.index()] - 0.118_728_766_402_377_73).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (r.state.psi[S5::A3.index()][S5::C3.index()] - 0.134_029_223_445_571_08).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (r.state.psi[S5::R1.index()][S5::B3.index()] - 0.151_301_435_041_803_57).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (r.state.psi[S5::R2.index()][S5::C3.index()] - 0.170_799_499_222_686_6).abs()
            <= REGRESSION_TOL
    );
    assert!(r.state.psi[S5::B3.index()][S5::C3.index()].abs() <= EPS);
    assert!(
        (r.state.structure.binding_gain - 0.181_508_508_639_410_8).abs() <= REGRESSION_TOL
    );
    assert!(
        (r.state.structure.slot_affinity[S5::B1.index()][S5::C1.index()]
            - 0.284_955_013_733_931_6)
            .abs()
            <= REGRESSION_TOL
    );
    assert!(
        (r.state.structure.slot_affinity[S5::B2.index()][S5::C2.index()]
            - 0.284_955_013_733_931_6)
            .abs()
            <= REGRESSION_TOL
    );
    assert!(
        (r.state.structure.slot_affinity[S5::B3.index()][S5::C3.index()]
            - 0.381_103_992_043_548_56)
            .abs()
            <= REGRESSION_TOL
    );

    let probe = r.model.probe_teacher_off(&r.state, S5::B3, 3);
    assert!(
        (probe.activation(1, S5::C3).unwrap() - 0.006_917_361_723_235_037).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (probe.activation(2, S5::C3).unwrap() - 0.006_917_361_723_235_037).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (probe.activation(3, S5::C3).unwrap() - 0.005_188_021_292_426_278).abs()
            <= REGRESSION_TOL
    );
}
