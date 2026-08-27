use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::profiles::language_v2::{
    CohfieldLanguageModelV2, InternalEquivalenceProfile, LanguageExperienceV2, LanguageStateV2,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_FLOOR: f64 = 1.0e-12;
const EPS_TRANSFER: f64 = 0.01;
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

fn source_state() -> LanguageStateV2 {
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

    LanguageStateV2::from_v1(&combined)
}

fn internalize(model: &CohfieldLanguageModelV2, state: &LanguageStateV2) -> LanguageStateV2 {
    model
        .adapt(
            state,
            &LanguageExperienceV2::InternalizeConsequenceEquivalence(
                InternalEquivalenceProfile::cf_lm_009(),
            ),
        )
        .expect("frozen equivalence acquisition must be valid")
}

fn teach_isolated_pair(
    model: &CohfieldLanguageModelV2,
    state: &LanguageStateV2,
    from: SurfaceSymbol,
    to: SurfaceSymbol,
) -> LanguageStateV2 {
    let mut next = state.clone();
    for _ in 0..EPISODES {
        next = model
            .expose(&next, &[from, to], 1)
            .expect("isolated two-symbol episode must be valid");
    }
    next
}

fn probe(
    model: &CohfieldLanguageModelV2,
    state: &LanguageStateV2,
    symbol: SurfaceSymbol,
    continuation_steps: usize,
) -> Vec<[f64; 4]> {
    let mut local = LanguageStateV2::equalized_from(state);
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

fn consequence_signature(
    model: &CohfieldLanguageModelV2,
    state: &LanguageStateV2,
    symbol: SurfaceSymbol,
) -> Vec<f64> {
    probe(model, state, symbol, 4)
        .into_iter()
        .flat_map(|x| [x[SurfaceSymbol::A.index()], x[SurfaceSymbol::B.index()]])
        .collect()
}

fn euclidean(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}

fn nontrivial_equivalence_pairs(state: &LanguageStateV2) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for left in 0..SurfaceSymbol::ALL.len() {
        for right in (left + 1)..SurfaceSymbol::ALL.len() {
            if state.relational.consequence_equivalence[left][right] {
                pairs.push((left, right));
            }
        }
    }
    pairs
}

#[test]
fn cf_lm_009_v1_to_v2_migration_preserves_sequential_state_and_starts_empty() {
    let source = source_state();

    assert_eq!(source.x, [0.0; 4]);
    assert_eq!(source.theta, [1.0; 4]);
    assert!(source
        .relational
        .consequence_equivalence
        .iter()
        .flatten()
        .all(|value| !*value));

    assert!((source.relational.sequential[0][2] - 0.984_081_650_505_525_9).abs() < REGRESSION_TOL);
    assert!((source.relational.sequential[2][1] - 1.004_164_949_495_434_6).abs() < REGRESSION_TOL);
    assert!((source.relational.sequential[0][3] - 0.984_081_650_505_525_9).abs() < REGRESSION_TOL);
    assert!((source.relational.sequential[3][1] - 1.004_164_949_495_434_6).abs() < REGRESSION_TOL);
}

#[test]
fn cf_lm_009_acquisition_discovers_only_the_cd_consequence_pair() {
    let model = CohfieldLanguageModelV2::default();
    let source = source_state();
    let acquired = internalize(&model, &source);

    assert_eq!(
        nontrivial_equivalence_pairs(&acquired),
        vec![(SurfaceSymbol::C.index(), SurfaceSymbol::D.index())]
    );
    assert!(
        acquired.relational.consequence_equivalence[SurfaceSymbol::C.index()]
            [SurfaceSymbol::D.index()]
    );
    assert!(
        acquired.relational.consequence_equivalence[SurfaceSymbol::D.index()]
            [SurfaceSymbol::C.index()]
    );
}

#[test]
fn cf_lm_009_acquisition_preserves_sequential_relations_fast_state_and_theta() {
    let model = CohfieldLanguageModelV2::default();
    let source = source_state();
    let acquired = internalize(&model, &source);

    assert_eq!(acquired.x, source.x);
    assert_eq!(acquired.theta, source.theta);
    assert_eq!(acquired.relational.sequential, source.relational.sequential);
    assert_ne!(acquired, source);
}

#[test]
fn cf_lm_009_no_internalization_control_does_not_transfer_new_relation() {
    let model = CohfieldLanguageModelV2::default();
    let source = source_state();
    let trained = teach_isolated_pair(&model, &source, SurfaceSymbol::C, SurfaceSymbol::A);
    let response = probe(&model, &trained, SurfaceSymbol::D, 4);

    assert!(response[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert!(
        trained.relational.sequential[SurfaceSymbol::D.index()][SurfaceSymbol::A.index()].abs()
            <= EPS_FLOOR
    );
}

#[test]
fn cf_lm_009_internalization_without_novel_relation_does_not_create_target_consequence() {
    let model = CohfieldLanguageModelV2::default();
    let acquired = internalize(&model, &source_state());
    let response = probe(&model, &acquired, SurfaceSymbol::D, 4);

    assert!(response
        .iter()
        .all(|x| x[SurfaceSymbol::A.index()].abs() <= EPS_FLOOR));
}

#[test]
fn cf_lm_009_internalized_equivalence_transfers_later_c_to_a_learning_to_d_probe() {
    let model = CohfieldLanguageModelV2::default();
    let acquired = internalize(&model, &source_state());
    let trained = teach_isolated_pair(&model, &acquired, SurfaceSymbol::C, SurfaceSymbol::A);
    let response = probe(&model, &trained, SurfaceSymbol::D, 4);

    assert!(
        response[2][SurfaceSymbol::A.index()] > EPS_TRANSFER,
        "A step-2 transfer was {}",
        response[2][SurfaceSymbol::A.index()]
    );
    assert!(
        (trained.relational.sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()]
            - 0.557_984_402_843_442_6)
            .abs()
            < REGRESSION_TOL
    );
    assert!(
        (response[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs() < REGRESSION_TOL
    );
    assert!(
        trained.relational.sequential[SurfaceSymbol::D.index()][SurfaceSymbol::A.index()].abs()
            <= EPS_FLOOR
    );
}

#[test]
fn cf_lm_009_surgical_equivalence_ablation_collapses_transfer_without_erasing_learning() {
    let model = CohfieldLanguageModelV2::default();
    let acquired = internalize(&model, &source_state());
    let trained = teach_isolated_pair(&model, &acquired, SurfaceSymbol::C, SurfaceSymbol::A);
    let learned_c_to_a =
        trained.relational.sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()];

    let mut ablated = trained.clone();
    ablated.relational.consequence_equivalence[SurfaceSymbol::C.index()]
        [SurfaceSymbol::D.index()] = false;
    ablated.relational.consequence_equivalence[SurfaceSymbol::D.index()]
        [SurfaceSymbol::C.index()] = false;

    let response = probe(&model, &ablated, SurfaceSymbol::D, 4);
    assert!(response[2][SurfaceSymbol::A.index()].abs() <= EPS_FLOOR);
    assert_eq!(
        ablated.relational.sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()],
        learned_c_to_a
    );
}

#[test]
fn cf_lm_009_reverse_direction_internalization_transfers_d_to_a_learning_to_c_probe() {
    let model = CohfieldLanguageModelV2::default();
    let acquired = internalize(&model, &source_state());
    let trained = teach_isolated_pair(&model, &acquired, SurfaceSymbol::D, SurfaceSymbol::A);
    let response = probe(&model, &trained, SurfaceSymbol::C, 4);

    assert!(response[2][SurfaceSymbol::A.index()] > EPS_TRANSFER);
    assert!(
        (response[2][SurfaceSymbol::A.index()] - 0.011_159_688_056_868_854).abs() < REGRESSION_TOL
    );
    assert!(
        trained.relational.sequential[SurfaceSymbol::C.index()][SurfaceSymbol::A.index()].abs()
            <= EPS_FLOOR
    );
}

#[test]
fn cf_lm_009_preupdate_consequence_distances_match_preregistered_discovery_geometry() {
    let model = CohfieldLanguageModelV2::default();
    let source = source_state();
    let signatures: Vec<_> = SurfaceSymbol::ALL
        .iter()
        .map(|&symbol| consequence_signature(&model, &source, symbol))
        .collect();

    let expected: [((usize, usize), f64); 6] = [
        ((0, 1), 0.808_461_499_583_201_6),
        ((0, 2), 0.589_184_158_804_122_9),
        ((0, 3), 0.589_184_158_804_122_9),
        ((1, 2), 0.522_975_282_118_704_5),
        ((1, 3), 0.522_975_282_118_704_5),
        ((2, 3), 0.0),
    ];

    for ((left, right), target) in expected {
        let distance = euclidean(&signatures[left], &signatures[right]);
        assert!((distance - target).abs() < REGRESSION_TOL);
    }
}

#[test]
fn cf_lm_009_transfer_trajectory_matches_preregistered_values() {
    let model = CohfieldLanguageModelV2::default();
    let acquired = internalize(&model, &source_state());
    let trained = teach_isolated_pair(&model, &acquired, SurfaceSymbol::C, SurfaceSymbol::A);
    let response = probe(&model, &trained, SurfaceSymbol::D, 4);
    let expected = [
        0.0,
        0.0,
        0.011_159_688_056_868_854,
        0.016_739_532_085_303_28,
        0.017_363_331_386_570_834,
    ];

    for (x, target) in response.iter().zip(expected) {
        assert!((x[SurfaceSymbol::A.index()] - target).abs() < REGRESSION_TOL);
    }
}
