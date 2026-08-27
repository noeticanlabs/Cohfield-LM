# CF-LM-013 — Experience-Learned Context-to-Abstraction Applicability

Status: frozen preimplementation protocol. No PASS claimed.

## Parent

Verified CF-LM-012 evidence:

`157c2aad9111eb3c83e812643431b4e54fb60508`

147/147 local tests.

## Scientific question

Can the organism learn a persistent context-to-profile applicability relation from supervised experience and generalize that relation to held-out contexts, without using CF-LM-012's designer-supplied projection-overlap inference rule?

## Architecture under test

Versioned language-domain V6 extends V5 with:

- append-only applicability acquisition history;
- derived profile applicability prototypes;
- append-only learned-selection history.

Existing sequential learning, abstraction assessment, context recognition, and continuation dynamics remain inherited/delegated from verified V5/V4 behavior.

Applicability history is internal model State, not CohBit Evidence or Verification.

## Assessed profiles

`P_AB = <[A,B],4,1e-12>` — only C/D equivalent.

`P_BC = <[B,C],4,1e-12>` — no nontrivial equivalence.

## Supervised applicability acquisition

Recognize each cue, then record the currently recognized context as applicable to the supplied already-assessed profile.

Frozen acquisition episodes, in this order:

1. `T_AB1=[C,C,C,D] -> P_AB`
2. `T_AB2=[C,C,D,D] -> P_AB`
3. `T_BC1=[A,A,A,D] -> P_BC`
4. `T_BC2=[A,A,D,D] -> P_BC`

Acquisition must append exactly four records and select no profile.

Derived prototypes:

- `mu_AB=[0,0,0.625,0.375]`
- `mu_BC=[0.625,0,0,0.375]`

## Learned inference

For current context activity `c` and profile prototype `mu_P`:

`distance(P|c) = Euclidean(c, mu_P)`.

Select the unique minimum only when:

- `min_distance <= 0.50`; and
- `runner_up - min_distance > 0.25`.

No inference request contains a profile identity.

No learned-inference calculation may read `InternalEquivalenceProfile::projection`.

## Held-out inversion controls

### K_C

Cue: `[B,C,C,D]`

Activity: `[0,0.25,0.50,0.25]`

Distances:

- to P_AB: `0.30618621784789724`
- to P_BC: `0.8477912478906585`

Expected learned selection: `P_AB`.

CF-LM-012's old projection score would select `P_BC` (`0.25` vs `0.75`). PASS therefore requires the learned result to contradict the old heuristic.

### K_A

Cue: `[A,A,B,D]`

Activity: `[0.50,0.25,0,0.25]`

Distances:

- to P_AB: `0.8477912478906585`
- to P_BC: `0.30618621784789724`

Expected learned selection: `P_BC`.

CF-LM-012's old projection score would select `P_AB` (`0.75` vs `0.25`). PASS again requires inversion.

## Fail-closed controls

### Ambiguous

`K_tie` contains 5 A, 5 C, 6 D symbols.

Activity: `[0.3125,0,0.3125,0.375]`.

Distance to each prototype: `0.4419417382415922`.

Expected: `AmbiguousApplicability` and no successor State.

### Unsupported

`K_none=[B,B]` -> `[0,1,0,0]`.

Distance to each prototype: `1.2374368670764582`.

Expected: `UnsupportedApplicability` and no successor State.

### No learning

Inference with no applicability acquisition history must fail closed as `NoApplicabilityExperience`.

## Transfer control

Before held-out inference, teach `C->A` for exactly eight isolated `[C,A]` episodes while `selected_profile=None`.

Frozen values:

- `Psi[C,A]=0.5579844028434426 +/- 1e-9`
- `Psi[D,A]<=1e-12`

Then:

1. `K_C` -> learned `P_AB` -> D probe -> `A_step2=0.011159688056868854 +/- 1e-9`.
2. `K_A` -> learned `P_BC` -> D probe -> `|A_step2|<=1e-12`.
3. `K_C` again -> learned `P_AB` -> response trajectory exactly equals step 1.

No reassessment, applicability retraining, or sequential relearning is permitted between those three held-out inferences.

## Migration

V5 -> V6 must preserve:

- X;
- Theta;
- sequential relations;
- selected profile;
- assessment history;
- current-context reference;
- context history;
- V5 selection history.

V6 applicability history and V6 learned-selection history start empty.

## Required tests

1. V5->V6 migration preserves parent State and starts empty learned-applicability State.
2. Four acquisition episodes append four applicability records without selecting or mutating substrate.
3. Derived profile prototypes exactly match preregistered values.
4. Held-out K_C selects P_AB with preregistered distances and explicitly inverts the old V5 heuristic.
5. Held-out K_A selects P_BC with preregistered distances and explicitly inverts the old V5 heuristic.
6. Inference without applicability experience fails closed.
7. K_tie fails closed as ambiguous.
8. K_none fails closed as unsupported.
9. Learned K_C -> K_A -> K_C selection causally enables, collapses, and identically restores transfer without reassessment/relearning.
10. Applicability provenance and complete cycle are deterministic.

## Expected full-suite count

Verified parent: 147 tests.

New CF-LM-013 tests: 10.

Expected full target if clean: **157/157**.

## Evidence boundary

Runtime conformance can establish only the frozen finite experimental claim. It cannot establish universal contextual learning, semantics, or autonomous self-grounding.
