use cohfield_lm::profiles::language::{
    CohfieldLanguageModelV1, LanguageInput, LanguageState, SurfaceSymbol,
};
use cohfield_lm::AdaptiveContinuationModel;

const EPS_STATE: f64 = 0.90;
const EPS_RICH: f64 = 0.15;
const REGRESSION_TOL: f64 = 1.0e-9;

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
const H_LOOP: [SurfaceSymbol; 2] = [SurfaceSymbol::D, SurfaceSymbol::D];

#[derive(Clone, Copy)]
enum HostProfile {
    Baseline,
    BackToA(f64),
    CrossRelay(f64),
}

#[derive(Clone, Copy)]
struct Profile {
    host: HostProfile,
    horizon: usize,
}

const SHORT_PROFILES: [Profile; 5] = [
    Profile { host: HostProfile::Baseline, horizon: 4 },
    Profile { host: HostProfile::BackToA(1.0), horizon: 4 },
    Profile { host: HostProfile::CrossRelay(0.5), horizon: 4 },
    Profile { host: HostProfile::CrossRelay(1.0), horizon: 4 },
    Profile { host: HostProfile::CrossRelay(2.0), horizon: 4 },
];

const FULL_PROFILES: [Profile; 8] = [
    Profile { host: HostProfile::Baseline, horizon: 4 },
    Profile { host: HostProfile::BackToA(1.0), horizon: 4 },
    Profile { host: HostProfile::CrossRelay(0.5), horizon: 4 },
    Profile { host: HostProfile::CrossRelay(1.0), horizon: 4 },
    Profile { host: HostProfile::CrossRelay(2.0), horizon: 4 },
    Profile { host: HostProfile::CrossRelay(0.5), horizon: 10 },
    Profile { host: HostProfile::CrossRelay(1.0), horizon: 10 },
    Profile { host: HostProfile::CrossRelay(2.0), horizon: 10 },
];

fn exposed(model: &CohfieldLanguageModelV1, pattern: &[SurfaceSymbol]) -> LanguageState {
    model
        .expose(&LanguageState::initial(), pattern, 64)
        .expect("frozen exposure must be valid")
}

fn fixture(model: &CohfieldLanguageModelV1) -> [LanguageState; 6] {
    let learned_c = exposed(model, &H_C);
    let learned_d = exposed(model, &H_D);
    let learned_loop = exposed(model, &H_LOOP);

    let mut core_c = LanguageState::initial();
    core_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()] =
        learned_c.psi[SurfaceSymbol::A.index()][SurfaceSymbol::C.index()];
    core_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] =
        learned_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()];

    let mut core_d = LanguageState::initial();
    core_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()] =
        learned_d.psi[SurfaceSymbol::A.index()][SurfaceSymbol::D.index()];
    core_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] =
        learned_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()];

    let mut core_l = core_c.clone();
    core_l.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()] =
        learned_loop.psi[SurfaceSymbol::D.index()][SurfaceSymbol::D.index()];

    let mut cut_c = core_c.clone();
    cut_c.psi[SurfaceSymbol::C.index()][SurfaceSymbol::B.index()] = 0.0;
    let mut cut_d = core_d.clone();
    cut_d.psi[SurfaceSymbol::D.index()][SurfaceSymbol::B.index()] = 0.0;
    let zero = LanguageState::initial();

    // Frozen blinded slot order: [R_L, R_D_cut, R_C, R_0, R_D, R_C_cut].
    [core_l, cut_d, core_c, zero, core_d, cut_c]
}

fn apply_host(state: &LanguageState, host: HostProfile) -> LanguageState {
    let mut hosted = state.clone();
    match host {
        HostProfile::Baseline => {}
        HostProfile::BackToA(weight) => {
            hosted.psi[SurfaceSymbol::B.index()][SurfaceSymbol::A.index()] = weight;
        }
        HostProfile::CrossRelay(weight) => {
            hosted.psi[SurfaceSymbol::C.index()][SurfaceSymbol::D.index()] = weight;
            hosted.psi[SurfaceSymbol::D.index()][SurfaceSymbol::C.index()] = weight;
        }
    }
    hosted
}

fn run_context(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    context: SurfaceSymbol,
    continuation_steps: usize,
) -> Vec<[f64; 4]> {
    let mut current = model
        .evolve(state, &LanguageInput::symbol(context), 1.0)
        .expect("context step must be valid");
    let mut records = vec![current.x];
    for _ in 0..continuation_steps {
        current = model
            .evolve(&current, &LanguageInput::zero(), 1.0)
            .expect("zero continuation must be valid");
        records.push(current.x);
    }
    records
}

fn profile_response(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    profile: Profile,
) -> Vec<f64> {
    let hosted = apply_host(state, profile.host);
    let mut out = Vec::with_capacity(4 * (profile.horizon + 1));
    for context in [SurfaceSymbol::A, SurfaceSymbol::B] {
        for x in run_context(model, &hosted, context, profile.horizon) {
            out.push(x[SurfaceSymbol::A.index()]);
            out.push(x[SurfaceSymbol::B.index()]);
        }
    }
    out
}

fn response_family(
    model: &CohfieldLanguageModelV1,
    state: &LanguageState,
    profiles: &[Profile],
) -> Vec<f64> {
    profiles
        .iter()
        .flat_map(|&profile| profile_response(model, state, profile))
        .collect()
}

fn signatures(
    model: &CohfieldLanguageModelV1,
    states: &[LanguageState],
    profiles: &[Profile],
) -> Vec<Vec<f64>> {
    states
        .iter()
        .map(|state| response_family(model, state, profiles))
        .collect()
}

fn partition_by_exact_response(signatures: &[Vec<f64>]) -> Vec<Vec<usize>> {
    let mut classes: Vec<Vec<usize>> = Vec::new();
    for (index, signature) in signatures.iter().enumerate() {
        if let Some(class) = classes.iter_mut().find(|class| {
            signatures[class[0]].as_slice() == signature.as_slice()
        }) {
            class.push(index);
        } else {
            classes.push(vec![index]);
        }
    }
    for class in &mut classes {
        class.sort_unstable();
    }
    classes.sort();
    classes
}

fn same_class_pairs(partition: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for class in partition {
        for (offset, &left) in class.iter().enumerate() {
            for &right in class.iter().skip(offset + 1) {
                pairs.push((left, right));
            }
        }
    }
    pairs
}

fn rich_response(model: &CohfieldLanguageModelV1, state: &LanguageState) -> Vec<f64> {
    let mut out = Vec::new();
    for context in SurfaceSymbol::ALL {
        for x in run_context(model, state, context, 10) {
            out.extend_from_slice(&x);
        }
    }
    out
}

fn euclidean(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}

#[test]
fn cf_lm_008_carrier_is_six_exact_different_states() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    for (left_index, left) in states.iter().enumerate() {
        for (right_index, right) in states.iter().enumerate().skip(left_index + 1) {
            let distance = CohfieldLanguageModelV1::psi_frobenius_distance(left, right);
            assert!(distance > EPS_STATE, "pair ({left_index},{right_index}) state distance {distance}");
            assert_ne!(left.psi, right.psi);
        }
    }
}

#[test]
fn cf_lm_008_short_profiles_recover_preregistered_partition_from_responses_only() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let partition = partition_by_exact_response(&signatures(&model, &states, &SHORT_PROFILES));
    assert_eq!(partition, vec![vec![0, 2, 4], vec![1, 3, 5]]);
}

#[test]
fn cf_lm_008_full_profiles_recover_preregistered_partition_from_responses_only() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let partition = partition_by_exact_response(&signatures(&model, &states, &FULL_PROFILES));
    assert_eq!(partition, vec![vec![0], vec![1, 3, 5], vec![2, 4]]);
}

#[test]
fn cf_lm_008_partition_class_sizes_match_frozen_targets() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let short = partition_by_exact_response(&signatures(&model, &states, &SHORT_PROFILES));
    let full = partition_by_exact_response(&signatures(&model, &states, &FULL_PROFILES));
    let mut short_sizes: Vec<_> = short.iter().map(Vec::len).collect();
    let mut full_sizes: Vec<_> = full.iter().map(Vec::len).collect();
    short_sizes.sort_unstable();
    full_sizes.sort_unstable();
    assert_eq!(short_sizes, vec![3, 3]);
    assert_eq!(full_sizes, vec![1, 2, 3]);
}

#[test]
fn cf_lm_008_full_partition_strictly_refines_short_without_merges() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let short = partition_by_exact_response(&signatures(&model, &states, &SHORT_PROFILES));
    let full = partition_by_exact_response(&signatures(&model, &states, &FULL_PROFILES));

    for full_class in &full {
        assert!(short.iter().any(|short_class| {
            full_class.iter().all(|member| short_class.contains(member))
        }));
    }
    assert!(full.len() > short.len());

    for (left_index, _) in states.iter().enumerate() {
        for (right_index, _) in states.iter().enumerate().skip(left_index + 1) {
            let short_same = short
                .iter()
                .any(|class| class.contains(&left_index) && class.contains(&right_index));
            let full_same = full
                .iter()
                .any(|class| class.contains(&left_index) && class.contains(&right_index));
            assert!(
                !full_same || short_same,
                "enrichment merged pair ({left_index},{right_index})"
            );
        }
    }
}

#[test]
fn cf_lm_008_same_class_members_remain_exact_different() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let short = partition_by_exact_response(&signatures(&model, &states, &SHORT_PROFILES));
    let full = partition_by_exact_response(&signatures(&model, &states, &FULL_PROFILES));

    for (left, right) in same_class_pairs(&short)
        .into_iter()
        .chain(same_class_pairs(&full))
    {
        assert_ne!(states[left].psi, states[right].psi);
        assert!(
            CohfieldLanguageModelV1::psi_frobenius_distance(&states[left], &states[right])
                > EPS_STATE
        );
    }
}

#[test]
fn cf_lm_008_rich_observer_distinguishes_every_short_class_pair() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let short = partition_by_exact_response(&signatures(&model, &states, &SHORT_PROFILES));
    for (left, right) in same_class_pairs(&short) {
        let distance = euclidean(
            &rich_response(&model, &states[left]),
            &rich_response(&model, &states[right]),
        );
        assert!(distance > EPS_RICH, "pair ({left},{right}) rich distance {distance}");
    }
}

#[test]
fn cf_lm_008_partition_function_uses_only_response_vectors() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let sigs = signatures(&model, &states, &FULL_PROFILES);
    assert_eq!(
        partition_by_exact_response(&sigs),
        partition_by_exact_response(&sigs)
    );
}

#[test]
fn cf_lm_008_construction_signatures_and_partitions_are_deterministic() {
    let model = CohfieldLanguageModelV1::default();
    let left = fixture(&model);
    let right = fixture(&model);
    let left_short = signatures(&model, &left, &SHORT_PROFILES);
    let right_short = signatures(&model, &right, &SHORT_PROFILES);
    let left_full = signatures(&model, &left, &FULL_PROFILES);
    let right_full = signatures(&model, &right, &FULL_PROFILES);
    assert_eq!(left_short, right_short);
    assert_eq!(left_full, right_full);
    assert_eq!(
        partition_by_exact_response(&left_short),
        partition_by_exact_response(&right_short)
    );
    assert_eq!(
        partition_by_exact_response(&left_full),
        partition_by_exact_response(&right_full)
    );
}

#[test]
fn cf_lm_008_matches_preregistered_response_family_cross_checks() {
    let model = CohfieldLanguageModelV1::default();
    let states = fixture(&model);
    let short = signatures(&model, &states, &SHORT_PROFILES);
    let full = signatures(&model, &states, &FULL_PROFILES);

    assert_eq!(full[2], full[4]);
    assert_eq!(full[1], full[5]);
    assert_eq!(full[1], full[3]);

    let c_l = euclidean(&full[2], &full[0]);
    let c_cut = euclidean(&full[2], &full[5]);
    let c_cut_short = euclidean(&short[2], &short[5]);
    let rich_cut_zero = euclidean(
        &rich_response(&model, &states[5]),
        &rich_response(&model, &states[3]),
    );

    assert!((c_l - 0.130_750_346_526_305_02).abs() < REGRESSION_TOL);
    assert!((c_cut - 0.159_569_540_935_668_7).abs() < REGRESSION_TOL);
    assert!((c_cut_short - 0.105_617_753_467_665_29).abs() < REGRESSION_TOL);
    assert!((rich_cut_zero - 0.169_387_841_508_85).abs() < REGRESSION_TOL);
}
