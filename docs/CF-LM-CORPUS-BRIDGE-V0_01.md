# CF-LM Corpus Bridge v0.01

## Status

**Prototype boundary and byte-visible adaptation harness**

Branch: `agent/cf-lm-corpus-bridge-v001`

This branch is deliberately separate from the Teacher Bridge capability ladder (`v0.01` through `v0.05`). The Teacher Bridge experiments ask what relational/abstraction capabilities CF-LM can acquire. The Corpus Bridge asks how governed real training material can enter CF-LM without importing teacher internals or bypassing the Training Database.

## Source contract

The expected producer is the Training-data branch:

```text
agent/cflm-teacher-data-bridge-v001
```

Its exporter consumes only explicitly admitted Training Database examples and writes split-separated `.cflm` packs.

The binary contract is:

```text
magic: CFLM-TEACHER-DATA-V001\n
repeat:
  uint64-be input_length
  input bytes
  uint64-be target_length
  target bytes
```

Record boundaries are explicit. CF-LM therefore never learns a synthetic transition from the end of one admitted example into the start of another.

## Representation boundary

The bridge exposes raw UTF-8 bytes to this experimental profile. It does not import:

- tokenizer IDs;
- embeddings;
- neural weights;
- teacher hidden states;
- logits;
- chain-of-thought/reasoning fields.

This does not mean byte identity is the final CF-LM representation. It is only a reversible external serialization boundary for the first governed corpus experiment.

## Byte-visible model

`ByteLanguageModel` generalizes the original small finite-symbol exposure law to a 256-value byte surface.

Persistent relation state is a directed 256x256 matrix `Psi`. For each observed adjacent byte pair `(i,j)`:

```text
Psi <- (1-rho) Psi
Psi[i,j] <- Psi[i,j] + eta
```

with the same default coefficients used by the early finite-symbol CF-LM language profile:

```text
rho = 0.02
eta = 0.08
```

Fast continuation remains:

```text
X_next = 0.50 X + 0.50 input + 0.20 Psi^T X
```

The implementation is intentionally simple. It establishes an ingestion/adaptation substrate, not a claim of competent language modeling.

## Training and evaluation separation

`train.cflm` is the only split permitted to update persistent relation state.

For validation/test/holdout records, the caller presents only the input bytes to construct transient state, removes the teacher, and measures continuation against the withheld target without modifying `Psi`.

The Rust API preserves this distinction:

```text
train(records, epochs)       -> persistent adaptation
present_input(state, input)  -> transient prompt state only
teacher_off(start, steps)    -> zero-input continuation
```

## Tests

`tests/corpus_bridge_v001.rs` checks:

1. exact pack parsing without a tokenizer or JSON dependency;
2. fail-closed malformed-pack handling;
3. persistent relation formation from visible byte experience;
4. teacher-off continuation using learned relations;
5. no-adaptation control;
6. record-boundary isolation;
7. UTF-8 multi-byte visibility as bytes rather than semantic symbol assignment;
8. non-mutating teacher-off evaluation;
9. deterministic repeatability.

The full inherited CF-LM suite must remain green.

## Claim ceiling

A positive v0.01 Corpus Bridge result supports only:

> Governed input/target examples can be ingested as record-bounded UTF-8 byte experience, can update CF-LM-owned persistent relation state, and can influence teacher-off continuation without transferring teacher internal representations.

It does not establish:

- natural-language semantics;
- grammar induction;
- long-context competence;
- abstraction from real corpus data;
- reasoning;
- full-corpus training success;
- superiority to neural language models.

The next empirical milestone should use a small explicitly admitted Training Database slice, freeze it by hash, train only on its train split, and evaluate untouched validation/holdout records teacher-off before any attempt to scale toward the full archive.
