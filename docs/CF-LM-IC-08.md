# CF-LM-IC-08 — Internal Consequence-Equivalence Acquisition Contract

Status: Draft / preregistered for CF-LM-009
Parent: CF-LM-IC-07 and verified CF-LM-008 evidence `bfa18a0bdef16a82fd866c6f5f1aa4487e0deca4`

## Purpose

CF-LM-008 demonstrated that an external observer can recover profile-relative observational classes from continuation behavior without inspecting internal relational structure. CF-LM-IC-08 defines the next downstream language-domain capability: the model may acquire a persistent internal consequence-equivalence relation from its own continuation behavior and later use that relation during future continuation without an external classifier at inference time.

This contract does not define semantic equivalence, canonical identity, admissibility, policy, authority, execution, commitment, or CohTrace substitution.

## Architectural boundary

The existing `CohfieldLanguageModelV1` and `LanguageState` remain unchanged. CF-LM-009 introduces a versioned language-domain model/state extension rather than mutating V1 semantics.

The V2 complete State remains a CF-ACP `State`. Its relational-configuration projection contains two semantically distinct components:

1. `sequential` — the existing exposure-derived directed relational configuration corresponding to V1 `Psi`;
2. `consequence_equivalence` — a persistent symmetric relation learned from declared continuation consequences.

These components must remain separate. Exposure adjacency must not silently become equivalence, and learned equivalence must not rewrite sequential history.

## Acquisition rule

A frozen consequence profile `K_int` evaluates each surface symbol in `SurfaceSymbol::ALL` from the same State using:

- single-symbol driven context;
- four autonomous continuation steps;
- projection to the external `A/B` consequence coordinates;
- Euclidean response distance;
- equality floor `epsilon_eq = 1e-12`.

For distinct symbols `s_i`, `s_j`, the model may set the internal symmetric relation

`consequence_equivalence[i][j] = consequence_equivalence[j][i] = true`

only when their pre-update `K_int` consequence signatures differ by at most `epsilon_eq`.

The acquisition pass must enumerate candidates without externally supplied pair labels or expected classes. All signatures are computed before any relation is written, preventing within-pass self-reinforcement.

## Inference use

A stored consequence-equivalence relation may participate in future domain dynamics through a separately identified equivalence coupling. It does not erase or replace sequential relation identity.

CF-LM-009 must demonstrate use by:

1. acquiring an internal equivalence relation before a later novel relation is learned;
2. learning the later relation only on one member of the acquired equivalence class;
3. showing a fresh probe of the other member produces the transferred consequence;
4. showing the same training without internalization does not transfer;
5. surgically deleting only the internal equivalence relation collapses the transferred consequence;
6. showing equivalence acquisition without later relation learning does not create the transferred consequence by itself.

## Identity and governance firewalls

`consequence_equivalence` is domain State content. It is not exact equality and does not merge State identity.

`consequence_equivalence != semantic equivalence != admissibility != policy != authority`.

Any State whose equivalence memory differs remains an exact-different State unless the domain later defines a separate equivalence relation for a declared purpose.

## Compatibility

- CF-ACP core trait: unchanged.
- CohfieldLanguageModelV1: unchanged.
- CF-LM-001 through CF-LM-008 evidence: unchanged.
- V1 -> V2 migration: copies `X`, `Theta`, and sequential `Psi` exactly and initializes consequence-equivalence memory empty.
- No new CohBit primitive is introduced.
