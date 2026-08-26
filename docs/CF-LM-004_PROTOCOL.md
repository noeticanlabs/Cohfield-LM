# CF-LM-004 — Context-General Consequence Equivalence

Status: **Preregistered implementation protocol v0.1**

Parent contract: `CF-LM-IC-03`

Parent executable evidence: `CF-LM-003` at `c4d94cf480c745fc378dbb7de7f447b42eb163d5`.

No CF-LM-004 implementation existed when this protocol was frozen.

## 1. Scientific question

Can two exact-different learned internal relational paths produce the same declared external continuation consequences across multiple shared contexts and shared path-sensitive interventions while remaining distinguishable to a richer observer?

This tests a precursor to stronger domain equivalence. It does not test or define semantic equivalence.

## 2. Model

Use `CohfieldLanguageModelV1` unchanged with the already verified parameters:

`beta = 0.50`

`input_gain = 0.50`

`relational_gain = 0.20`

`psi_decay = 0.02`

`psi_gain = 0.08`.

No production model changes are permitted for CF-LM-004.

## 3. Matched histories

Use 256 observations per history:

`H_C = (A C B D)^64`

`H_D = (A D B C)^64`.

Both histories contain exactly:

`A=64, B=64, C=64, D=64`.

The intended learned routes are:

`H_C: A -> C -> B`

and

`H_D: A -> D -> B`.

Direct learned `A -> B` relation must remain absent to numerical floor.

## 4. Comparison State

After exposure, equalize:

`X_C = X_D = 0`

`Theta_C = Theta_D = (1,1,1,1)`.

Do not alter `Psi_C` or `Psi_D` before the baseline comparison.

## 5. Exact-State difference

Use Frobenius distance:

`D_Psi = ||Psi_C - Psi_D||_F`.

Freeze:

`epsilon_state = 2.0`.

Require:

`D_Psi > 2.0`.

## 6. Direct-edge absence

Freeze:

`epsilon_floor = 1e-12`.

Require:

`|Psi_C[A][B]| <= epsilon_floor`

and

`|Psi_D[A][B]| <= epsilon_floor`.

Thus any shared `A`-to-`B` consequence cannot be attributed to a directly learned `A -> B` coefficient.

## 7. Context family

Freeze the ordered shared surface contexts:

1. `A`
2. `B`
3. `AB`
4. `BA`.

For each context:

1. initialize fast State at zero;
2. apply each context symbol in order;
3. record the projected `A/B` coordinates after every driven step;
4. apply four zero-input continuation steps;
5. record projected `A/B` coordinates after every autonomous step.

Concatenate contexts in the frozen order to form the consequence-family response.

## 8. Consequence observer

Define `O_AB` as the projection of the full language State response onto coordinates `[A,B]` only.

The underlying dynamics remain four-dimensional. `C/D` are hidden from this declared observer, not removed from the model.

Use Euclidean distance over the concatenated projected response.

## 9. Shared intervention family

Freeze exactly three profiles:

### I0 — baseline

No relational modification.

### I_A — outgoing-A attenuation

`Psi[A][j] := 0.5 * Psi[A][j]` for every `j`.

### I_B — incoming-B attenuation

`Psi[i][B] := 0.5 * Psi[i][B]` for every `i`.

Each intervention is applied identically to the paired exact-different States.

## 10. Context-general equivalence target

For every frozen intervention `I_k`, compute:

`D_AB(k) = ||R_AB(I_k,z_C) - R_AB(I_k,z_D)||_2`.

Require:

`D_AB(k) <= epsilon_floor`

for all three intervention profiles.

One-context success is insufficient; all frozen contexts are concatenated into every `D_AB(k)`.

## 11. Nondegeneracy

The observed consequence family must respond materially to the path-sensitive interventions.

Define within-state intervention displacement:

`Delta_A = ||R_AB(I0,z_C) - R_AB(I_A,z_C)||_2`

`Delta_B = ||R_AB(I0,z_C) - R_AB(I_B,z_C)||_2`.

Freeze:

`epsilon_nondeg = 0.04`.

Require:

`Delta_A > 0.04`

and

`Delta_B > 0.04`.

The same values should hold for the D-path State by symmetry.

## 12. Nontrivial shared consequence

Under baseline, probe `A` followed by two autonomous zero-input steps and measure the `B` coordinate after the second autonomous step.

Freeze:

`epsilon_effect = 0.015`.

Require both paths to produce:

`B_2 > 0.015`.

This prevents equivalence from being established only because neither route produces the target external consequence.

## 13. Rich-observer discrimination

Use the same baseline State pair and the single `A` context, but retain all four coordinates at every driven/autonomous step.

Define:

`D_full_A = ||R_full,A(z_C) - R_full,A(z_D)||_2`.

Freeze:

`epsilon_rich = 0.20`.

Require:

`D_full_A > 0.20`.

Thus the two routes remain observably distinct when the observer is permitted to see their internal `C/D` consequences.

## 14. Preimplementation cross-check

The frozen equations predict approximately:

`||Psi_C-Psi_D||_F = 2.8127788519116232`

`Psi_C[A][C] = 0.9840816505055259`

`Psi_C[C][B] = 1.0041649494954346`

`Psi_D[A][D] = 0.9840816505055259`

`Psi_D[D][B] = 1.0041649494954346`

`Psi_C[A][B] = Psi_D[A][B] = 0`

`D_AB(I0) = 0`

`D_AB(I_A) = 0`

`D_AB(I_B) = 0`

`Delta_A = 0.04262366973810628`

`Delta_B = 0.04262366973810628`

`B_2(C-path) = B_2(D-path) = 0.019763606017585308`

`D_full_A = 0.22770041375557584`.

Regression comparisons to these decimal predictions may use `1e-9`; the actual PASS thresholds above remain frozen independently.

## 15. PASS

CF-LM-004 passes only if all are true:

1. matched counts confirmed;
2. exact `Psi` difference exceeds `2.0`;
3. direct `A -> B` coefficient absent to floor in both States;
4. baseline projected consequence family is equivalent to floor;
5. outgoing-A intervention preserves projected equivalence to floor;
6. incoming-B intervention preserves projected equivalence to floor;
7. both interventions materially change the observed consequence family by more than `0.04`;
8. both paths produce nontrivial shared `B_2 > 0.015` consequence;
9. the full-coordinate A-context observer distinguishes the paths above `0.20`;
10. deterministic repeats remain at floor.

Any failed required condition yields CF-LM-004 FAIL under this protocol.

## 16. Claim boundary

A PASS supports only:

> Exact-different learned internal paths can be equivalent with respect to a declared external continuation-consequence family across multiple contexts and shared path-sensitive interventions, while remaining distinguishable under a richer observer.

A PASS does not establish semantic equivalence or authorize identity substitution.
