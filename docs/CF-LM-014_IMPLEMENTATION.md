# CF-LM-014 Implementation Boundary

Status: staged, executable PASS unset
Protocol parent: `docs/CF-LM-014_PROTOCOL.md`
Verified parent evidence: CF-LM-013 `c6472bf8b15408cc0adadc4f781422996127f582`

## Implementation scope

CF-LM-014 adds `language_v7.rs` as an additive language-domain version after V6.

V7 preserves all V6 State components and adds two append-only histories:

- `outcome_applicability_history` — consequence-grounded context/profile inference records;
- `outcome_selection_history` — later held-out context selections derived from outcome-grounded applicability prototypes.

V1-V6 and CF-ACP semantics are unchanged.

## Acquisition boundary

`RecordObservedConsequence([f64;5])` carries no profile identity.

For each already-assessed profile, V7 generates a counterfactual five-point A-coordinate consequence prediction from cloned State, compares it with the externally supplied domain observation, and infers a unique winning profile only when the frozen support and margin thresholds hold.

Candidate prediction does not mutate actual State.

Outcome-grounded acquisition appends history but does not select a runtime profile.

## Provenance boundary

Each consequence-grounded applicability record preserves:

- context epoch;
- recognized context activity;
- observed continuation signature;
- every candidate prediction error;
- inferred profile;
- acquisition epoch.

Derived context prototypes are reconstructed from this history and are not independently mutable State.

The implementation validates that each outcome-applicability record still matches the context activity of the context epoch it cites.

## Governance firewall

The observed continuation is domain-model experience. It is not promoted to canonical CohBit Evidence or Verification.

Prediction error is not canonical valuation, admissibility, policy, or authority.

No CF-LM-014 result grants execution or commitment permission.

## Test surface

`tests/language_consequence_grounded_applicability.rs` adds ten preregistered tests covering:

- V6->V7 migration;
- counterfactual consequence prediction and actual-State nonmutation;
- unlabeled consequence-grounded applicability acquisition;
- history-derived prototypes;
- held-out applicability generalization and inversion of the old projection heuristic;
- ambiguous outcome fail-closed behavior;
- unsupported outcome fail-closed behavior;
- no-experience fail-closed behavior;
- causal transfer under outcome-derived applicability;
- provenance and deterministic replay.

## Local gate

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Verified parent suite: 157 tests.
New CF-LM-014 tests: 10.
Expected full target if implementation conforms: 167/167.

No CF-LM-014 PASS is claimed until the complete local gate succeeds on the frozen protocol.
