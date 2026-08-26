# CF-LM-IC-06 — Cross-Profile Equivalence Transfer Contract

Status: **Pre-canonical downstream implementation contract v0.1**

Parent evidence: `CF-LM-006` verified at `edee108eb470913e7dab43f83dec91e1115f4650`.

## 1. Purpose

`CF-LM-006` established executable finite-case evidence that a contextual-consequence relation `~_K` behaves as an equivalence relation on a frozen three-State carrier under one declared composition/observation profile `K`.

`CF-LM-IC-06` governs the next question:

> Does that earned relation transfer unchanged to a genuinely new composition/observation profile, and where does that transfer fail as the observation horizon expands?

This contract tests **profile-relative domain of validity**. It does not define semantic equivalence, universal congruence, universal substitution, or identity merging.

## 2. Architectural position

This contract is downstream of:

`CF-LM-000 -> CF-LM-001 -> ... -> CF-LM-006`.

It does not amend:

- `State`;
- `Action`;
- `Transition`;
- `Atomic Transition`;
- `CohAtom`;
- `CohField`;
- `CohBit`;
- `CohTrace`;
- `AdaptiveContinuationModel`;
- `CohfieldLanguageModelV1`.

The canonical firewalls remain:

`exact identity != observational equivalence != semantic equivalence`.

## 3. Frozen carrier

Reuse the three relational cores from `CF-LM-006`:

- `R_C`: learned `A -> C -> B` route from `(A C B D)^64`;
- `R_D`: learned `A -> D -> B` route from `(A D B C)^64`;
- `R_L`: `R_C` plus the independently learned `D -> D` latent loop from `(D D)^64`.

All route and loop weights are copied exactly from the unchanged model's learned States. No route weight is normalized, averaged, or retuned.

## 4. New transfer profile

Define a new symmetric cross-relay host family `K_x` by adding exactly:

`Psi[C][D] = w`

and

`Psi[D][C] = w`

for:

`w in {0.5, 1.0, 2.0}`.

The `C -> D` and `D -> C` adjacencies must be programmatically verified absent from all three source histories used to construct the carrier.

These host edges are test-composition fixtures. They do not alter learned route or latent-loop weights.

## 5. Observer and horizons

For each host State:

- drive context `A`, then separately context `B`;
- record projected `A/B` coordinates after the driven step;
- continue under zero input and record projected `A/B` coordinates after each autonomous step.

Use Euclidean distance on the concatenated projected response.

Freeze two horizons:

- short horizon `h_s = 4` autonomous steps;
- long horizon `h_l = 10` autonomous steps.

Freeze:

- `epsilon_floor = 1e-12`;
- `epsilon_split = 0.005`;
- `epsilon_onset = 1e-4`;
- `epsilon_host = 0.001`;
- `epsilon_distinct = 0.70`.

## 6. Required transfer pattern

### 6.1 Short-horizon transfer

At `h_s = 4`, every distinct carrier pair must remain projected-equivalent to `epsilon_floor` independently for every cross-host weight.

### 6.2 Long-horizon route equivalence

At `h_l = 10`, `R_C` and `R_D` must remain projected-equivalent to `epsilon_floor` independently for every cross-host weight.

### 6.3 Long-horizon latent split

At `h_l = 10`, `R_L` must separate from both `R_C` and `R_D` by more than `epsilon_split` independently for every cross-host weight.

The split magnitude must increase strictly with cross-host strength across the frozen weight family.

### 6.4 Temporal onset

For `R_C` versus `R_L`, distance must remain at floor through `h_s = 4`, then exceed `epsilon_onset` at five autonomous steps for every frozen weight.

### 6.5 Causal loop ablation

Setting only the independently learned latent edge

`Psi[D][D] := 0`

in `R_L` must restore exact relational equality with `R_C` and restore long-horizon projected equivalence to `epsilon_floor` for every cross-host weight.

### 6.6 Host nondegeneracy

For `R_C`, adding the cross-relay host must change its own long-horizon projected consequence response by more than `epsilon_host` for every frozen weight.

This prevents PASS through a host that has no measurable effect.

### 6.7 Identity separation

Common host composition must not erase exact carrier distinction. Every distinct carrier pair must retain `D_Psi > epsilon_distinct` at `w = 1.0`.

## 7. Claim ceiling

A PASS may support only:

> On the frozen carrier, the `CF-LM-006` consequence-equivalence behavior transfers to a new unseen symmetric cross-relay composition profile at the original four-step horizon, while an independently learned latent structure becomes consequence-relevant at a longer declared horizon. The learned C-route and D-route alternatives remain equivalent under the same expanded profile.

A PASS does **not** establish:

- semantic equivalence;
- universal observer equivalence;
- universal profile transfer;
- universal congruence;
- unrestricted substitution;
- identity equality;
- governance equivalence.

## 8. Failure discipline

Any failed criterion is evidence about the domain of validity of the relation.

No history, carrier member, host edge, host weight, horizon, projection, metric, threshold, or model parameter may change after observing a failed gate except through a versioned successor experiment.
