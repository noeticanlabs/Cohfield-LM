use cohfield_lm::corpus_bridge_v001::{
    activation, parse_pack, ByteLanguageModel, CorpusPackError, CorpusRecord, MAGIC,
};

const EPS: f64 = 1.0e-12;

fn pack(records: &[(&[u8], &[u8])]) -> Vec<u8> {
    let mut out = Vec::from(MAGIC);
    for (input, target) in records {
        out.extend_from_slice(&(input.len() as u64).to_be_bytes());
        out.extend_from_slice(input);
        out.extend_from_slice(&(target.len() as u64).to_be_bytes());
        out.extend_from_slice(target);
    }
    out
}

#[test]
fn parses_training_data_pack_without_tokenizer_or_json_dependency() {
    let blob = pack(&[
        (b"User: alpha\n\nAssistant: ", b"beta"),
        (b"prompt", b"answer"),
    ]);
    let records = parse_pack(&blob).expect("valid bridge pack must parse");
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].input, b"User: alpha\n\nAssistant: ");
    assert_eq!(records[0].target, b"beta");
    assert_eq!(records[1].input, b"prompt");
    assert_eq!(records[1].target, b"answer");
}

#[test]
fn malformed_packs_fail_closed() {
    assert_eq!(parse_pack(b"wrong").unwrap_err(), CorpusPackError::BadMagic);

    let mut truncated_len = Vec::from(MAGIC);
    truncated_len.extend_from_slice(&[0, 1, 2]);
    assert_eq!(
        parse_pack(&truncated_len).unwrap_err(),
        CorpusPackError::TruncatedInputLength
    );

    let mut truncated_payload = Vec::from(MAGIC);
    truncated_payload.extend_from_slice(&10u64.to_be_bytes());
    truncated_payload.extend_from_slice(b"tiny");
    assert_eq!(
        parse_pack(&truncated_payload).unwrap_err(),
        CorpusPackError::TruncatedInputPayload
    );
}

#[test]
fn training_updates_visible_byte_relations_and_teacher_off_uses_them() {
    let model = ByteLanguageModel::default();
    let input = b"User: x\n\nAssistant: ".to_vec();
    let records = vec![CorpusRecord {
        input: input.clone(),
        target: b"z".to_vec(),
    }];
    let trained = model.train(&records, 64);

    // The visible answer boundary ends in a space, so the first target byte is
    // learned as an ordinary persistent relation, not injected at evaluation.
    assert!(trained.relation(b' ', b'z') > 0.0);

    let start = model.present_input(&trained, &input);
    let trajectory = model.teacher_off(&start, 1);
    assert!(activation(&trajectory[1], b'z') > EPS);
}

#[test]
fn no_adaptation_control_has_no_teacher_off_target_activation() {
    let model = ByteLanguageModel::without_adaptation();
    let input = b"User: x\n\nAssistant: ".to_vec();
    let records = vec![CorpusRecord {
        input: input.clone(),
        target: b"z".to_vec(),
    }];
    let trained = model.train(&records, 64);
    assert!(trained.relation(b' ', b'z').abs() <= EPS);

    let start = model.present_input(&trained, &input);
    let trajectory = model.teacher_off(&start, 1);
    assert!(activation(&trajectory[1], b'z').abs() <= EPS);
}

#[test]
fn record_boundaries_do_not_create_cross_example_relations() {
    let model = ByteLanguageModel::default();
    let records = vec![
        CorpusRecord {
            input: vec![1],
            target: vec![2],
        },
        CorpusRecord {
            input: vec![3],
            target: vec![4],
        },
    ];
    let trained = model.train(&records, 1);
    assert!(trained.relation(1, 2) > 0.0);
    assert!(trained.relation(3, 4) > 0.0);
    assert!(trained.relation(2, 3).abs() <= EPS);
}

#[test]
fn utf8_is_visible_as_bytes_not_as_preassigned_semantic_symbols() {
    let model = ByteLanguageModel::default();
    let input = "π".as_bytes().to_vec();
    let target = "λ".as_bytes().to_vec();
    assert_eq!(input, vec![0xCF, 0x80]);
    assert_eq!(target, vec![0xCE, 0xBB]);

    let trained = model.train(&[CorpusRecord { input, target }], 1);
    assert!(trained.relation(0xCF, 0x80) > 0.0);
    assert!(trained.relation(0x80, 0xCE) > 0.0);
    assert!(trained.relation(0xCE, 0xBB) > 0.0);
}

#[test]
fn teacher_off_evaluation_does_not_mutate_persistent_relations() {
    let model = ByteLanguageModel::default();
    let input = b"User: q\n\nAssistant: ".to_vec();
    let trained = model.train(
        &[CorpusRecord {
            input: input.clone(),
            target: b"r".to_vec(),
        }],
        8,
    );
    let before = trained.psi.clone();
    let start = model.present_input(&trained, &input);
    let _ = model.teacher_off(&start, 4);
    assert_eq!(trained.psi, before);
}

#[test]
fn repeated_training_is_deterministic() {
    let model = ByteLanguageModel::default();
    let records = vec![CorpusRecord {
        input: b"User: deterministic\n\nAssistant: ".to_vec(),
        target: b"yes".to_vec(),
    }];
    assert_eq!(model.train(&records, 16), model.train(&records, 16));
}
