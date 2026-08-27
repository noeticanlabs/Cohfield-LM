# CF-LM-IC-10 — Multi-Profile Internal Equivalence Coexistence Contract

**Status:** preregistered contract extension v0.1  
**Parent:** CF-LM-IC-09 / verified CF-LM-010 evidence  
**Layer:** Cohfield-LM language-domain State and adaptation  

## Purpose

CF-LM-010 established that internal consequence-equivalence is profile-relative and revisable. Its V3 State nevertheless exposes only one currently active profile at a time, even though append-only assessment history already retains prior incompatible assessments.

CF-LM-011 extends that architecture without changing CohBit primitives or CF-ACP semantics. The required capability is:

> retain multiple independently assessed internal equivalence profiles simultaneously, select among already-assessed profiles without reassessment or history deletion, and make future continuation use only the selected profile's latest completed assessment.

This is **multi-profile coexistence and reversible selection**, not semantic equivalence, policy selection, authority, or governance.

## Canonical firewalls

The implementation MUST preserve:

- exact State identity != observational equivalence;
- assessment != selection;
- selected profile != field policy;
- internal assessment record != CohBit Evidence or Verification;
- profile switch != history mutation;
- sequential relation != consequence-equivalence relation;
- profile-relative equivalence != semantic equivalence.

## Required State structure

A conforming versioned language State MUST make separately inspectable:

1. fast State `X`;
2. local condition `Theta`;
3. sequential relational configuration;
4. selected consequence profile, if any;
5. append-only consequence-equivalence assessment history.

The assessment history is the single source of truth for stored profile-relative equivalence. A second mutable active-equivalence matrix MUST NOT become an independent source of truth.

## Profile assessment

For a declared profile `P`, assessment MUST:

1. measure all unordered surface-symbol pairs using sequential relations only;
2. disable any selected equivalence relation during witness generation;
3. append one complete six-pair epoch for `P`;
4. preserve all earlier assessment epochs;
5. leave profile selection unchanged unless selection is separately invoked.

## Profile selection

Selection MUST be a distinct domain operation from assessment.

Selecting `P` is permitted only if the State contains at least one complete assessment epoch for exactly `P`.

Runtime continuation MUST derive the selected equivalence relation from the latest complete assessment epoch for `P`.

Selecting an unassessed profile MUST fail closed.

Selection MUST NOT:

- append assessment records;
- modify sequential relations;
- modify `X` or `Theta`;
- rewrite earlier assessments.

## Coexistence requirement

At minimum, CF-LM-011 MUST retain simultaneous assessments for the frozen profiles:

- `P_AB = <projection=[A,B], continuation_steps=4, epsilon=1e-12>`;
- `P_BC = <projection=[B,C], continuation_steps=4, epsilon=1e-12>`.

Their established dispositions are incompatible:

- under `P_AB`, only `C/D` is equivalent;
- under `P_BC`, no nontrivial pair is equivalent.

Both assessments must remain present while either profile is selected.

## Reversible use requirement

Using the same frozen eight isolated `C->A` teaching episodes as CF-LM-009/010:

- selecting `P_AB` must enable transfer from later `C->A` learning to a fresh `D` probe;
- switching to already-assessed `P_BC` without reassessment must collapse that transfer while preserving the `C->A` sequential relation and all assessment history;
- switching back to already-assessed `P_AB`, again without reassessment, must restore transfer identically.

## Compatibility

V1, V2, and V3 remain unchanged. CF-ACP remains unchanged.

A V3 -> V4 migration may preserve the V3 selected profile and all assessment history, but V4 runtime equivalence must be derived from history rather than copied into a second mutable relation matrix.

## Claim ceiling

A PASS may support only:

> The Cohfield-LM organism can retain multiple incompatible profile-scoped abstraction assessments simultaneously and reversibly select which previously assessed abstraction governs continuation, without reassessment, history deletion, or sequential-learning mutation.

A PASS does not establish semantic understanding, endogenous context inference, universal equivalence, admissibility, policy, authority, execution, commitment, or CohTrace substitution.
