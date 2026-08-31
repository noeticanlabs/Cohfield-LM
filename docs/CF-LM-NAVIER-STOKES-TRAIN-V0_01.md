# CF-LM Navier–Stokes Training v0.01

## Status

**SANDBOX TRAINING RUN COMPLETE — repo-native Rust replay pending**

Branch: `agent/cf-lm-navier-stokes-train-v001`

Parent: `agent/cf-lm-corpus-pilot-v001` at `4bd35ea0a3406ec201be64ec5db909cd920286a2`.

## Dataset

Source archive: `Navier_Stokes_Machine_Readable_MultiZip_DAT_JSONL_Dataset_2026-08-31.zip`.

Dataset manifest reports:

- 263 formula records;
- 204 train / 23 validation / 36 test;
- 15 formula families;
- 4 status classes;
- deterministic split by formula SHA-256 prefix;
- dataset validation status PASS.

## Training task

The run uses a visible-byte formula classification task:

```text
input  = "Formula: <formula_raw>\nFamily: "
target = primary_family UTF-8 bytes
```

No tokenizer, embedding model, hidden teacher state, logits, chain-of-thought, or semantic label vector is injected. The family string is only the visible supervised target.

The adaptation law matches the lazy first-order byte relation law frozen by CF-LM Corpus Pilot v0.01:

```text
surface states = 256
beta = 0.50
input_gain = 0.50
relational_gain = 0.20
psi_decay = 0.02
psi_gain = 0.08
```

The sandbox runner independently reimplemented that frozen law for this dataset. Because this run did not execute the repository Rust binary byte-for-byte, the result is **equivalent-law experimental evidence**, not yet the repo-native evidence record.

## Training exposure

At one epoch:

```text
204 train records
21,091 adaptation events
843 nonzero byte relations
```

Four epochs produced 84,364 adaptation events and sixteen epochs produced 337,456 events. The measured validation/test continuation metrics were unchanged at the shown precision across 1, 4, and 16 epochs, which is itself diagnostic under the current decay law.

## Holdout results

### Validation, epoch 1

```text
mean correct first-byte activation = 0.004969870511499436
mean rank                          = 21.08695652173913
top-1-or-tied rate                 = 0.0
true-vs-shuffled pairing delta     = -0.00037680359053006776
actual-vs-rotated-prompt delta     = 0.00002375117282945368
actual-vs-boundary-only delta      = 0.00007655830612020038
```

### Test, epoch 1

```text
mean correct first-byte activation = 0.005172096056458408
mean rank                          = 20.583333333333332
top-1-or-tied rate                 = 0.0
true-vs-shuffled pairing delta     = +0.00044845185629728817
actual-vs-rotated-prompt delta     = 0.00001912220689960671
actual-vs-boundary-only delta      = 0.00006407980274767897
```

The trained model has higher mean target-byte activation than the untrained control, but this is not sufficient evidence of formula-conditioned classification. Validation is worse than shuffled-target training, test is only slightly better, prompt-rotation deltas are very small, and top-1 remains zero.

## Scientific interpretation

The correct result is currently **NULL / BOUNDARY EXPOSED**, not training failure.

The first-order byte substrate absorbed corpus statistics, but the run does not show robust prompt-conditioned family prediction. Increasing exposure from 1 to 16 epochs did not improve the measured holdout metrics.

This is consistent with the frozen Corpus Pilot v0.01 decision rule: when true pairing does not materially exceed shuffled-target, rotated-prompt, and boundary controls, more data or more epochs should not be assumed to solve the problem. The next architecture should test path/history-conditioned context rather than repeating the same first-order exposure law.

## Claim ceiling

This run does not establish mathematical understanding, Navier–Stokes reasoning, theorem validation, semantic classification competence, or language competence.

It establishes that the existing CF-LM first-order byte adaptation mechanism can ingest the supplied machine-readable formula corpus at this scale and that its present context geometry is insufficient for robust held-out formula-family prediction under the tested task.

## Next gate

1. reproduce the exact adapter as a repository tool;
2. generate deterministic `.cflm` train/validation/test packs;
3. execute the repository Rust `corpus_pilot_v001` path against those packs;
4. require exact dataset hashes and result receipt;
5. if the null result reproduces, begin CF-LM Corpus Pilot v0.02 with history/path-conditioned state rather than increasing epochs.