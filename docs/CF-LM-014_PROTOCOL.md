# CF-LM-014 — Consequence-Grounded Abstraction Applicability Acquisition

Status: frozen preregistration
Parent: CF-LM-013 verified evidence `c6472bf8b15408cc0adadc4f781422996127f582`
Contract: `CF-LM-IC-13`

## Research question

Can the organism learn which already-assessed abstraction applies to a context from an observed continuation consequence rather than from a supplied abstraction/profile label?

## Frozen boundary

The candidate abstractions remain the previously assessed `P_AB` and `P_BC` profiles. CF-LM-014 does not invent new abstractions. It removes the profile label only from applicability acquisition.

Observed continuation is a domain observation used as adaptive experience. It is not canonical CohBit Evidence, Verification, valuation, admissibility, policy, or authority.

## Frozen source substrate

Reuse the combined learned route substrate:

- `A->C = 0.9840816505055259`
- `C->B = 1.0041649494954346`
- `A->D = 0.9840816505055259`
- `D->B = 1.0041649494954346`

Assess both profiles before consequence-grounded acquisition.

Teach eight isolated `[C,A]` episodes with no profile selected. Expected:

- `sequential[C][A] = 0.5579844028434426`
- `sequential[D][A] = 0` within `1e-12`.

## Frozen profiles

`P_AB = <projection=[A,B], continuation_steps=4, epsilon=1e-12>`

`P_BC = <projection=[B,C], continuation_steps=4, epsilon=1e-12>`

`P_AB` contains C/D consequence equivalence; `P_BC` contains no nontrivial consequence equivalence.

## Frozen observed consequence signatures

For a fresh equalized D probe, record the A coordinate after the drive and after four zero-input continuation steps.

`Y_TRANSFER`:

```text
[
  0.0,
  0.0,
  0.011159688056868854,
  0.01673953208530328,
  0.017363331386570834
]
```

`Y_ZERO`:

```text
[0.0, 0.0, 0.0, 0.0, 0.0]
```

Preregistered Euclidean separation:

`D(Y_TRANSFER,Y_ZERO) = 0.026575098283946105`.

## Frozen outcome matching

For each assessed profile, generate a counterfactual predicted consequence signature from a cloned State. The actual State must not be mutated by candidate evaluation.

`error(P) = ||Pred(P) - Y_observed||_2`.

Thresholds:

- `epsilon_outcome = 0.020`
- `delta_outcome = 0.010`, strict margin.

Successful acquisition requires a unique winning profile with error at or below `epsilon_outcome` and runner-up minus winner strictly above `delta_outcome`.

## Frozen unlabeled acquisition episodes

No profile identity appears in the acquisition experience.

1. `T_C1=[C,C,C,D]`, observe `Y_TRANSFER`.
2. `T_C2=[C,C,D,D]`, observe `Y_TRANSFER`.
3. `T_A1=[A,A,A,D]`, observe `Y_ZERO`.
4. `T_A2=[A,A,D,D]`, observe `Y_ZERO`.

Required internally inferred applicability:

- T_C1 -> P_AB
- T_C2 -> P_AB
- T_A1 -> P_BC
- T_A2 -> P_BC

Outcome acquisition records must append without selecting a runtime profile.

## Frozen derived prototypes

From the inferred winners and recognized context activities:

- `mu_AB=[0.0,0.0,0.625,0.375]`
- `mu_BC=[0.625,0.0,0.0,0.375]`

Prototypes are derived views over immutable history, not independent mutable State.

## Frozen held-out contexts

`K_C=[B,C,C,D]`

Expected distances:

- to mu_AB: `0.30618621784789724`
- to mu_BC: `0.8477912478906585`

Required outcome-learned inference: `P_AB`.

`K_A=[A,A,B,D]`

Expected distances:

- to mu_AB: `0.8477912478906585`
- to mu_BC: `0.30618621784789724`

Required outcome-learned inference: `P_BC`.

The old CF-LM-012 projection-overlap heuristic would choose the opposite profile in both held-out cases. The CF-LM-014 inference path must not use `profile.projection`.

## Frozen held-out inference thresholds

- maximum context-to-prototype distance: `0.50`
- minimum winning context margin: strictly greater than `0.25`.

## Frozen negative outcome cases

### Ambiguous outcome

`Y_MID = 0.5 * Y_TRANSFER`.

Both profile prediction errors are expected to be:

`0.0132875491419730525` approximately.

Because the errors tie, acquisition must fail as `AmbiguousOutcome` even though each error is inside `epsilon_outcome`.

### Unsupported outcome

`Y_FAR=[1.0,1.0,1.0,1.0,1.0]`.

Minimum candidate prediction error must exceed `epsilon_outcome`; acquisition fails as `UnsupportedOutcome`.

### No learned applicability

Held-out applicability inference before any accepted consequence-grounded acquisition record fails as `NoOutcomeApplicabilityExperience`.

## Frozen causal test

After unlabeled outcome-grounded acquisition:

1. recognize K_C;
2. infer P_AB from outcome-derived applicability prototypes;
3. fresh D probe must reproduce `A_2=0.011159688056868854 +/- 1e-9`;
4. recognize K_A;
5. infer P_BC;
6. fresh D probe A consequence must remain at floor `<=1e-12`;
7. recognize K_C again;
8. infer P_AB again;
9. the complete original transfer trajectory must be restored exactly.

No profile reassessment, outcome-applicability retraining, or sequential relearning is allowed during the held-out cycle.

## Frozen conformance targets

The implementation suite must verify at least:

1. V6->V7 migration preserves parent State and starts empty outcome-applicability histories.
2. Counterfactual P_AB/P_BC predictions reproduce Y_TRANSFER/Y_ZERO while leaving actual State unchanged.
3. Unlabeled T_C1/T_C2 observations infer P_AB and T_A1/T_A2 infer P_BC.
4. Outcome-grounded acquisition appends four records without runtime selection or substrate mutation.
5. Derived prototypes equal the preregistered values.
6. Held-out K_C/K_A generalize and invert the old projection heuristic.
7. Y_MID fails ambiguous with unchanged State.
8. Y_FAR fails unsupported with unchanged State.
9. Held-out inference without outcome applicability history fails closed.
10. Outcome-derived applicability causally controls transfer and restores it deterministically.

## Freeze rule

After the first executable gate begins, do not change:

- profile definitions;
- route substrate;
- eight C->A teaching episodes;
- consequence signature shape;
- Y_TRANSFER;
- Y_ZERO;
- Y_MID;
- Y_FAR;
- outcome error metric;
- outcome thresholds;
- acquisition contexts/order;
- context prototype construction;
- held-out contexts;
- context-distance thresholds;
- transfer thresholds;
- V1-V6 parameters.

Mechanical format/type/lint fixes that do not change experimental semantics may be applied only when explicitly recorded.

## Claim ceiling

A successful gate supports only finite, preregistered evidence that consequence observations can ground applicability acquisition over the fixed candidate abstraction set and finite context family. It does not establish general intelligence, semantic truth, autonomous abstraction invention, general RL, or governance authority.
