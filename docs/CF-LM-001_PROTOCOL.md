# CF-LM-001 — Language-Induced Relational Continuation Plasticity

Status: **Preregistered implementation protocol v0.1**

Parent contracts:

- `CF-LM-000`
- `CF-LM-IC-00`

No Rust language-profile implementation existed when this protocol was frozen.

## 1. Scientific question

Can ordered surface-language exposure create persistent relational configuration in the existing CF-ACP model such that future continuation response differs after fast state and local condition are equalized, with the difference collapsing under direct relational-configuration replacement?

## 2. Claim boundary

This experiment tests language-domain occupancy and relational continuation plasticity only.

It does not test semantic understanding, grammar, reasoning, generation, or open-domain natural language.

## 3. Surface alphabet

Use the controlled four-symbol surface alphabet:

`Sigma = {A, B, C, D}`.

Each surface symbol is represented by a deterministic one-hot domain input in `R^4`.

This one-hot surface mapping is a boundary encoding, not a learned embedding and not a token-prediction vocabulary.

No learned tokenizer is permitted.

## 4. State profile

Use:

- `X_L in R^4` — fast activity;
- `Theta_L = (1,1,1,1)` — fixed local condition for CF-LM-001;
- `Psi_L in R^(4x4)` — directed persistent relational configuration.

Initial state:

`X_0 = 0`

`Theta_0 = (1,1,1,1)`

`Psi_0 = 0`.

`Theta` MUST remain unchanged throughout CF-LM-001.

## 5. Exposure histories

Use exactly 128 surface observations per history.

History A:

`H_A = (A B C D)^32`

History B:

`H_B = (A D C B)^32`.

Both histories contain exactly 32 occurrences of each symbol.

Therefore symbol-frequency counts are matched; the intended experimental difference is ordered adjacency structure.

## 6. Relational adaptation

For consecutive observed symbols `(s_(t-1), s_t)`, let `e_s` be the deterministic one-hot surface vector.

Update directed relational configuration using:

`Psi_(t+1) = (1 - rho) Psi_t + eta e_(s_(t-1)) e_(s_t)^T`.

Freeze:

`rho = 0.02`

`eta = 0.08`.

No external evaluation signal enters this update.

The first symbol of an exposure applies decay only because no predecessor exists.

## 7. Comparison-state equalization

After exposure, construct the comparison state by setting:

`X_A = X_B = 0`

and preserving:

`Theta_A = Theta_B = (1,1,1,1)`.

Do not modify `Psi_A` or `Psi_B` before the pre-intervention probe.

Thus the intended comparison is:

`same X`

`same Theta`

`different exposure-derived Psi`.

## 8. Finite language continuation dynamics

For a surface input vector `u_t`, use the discrete finite-step evolution law:

`X_(t+1) = beta X_t + g_u u_t + gamma Psi^T X_t`.

Freeze:

`beta = 0.50`

`g_u = 0.50`

`gamma = 0.20`.

`Theta` is fixed in CF-LM-001 and therefore does not enter this first specialized evolution law beyond remaining an explicitly equalized State role. Later experiments may give `Theta_L` active dynamics only through a versioned extension.

This is a domain-specific linear continuation law. It is not a neural layer and is not trained by gradient descent.

## 9. Frozen fresh probe family

Use the four ordered two-symbol probes:

1. `A C`
2. `B D`
3. `C A`
4. `D B`.

These ordered pairs do not occur verbatim as adjacent pairs in either periodic exposure history.

For each probe:

1. initialize `X = 0`;
2. apply the first surface input and record resulting `X`;
3. apply the second surface input and record resulting `X`;
4. apply four zero-input continuation steps, recording `X` after each step.

Flatten all recorded vectors in fixed probe order to form the continuation response `R_L(z)`.

Response dimension:

`4 probes * 6 recorded steps * 4 coordinates = 96`.

## 10. Distance measure

Use Euclidean response distance:

`D_R(A,B) = ||R_L(z_A) - R_L(z_B)||_2`.

No alternate response metric may replace this after results are observed without a versioned amendment.

## 11. Preregistered thresholds

Freeze:

`epsilon_floor = 1e-12`

`epsilon_R = 0.10`.

Primary pre-intervention target:

`D_R(A,B) > 0.10`.

Direct replacement target:

construct

`z_I = z_A[Psi := Psi_B]`

while keeping comparison `X` and `Theta` unchanged, then require

`D_R(I,B) <= 1e-12`.

## 12. Negative controls

### 12.1 Identical-history control

Two independent runs of `H_A` from identical initial state and parameters MUST produce identical `Psi` and response to numerical floor.

### 12.2 Matched-count order control

The implementation MUST verify that `H_A` and `H_B` have identical per-symbol counts before computing the target comparison.

### 12.3 No-adaptation control

Repeat both histories with `eta = 0`.

After comparison-state equalization, require:

`D_R(no-adapt A, no-adapt B) <= epsilon_floor`.

### 12.4 Direct-Psi replacement

Required primary causal intervention described above.

### 12.5 Fresh-probe control

The implementation MUST verify programmatically that none of the four probe pairs occurs as an adjacent pair in either periodic exposure pattern.

## 13. Preregistered expected disposition rules

### PASS

CF-LM-001 passes only if all are true:

1. matched symbol counts confirmed;
2. `||Psi_A - Psi_B||_F > 0`;
3. pre-intervention `D_R > 0.10`;
4. direct `Psi_A -> Psi_B` replacement gives `D_R <= 1e-12`;
5. identical-history control is at numerical floor;
6. no-adaptation control is at numerical floor;
7. fresh-probe condition is satisfied.

### FAIL

Any failed required condition yields CF-LM-001 FAIL under this protocol.

The failure MUST be recorded before changing any exposure, parameter, threshold, probe family, response metric, or adaptation law.

## 14. Interpretation of PASS

A PASS supports:

> Ordered surface-language exposure can create persistent directed relational configuration inside the CF-ACP model, and that configuration can causally alter finite future continuation response to fresh ordered surface probes after fast state and local condition are equalized.

A PASS does not establish semantic meaning or natural-language competence.

## 15. Next experiment boundary

Only after CF-LM-001 disposition should a later experiment ask whether learned relational structure supports abstraction beyond surface adjacency, such as equivalence, compositionality, or generalized relation transfer.
