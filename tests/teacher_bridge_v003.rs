use cohfield_lm::teacher_bridge_v003::{run, Mechanism, V3Curriculum, V3Runner, S};

const EPS: f64 = 1.0e-12;
const POSITIVE_FLOOR: f64 = 1.0e-6;

fn runner(mechanism: Mechanism, curriculum: &V3Curriculum) -> V3Runner {
    run(mechanism, curriculum)
}

/// The three v0.03 mechanisms are trained on the exact same visible curriculum
/// with identical epochs, initialization, and single-mechanism toggle. This
/// locks that equivalence so the composition-vs-abstraction comparison is
/// matched except for the mechanism under test.
#[test]
fn abstraction_arms_are_exact_matches_except_mechanism() {
    let curriculum = V3Curriculum::llm_authored();
    let plain = runner(Mechanism::Plain, &curriculum);
    let member = runner(Mechanism::MemberAbstraction, &curriculum);
    let target = runner(Mechanism::Target, &curriculum);

    // Every arm stores the same persistent edge relations and the same
    // abstraction-layer accumulated weights; only the mechanism toggle differs.
    assert_eq!(plain.state.psi, member.state.psi);
    assert_eq!(plain.state.psi, target.state.psi);
    assert_eq!(member.state.w_abs_b, target.state.w_abs_b);
    assert_eq!(member.state.w_pool_c, target.state.w_pool_c);

    // The held-out edge is never taught in any arm.
    assert!(plain.state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
    assert!(member.state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
    assert!(target.state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
}

/// v0.03b boundary: plain composition cannot transfer to the unseen member —
/// the held-out `B3->?` dead-ends. Composition of learned edges is not the same
/// as inference of an unlearned relation.
#[test]
fn plain_composition_leaves_held_out_member_silent() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::Plain, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);

    // A3 reaches B3 (its taught target) but B3 has no outgoing learned edge.
    assert!(probe.activation(3, S::B3).unwrap() > POSITIVE_FLOOR);
    for symbol in [S::C1, S::C2, S::C3] {
        for step in 0..=3 {
            assert!(
                probe.activation(step, symbol).unwrap().abs() <= EPS,
                "Plain arm produced illegal {symbol:?} activation at step {step}"
            );
        }
    }
}

/// v0.03b positive control: if `B3->C3` is taught directly, the same Plain
/// substrate DOES propagate it teacher-off. This proves the 9-symbol surface can
/// carry the required continuation; the null above is therefore about the
/// *unlearned* relation, not a substrate limitation.
#[test]
fn direct_teaching_positive_control_propagates() {
    let curriculum = V3Curriculum::llm_authored_with_direct_b3_c3();
    let r = runner(Mechanism::Plain, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);
    assert!(
        probe.activation(3, S::C3).unwrap() > POSITIVE_FLOOR,
        "directly-taught B3->C3 must propagate teacher-off"
    );
}

/// v0.03c (Member arm): the derived B-family abstraction lets the unseen member
/// B3 drive the pooled taught consequences of its siblings (C1, C2) — a genuine
/// member-axis transfer plain composition lacks — but a wholly-unseen target
/// that was never a relation target (C3) stays silent. Hedge stands.
#[test]
fn member_abstraction_reaches_taught_sibling_targets_only() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::MemberAbstraction, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);

    // Abstraction formed over the B family with a learned relation only to the
    // targets that were taught through member experience.
    assert!(r.state.b_abstraction);
    assert!(r.state.w_abs_b[S::C1.index()] > 0.0);
    assert!(r.state.w_abs_b[S::C2.index()] > 0.0);
    assert!(r.state.w_abs_b[S::C3.index()].abs() <= EPS);

    // B3 reaches the taught family targets...
    assert!(probe.activation(3, S::C1).unwrap() > POSITIVE_FLOOR);
    assert!(probe.activation(3, S::C2).unwrap() > POSITIVE_FLOOR);
    // ...but the unseen target stays silent.
    assert!(probe.activation(3, S::C3).unwrap().abs() <= EPS);
}

/// v0.03c (Target arm): the pooled target generalization is the only route to
/// the withheld new target C3. It arrives through the abstraction pathway while
/// `Psi[B3,C3]` remains exactly zero — never a stored direct shortcut.
#[test]
fn target_generalization_reaches_withheld_target_through_pool() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::Target, &curriculum);
    let probe = r.model.probe_teacher_off(&r.state, S::A3, 3);

    assert!(r.state.b_abstraction);
    assert!(r.state.w_pool_c > 0.0);
    // The withheld target is reached, and NOT through a direct stored edge.
    assert!(probe.activation(3, S::C3).unwrap() > POSITIVE_FLOOR);
    assert!(r.state.psi[S::B3.index()][S::C3.index()].abs() <= EPS);
}

/// Causal control (Member): surgically removing only the abstraction->C1
/// relation collapses the B3->C1 transfer while leaving B3->C2 and every
/// persistent Psi edge intact.
#[test]
fn member_ablation_collapses_target_without_touching_psi() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::MemberAbstraction, &curriculum);
    let intact = r.model.probe_teacher_off(&r.state, S::A3, 3);
    assert!(intact.activation(3, S::C1).unwrap() > POSITIVE_FLOOR);
    assert!(intact.activation(3, S::C2).unwrap() > POSITIVE_FLOOR);

    let mut ablated = r.state.clone();
    ablated.w_abs_b[S::C1.index()] = 0.0; // remove only the C1 abstraction relation
    let after = r.model.probe_teacher_off(&ablated, S::A3, 3);

    assert!(after.activation(3, S::C1).unwrap().abs() <= EPS);
    assert!(after.activation(3, S::C2).unwrap() > POSITIVE_FLOOR);
    // Every persistent Psi edge is byte-identical; only the abstraction layer moved.
    assert_eq!(ablated.psi, r.state.psi);
    assert_eq!(
        ablated.w_abs_b[S::C2.index()],
        r.state.w_abs_b[S::C2.index()]
    );
}

/// Causal control (Target): surgically removing only the pooled B->C relation
/// collapses ALL C-family transfer (including the withheld C3) while every
/// persistent Psi edge stays intact — proving C3 arrived via the abstraction,
/// not a stored direct shortcut.
#[test]
fn target_ablation_collapses_pool_without_touching_psi() {
    let curriculum = V3Curriculum::llm_authored();
    let r = runner(Mechanism::Target, &curriculum);
    let intact = r.model.probe_teacher_off(&r.state, S::A3, 3);
    assert!(intact.activation(3, S::C3).unwrap() > POSITIVE_FLOOR);

    let mut ablated = r.state.clone();
    ablated.w_pool_c = 0.0; // remove only the pooled B->C generalization
    let after = r.model.probe_teacher_off(&ablated, S::A3, 3);

    for symbol in [S::C1, S::C2, S::C3] {
        assert!(after.activation(3, symbol).unwrap().abs() <= EPS);
    }
    assert_eq!(ablated.psi, r.state.psi);
}

/// Teacher-off purity: evaluation never mutates persistent state or relations.
#[test]
fn teacher_off_probe_does_not_mutate_persistent_state() {
    let curriculum = V3Curriculum::llm_authored();
    for mech in [
        Mechanism::Plain,
        Mechanism::MemberAbstraction,
        Mechanism::Target,
    ] {
        let r = runner(mech, &curriculum);
        let before_x = r.state.x;
        let before_psi = r.state.psi;
        let _ = r.model.probe_teacher_off(&r.state, S::A3, 3);
        assert_eq!(r.state.x, before_x);

#[test]
fn diag_exact() {
    let c = V3Curriculum::llm_authored();
    for mech in [
        Mechanism::Plain,
        Mechanism::MemberAbstraction,
        Mechanism::Target,
    ] {
        let r = runner(mech, &c);
        let p = r.model.probe_teacher_off(&r.state, S::A3, 3);
        let f = |s: S| p.activation(3, s).unwrap();
        println!(
            "{mech:?} c1={:.17e} c2={:.17e} c3={:.17e}",
            f(S::C1),
            f(S::C2),
            f(S::C3)
        );
    }
}

        assert_eq!(r.state.psi, before_psi);
    }
}

/// Every arm must be fully deterministic across repeated executions.
#[test]
fn all_arms_are_deterministic() {
    let curriculum = V3Curriculum::llm_authored();
    for mech in [
        Mechanism::Plain,
        Mechanism::MemberAbstraction,
        Mechanism::Target,
    ] {
        let a = runner(mech, &curriculum);
        let b = runner(mech, &curriculum);
        let pa = a.model.probe_teacher_off(&a.state, S::A3, 3);
        let pb = b.model.probe_teacher_off(&b.state, S::A3, 3);
        assert_eq!(a.state, b.state);
        assert_eq!(pa, pb, "mechanism {mech:?} must replay deterministically");
    }
}
