use cohfield_lm::corpus_bridge_v001::CorpusRecord;
use cohfield_lm::corpus_pilot_v002::HistoryModel;
use cohfield_lm::corpus_pilot_v003::TraceModel;

fn record(input: &[u8], target: &[u8]) -> CorpusRecord {
    CorpusRecord { input: input.to_vec(), target: target.to_vec() }
}

#[test]
fn v002_training_is_deterministic_and_path_is_causal() {
    let data = vec![record(b"ab", b"x"), record(b"cb", b"y")];
    let model = HistoryModel::default();
    let a = model.train(&data, 4);
    let b = model.train(&data, 4);
    assert_eq!(a.adaptation_step, b.adaptation_step);
    assert_eq!(HistoryModel::learned_paths(&a), HistoryModel::learned_paths(&b));
    let with_path = model.continuation_field(&a, b"ab", true);
    let without_path = model.continuation_field(&a, b"ab", false);
    assert_ne!(with_path, without_path);
}

#[test]
fn v003_training_is_deterministic_and_trace_is_causal() {
    let data = vec![record(b"abc", b"x"), record(b"dbc", b"y")];
    let model = TraceModel::default();
    let a = model.train(&data, 4);
    let b = model.train(&data, 4);
    assert_eq!(a.adaptation_step, b.adaptation_step);
    assert_eq!(TraceModel::learned_trace_relations(&a), TraceModel::learned_trace_relations(&b));
    let with_trace = model.continuation_field(&a, b"abc", true);
    let without_trace = model.continuation_field(&a, b"abc", false);
    assert_ne!(with_trace, without_trace);
}

#[test]
fn record_boundaries_do_not_create_cross_record_history() {
    let model = HistoryModel::default();
    let separate = model.train(&[record(b"a", b"b"), record(b"c", b"d")], 1);
    let joined = model.train(&[record(b"ab", b"cd")], 1);
    assert!(HistoryModel::learned_paths(&joined) > HistoryModel::learned_paths(&separate));
}
