# CF-LM-007 — Horizon-Resolved Cross-Profile Equivalence Boundary

Status: **Preregistered experimental protocol v0.1**

Parent contract: `CF-LM-IC-06`

Verified parent evidence: `edee108eb470913e7dab43f83dec91e1115f4650`.

## Question

Does the finite contextual-consequence equivalence demonstrated by `CF-LM-006` survive transfer to a new symmetric cross-relay composition profile, and does that equivalence remain stable when the continuation horizon is extended?

## Frozen model

Use the unchanged `CohfieldLanguageModelV1`.

No production parameter, adaptation law, observer metric, or State role may change.

## Frozen source histories

`H_C = (A C B D)^64`

`H_D = (A D B C)^64`

`H_loop = (D D)^64`.

Construct the same three relational cores as `CF-LM-006`:

- `R_C`: only learned `A -> C` and `C -> B`;
- `R_D`: only learned `A -> D` and `D -> B`;
- `R_L`: `R_C` plus only the independently learned `D -> D` loop.

All `X` begin at zero and all `Theta` remain `(1,1,1,1)`.

## New cross-relay host profile

For each carrier member add the same two host edges:

`C -> D = w`

`D -> C = w`

for:

`w in {0.5,1.0,2.0}`.

The protocol must verify that neither `C -> D` nor `D -> C` occurs as an exposure adjacency in `H_C`, `H_D`, or `H_loop`.

## Frozen observer

Contexts:

`A`

and

`B`.

After the driven context step, retain only the `A/B` coordinates. Then run autonomous zero-input continuation and retain `A/B` after each continuation step.

Distance is Euclidean distance on the concatenated projected response.

Two horizons are frozen:

- `h_short = 4` autonomous continuation steps;
- `h_long = 10` autonomous continuation steps.

An onset check is frozen at `h_onset = 5`.

## Frozen thresholds

`epsilon_floor = 1e-12`

`epsilon_split = 0.005`

`epsilon_onset = 1e-4`

`epsilon_host = 0.001`

`epsilon_distinct = 0.70`

Regression tolerance for preregistered decimal cross-checks:

`1e-9`.

## Preimplementation numerical cross-check

The unchanged equations predict the following projected distances.

### Short horizon, h=4

For every `w in {0.5,1.0,2.0}`:

`D(C,D) = 0`

`D(C,L) = 0`

`D(D,L) = 0`.

### Long horizon, h=10

For `w = 0.5`:

`D(C,D) = 0`

`D(C,L) = D(D,L) = 0.006362262217818672`.

For `w = 1.0`:

`D(C,D) = 0`

`D(C,L) = D(D,L) = 0.0267033265859906`.

For `w = 2.0`:

`D(C,D) = 0`

`D(C,L) = D(D,L) = 0.1278362510615142`.

Thus the expected long-horizon pattern is:

`R_C ~ R_D`

while:

`R_C !~ R_L`

and

`R_D !~ R_L`.

### First visible step

For `R_C` versus `R_L` at `h=5`:

- `w=0.5`: `0.0001459562877565053`;
- `w=1.0`: `0.0005838251510260281`;
- `w=2.0`: `0.0023353006041041194`.

The difference is predicted to be exactly zero through `h=4` and nonzero at `h=5`.

### Host nondegeneracy, h=10

Distance between the `R_C` response with no cross-relay host and with the cross-relay host is predicted as:

- `w=0.5`: `0.0018602257411907692`;
- `w=1.0`: `0.007800361844208359`;
- `w=2.0`: `0.03773737979527526`.

### Exact relational distinction

At common `w=1.0`, host composition leaves pairwise `Psi` distances unchanged because the same host edges are added to both States:

`D_Psi(C,D) = 1.988348028216815`

`D_Psi(C,L) = 3.692552048108993`

`D_Psi(D,L) = 4.193860811866271`.

## Causal interpretation

The `D -> D` latent loop is not visible to the declared `A/B` consequence observer at the short horizon even after the new `C <-> D` host is added.

At the longer horizon, the common cross-relay host permits influence to circulate through the latent D structure and return to the declared consequence coordinates.

The critical intervention is therefore:

`R_L[D][D] := 0`.

Only that edge may be removed. This intervention must restore `R_L` to exact `R_C` relational structure and collapse every long-horizon `C/L` distance back to floor.

## Frozen test set

1. Cross-relay host adjacencies are absent from all source histories.
2. Carrier members remain exact-different under common host composition.
3. All carrier pairs transfer equivalently at `h=4` for every host weight.
4. `R_C` and `R_D` remain equivalent at `h=10` for every host weight.
5. `R_L` separates from both route members at `h=10` for every host weight.
6. Latent split magnitude increases strictly with host strength.
7. `R_C/R_L` remains at floor through `h=4` and exceeds `epsilon_onset` at `h=5` for every host weight.
8. Direct latent-loop ablation restores exact structure and long-horizon equivalence.
9. Cross-relay host has a nontrivial measurable effect on `R_C` itself at `h=10`.
10. Construction and observation are deterministic to floor.
11. Preregistered numerical values reproduce to `1e-9`.

## PASS

`CF-LM-007` PASSES only if the complete mixed transfer/boundary pattern holds without post-result tuning.

A PASS is **not** “everything stays equivalent.”

The required PASS includes both:

- preservation where the preregistered profile predicts preservation; and
- loss of equivalence where the longer-horizon profile exposes latent causal structure.

## FAIL

Any deviation from the frozen pattern is recorded as a failed `CF-LM-007` result. No post-hoc change may be made to the carrier, cross-relay edges, host strengths, horizons, projection, thresholds, metric, or model parameters except through a versioned successor.

## Claim ceiling

A PASS supports finite executable evidence that the learned consequence-equivalence relation is **profile- and horizon-relative**, with a demonstrated transfer region and a demonstrated causal boundary.

A PASS does not establish semantic equivalence or a universal theorem about observer refinement.
