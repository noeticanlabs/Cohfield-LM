# CF-LM Navier-Stokes Native v0.01

## Status

**REPO-NATIVE REPLAY PASS — FIRST-ORDER CONTEXT LIMIT CONFIRMED**

Branch: `agent/cf-lm-navier-stokes-native-v001`

GitHub Actions run: `33415527576`

Head commit: `063d39a91a51d81814ab3da6ca61cc86ee551389`

## Data contract

The replay consumes deterministic `CFLM-TEACHER-DATA-V001` packs derived from the machine-readable Navier-Stokes formula dataset.

Visible task:

```text
Formula: <formula>
Family: 
```

Target: UTF-8 bytes of `primary_family`.

Split hashes:

- train: 204 records, SHA-256 `4d0ec2bea1b856fe513f1af5469f0757d12245a1e5fb163693d275420e872fe5`
- validation: 23 records, SHA-256 `e97b96d4bb26044266759f9797eac0facfe033f976fd095e06600b295a81568b`
- test: 36 records, SHA-256 `8324b3f9aa978f7959f4363ece7378480d2ba9c87e53382c236e649b78d06b64`

The workflow verifies all hashes before execution.

## Runtime

The run uses the exact repository `corpus_pilot_v001` binary and inherited source on the `agent/cf-lm-corpus-pilot-v001` lineage. It evaluates epochs 1, 4, and 16 against both validation and test holdouts.

## Native results

The metrics are invariant across epochs 1, 4, and 16 at printed precision.

### Validation

```text
true mean correct activation        0.00496987051149944
mean rank                          21.08695652173912904
top-1-or-tied rate                  0.0
shuffled-target activation          0.00534667410202950
pairing activation delta           -0.00037680359053007
rotated-prompt activation           0.00494611933866998
prompt-pair activation delta        0.00002375117282945
boundary-only activation            0.00496670140756271
boundary activation delta           0.00000316910393672
```

### Test

```text
true mean correct activation        0.00517209605645841
mean rank                          20.58333333333333215
top-1-or-tied rate                  0.0
shuffled-target activation          0.00472364420016112
pairing activation delta            0.00044845185629729
rotated-prompt activation           0.00515297384955880
prompt-pair activation delta        0.00001912220689961
boundary-only activation            0.00516885820375921
boundary activation delta           0.00000323785269920
```

## Interpretation

The first-order byte-relation substrate absorbs corpus statistics but does not demonstrate robust formula-conditioned classification. Validation pairing advantage is negative, test pairing advantage is small, prompt-conditioned deltas are tiny, and top-1 accuracy remains zero.

Increasing exposure from 1 to 16 epochs does not improve the reported holdout metrics. Under the preregistered corpus-pilot decision rule, the next architectural variable should therefore be history/path-conditioned representation rather than additional epochs alone.

## Repository gate

The same workflow also ran `cargo test --all-targets`; the inherited regression suite passed. Training receipts were uploaded as artifact `cflm-navier-stokes-native-v001-results` with artifact digest:

`sha256:482f6015ef4fd56531aa576a4477697a5b3c32764f671093812c62609f073a9d`

## Claim ceiling

This replay establishes reproducible execution of the frozen first-order CF-LM corpus law on the Navier-Stokes formula-family task and confirms a context/history limitation under this task. It does not establish semantic understanding, mathematical reasoning, general language competence, or that history-conditioning will necessarily solve the limitation.
