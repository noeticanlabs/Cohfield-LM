# CF-LM-009 — Internal Consequence-Equivalence Acquisition and Transfer

Status: Preregistered / executable evidence pending
Parent evidence: CF-LM-008 `bfa18a0bdef16a82fd866c6f5f1aa4487e0deca4`
Contract: CF-LM-IC-08

## Question

Can the Cohfield language organism acquire a persistent internal consequence-equivalence relation from its own continuation behavior and later use that relation to transfer a newly learned consequence to an equivalent member without an external classifier at inference time?

## Frozen source construction

Reuse the verified CF-LM learned two-hop routes:

- `R_C : A -> C -> B` from `(A C B D)^64`;
- `R_D : A -> D -> B` from `(A D B C)^64`.

Construct one V2 State containing both route cores simultaneously:

- `A -> C = 0.9840816505055259`
- `C -> B = 1.0041649494954346`
- `A -> D = 0.9840816505055259`
- `D -> B = 1.0041649494954346`

All consequence-equivalence memory starts empty.

## Frozen internal acquisition profile K_int

For every symbol in `{A,B,C,D}`:

1. equalize fast State `X=0` and `Theta=(1,1,1,1)`;
2. drive that symbol once;
3. run four zero-input continuation steps;
4. record only A/B coordinates after every step;
5. compare candidate signatures by Euclidean distance.

Frozen equality floor:

`epsilon_eq = 1e-12`.

The preimplementation equations predict candidate distances:

```text
A/B = 0.8084614995832016
A/C = 0.5891841588041229
A/D = 0.5891841588041229
B/C = 0.5229752821187045
B/D = 0.5229752821187045
C/D = 0
```

Therefore the acquisition pass must discover exactly one nontrivial pair:

`C ~int D`.

The model receives no C/D target label. It enumerates all distinct candidate pairs and compares only its own consequence signatures.

## Frozen internal representation

The V2 relational configuration stores:

- `sequential : [[f64;4];4]`
- `consequence_equivalence : [[bool;4];4]`

The learned equivalence relation is symmetric. Ordinary sequential exposure does not rewrite it.

During future evolution, an active equivalence edge contributes a fixed relational coupling of `1.0` before the existing global `relational_gain = 0.20` is applied.

This does not reinterpret sequential `Psi`; the two relation types remain separately inspectable State components.

## Novel relation phase

After internal equivalence acquisition, teach only the new directed sequential relation:

`C -> A`

using eight isolated two-symbol episodes `[C,A]`, each episode resetting predecessor context so no cross-episode `A -> C` adjacency is introduced.

The existing V1 adaptation constants remain:

`psi_decay = 0.02`

`psi_gain = 0.08`.

Frozen prediction after eight episodes:

`sequential[C][A] = 0.5579844028434426`.

No direct `D -> A` sequential relation may be learned.

## Fresh transfer probe

Equalize `X` and `Theta`, drive `D` once, then run four autonomous continuation steps.

With internalized `C ~int D`, the frozen equations predict A-coordinate continuation:

```text
step 0: 0
step 1: 0
step 2: 0.011159688056868854
step 3: 0.01673953208530328
step 4: 0.017363331386570834
```

Frozen transfer threshold:

`epsilon_transfer = 0.01` at step 2.

Required:

`A_step2(D probe | internalized then C->A learned) > 0.01`.

## Falsification controls

1. **No internalization**: same source and same eight C->A learning episodes, but no acquired equivalence memory. D probe must keep A at floor `<=1e-12`.
2. **No novel relation**: internalize C~D but do not learn C->A. D probe must keep A at floor.
3. **Surgical equivalence ablation**: after internalization and C->A learning, set only `consequence_equivalence[C][D]` and `[D][C]` to false. D-probe A consequence must collapse to floor while sequential C->A remains unchanged.
4. **No direct leakage**: `sequential[D][A] <=1e-12` throughout.
5. **Reverse-direction symmetry**: independently internalize C~D, teach only `D -> A` for eight isolated episodes, then probe C. The predicted A step-2 value is the same `0.011159688056868854 > 0.01` with no direct C->A training.
6. **Acquisition selectivity**: no nontrivial pair other than C/D may be written into consequence-equivalence memory.
7. **State identity separation**: V1->V2 migration with empty equivalence memory and the internalized V2 State remain exact-different domain States.

## PASS claim ceiling

A PASS supports only:

> A versioned Cohfield language State can acquire a persistent internal relation from self-observed consequence equivalence and later use that relation to transfer a newly learned continuation consequence to an equivalent member without an external classifier at inference time.

It does not establish semantic equivalence, natural-language meaning, universal abstraction learning, authority, or governed substitution.

## Frozen no-change list after first executable result

Do not change without a versioned successor:

- source route weights;
- candidate enumeration;
- K_int observer;
- four-step acquisition horizon;
- epsilon_eq;
- equivalence coupling;
- eight C->A / D->A episodes;
- psi_decay / psi_gain;
- D/C transfer probes;
- epsilon_transfer;
- ablation definition;
- model V1 equations.
