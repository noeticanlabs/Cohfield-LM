# CF-LM-009 Implementation Boundary

Status: Executable implementation staged; local conformance pending
Protocol parent: `docs/CF-LM-009_PROTOCOL.md`
Contract parent: `docs/CF-LM-IC-08.md`
Verified dependency: CF-LM-008 `bfa18a0bdef16a82fd866c6f5f1aa4487e0deca4`

## Added production profile

CF-LM-009 introduces `CohfieldLanguageModelV2` in `src/profiles/language_v2.rs` without modifying `CohfieldLanguageModelV1`.

V2 preserves the CF-ACP `AdaptiveContinuationModel` interface. Its relational-configuration projection is a versioned domain structure containing:

```text
sequential
consequence_equivalence
```

The first component preserves V1 exposure-derived directed relations. The second stores behaviorally acquired symmetric consequence-equivalence relations. They are never silently merged.

## Internal acquisition

`LanguageExperienceV2::InternalizeConsequenceEquivalence` causes one preregistered acquisition pass. The model:

1. enumerates all surface symbols;
2. computes their pre-update continuation signatures under `K_int`;
3. compares signatures by Euclidean distance;
4. records every nontrivial pair within the frozen equality floor symmetrically.

No expected pair label is supplied to the acquisition operation.

## Future use

V2 evolution combines sequential coupling with a separately parameterized consequence-equivalence coupling while retaining both sources in State. This permits a previously acquired relation to affect later continuation without invoking an external classifier at inference time.

CF-LM-009 tests whether a C/D relation acquired before later C->A learning transfers that new consequence to a fresh D probe, and whether removing only the stored equivalence relation destroys that transfer.

## Evidence boundary

Until the local Rust gate passes, this branch claims only implementation against a frozen protocol.

Even after PASS, the result would establish only a controlled internal consequence-equivalence acquisition/transfer capability. It would not establish natural-language semantics, semantic equivalence, universal class learning, canonical identity substitution, admissibility, policy, authority, execution, or commitment.

## Local gate

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No frozen protocol parameter, source route, observer, candidate enumeration, equivalence coupling, teaching episode count, threshold, ablation, or V1 equation may change after a failed gate without a versioned successor.
