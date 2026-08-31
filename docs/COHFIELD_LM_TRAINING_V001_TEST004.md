# Cohfield-LM Training v0.01 — Test 004 Native Replay and Seal Gate

Status: **PRE-SEALED — exact repo-native Rust replay pending**

Test 004 does not introduce a new learning law. It freezes the Test 003 implementation and asks whether the committed Rust executable reproduces the already frozen reference execution on the byte-identical curriculum.

## 1. Scientific question

Can the repository-native Rust implementation of `src/bin/training_v001_test003.rs`, without any change to the learning law, reproduce the frozen Test 003 metrics from the exact frozen training, validation, and test files?

A PASS is a reproducibility result. It is not a new capability claim.

## 2. Frozen runtime

Executable:

`src/bin/training_v001_test003.rs`

The implementation must remain unchanged during the parity run. Any code change resets Test 004 and requires a new source identity.

The runtime learns transparent persistent feature-target counts over:

- source type;
- typed relational context;
- source-type/context conjunction;
- lower-cased raw source-label byte pairs conditioned on context.

No tokenizer, embeddings, teacher hidden state, Transformer attention, gradient descent, or holdout adaptation are introduced by Test 004.

## 3. Frozen curriculum identities

Expected SHA-256:

- `train.tsv`: `2a556797023f7d0b3181aa761127d6510a1801416bc51808faeea0e63c9f9cbe`
- `validation.tsv`: `042f2623260d87092bb24eace3bdee8ff59e2dac6484fa65c77e1204448590ac`
- `test.tsv`: `23fb3bb4c023ab9d68939b56cd824ef1684b928014d740afd8fea86fff98bc07`

Expected cardinalities:

- training examples: 4,540
- learned feature keys: 20,287
- target vocabulary: 530

The native run is invalid if any curriculum hash differs.

## 4. Frozen parity targets

### Validation supported-target surface

- true-context accuracy: `0.5149812734082397`
- true-context mean rank: `6.700374531835206`
- context-ablation accuracy: `0.2247191011235955`
- wrong-context accuracy: `0.0`
- shuffled-context accuracy: `0.04119850187265917`
- paired supported sources: `255`
- paired both-correct: `78`
- paired accuracy: `0.3058823529411765`
- context-majority baseline: `0.39325842696629215`
- source-type-majority baseline: `0.2247191011235955`
- source-type/context-majority baseline: `0.3951310861423221`

### Test supported-target surface

- true-context accuracy: `0.5486238532110091`
- true-context mean rank: `5.027522935779817`
- context-ablation accuracy: `0.20550458715596331`
- wrong-context accuracy: `0.0`
- shuffled-context accuracy: `0.029357798165137616`
- paired supported sources: `258`
- paired both-correct: `87`
- paired accuracy: `0.3372093023255814`
- context-majority baseline: `0.3853211009174312`
- source-type-majority baseline: `0.20550458715596331`
- source-type/context-majority baseline: `0.3871559633027523`

Floating-point parity should be checked with an absolute tolerance no larger than `5e-12` for the printed metrics.

## 5. Required native execution

The seal run must execute, from the repository source identity being sealed:

```text
cargo fmt --all -- --check
cargo test --all-targets
cargo run --release --bin training_v001_test003 -- train.tsv validation.tsv test.tsv
```

The run must emit:

1. repository commit SHA;
2. SHA-256 of `src/bin/training_v001_test003.rs`;
3. all three dataset hashes;
4. exact Rust JSON result;
5. parity-gate result;
6. deterministic replay result from a second independent invocation.

## 6. PASS gate

Test 004 passes only if all of the following are true:

- repository compile/test gates pass;
- all frozen dataset hashes match;
- the Rust executable completes without modification;
- all frozen metrics match within the declared tolerance;
- paired-source counts match exactly;
- two independent Rust invocations produce byte-identical JSON results;
- the seal receipt records source, data, result, and execution identities.

If any item fails, status remains `NATIVE_REPLAY_NOT_SEALED` and the discrepancy must be investigated before changing the learning law.

## 7. Current status

The reference Test 003 execution and its curriculum hashes are frozen. The repository contains the Rust implementation. The present environment does not contain a Rust toolchain and therefore cannot itself establish the repo-native Rust parity result. Test 004 is consequently **pre-sealed, not PASS** until the exact native execution receipt exists.

This distinction is intentional: reference-algorithm agreement is evidence, but it is not substituted for execution of the implementation being claimed.

## 8. Claim boundary after PASS

A successful Test 004 may support the following narrow statement:

> The committed Cohfield-LM Training v0.01 Test 003 Rust implementation reproducibly realizes the frozen context-conditioned relational-selection reference algorithm on the byte-identical controlled mathematical-graph curriculum.

The previously measured contextual-selection effect remains evidence of controlled relational discrimination. Test 004 itself does not establish semantic understanding, mathematical reasoning, theorem proving, language competence, or general intelligence.
