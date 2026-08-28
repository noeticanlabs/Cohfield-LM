# CF-LM-IC-14 — Derived Abstraction Object Formation Contract

## Status

Experimental language-domain implementation contract. This contract extends verified CF-LM-014 without redefining CF-ACP, CohField, State, CohAtom, CohBit, or CohTrace.

## Purpose

CF-LM-014 can choose among already-existing abstractions from consequence-grounded experience. CF-LM-015 tests whether the language organism can construct a new persistent internal abstraction object from its own previously assessed consequence-equivalence structure and later learn/use a relation to that abstraction object.

## Canonical firewalls

- derived abstraction object != SurfaceSymbol
- derived abstraction identity != representation/storage location
- derived abstraction identity != formation provenance
- derived abstraction != semantic truth
- derived abstraction != canonical CohBit Evidence
- abstraction activation != policy/authority
- abstraction-mediated continuation != admissibility/execution/commitment

The abstraction object is domain-specific State, not a new CohBit primitive.

## Structural identity

A derived abstraction identity is the immutable pair

`DerivedAbstractionIdentity = <profile, member_set>`.

Formation epoch and source assessment epoch are historical provenance and MUST remain separate from semantic identity.

Re-deriving the same `<profile, member_set>` from a later conforming assessment MUST preserve abstraction identity and append new provenance rather than minting a different semantic abstraction.

## Formation

Given an already-assessed profile `P`, derive its latest complete executable equivalence relation `E_P`. Formation MUST:

1. validate symmetry and transitivity, treating reflexivity as implicit;
2. derive nontrivial equivalence classes without receiving their members as input;
3. create one persistent abstraction object per class of size >= 2;
4. append formation provenance binding the abstraction identity to the source assessment epoch;
5. leave X, Theta, sequential relations, assessments, and runtime profile selection unchanged.

For frozen `P_AB=<[A,B], h=4, epsilon=1e-12>`, the only derived abstraction is `{C,D}`.

For frozen `P_BC=<[B,C], h=4, epsilon=1e-12>`, no nontrivial abstraction exists and formation fails closed.

## Abstraction relation learning

Derived abstraction objects and mutable relations from them are separate State components.

For abstraction alpha and target symbol t, let `W_abs[alpha,t]` be the learned abstraction-to-symbol relation. On each sequential experience `(predecessor,current)`:

`W' = (1-rho) W + eta * I[predecessor in members(alpha) and current=t]`

using the existing frozen `rho=0.02` and `eta=0.08`.

Eight isolated sequential `C->A` adaptation events after `{C,D}` formation predict:

`W_abs[{C,D},A] = 0.5969479096728575`.

The ordinary direct sequential substrate simultaneously predicts:

`Psi[C,A] = 0.5969479096728575`

and

`Psi[D,A] = 0`.

## Abstraction-mediated continuation

CF-LM-015 introduces an explicitly active derived abstraction for the causal experiment. This activation is language-domain State and is not an authority grant.

With parent `selected_profile=None`, active alpha, and source State x, abstraction activation is

`a_alpha(x) = mean(x_s for s in members(alpha))`.

The additional target contribution is

`relational_gain * W_abs[alpha,target] * a_alpha(x)`.

This is additive to the verified V7 one-step dynamics. Pairwise consequence-equivalence coupling remains inactive in the causal probe because the parent selected profile is None.

## Frozen D-probe trajectory

After forming alpha={C,D}, activating alpha, and applying eight isolated `C->A` adaptation events, a fresh D drive followed by four zero-input continuation steps predicts the A-coordinate trajectory:

`[0.0, 0.029847395483642875, 0.029847395483642875, 0.022385546612732156, 0.014923697741821437]`.

Tolerance: `1e-9`.

## Required controls

- same C->A learning without a derived abstraction: D-probe A remains at floor;
- formed/active abstraction without learned abstraction->A relation: D-probe A remains at floor;
- surgical deletion of only `W_abs[alpha,A]` collapses D transfer while preserving the abstraction object and direct `Psi[C,A]`;
- parent `selected_profile` remains None throughout the causal probe;
- re-formation against the same assessment is idempotent;
- re-formation after a later same-profile assessment preserves semantic identity but appends distinct provenance;
- deterministic replay.

## Claim ceiling

A PASS supports only:

> On the frozen four-symbol language domain, the organism can derive a persistent internal abstraction object from its own assessed consequence-equivalence structure, preserve its identity separately from provenance, learn a relation to that derived object through member experience, and use that object to mediate a later continuation from another member.

It does not establish semantic concepts, open-ended concept invention, autonomous abstraction selection, hierarchical abstraction, or general intelligence.
