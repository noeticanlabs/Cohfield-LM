# CF-LM-010 Implementation Boundary

Status: staged implementation; local PASS/FAIL unset.

## Parent evidence

Verified CF-LM-009 head: `4ac241a09bccccb2e91530f91983a9e1a915f736`.

## Added implementation

- `src/profiles/language_v3.rs`
- `src/profiles/mod.rs` export
- `tests/language_profile_scoped_equivalence.rs`

V1 and V2 production code are unchanged.

## V3 State purpose

V3 separates:

```text
sequential relations
active consequence-equivalence relation
active assessment profile
append-only assessment history
```

Each assessment record binds the epoch, symbol pair, exact profile, measured consequence distance, and equivalent/non-equivalent disposition.

The assessment witness is calculated with the currently active consequence-equivalence relation disabled so the relation cannot provide its own confirming measurement.

## Important firewall

`ConsequenceEquivalenceAssessment` is model-internal derivation State. It is **not** canonical CohBit Evidence, Verification, authority, or a receipt.

## Frozen protocol implementation

The test suite implements the preregistered P_AB / P_BC revision protocol, eight isolated C->A teaching episodes, transfer/no-transfer/reacquisition controls, history immutability, and deterministic regression checks.

No CF-LM-010 PASS is claimed until the local gate runs:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Existing verified suite: 117 tests. CF-LM-010 adds 10 tests. Expected full count if all tests compile and pass: 127/127.
