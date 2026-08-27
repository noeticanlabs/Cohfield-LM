# CF-LM-013 Implementation Boundary

Status: implementation staged; local executable gate pending.

Protocol parent: `agent/cf-lm-013-learned-applicability-contract` at `006f79c834a5febedd5ad0598944fa1cc27c85ce`.

Verified evidence parent: CF-LM-012 `157c2aad9111eb3c83e812643431b4e54fb60508`, 147/147.

## Added implementation

- `src/profiles/language_v6.rs`
- `src/profiles/mod.rs` V6 export
- `tests/language_learned_applicability.rs`
- this implementation boundary

V1-V5 and CF-ACP semantics are not modified.

## V6 State separation

V6 preserves the V5 substrate and keeps distinct:

- sequential relational learning;
- consequence-equivalence assessment history;
- recognized-context history;
- legacy CF-LM-012 projection-selection history;
- supervised applicability-acquisition history;
- learned-applicability selection history;
- currently selected profile.

The V5 projection-selection history is preserved for migration/audit but is not used by the new learned inference operation.

## Learned applicability

`RecordContextApplicability(profile)` requires a current recognized context and an already-assessed profile. It appends an applicability record and does not select the profile.

For each profile represented in applicability history, V6 derives a mean context-activity prototype. The prototype is derived from history and is not independently mutable State.

`InferConsequenceProfileFromLearnedApplicability` accepts no profile identity. It compares the current recognized context to every learned profile prototype by Euclidean distance and selects only a unique sufficiently near and sufficiently separated winner.

The learned distance calculation does not use `InternalEquivalenceProfile::projection`.

## Frozen inversion control

Applicability acquisition is deliberately opposite CF-LM-012's projection-overlap heuristic.

The held-out `K_C=[B,C,C,D]` context must select P_AB by learned prototype distance even though the old projection rule would select P_BC.

The held-out `K_A=[A,A,B,D]` context must select P_BC by learned prototype distance even though the old projection rule would select P_AB.

This prevents a PASS from being explained by reuse of the previous designer-supplied selection rule.

## Frozen causal consequence

After exactly eight isolated C->A teaching episodes with no profile selected:

- K_C -> learned P_AB -> D-probe A step 2 = `0.011159688056868854 +/- 1e-9`;
- K_A -> learned P_BC -> D-probe A step 2 stays at floor;
- K_C again restores the identical first trajectory.

No reassessment, applicability retraining, or sequential relearning may occur between those held-out selections.

## Tests

CF-LM-013 adds 10 tests. Parent suite is 147, so the expected clean total is **157/157**.

Local gate:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No PASS is claimed until the local gate executes.

## Claim ceiling

A clean gate would provide finite executable evidence that supervised context-to-profile applicability can be learned into persistent State, generalized to held-out contexts, override the prior hand-supplied heuristic, and causally control later abstraction-mediated transfer.

It would not establish semantic understanding, autonomous relevance discovery, consequence-grounded self-supervision, universal generalization, or any governance authority.
