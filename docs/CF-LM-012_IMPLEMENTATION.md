# CF-LM-012 Implementation Boundary

Status: Implementation staged; local executable gate pending.

Protocol parent: `docs/CF-LM-012_PROTOCOL.md` at contract head `8af7ff562bc8653206e2acc365251ee91fcda39d`.

## Implementation

CF-LM-012 adds:

- `src/profiles/language_v5.rs` — additive V5 context-recognition and context-conditioned profile-inference State/model;
- `src/profiles/mod.rs` export;
- `tests/language_context_conditioned_selection.rs` — ten preregistered conformance tests;
- this implementation-boundary document.

V1-V4 and CF-ACP semantics are unchanged.

## V5 State separation

V5 preserves V4:

```text
sequential
selected_profile
assessment_history
```

and adds:

```text
current_context_epoch
context_history
selection_history
```

`assessment_history` remains the source of truth for each profile's stored equivalence disposition. `selected_profile` remains the runtime selector. Context history records how a surface cue was recognized; selection history records which assessed profiles were scored and which profile was inferred.

## Context-recognition boundary

`RecognizeContext(cue)` accepts only a surface-symbol cue and computes normalized symbol activity. It appends one context record and updates the current-context reference.

Recognition does not alter:

- `x`;
- `theta`;
- sequential relations;
- assessment history;
- selected profile;
- selection history.

Therefore:

```text
context recognition != profile selection
```

## Inference boundary

`InferConsequenceProfileFromContext` accepts no profile identity.

It:

1. resolves the current recognized context;
2. enumerates every distinct assessed profile found in assessment history;
3. requires each profile to have a complete stored assessment;
4. computes the same compatibility score for every profile;
5. requires the frozen support and winning-margin thresholds;
6. updates only `selected_profile` plus append-only selection provenance on success.

The rule is:

```text
score(P|context) = sum(context_activity[s] for s in P.projection)
```

with:

```text
minimum_context_score = 0.50
minimum_context_margin = 0.25
```

There is no `P_AB`/`P_BC` branch inside the inference implementation.

## Delegation boundary

V5 delegates pre-existing V4 mechanics back to `CohfieldLanguageModelV4`:

- evolution;
- sequential exposure/adaptation;
- consequence-equivalence assessment;
- observation;
- profile-equivalence reconstruction.

This prevents CF-LM-012 from silently changing previously verified language dynamics while adding context-conditioned selection.

## Frozen contexts

```text
K_AB   = [A,A,B,D] -> activity [0.50,0.25,0.00,0.25]
K_BC   = [B,C,C,D] -> activity [0.00,0.25,0.50,0.25]
K_tie  = [B,D]
K_none = [D,D]
```

Expected assessed-profile scores:

```text
K_AB: P_AB=0.75, P_BC=0.25
K_BC: P_AB=0.25, P_BC=0.75
K_tie: P_AB=0.50, P_BC=0.50
K_none: P_AB=0.00, P_BC=0.00
```

## Transfer boundary

The organism is taught eight isolated `C->A` episodes while no profile is selected.

Expected frozen sequential relation:

```text
sequential[C][A] = 0.5579844028434426
sequential[D][A] = 0
```

Then:

```text
recognize K_AB -> infer -> selected P_AB -> D probe A_step2 = 0.011159688056868854
recognize K_BC -> infer -> selected P_BC -> D probe A_step2 at floor
recognize K_AB -> infer -> selected P_AB -> original D-probe trajectory restored
```

No profile reassessment or additional teaching occurs in this cycle.

## Fail-closed boundary

`K_tie` must return `AmbiguousContext`.

`K_none` must return `UnsupportedContext`.

Failed inference operates on immutable input State and returns no successor State, so no selected profile, assessment, sequential relation, context record, or selection record is changed by the failed inference attempt.

## Evidence boundary

Until the local gate runs, CF-LM-012 has only:

- a frozen implementation contract;
- a frozen protocol;
- implementation code;
- preregistered deterministic numerical expectations.

It does not yet have executable PASS evidence.

A successful local gate supports only the claim ceiling in `CF-LM-IC-11`. Context records and selection records are internal model State; they are not canonical CohBit Evidence or Verification.
