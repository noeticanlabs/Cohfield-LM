# CF-LM Corpus Pilot v0.01

## Status

**IMPLEMENTATION PASS — governed real-corpus run pending**

Branch: `agent/cf-lm-corpus-pilot-v001`

This milestone is the first CF-LM experiment designed to consume the governed byte packs produced by the Training-data repository and evaluate an untouched holdout after the teacher/data source is removed.

It deliberately does **not** claim language learning yet. The experiment first asks whether the existing first-order CF-LM exposure mechanism carries any prompt-conditioned signal into held-out real dialogue.

## Why a new runtime was required

`corpus_bridge_v001` reproduced the original CF-LM V1 law literally. On every observed byte pair it multiplied all 65,536 persistent byte relations by the global decay factor before updating one edge. That is mathematically clean but computationally unsuitable for even a small real corpus.

v0.01 therefore implements the **same decay/update law lazily**.

For each relation it stores:

```text
weight at last update
last adaptation step
```

At step `t`, its current value is reconstructed as:

```text
weight(t) = stored_weight * (1 - psi_decay)^(t - last_update_step)
```

When an edge is observed, only that edge is materialized, decayed to the new global adaptation step, and incremented by `psi_gain`.

This changes the implementation cost, not the persistent learning equation. A frozen regression test compares the lazy implementation directly against the dense v0.01 implementation on the same curriculum and requires relation equality within `1e-12`.

## Frozen model

```text
surface states: 256 visible byte values
beta = 0.50
input_gain = 0.50
relational_gain = 0.20
psi_decay = 0.02
psi_gain = 0.08
```

Training still observes only adjacent visible bytes inside each governed record. Record boundaries never create learned cross-example edges.

No tokenizer, embedding model, teacher hidden state, logits, or reasoning trace is supplied.

## Pilot data contract

The paired Training-data branch is:

```text
agent/cflm-corpus-pilot-v001
```

Its frozen defaults request approximately:

```text
128 train records
32 holdout records
```

from explicitly admitted, non-personal, unassigned `multi_turn_dialogue` examples, split by whole `split_group` with deterministic hash ordering. The holdout is frozen before training groups are selected.

## Evaluation

For each holdout record, the teacher is absent and adaptation is disabled. The model receives only the visible input bytes and then performs one zero-input continuation step.

The primary observable is activation and rank of the **first withheld target byte**.

v0.01 reports four matched views:

1. true training pairs;
2. rotated/shuffled training targets, preserving the same input and target marginals while breaking pairing;
3. rotated holdout prompts with the correct target retained;
4. the common `\n\nAssistant: ` answer boundary alone.

An untrained state is also measured.

Reported diagnostics include:

```text
mean correct first-byte activation
mean rank
Top-1-or-tied rate
true-vs-shuffled-target activation delta
actual-vs-rotated-prompt activation delta
actual-vs-boundary-only activation delta
L1 field distance to rotated-prompt control
L1 field distance to boundary-only control
```

## Why the boundary control matters

All dialogue examples produced by the governed exporter end in the same visible answer boundary:

```text
\n\nAssistant: 
```

A first-order relation model can therefore learn strong global relations from that final space to common first answer bytes without using the substantive prompt at all.

The boundary-only and rotated-prompt controls are mandatory so such global answer statistics cannot be mistaken for prompt-conditioned language learning.

A null conditional result is scientifically useful. It would show that the first-order byte relation substrate can absorb corpus statistics but lacks sufficient history/context geometry for real prompt-conditioned continuation.

## Executable runner

After Training-data generates `train.cflm` and `holdout.cflm`:

```shell
cargo run --bin corpus_pilot_v001 -- \
  /path/to/train.cflm \
  /path/to/holdout.cflm \
  1
```

The runner prints the frozen pilot metrics as JSON-compatible output.

## CI gate

GitHub Actions run `33292036417` is green at branch head `3e225f972e7be52678e3408f3a6c47bec510a387`:

```text
rustfmt new pilot surface: PASS
corpus_pilot_v001: 8/8 PASS
pilot CLI compile: PASS
full cargo test --all-targets: PASS
```

The gate includes direct dense-vs-lazy equivalence, exact adaptation-event accounting, record-boundary isolation, teacher-off nonmutation, shuffled-target construction, explicit answer-boundary diagnosis, report construction, and no-adaptation/empty-holdout controls.

## Claim ceiling

The implementation establishes that CF-LM can efficiently preserve its original first-order byte adaptation law at pilot corpus scale and that the real-data experiment has matched controls capable of distinguishing global answer-byte statistics from prompt-conditioned continuation.

The real governed corpus has **not yet been run through this pilot**, so there is currently no empirical claim of real-language transfer, semantics, grammar induction, or language competence.

## Decision rule after the real run

If true paired training materially exceeds shuffled-target, rotated-prompt, and boundary-only controls, the next experiment should test whether the signal survives larger and structurally harder holdouts.

If it does not, the expected next architectural step is a path/history-conditioned corpus substrate rather than more data or more epochs. That would test whether the missing variable is representational memory rather than exposure volume.
