use cohfield_lm::corpus_bridge_v001::{ByteLanguageModel, CorpusRecord};
use cohfield_lm::corpus_pilot_v001::{
    evaluate_holdout, rotate_training_targets, run_pilot, LazyByteModel, LazyByteState,
    ANSWER_BOUNDARY,
};

const EPS: f64 = 1.0e-12;

fn record(input: &[u8], target: &[u8]) -> CorpusRecord {
    CorpusRecord {
        input: input.to_vec(),
        target: target.to_vec(),
    }
}

#[test]
fn lazy_decay_matches_dense_v001_persistent_relation_law() {
    let records = vec![record(b"ab", b"cd"), record(b"ax", b"cy")];
    let dense_model = ByteLanguageModel::default();
    let lazy_model = LazyByteModel::default();
    let dense = dense_model.train(&records, 3);
    let lazy = lazy_model.train(&records, 3);

    for source in [b'a', b'b', b'c', b'd', b'x', b'y'] {
        for target in [b'a', b'b', b'c', b'd', b'x', b'y'] {
            let expected = dense.relation(source, target);
            let observed = lazy_model.relation(&lazy, source, target);
            assert!(
                (expected - observed).abs() <= EPS,
                "relation {source}->{target}: dense={expected} lazy={observed}"
            );
        }
    }
}

#[test]
fn lazy_training_counts_exact_adaptation_events_without_cross_record_edges() {
    let model = LazyByteModel::default();
    let records = vec![record(&[1], &[2]), record(&[3], &[4])];
    let trained = model.train(&records, 7);
    assert_eq!(
        trained.adaptation_step,
        LazyByteModel::adaptation_events(&records, 7)
    );
    assert!(model.relation(&trained, 1, 2) > 0.0);
    assert!(model.relation(&trained, 3, 4) > 0.0);
    assert!(model.relation(&trained, 2, 3).abs() <= EPS);
}

#[test]
fn evaluation_is_teacher_off_and_does_not_mutate_persistent_state() {
    let model = LazyByteModel::default();
    let train = vec![record(b"User: x\n\nAssistant: ", b"alpha")];
    let holdout = vec![record(b"User: y\n\nAssistant: ", b"alpha")];
    let trained = model.train(&train, 8);
    let before = trained.clone();
    let metrics = evaluate_holdout(&model, &trained, &holdout);
    assert_eq!(metrics.samples, 1);
    assert_eq!(trained, before);
}

#[test]
fn rotated_target_control_changes_only_training_pairing() {
    let records = vec![
        record(b"prompt-a", b"alpha"),
        record(b"prompt-b", b"beta"),
        record(b"prompt-c", b"gamma"),
    ];
    let rotated = rotate_training_targets(&records);
    assert_eq!(rotated[0].input, records[0].input);
    assert_eq!(rotated[1].input, records[1].input);
    assert_eq!(rotated[2].input, records[2].input);
    assert_eq!(rotated[0].target, records[1].target);
    assert_eq!(rotated[1].target, records[2].target);
    assert_eq!(rotated[2].target, records[0].target);
}

#[test]
fn common_answer_boundary_is_an_explicit_first_order_boundary_control() {
    let model = LazyByteModel::default();
    let mut input_a = b"User: one".to_vec();
    input_a.extend_from_slice(ANSWER_BOUNDARY);
    let mut input_b = b"User: two".to_vec();
    input_b.extend_from_slice(ANSWER_BOUNDARY);
    let train = vec![record(&input_a, b"apple"), record(&input_b, b"berry")];
    let trained = model.train(&train, 4);

    let boundary_last = *ANSWER_BOUNDARY.last().unwrap();
    assert!(model.relation(&trained, boundary_last, b'a') > 0.0);
    assert!(model.relation(&trained, boundary_last, b'b') > 0.0);

    // There is no direct first-order prompt-identity shortcut to the target.
    assert!(model.relation(&trained, b'e', b'a').abs() <= EPS);
    assert!(model.relation(&trained, b'o', b'b').abs() <= EPS);
}

#[test]
fn pilot_report_exposes_true_shuffled_prompt_and_boundary_controls() {
    let mut p1 = b"User: ax".to_vec();
    p1.extend_from_slice(ANSWER_BOUNDARY);
    let mut p2 = b"User: by".to_vec();
    p2.extend_from_slice(ANSWER_BOUNDARY);
    let mut h1 = b"User: az".to_vec();
    h1.extend_from_slice(ANSWER_BOUNDARY);
    let mut h2 = b"User: bz".to_vec();
    h2.extend_from_slice(ANSWER_BOUNDARY);

    let train = vec![record(&p1, b"alpha"), record(&p2, b"beta")];
    let holdout = vec![record(&h1, b"alpha"), record(&h2, b"beta")];
    let report = run_pilot(&train, &holdout, 8);

    assert_eq!(report.true_trained.samples, 2);
    assert_eq!(report.shuffled_target_trained.samples, 2);
    assert_eq!(report.untrained.samples, 2);
    assert!(report.true_trained.mean_rank.is_finite());
    assert!(report.pairing_activation_delta.is_finite());
    assert!(report.prompt_pair_activation_delta.is_finite());
    assert!(report.boundary_activation_delta.is_finite());
}

#[test]
fn no_adaptation_state_contains_no_persistent_relations() {
    let model = LazyByteModel::without_adaptation();
    let train = vec![record(b"abc", b"def")];
    let state = model.train(&train, 5);
    assert_eq!(state.adaptation_step, 0);
    for source in b'a'..=b'f' {
        for target in b'a'..=b'f' {
            assert!(model.relation(&state, source, target).abs() <= EPS);
        }
    }
}

#[test]
fn empty_holdout_fails_closed_to_zero_sample_metrics_not_false_positive() {
    let model = LazyByteModel::default();
    let metrics = evaluate_holdout(&model, &LazyByteState::initial(), &[]);
    assert_eq!(metrics.samples, 0);
    assert_eq!(metrics.mean_correct_activation, 0.0);
    assert_eq!(metrics.top1_or_tied_rate, 0.0);
}
