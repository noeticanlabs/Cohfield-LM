# CF-LM-011 — Multi-Profile Internal Equivalence Coexistence and Reversible Selection

**Status:** frozen preregistration before executable implementation  
**Parent evidence:** CF-LM-010 verified head `3dd0fd7b6980ab9b72285e897c144a58ae73b921`  
**Contract:** CF-LM-IC-10  

## Question

Can the Cohfield-LM organism retain two incompatible profile-scoped internal equivalence assessments at the same time and reversibly select which already-assessed relation participates in future continuation, without reassessment, history deletion, or sequential-learning mutation?

## Source substrate

Reuse the exact dual-route source established by CF-LM-009/010:

- `A->C = 0.9840816505055259`;
- `C->B = 1.0041649494954346`;
- `A->D = 0.9840816505055259`;
- `D->B = 1.0041649494954346`.

All other sequential relations begin at zero.

## Frozen profiles

`P_AB`:

```text
projection = [A,B]
continuation_steps = 4
epsilon = 1e-12
```

Expected nontrivial equivalence: only `C/D`.

`P_BC`:

```text
projection = [B,C]
continuation_steps = 4
epsilon = 1e-12
```

Expected nontrivial equivalence: none.

The CF-LM-010 preregistered pair geometries remain frozen and may be used as regression targets.

## Frozen assessment order

Starting from migrated V4 source State with no selected profile:

1. assess `P_AB`;
2. assess `P_BC`;
3. do not reassess either profile during the subsequent selection/transfer experiment.

After both assessments:

- history length must be exactly 12 pair records;
- both six-record epochs must remain intact;
- selected profile must still be `None`;
- `X`, `Theta`, and sequential relations must match the pre-assessment State exactly.

## Latest-epoch derivation rule

For any assessed profile `P`, runtime equivalence is derived from the latest complete six-pair assessment epoch whose profile equals `P`.

No mutable active-equivalence matrix is authoritative in V4.

## Frozen selection sequence

After both assessments exist:

1. select `P_AB`;
2. teach only `C->A` for eight isolated `[C,A]` episodes;
3. probe `D` for four continuation steps;
4. switch to already-assessed `P_BC` without reassessment;
5. probe `D` again;
6. switch back to already-assessed `P_AB` without reassessment;
7. probe `D` a third time.

The teaching sequence is exactly the CF-LM-009/010 sequence and must produce:

`sequential[C][A] = 0.5579844028434426 +/- 1e-9`.

No direct `D->A` sequential relation may be learned.

## Frozen transfer outcomes

With `P_AB` selected:

`A_step2(D) = 0.011159688056868854 +/- 1e-9` and `> 0.01`.

With `P_BC` selected:

`A_step2(D) <= 1e-12`.

After selecting `P_AB` again:

`A_step2(D) = 0.011159688056868854 +/- 1e-9`.

The first and third `P_AB` trajectories must be exactly deterministic copies under the same State substrate and selected profile.

## Frozen nonmutation requirements

Profile selection must not alter:

- assessment-history length or record contents;
- `X`;
- `Theta`;
- sequential relations.

The full 12-record history must remain byte-for-byte equal through `P_AB -> P_BC -> P_AB` selection.

## Fail-closed selection control

Define an unassessed profile:

`P_AC = <projection=[A,C], continuation_steps=4, epsilon=1e-12>`.

Attempting to select `P_AC` before any `P_AC` assessment must return a typed failure and leave the State unchanged.

## V3 -> V4 migration control

A conforming migration from a CF-LM-010 V3 State must preserve:

- `X`;
- `Theta`;
- sequential relations;
- full assessment history;
- selected profile corresponding to V3 `active_profile` when that profile has a complete matching assessment epoch.

V4 must not copy V3's active-equivalence matrix as a second source of truth.

## PASS

PASS requires all frozen requirements above.

Any failure is a CF-LM-011 FAIL under this protocol. Do not change source weights, profiles, assessment order, teaching episodes, thresholds, selection sequence, migration rule, or response metric after observing a failure without a versioned successor protocol.

## Claim ceiling

PASS supports only:

> Multiple incompatible profile-scoped abstraction assessments can coexist in Cohfield-LM State, and the organism can reversibly select and use an already-assessed abstraction without reassessment or destructive rewriting.

PASS does not establish endogenous context selection or semantic equivalence.
