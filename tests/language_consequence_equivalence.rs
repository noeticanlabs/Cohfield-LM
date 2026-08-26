use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_STATE: f64 = 2.0;
const EPS_NONDEG: f64 = 0.04;
const EPS_EFFECT: f64 = 0.015;
const EPS_RICH: f64 = 0.20;

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

#[derive(Clone, Copy)]
enum Intervention {
    Baseline,
    OutgoingAHalf,
    IncomingBHalf,
}

fn counts(pattern: &[SurfaceSymbol], repeats: usize) -> [usize; 4] {
    let mut out = [0; 4];
    for _ in 0..repeats {
        for &symbol in pattern {
            out[symbol.index()] += 1;
        }
    }
    out
}

fn exposed(model: &CohfieldLanguageModelV1, pattern: &[SurfaceSymbol]) -> LanguageState {
    model
        .expose(&LanguageState::initial(), pattern, 64)
        .expect("frozen exposure must be valid")
}

fn apply_intervention(state: &LanguageState, intervention: Intervention) -> LanguageState {
    let mut next = LanguageState::equalized_from(state);
    match intervention {
        Intervention::Baseline => {}
        Intervention::OutgoingAHalf => {
            for value in &mut next.psi[SurfaceSymbol::A.index()] {
                *value *= 0.5;
            }
        }
        Intervention::IncomingBHalf => {
            for row in &mut next.psi {
                row[SurfaceSymbol::B.index()] *= 0.5;
            }
        }
    }
    next
}

fn contexts() -> Vec<Vec<SurfaceSymbol>> {
    vec![
        vec![SurfaceSymbol::A],
        vec![SurfaceSymbol::B],
        vec![SurfaceSymbol::A, SurfaceSymbol::B],
        vec![SurfaceSymbol::B, SurfaceSymbol::A],
    ]
}

fn projected_context_response(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    context: &[SurfaceSymbol],
) -> Vec<f64> {
    let mut local = state.clone();
    let mut out = Vec::new();
    for &symbol in context {
        local = model
            .evolve(&local, &LanguageInput::symbol(symbol), 1.0)
            .expect("context evolution");
        out.push(local.x[SurfaceSymbol::A.index()]);
        out.push(local.x[SurfaceSymbol::B.index()]);
    }
    for _ in 0..4 {
        local = model
            .evolve(&local, &LanguageInput::zero(), 1.0)
            .expect("autonomous continuation");
        out.push(local.x[SurfaceSymbol::A.index()]);
        out.push(local.x[SurfaceSymbol::B.index()]);
    }
    out
}

fn consequence_family(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    intervention: Intervention,
) -> Vec<f64> {
    let intervened = apply_intervention(state, intervention);
    contexts()
        .iter()
        .flat_map(|context| projected_context_response(model, &intervened, context))
        .collect()
}

fn full_a_response(model: &CohfieldLanguageModelV1, state: &LanguageState) -> Vec<f64> {
    let mut local = LanguageState::equalized_from(state);
    let mut out = Vec::new();
    local = model
        .evolve(&local, &LanguageInput::symbol(SurfaceSymbol::A), 1.0)
        .unwrap();
    out.extend_from_slice(&local.x);
    for _ in 0..4 {
        local = model.evolve(&local, &LanguageInput::zero(), 1.0).unwrap();
        out.extend_from_slice(&local.x);
    }
    out
}

fn distance(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}

fn b_after_two_zero_steps(model: &CohfieldLanguageModelV1, state: &LanguageState) -> f64 {
    let mut local = LanguageState::equalized_from(state);
    local = model
        .evolve(&local, &LanguageInput::symbol(SurfaceSymbol::A), 1.0)
        .unwrap();
    local = model.evolve(&local, &LanguageInput::zero(), 1.0).unwrap();
    local = model.evolve(&local, &LanguageInput::zero(), 1.0).unwrap();
    local.x[SurfaceSymbol::B.index()]
}

#[test]
fn cf_lm_004_histories_have_exactly_matched_symbol_counts() {
    assert_eq!(counts(&H_C, 64), [64, 64, 64, 64]);
    assert_eq!(counts(&H_D, 64), [64, 64, 64, 64]);
    assert_eq!(counts(&H_C, 64), counts(&H_D, 64));
}

#[test]
fn cf_lm_004_paths_are_exactly_different_relational_states() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);
    let d_psi = CohfieldLanguageModelV1::psi_frobenius_distance(&c_path, &d_path);

    assert!(d_psi > EPS_STATE, "Psi distance {d_psi}");
    assert_ne!(c_path.psi, d_path.psi);
}

#[test]
fn cf_lm_004_direct_a_to_b_relation_is_absent_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);

    assert!(c_path.psi[SurfaceSymbol::A.index()][SurfaceSymbol::B.index()].abs() <= EPS_FLOOR);
    assert!(d_path.psi[SurfaceSymbol::A.index()][SurfaceSymbol::B.index()].abs() <= EPS_FLOOR);
}

#[test]
fn cf_lm_004_baseline_consequence_family_is_equivalent_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);

    let d = distance(
        &consequence_family(&model, &c_path, Intervention::Baseline),
        &consequence_family(&model, &d_path, Intervention::Baseline),
    );
    assert!(d <= EPS_FLOOR, "baseline consequence distance {d}");
}

#[test]
fn cf_lm_004_outgoing_a_intervention_preserves_consequence_equivalence() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);

    let d = distance(
        &consequence_family(&model, &c_path, Intervention::OutgoingAHalf),
        &consequence_family(&model, &d_path, Intervention::OutgoingAHalf),
    );
    assert!(d <= EPS_FLOOR, "outgoing-A consequence distance {d}");
}

#[test]
fn cf_lm_004_incoming_b_intervention_preserves_consequence_equivalence() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);

    let d = distance(
        &consequence_family(&model, &c_path, Intervention::IncomingBHalf),
        &consequence_family(&model, &d_path, Intervention::IncomingBHalf),
    );
    assert!(d <= EPS_FLOOR, "incoming-B consequence distance {d}");
}

#[test]
fn cf_lm_004_shared_interventions_materially_change_observed_consequence_family() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let baseline = consequence_family(&model, &c_path, Intervention::Baseline);
    let outgoing = consequence_family(&model, &c_path, Intervention::OutgoingAHalf);
    let incoming = consequence_family(&model, &c_path, Intervention::IncomingBHalf);

    let delta_a = distance(&baseline, &outgoing);
    let delta_b = distance(&baseline, &incoming);
    assert!(delta_a > EPS_NONDEG, "outgoing-A displacement {delta_a}");
    assert!(delta_b > EPS_NONDEG, "incoming-B displacement {delta_b}");
}

#[test]
fn cf_lm_004_both_internal_paths_produce_same_nontrivial_a_to_b_consequence() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);

    let c_b2 = b_after_two_zero_steps(&model, &c_path);
    let d_b2 = b_after_two_zero_steps(&model, &d_path);
    assert!(c_b2 > EPS_EFFECT, "C-path B2 {c_b2}");
    assert!(d_b2 > EPS_EFFECT, "D-path B2 {d_b2}");
    assert!((c_b2 - d_b2).abs() <= EPS_FLOOR);
}

#[test]
fn cf_lm_004_rich_observer_distinguishes_internal_paths() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);

    let d = distance(
        &full_a_response(&model, &c_path),
        &full_a_response(&model, &d_path),
    );
    assert!(d > EPS_RICH, "full-coordinate A-context distance {d}");
}

#[test]
fn cf_lm_004_consequence_equivalence_is_deterministic_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let left = exposed(&model, &H_C);
    let right = exposed(&model, &H_C);

    assert_eq!(left.psi, right.psi);
    for intervention in [
        Intervention::Baseline,
        Intervention::OutgoingAHalf,
        Intervention::IncomingBHalf,
    ] {
        let d = distance(
            &consequence_family(&model, &left, intervention),
            &consequence_family(&model, &right, intervention),
        );
        assert!(d <= EPS_FLOOR);
    }
}

#[test]
fn cf_lm_004_matches_preregistered_preimplementation_cross_check() {
    let model = CohfieldLanguageModelV1::default();
    let c_path = exposed(&model, &H_C);
    let d_path = exposed(&model, &H_D);

    let d_psi = CohfieldLanguageModelV1::psi_frobenius_distance(&c_path, &d_path);
    let baseline_c = consequence_family(&model, &c_path, Intervention::Baseline);
    let baseline_d = consequence_family(&model, &d_path, Intervention::Baseline);
    let outgoing_c = consequence_family(&model, &c_path, Intervention::OutgoingAHalf);
    let incoming_c = consequence_family(&model, &c_path, Intervention::IncomingBHalf);
    let rich = distance(
        &full_a_response(&model, &c_path),
        &full_a_response(&model, &d_path),
    );

    assert!((d_psi - 2.812_778_851_911_623_2).abs() < 1.0e-9);
    assert!(
        (c_path.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()] - 0.984_081_650_505_525_9)
            .abs()
            < 1.0e-9
    );
    assert!(
        (c_path.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] - 1.004_164_949_495_434_6)
            .abs()
            < 1.0e-9
    );
    assert!(
        (d_path.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()] - 0.984_081_650_505_525_9)
            .abs()
            < 1.0e-9
    );
    assert!(
        (d_path.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] - 1.004_164_949_495_434_6)
            .abs()
            < 1.0e-9
    );
    assert!(distance(&baseline_c, &baseline_d) <= EPS_FLOOR);
    assert!((distance(&baseline_c, &outgoing_c) - 0.042_623_669_738_106_28).abs() < 1.0e-9);
    assert!((distance(&baseline_c, &incoming_c) - 0.042_623_669_738_106_28).abs() < 1.0e-9);
    assert!((b_after_two_zero_steps(&model, &c_path) - 0.019_763_606_017_585_308).abs() < 1.0e-9);
    assert!((rich - 0.227_700_413_755_575_84).abs() < 1.0e-9);
}
