use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_COMP: f64 = 0.005;
const EPS_FIRST_HOP: f64 = 0.05;

const H_CHAIN: [SurfaceSymbol; 6] = [
    SurfaceSymbol::A,
    SurfaceSymbol::B,
    SurfaceSymbol::D,
    SurfaceSymbol::B,
    SurfaceSymbol::C,
    SurfaceSymbol::D,
];

const H_BREAK: [SurfaceSymbol; 6] = [
    SurfaceSymbol::A,
    SurfaceSymbol::B,
    SurfaceSymbol::D,
    SurfaceSymbol::C,
    SurfaceSymbol::B,
    SurfaceSymbol::D,
];

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
        .expose(&LanguageState::initial(), pattern, 32)
        .expect("frozen exposure must be valid")
}

fn two_hop_probe(model: &CohfieldLanguageModelV1, state: &LanguageState) -> (f64, f64) {
    let equalized = LanguageState::equalized_from(state);
    let after_a = model
        .evolve(&equalized, &LanguageInput::symbol(SurfaceSymbol::A), 1.0)
        .unwrap();
    let after_one_zero = model.evolve(&after_a, &LanguageInput::zero(), 1.0).unwrap();
    let after_two_zero = model
        .evolve(&after_one_zero, &LanguageInput::zero(), 1.0)
        .unwrap();

    (
        after_one_zero.x[SurfaceSymbol::B.index()],
        after_two_zero.x[SurfaceSymbol::C.index()],
    )
}

#[test]
fn cf_lm_002_histories_have_exactly_matched_symbol_counts() {
    assert_eq!(counts(&H_CHAIN, 32), [32, 64, 32, 64]);
    assert_eq!(counts(&H_BREAK, 32), [32, 64, 32, 64]);
    assert_eq!(counts(&H_CHAIN, 32), counts(&H_BREAK, 32));
}

#[test]
fn cf_lm_002_direct_a_to_c_relation_is_absent_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let chain = exposed(&model, &H_CHAIN);
    let broken = exposed(&model, &H_BREAK);

    assert!(chain.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()].abs() <= EPS_FLOOR);
    assert!(broken.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()].abs() <= EPS_FLOOR);
}

#[test]
fn cf_lm_002_target_history_learns_both_required_chain_edges() {
    let model = CohfieldLanguageModelV1::default();
    let chain = exposed(&model, &H_CHAIN);

    assert!(chain.psi[SurfaceSymbol::A.index()][SurfaceSymbol::B.index()] > 0.0);
    assert!(chain.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] > 0.0);
}

#[test]
fn cf_lm_002_target_chain_produces_two_hop_continuation() {
    let model = CohfieldLanguageModelV1::default();
    let chain = exposed(&model, &H_CHAIN);
    let (b1, c2) = two_hop_probe(&model, &chain);

    assert!(b1 > EPS_FIRST_HOP, "first-hop B activity {b1}");
    assert!(c2 > EPS_COMP, "two-hop C activity {c2}");
}

#[test]
fn cf_lm_002_broken_bridge_preserves_first_hop_but_blocks_second_hop_c() {
    let model = CohfieldLanguageModelV1::default();
    let broken = exposed(&model, &H_BREAK);
    let (b1, c2) = two_hop_probe(&model, &broken);

    assert!(b1 > EPS_FIRST_HOP, "first-hop B activity {b1}");
    assert!(c2 <= EPS_FLOOR, "broken-bridge C activity {c2}");
}

#[test]
fn cf_lm_002_surgical_b_to_c_removal_collapses_only_the_second_hop() {
    let model = CohfieldLanguageModelV1::default();
    let chain = exposed(&model, &H_CHAIN);
    let mut surgical = chain.clone();
    surgical.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] = 0.0;

    let (b1, c2) = two_hop_probe(&model, &surgical);
    assert!(b1 > EPS_FIRST_HOP, "first-hop B activity {b1}");
    assert!(c2 <= EPS_FLOOR, "surgical C activity {c2}");
}

#[test]
fn cf_lm_002_no_adaptation_control_has_no_two_hop_effect() {
    let model = CohfieldLanguageModelV1::without_adaptation();
    let chain = exposed(&model, &H_CHAIN);
    let broken = exposed(&model, &H_BREAK);

    let (_, chain_c2) = two_hop_probe(&model, &chain);
    let (_, broken_c2) = two_hop_probe(&model, &broken);

    assert!(chain_c2 <= EPS_FLOOR);
    assert!(broken_c2 <= EPS_FLOOR);
}

#[test]
fn cf_lm_002_target_history_is_deterministic_to_floor() {
    let model = CohfieldLanguageModelV1::default();
    let left = exposed(&model, &H_CHAIN);
    let right = exposed(&model, &H_CHAIN);

    assert_eq!(left.psi, right.psi);
    let (left_b1, left_c2) = two_hop_probe(&model, &left);
    let (right_b1, right_c2) = two_hop_probe(&model, &right);
    assert!((left_b1 - right_b1).abs() <= EPS_FLOOR);
    assert!((left_c2 - right_c2).abs() <= EPS_FLOOR);
}

#[test]
fn cf_lm_002_matches_preregistered_preimplementation_cross_check() {
    let model = CohfieldLanguageModelV1::default();
    let chain = exposed(&model, &H_CHAIN);
    let broken = exposed(&model, &H_BREAK);

    let (chain_b1, chain_c2) = two_hop_probe(&model, &chain);
    let (broken_b1, broken_c2) = two_hop_probe(&model, &broken);

    assert!(
        (chain.psi[SurfaceSymbol::A.index()][SurfaceSymbol::B.index()] - 0.633_019_445_9).abs()
            < 1.0e-9
    );
    assert!(
        (chain.psi[SurfaceSymbol::B.index()][SurfaceSymbol::C.index()] - 0.672_572_063_8).abs()
            < 1.0e-9
    );
    assert!((chain_b1 - 0.063_301_944_6).abs() < 1.0e-9);
    assert!((chain_c2 - 0.008_515_023_9).abs() < 1.0e-9);
    assert!((broken_b1 - 0.063_301_944_6).abs() < 1.0e-9);
    assert!(broken_c2 <= EPS_FLOOR);
}
