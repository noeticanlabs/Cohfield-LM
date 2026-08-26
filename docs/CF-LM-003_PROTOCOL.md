# CF-LM-003 — Observer-Relative Continuation Equivalence

Status: **Preregistered implementation protocol v0.1**

Parent contracts:

- `CF-LM-000`
- `CF-LM-IC-00`
- `CF-LM-IC-02`

Parent executable evidence:

- CF-LM-001: `f52641e68f34377e40aab7fc1be4293dcf113e93`
- CF-LM-002: `a0c5afe8189b3d42128e72e375ab3b2f2100fb91`

No CF-LM-003 implementation existed when this protocol was frozen.

## 1. Scientific question

Can two language States that are exactly different in persistent relational configuration be indistinguishable through one frozen continuation observer while remaining distinguishable through a strictly richer frozen observer?

## 2. Claim boundary

CF-LM-003 tests observer-relative continuation equivalence only.

It does not test semantic equivalence.

The target distinction is:

`exact inequality != observer-relative equivalence != semantic equivalence`.

## 3. Existing model

Use the verified `CohfieldLanguageModelV1` unchanged:

`Psi_(t+1) = 0.98 Psi_t + 0.08 e_prev e_current^T`

`X_(t+1) = 0.50 X_t + 0.50 u_t + 0.20 Psi^T X_t`.

No new model parameter is introduced.

## 4. Exposure histories

Use exactly 128 observations per history.

History CD:

`H_CD = (C D)^64`.

History DC:

`H_DC = (D C)^64`.

Both histories contain:

`C = 64`

`D = 64`

and zero `A` or `B` observations.

Thus surface counts are exactly matched; only order differs.

## 5. Comparison-state equalization

After exposure set:

`X_CD = X_DC = 0`

and preserve:

`Theta_CD = Theta_DC = (1,1,1,1)`.

Do not modify either exposure-derived `Psi` before observation.

## 6. Exact-State difference observable

Use Frobenius distance over `Psi`:

`D_Psi = ||Psi_CD - Psi_DC||_F`.

Freeze:

`epsilon_state = 0.05`.

Require:

`D_Psi > 0.05`.

Independent preimplementation evaluation of the frozen equations predicts:

`D_Psi ~= 0.061531831442227035`.

Representative learned edges are predicted as:

`Psi_CD[C][D] ~= 1.868030811690309`

`Psi_CD[D][C] ~= 1.8245212364186827`

`Psi_DC[C][D] ~= 1.8245212364186827`

`Psi_DC[D][C] ~= 1.868030811690309`.

Thus the comparison States are not exactly equal.

## 7. Restricted observer

Define `O_restricted` using the existing `LanguageObservationProfile` with exactly two probes:

1. `A B`
2. `B A`

and:

`continuation_steps = 4`.

The observer records the existing full four-coordinate continuation response at every probe/continuation step.

It does not mask `C` or `D`; the restriction is probe access only.

Use Euclidean response distance:

`D_restricted = ||R_Orestricted(z_CD) - R_Orestricted(z_DC)||_2`.

Freeze:

`epsilon_floor = 1e-12`.

Require:

`D_restricted <= 1e-12`.

The frozen equations predict exact numerical equality because neither State contains a learned path from `A` or `B` into the `C/D` relational substructure.

## 8. Enriched observer

Define `O_enriched` by retaining the two restricted probes and adding:

3. `C D`
4. `D C`

with the same:

- continuation steps;
- response representation;
- Euclidean metric;
- comparison States;
- model parameters.

Thus:

`O_restricted subset O_enriched`

in probe access.

Define:

`D_enriched = ||R_Oenriched(z_CD) - R_Oenriched(z_DC)||_2`.

Freeze:

`epsilon_discrim = 0.01`.

Require:

`D_enriched > 0.01`.

Independent preimplementation evaluation predicts:

`D_enriched ~= 0.01652979019225732`.

## 9. Profile-relative equivalence criterion

CF-LM-003 targets the joint result:

`z_CD != z_DC`

`z_CD ~=_(O_restricted) z_DC`

`z_CD !~=_(O_enriched) z_DC`.

This means the relation is observer-profile-relative.

It MUST NOT be promoted to semantic equivalence.

## 10. Negative and integrity controls

### 10.1 Matched-count control

Programmatically verify both histories contain identical counts.

### 10.2 Exact-difference control

Require `D_Psi > epsilon_state` after comparison equalization.

### 10.3 Restricted repeat control

Repeated observation of the same cloned State under `O_restricted` MUST remain at numerical floor.

### 10.4 Enriched repeat control

Repeated observation of the same cloned State under `O_enriched` MUST remain at numerical floor.

### 10.5 Observer-refinement control

Programmatically verify that the first two probes of `O_enriched` are exactly the two probes of `O_restricted`, and that only `CD` and `DC` are added.

### 10.6 Equalization control

Verify comparison `X` and `Theta` are exactly equal while `Psi` remains different.

## 11. PASS

CF-LM-003 passes only if all are true:

1. matched counts confirmed;
2. `D_Psi > 0.05`;
3. `D_restricted <= 1e-12`;
4. `D_enriched > 0.01`;
5. both observer repeat controls are at floor;
6. observer refinement is exact;
7. equalized `X` and `Theta` are identical.

## 12. FAIL

Any failed required condition yields CF-LM-003 FAIL under this protocol.

No history, observer, threshold, distance metric, continuation depth, or model parameter may be changed after observing failure without a versioned amendment.

## 13. Interpretation of PASS

A PASS supports:

> Exact-different CF-LM language States can be indistinguishable through one declared continuation interface while distinguishable through a richer interface, demonstrating observer-relative continuation equivalence.

It does not support semantic equivalence or natural-language understanding.

## 14. Next experiment boundary

Only after CF-LM-003 disposition should a later experiment test a stronger consequence-equivalence relation across multiple shared contexts and interventions. Semantic equivalence would still require an explicit domain relation and stronger evidence than observer equality alone.
