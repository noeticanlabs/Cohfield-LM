# CF-LM-006 — Algebraic Closure of Contextual Consequence Equivalence

Status: **Preregistered experimental protocol v0.1**

Parent contract: `CF-LM-IC-05`

Verified parent evidence: `cbb42a50c472f93ab7cef02ea86d6e2e7b451cee`.

## Question

Does the contextual-consequence relation earned by CF-LM-004/005 satisfy reflexivity, symmetry, nontrivial transitivity, and preservation across the frozen host-composition family on a three-State carrier without collapsing exact identity?

## Frozen model

Use the unchanged `CohfieldLanguageModelV1`.

No production parameter or adaptation law changes are permitted.

## Source histories

`H_C = (A C B D)^64`

`H_D = (A D B C)^64`

`H_loop = (D D)^64`.

Extract the learned route weights:

- `A -> C` and `C -> B` from `H_C`;
- `A -> D` and `D -> B` from `H_D`;
- `D -> D` only from `H_loop`.

## Frozen relational cores

`R_C` contains only:

`A -> C`, `C -> B`.

`R_D` contains only:

`A -> D`, `D -> B`.

`R_L` contains:

`A -> C`, `C -> B`, plus the independently learned `D -> D` latent loop.

All `X` values begin at zero and all `Theta` values remain `(1,1,1,1)`.

## Preimplementation numerical cross-check

The unchanged adaptation law predicts the learned latent loop approximately:

`Psi_loop[D][D] = 3.692552048108993`.

Using the already verified learned route weights:

`A->relay = 0.9840816505055259`

`relay->B = 1.0041649494954346`,

the base `w=1.0` relational distances are approximately:

`D_Psi(R_C,R_D) = 1.988348028216815`

`D_Psi(R_C,R_L) = 3.692552048108993`

`D_Psi(R_D,R_L) = 4.193710...`.

All are well above the frozen distinctness threshold `0.70`.

## Host-composition family

For each core add exactly one common host relation:

`Psi[B][A] = w`

for:

`w in {0.5,1.0,2.0}`.

The host edge is a test composition fixture and does not alter learned route weights.

## Declared consequence response

For each host State run context `A`, then separately context `B`.

After the driven input, record A/B coordinates, then run four zero-input continuation steps and record A/B after each step.

Concatenate both context responses.

Use Euclidean distance.

Freeze:

`epsilon_floor = 1e-12`.

Define `related(left,right)` to require projected distance `<= epsilon_floor` independently for every host weight.

## Frozen equivalence-law tests

1. **Carrier construction** — route and latent-loop extraction use only the specified learned weights.
2. **Exact distinction** — every distinct carrier pair has `D_Psi > 0.70` at `w=1.0`.
3. **Reflexivity** — `R_C~R_C`, `R_D~R_D`, `R_L~R_L`.
4. **Symmetry** — every distinct related pair is executed in both argument orders.
5. **Nontrivial transitivity** — establish `R_C~R_D` and `R_D~R_L`, then independently require `R_C~R_L`.
6. **Per-host closure** — every carrier pair remains related separately at `w=0.5,1.0,2.0`.
7. **Rich-observer separation** — full-coordinate contexts `A,B,C,D` distinguish every distinct carrier pair by more than `epsilon_rich=0.13`.
8. **Broken-route counterexample** — delete `C->B` from `R_C`; relation membership against every carrier member must be false, and at `w=1.0` projected distance must exceed `epsilon_break=0.045`.
9. **Determinism** — repeated construction and relation evaluation are identical to floor.
10. **Regression cross-check** — learned loop and selected distances match preregistered numerical values to `1e-9` where exact decimal regression is declared.

## Expected cross-check values

The projected A/B relation distances for all three carrier pairs are predicted to be exactly numerical zero at all three host strengths.

At `w=1.0`, the broken-route projected distance is predicted to be approximately:

`0.048012141014796256`.

For the rich full-coordinate observer over contexts `A,B,C,D`, the expected pairwise distances are comfortably nonzero; the smallest is predicted above `0.13`.

## PASS

CF-LM-006 PASSES only if all algebraic-law, identity-firewall, composition-profile, counterexample, determinism, and regression tests pass without post-result tuning.

## FAIL

Any failed law is recorded as a failed CF-LM-006 result. Thresholds, carrier construction, relation profile, or source histories may not be changed after observing failure except through a versioned successor experiment.

## Claim ceiling

PASS supports a finite executable equivalence relation over the declared carrier/profile only.

PASS does not establish universal semantic equivalence or a universal algebraic theorem.
