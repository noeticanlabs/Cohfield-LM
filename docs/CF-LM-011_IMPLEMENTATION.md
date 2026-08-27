# CF-LM-011 — Multi-Profile Internal Equivalence Coexistence and Reversible Selection

**Status:** implementation complete; local conformance pending  
**Protocol:** `docs/CF-LM-011_PROTOCOL.md`  
**Contract:** `docs/CF-LM-IC-10.md`  
**Verified parent:** CF-LM-010 `3dd0fd7b6980ab9b72285e897c144a58ae73b921`  

## Implementation boundary

CF-LM-011 introduces `CohfieldLanguageModelV4` and `LanguageStateV4` as a versioned downstream language-domain extension. V1, V2, V3, and CF-ACP semantics are unchanged.

V4 replaces V3's duplicated mutable active-equivalence matrix with:

```text
sequential
selected_profile
assessment_history
```

Assessment history is the single source of truth for profile-scoped internal consequence equivalence. Runtime equivalence is derived from the latest complete six-pair epoch for the selected profile.

## Separation of operations

V4 distinguishes:

```text
AssessConsequenceEquivalence(profile)
```

from:

```text
SelectConsequenceProfile(profile)
```

Assessment appends a new epoch and preserves selection. Selection changes only `selected_profile` and is allowed only for a profile with a complete stored assessment.

Neither operation is CohBit Verification, Policy, Authority, Execution, Commitment, or Receipt.

## Witness discipline

Assessment witness generation clears `selected_profile` in the local witness State. This preserves the CF-LM-010 rule that an internal abstraction may not participate in the measurement used to justify itself.

## Migration

`migrate_from_v3` preserves:

- `X`;
- `Theta`;
- sequential relations;
- full assessment history;
- V3 active profile as V4 selected profile.

When V3 has an active profile, migration derives that profile's latest relation from history and requires it to equal V3's stored active-equivalence matrix. A mismatch fails closed.

## Preregistered tests

`tests/language_multiprofile_coexistence.rs` adds ten tests:

1. V3 -> V4 migration preserves substrate/history/selection and runtime equivalence;
2. `P_AB` and `P_BC` assessments coexist without implicit selection;
3. stored profile views reproduce the incompatible frozen dispositions;
4. selecting `P_AB` enables the frozen D-probe transfer after C->A teaching;
5. switching to `P_BC` collapses transfer without reassessment or learning loss;
6. switching back to `P_AB` restores the identical transfer trajectory;
7. selection changes only the selected profile;
8. unassessed `P_AC` selection fails closed;
9. assessment witnesses ignore current selection and preserve it;
10. the full assess/select/train/switch cycle is deterministic.

## Frozen numerical regressions

The implementation retains:

```text
P_AB D(C,D) = 0
P_BC D(C,D) = 0.5770682910193559
sequential[C][A] = 0.5579844028434426
P_AB-selected A_step2(D) = 0.011159688056868854
P_BC-selected A_step2(D) = 0 to 1e-12 floor
```

## Evidence status

No executable CF-LM-011 PASS is claimed yet. The available environment used to stage this branch does not provide the project's local Rust gate as authoritative evidence.

Required local gate:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Verified parent suite: 127 tests.  
New CF-LM-011 suite: 10 tests.  
Expected full discovery if all targets pass: 137/137.

Any semantic or numerical failure must be classified before changing frozen protocol values.

## Claim ceiling

A PASS would support only:

> Cohfield-LM can retain multiple incompatible profile-scoped abstraction assessments simultaneously and reversibly select which already-assessed abstraction participates in continuation, without reassessment, destructive history rewriting, or sequential-learning mutation.

It would not establish endogenous profile selection, semantic equivalence, or governed authority.
