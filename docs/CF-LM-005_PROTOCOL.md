# CF-LM-005 — Contextual Substitution Preservation

Status: **Preregistered implementation protocol v0.1**

Parent contract: `CF-LM-IC-04`

Parent executable evidence: `CF-LM-004` at `1cf39d937ad2139c02aefbeaca19b2754a5ea0a1`.

No CF-LM-005 implementation existed when this protocol was frozen.

## 1. Scientific question

Can two exact-different learned route cores that are consequence-equivalent under CF-LM-004 be explicitly substituted for one another inside a previously unseen larger composition while preserving the declared external continuation consequence?

## 2. Claim boundary

This experiment tests contextual substitution preservation only.

It does not define semantic equivalence or general congruence.

## 3. Frozen source histories

Use exactly:

`H_C = (A C B D)^64`

`H_D = (A D B C)^64`.

Both contain exactly 64 occurrences of each symbol.

Train from `LanguageState::initial()` with the unchanged `CohfieldLanguageModelV1::default()` profile.

## 4. Frozen route extraction

From the learned `H_C` State copy only:

- `Psi[A][C]`;
- `Psi[C][B]`.

From the learned `H_D` State copy only:

- `Psi[A][D]`;
- `Psi[D][B]`.

Every other route-core `Psi` entry is zero.

Do not normalize or equalize the copied magnitudes.

## 5. Frozen host family

Construct new fresh States with:

`X = 0`

`Theta = (1,1,1,1)`

and the selected extracted route core.

Add the common host relation:

`Psi[B][A] = w`

for:

`w in {0.5, 1.0, 2.0}`.

All non-route, non-host relational entries remain zero.

Programmatically verify that `B -> A` is absent from both frozen source exposure histories, including repeat-boundary adjacency.

## 6. Frozen context family

Use exactly two contexts:

1. `[A]`
2. `[B]`.

For each context:

1. start from the corresponding host State;
2. apply the context symbol once;
3. record the resulting four-coordinate fast State;
4. apply four zero-input continuation steps;
5. record the four-coordinate State after each step.

## 7. Frozen substitution observer

For the primary substitution relation retain only coordinates:

`A, B`.

Flatten records in this order:

`A-context driven state`

`A-context zero steps 1..4`

`B-context driven state`

`B-context zero steps 1..4`.

Distance is Euclidean distance over the resulting flattened projected response.

## 8. Frozen thresholds

`epsilon_floor = 1e-12`

`epsilon_state = 1.9`

`epsilon_break = 0.045`

`epsilon_rich = 0.23`.

Regression-only tolerance for the preimplementation decimal cross-check is `1e-9`.

## 9. Primary host-family substitution test

For every `w in {0.5,1.0,2.0}` require:

1. `||Psi_C(w)-Psi_D(w)||_F > 1.9`;
2. `D_AB(C(w),D(w)) <= 1e-12`.

Any failed host member yields CF-LM-005 FAIL.

## 10. Explicit full substitution test

At `w = 1.0`, begin with the C-host.

Construct a substituted State by:

- setting `Psi[A][C] = 0`;
- setting `Psi[C][B] = 0`;
- setting `Psi[A][D]` to the learned D-route first-hop magnitude;
- setting `Psi[D][B]` to the learned D-route second-hop magnitude;
- preserving `Psi[B][A] = 1.0`;
- preserving every other entry exactly.

Require the resulting `Psi` matrix to equal the independently constructed D-host matrix exactly.

Require the projected response before versus after substitution to remain within `1e-12`.

## 11. First-hop-only negative control

At `w = 1.0`, begin with the C-host.

Replace only:

`A -> C`

with:

`A -> D`.

Leave `C -> B` intact and do not insert `D -> B`.

Require projected response distance from intact C-host to be:

`> 0.045`.

## 12. Second-hop-only negative control

At `w = 1.0`, begin with the C-host.

Replace only:

`C -> B`

with:

`D -> B`.

Leave `A -> C` intact and do not insert `A -> D`.

Require projected response distance from intact C-host to be:

`> 0.045`.

## 13. Route-use cut control

At `w = 1.0`, delete only:

`C -> B`

from the intact C-host.

Require projected response distance from the intact C-host to be:

`> 0.045`.

This establishes that the primary observer depends materially on the inserted route.

## 14. Rich-observer control

At `w = 1.0`, repeat the same `[A]` and `[B]` contexts but retain all four coordinates.

Require:

`D_full(C,D) > 0.23`.

Thus the substitution relation preserves the declared consequence while the exact route realization remains distinguishable.

## 15. Determinism

Repeated construction and observation of each frozen host profile must reproduce the same relational matrix and numerical response to `epsilon_floor`.

## 16. Preimplementation numerical record

Using the already verified CF-LM-001 equations and CF-LM-004 learned route magnitudes, the frozen protocol predicts:

`Psi_C[A][C] = 0.9840816505055259`

`Psi_C[C][B] = 1.0041649494954346`

`Psi_D[A][D] = 0.9840816505055259`

`Psi_D[D][B] = 1.0041649494954346`

`||Psi_host_C-Psi_host_D||_F = 1.988348028216815`.

At `w = 1.0`:

`D_AB(C,D) = 0`

`D_full(C,D) = 0.24267014285915262`

`D_AB(route_cut) = 0.048012141014796256`

`D_AB(first-hop-only hybrid) = 0.048012141014796256`

`D_AB(second-hop-only hybrid) = 0.048012141014796256`.

## 17. PASS disposition

CF-LM-005 passes only if all required primary, explicit-substitution, negative-control, nondegeneracy, rich-observer, determinism, and frozen-cross-check conditions pass without modifying the preregistered experiment.

## 18. Interpretation of PASS

A PASS supports only:

> Within the declared CF-LM-005 host family and A/B consequence observer, the two learned route cores are explicitly substitutable as whole routes: replacing one complete route with the other preserves downstream continuation consequence, while incomplete substitution does not.

This remains pre-semantic evidence.