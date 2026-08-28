use cohfield_lm::teacher_bridge_v003::{run, Mechanism, V3Curriculum, V3Runner, S};

const EPS: f64 = 1.0e-12;
const POSITIVE_FLOOR: f64 = 1.0e-6;
const REGRESSION_TOL: f64 = 1.0e-9;

fn runner(mechanism: Mechanism, curriculum: &V3Curriculum) -> V3Runner {
    run(mechanism, curriculum)
}

/// The three v0.03 arms receive the same visible curriculum with identical
/// epochs and initialization. Their learned ordinary relations and accumulated
/// abstraction-layer weights must match. The Plain arm intentionally leaves the
/// abstraction runtime gate disabled; MemberAbstraction and Target enable it.
#[test]
fn arms_share_identical_training_weights_and_withhold_b3_c3() {
    let curriculum = V3Curriculum::llm_authored();
    let plain = runner(Mechanism::Plain, &curriculum);
    let member = runner(Mechanism::MemberAbstraction, &curriculum);
    let target = runner(Mechanism::Target, &curriculum);

    assert_eq!(plain.state.psi, member.state.psi);
    assert_eq!(plain.state.psi, target.state.psi);
    assert_eq!(plain.state.w_abs_b, member.state.w_abs_b);
    assert_eq!(plain.state.w_abs_b, target.state.w_abs_b);
    assert_eq!(plain.state.w_pool_c, member.state.w_pool_c);
    assert_eq!(plain.state.w_pool_c, target.state.w_pool_c);

    assert!(!plain.state.b_abstraction);
    assert!(member.state.b_abstraction);
    assert!(target.state.b_abstraction);

    for state in [&plain.state, &member.state, &target.state] {
        assert!(state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
    }
}

/// v0.03b boundary: plain composition reaches the taught A3->B3 edge but
/// cannot infer the withheld B3->C3 relation.
#[test]
fn plain_composition_leaves_held_out_member_silent() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::Plain, &curriculum);

    assert!(r.state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);
    assert!(probe.activation(1, S::B3).unwrap() > POSITIVE_FLOOR);

    for symbol in [S::C1, S::C2, S::C3] {
        for step in 0..=3 {
            assert!(
                probe.activation(step, symbol).unwrap().abs() <= EPS,
                "Plain arm produced non-zero {symbol:?} activation at step {step}"
            );
        }
    }
}

/// Positive substrate control: directly teaching B3->C3 creates the missing
/// relation and the same Plain dynamics propagate it teacher-off.
#[test]
fn direct_teaching_positive_control_propagates() {
    let curriculum = V3Curriculum::llm_authored_with_direct_b3_c3();
    let r = runner(Mechanism::Plain, &curriculum);

    assert!(r.state.psi[S::B3.index()][S::C3.index()] > POSITIVE_FLOOR);

    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);
    assert!(probe.activation(2, S::C3).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(3, S::C3).unwrap() > POSITIVE_FLOOR);
}

/// CF-LM-015-style member abstraction transfers B3 activity to targets that
/// were already learned from other B-family members, but it does not invent the
/// wholly unseen target C3.
#[test]
fn member_abstraction_reaches_taught_sibling_targets_only() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::MemberAbstraction, &curriculum);

    assert!(r.state.psi[S::B3.index()][S::C1.index()].abs() <= EPS);
    assert!(r.state.psi[S::B3.index()][S::C2.index()].abs() <= EPS);
    assert!(r.state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);
    assert!(probe.activation(2, S::C1).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(2, S::C2).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(3, S::C1).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(3, S::C2).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(3, S::C3).unwrap().abs() <= EPS);
}

/// Surgical member-abstraction ablation: remove only the abstraction->C1
/// relation. C1 transfer must collapse while C2 survives and ordinary Psi is
/// untouched.
#[test]
fn member_ablation_collapses_one_target_without_touching_psi() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::MemberAbstraction, &curriculum);
    let baseline = r.model.probe_teacher_off(&r.state, S::A3, 3);

    assert!(baseline.activation(3, S::C1).unwrap() > POSITIVE_FLOOR);
    assert!(baseline.activation(3, S::C2).unwrap() > POSITIVE_FLOOR);

    let before_psi = r.state.psi;
    let before_pool = r.state.w_pool_c;
    let mut ablated = r.state.clone();
    ablated.w_abs_b[S::C1.index()] = 0.0;

    let after = r.model.probe_teacher_off(&ablated, S::A3, 3);
    assert!(after.activation(3, S::C1).unwrap().abs() <= EPS);
    assert!(after.activation(3, S::C2).unwrap() > POSITIVE_FLOOR);
    assert!(
        (after.activation(3, S::C2).unwrap() - baseline.activation(3, S::C2).unwrap()).abs()
            <= REGRESSION_TOL
    );
    assert_eq!(ablated.psi, before_psi);
    assert_eq!(ablated.w_pool_c, before_pool);
}

/// Exploratory target-pool mechanism reaches C3 without a direct B3->C3 edge.
/// The equality control is essential: the mechanism broadcasts equally across
/// the C family, so this is pooled family activation, not evidence that the
/// system selected the matched structural target C3 specifically.
#[test]
fn target_pool_reaches_withheld_target_but_is_nonspecific() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::Target, &curriculum);

    assert!(r.state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);

    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);
    let c1 = probe.activation(3, S::C1).unwrap();
    let c2 = probe.activation(3, S::C2).unwrap();
    let c3 = probe.activation(3, S::C3).unwrap();

    assert!(c3 > POSITIVE_FLOOR);
    assert!((c1 - c2).abs() <= EPS);
    assert!((c2 - c3).abs() <= EPS);
}

/// Surgical target-pool ablation: zero only the pooled B->C-family weight.
/// The entire pooled C response must disappear while Psi and member weights are
/// unchanged.
#[test]
fn target_pool_ablation_collapses_response_without_touching_psi() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::Target, &curriculum);
    let baseline = r.model.probe_teacher_off(&r.state, S::A3, 3);
    assert!(baseline.activation(3, S::C3).unwrap() > POSITIVE_FLOOR);

    let before_psi = r.state.psi;
    let before_member_weights = r.state.w_abs_b;
    let mut ablated = r.state.clone();
    ablated.w_pool_c = 0.0;

    let after = r.model.probe_teacher_off(&ablated, S::A3, 3);
    for symbol in [S::C1, S::C2, S::C3] {
        assert!(after.activation(3, symbol).unwrap().abs() <= EPS);
    }
    assert_eq!(ablated.psi, before_psi);
    assert_eq!(ablated.w_abs_b, before_member_weights);
}

#[test]
fn teacher_off_probe_does_not_mutate_persistent_state() {
    let curriculum = V3Curriculum::llm_authored();
    for mechanism in [
        Mechanism::Plain,
        Mechanism::MemberAbstraction,
        Mechanism::Target,
    ] {
        let r = runner(mechanism, &curriculum);
        let before = r.state.clone();
        let _ = r.model.probe_teacher_off(&r.state, S::A3, 3);
        assert_eq!(r.state, before);
    }
}

#[test]
fn all_arms_are_deterministic() {
    let curriculum = V3Curriculum::llm_authored();
    for mechanism in [
        Mechanism::Plain,
        Mechanism::MemberAbstraction,
        Mechanism::Target,
    ] {
        let a = runner(mechanism, &curriculum);
        let b = runner(mechanism, &curriculum);
        assert_eq!(a.state, b.state);
        assert_eq!(
            a.model.probe_teacher_off(&a.state, S::A3, 3),
            b.model.probe_teacher_off(&b.state, S::A3, 3)
        );
    }
}

/// Frozen numerical diagnostics are executable regression assertions rather
/// than an inner diagnostic function. This prevents the previous mangling from
/// silently producing `0 tests` while looking superficially successful.
#[test]
fn frozen_exact_diagnostics_are_stable() {
    let curriculum = V3Curriculum::llm_authored();
    let plain = runner(Mechanism::Plain, &curriculum);
    let member = runner(Mechanism::MemberAbstraction, &curriculum);
    let target = runner(Mechanism::Target, &curriculum);

    assert!(
        (plain.state.psi[S::A3.index()][S::B3.index()] - 0.403_380_561_493_748_46).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (plain.state.psi[S::B1.index()][S::C1.index()] - 0.420_013_079_439_554_85).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (plain.state.psi[S::B2.index()][S::C2.index()] - 0.437_331_402_998_287).abs()
            <= REGRESSION_TOL
    );
    assert!((plain.state.w_pool_c - 0.857_344_482_437_841_2).abs() <= REGRESSION_TOL);

    let plain_probe = plain.model.probe_teacher_off(&plain.state, S::A3, 3);
    assert!(
        (plain_probe.activation(1, S::B3).unwrap() - 0.040_338_056_149_374_85).abs()
            <= REGRESSION_TOL
    );

    let member_probe = member.model.probe_teacher_off(&member.state, S::A3, 3);
    assert!(
        (member_probe.activation(3, S::C1).unwrap() - 0.001_694_251_118_190_460_4).abs()
            <= REGRESSION_TOL
    );
    assert!(
        (member_probe.activation(3, S::C2).unwrap() - 0.001_764_109_869_002_978_1).abs()
            <= REGRESSION_TOL
    );
    assert!(member_probe.activation(3, S::C3).unwrap().abs() <= EPS);

    let target_probe = target.model.probe_teacher_off(&target.state, S::A3, 3);
    for symbol in [S::C1, S::C2, S::C3] {
        assert!(
            (target_probe.activation(3, symbol).unwrap() - 0.003_458_360_987_193_436).abs()
                <= REGRESSION_TOL
        );
    }
}
