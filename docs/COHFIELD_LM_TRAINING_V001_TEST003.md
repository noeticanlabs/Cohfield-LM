# Cohfield-LM Training v0.01 — Test 003

Status: reference execution complete; exact repository-native Rust replay pending.

## Purpose

Test 003 is the first executable runtime under the Training v0.01 contract. Its purpose is to test whether a transparent persistent relational model can use a held-out source distinction together with a typed relational context to recover the correct consequence more often than matched shortcut controls.

This is not a semantic-understanding test. It is an interpretable relational-selection test.

## Runtime

The persistent state is a feature-to-target relation table. Training evidence updates target counts for four transparent feature classes:

1. source node type;
2. context edge type;
3. source-type/context conjunction;
4. raw source-label byte-pairs conditioned on context edge type.

No tokenizer, embedding model, hidden teacher state, Transformer attention, gradient descent, or validation/test adaptation is used.

For feature f and target y, training creates persistent count N(f,y). At inference, each observed feature contributes a bounded evidence score

    s_f(y) = ln(1 + N(f,y)) / ln(2 + sum_z N(f,z)).

The consequence score is

    S(y | source, context) = sum_f s_f(y),

with deterministic target-prior and lexical tie breaks only after equal scores.

## Why this runtime is interpretable

Every contribution to a consequence can be traced to a named feature class and an observed training target. The model therefore makes it possible to record what relation changed, which evidence produced it, and whether removing the context-conditioned relations changes held-out behavior.

## Frozen supported-target surface

As established by Test 002, exact-target accuracy is evaluated on held-out examples whose target identity appeared in the training target vocabulary. Previously unseen target identities remain an open-target surface and are not silently counted as ordinary closed-vocabulary failures.

## Controls

The runtime is evaluated against:

- context ablation;
- deterministic wrong-context substitution;
- deterministic shuffled-context substitution;
- context-only majority baseline;
- source-type-only majority baseline;
- source-type + context majority baseline.

Paired-source accuracy additionally requires both contexts for one held-out source to recover their respective targets.

## Reference full-curriculum execution

The frozen Test 001 curriculum was converted losslessly to five-column TSV records:

    source_id<TAB>source_type<TAB>source_label_utf8_hex<TAB>context_edge_type<TAB>target_id

Training examples: 4,540.
Training target vocabulary: 530 target identities.
Persistent feature relations: 20,287 feature keys.

Dataset hashes:

- train.tsv: `2a556797023f7d0b3181aa761127d6510a1801416bc51808faeea0e63c9f9cbe`
- validation.tsv: `042f2623260d87092bb24eace3bdee8ff59e2dac6484fa65c77e1204448590ac`
- test.tsv: `23fb3bb4c023ab9d68939b56cd824ef1684b928014d740afd8fea86fff98bc07`

### Validation supported-target surface

- supported examples: 534
- true-context exact-target accuracy: 0.5149812734082397
- true-context mean rank: 6.700374531835206
- context-ablation accuracy: 0.2247191011235955
- wrong-context accuracy: 0.0
- shuffled-context accuracy: 0.04119850187265917
- context-majority baseline: 0.39325842696629215
- source-type baseline: 0.2247191011235955
- source-type + context baseline: 0.3951310861423221
- paired supported sources: 255
- both contexts correct: 78
- paired-source accuracy: 0.3058823529411765

### Test supported-target surface

- supported examples: 545
- true-context exact-target accuracy: 0.5486238532110091
- true-context mean rank: 5.027522935779817
- context-ablation accuracy: 0.20550458715596331
- wrong-context accuracy: 0.0
- shuffled-context accuracy: 0.029357798165137616
- context-majority baseline: 0.3853211009174312
- source-type baseline: 0.20550458715596331
- source-type + context baseline: 0.3871559633027523
- paired supported sources: 258
- both contexts correct: 87
- paired-source accuracy: 0.3372093023255814

## Interpretation

The reference execution produces a strong context-sensitive effect. True context exceeds the strongest simple majority shortcut by approximately 0.120 on validation and 0.161 on test. Removing context reduces accuracy by approximately 0.290 on validation and 0.343 on test. Wrong or shuffled contexts severely disrupt consequence resolution.

This supports the narrower statement that source observation plus typed context contains predictive relational structure that a transparent persistent model can acquire and use on source-disjoint held-out cases.

The paired-source result is especially important: the model recovers both different consequences for 30.6% of eligible validation sources and 33.7% of eligible test sources. This is materially stronger than a single-context success measure, but it is far from complete relational resolution.

## Scientific boundary

This result does not establish mathematical understanding, theorem proving, semantic language competence, or general intelligence. It also does not yet constitute the final repository-native evidence receipt because the exact Rust executable committed in `src/bin/training_v001_test003.rs` has not yet been replayed against the hashed full curriculum in GitHub Actions.

Until that replay is complete, the status is:

    REFERENCE_RUNTIME_EXECUTED — RUST_REPO_NATIVE_PENDING

## Next gate

Test 004 should package the exact hashed curriculum into a reproducible repository/CI-accessible artifact, execute `training_v001_test003` without changing its learning law, compare the exact Rust metrics to this frozen reference receipt, and only then decide whether Training v0.01 Test 003 is sealed as a native PASS.
