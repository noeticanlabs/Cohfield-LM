# CF-LM-008 — Multi-Profile Observational Equivalence Partition Discovery

Status: **Preregistered experimental protocol v0.1**

Parent contract: `CF-LM-IC-07`

Verified parent evidence: `fafc0dcc980839e60d31a12bc54fe7d0c1c222e0`.

## Question

Can profile-relative observational classes be recovered from continuation responses alone, without structural labels or `Psi` inspection, and can a richer profile family refine those classes exactly where CF-LM-007 showed previously latent consequence becomes reachable?

## Frozen model

Use unchanged `CohfieldLanguageModelV1`.

No production model code, parameter, or adaptation-law changes are permitted.

## Source histories

Reuse:

- `H_C = (A C B D)^64`;
- `H_D = (A D B C)^64`;
- `H_loop = (D D)^64`.

## Frozen six-State carrier

Construct:

1. `R_C` from learned `A->C` and `C->B`;
2. `R_D` from learned `A->D` and `D->B`;
3. `R_L = R_C + learned D->D loop`;
4. `R_C_cut` by deleting only `C->B` from `R_C`;
5. `R_D_cut` by deleting only `D->B` from `R_D`;
6. `R_0` with zero `Psi`.

Every pair within a discovered multi-member class must remain exact-different in `Psi` by more than `epsilon_state = 0.90`.

## Frozen blinded order

The classifier receives candidates only in this fixed slot order:

`[R_L, R_D_cut, R_C, R_0, R_D, R_C_cut]`.

The partition function receives only response-family vectors, not these names.

## Observation primitive

For each profile:

- start from equalized `X=0`;
- use contexts `A` and `B` separately;
- retain only A/B coordinates after the driven step and after every autonomous continuation step;
- concatenate in deterministic profile order.

No candidate labels or relational entries may enter the partition algorithm.

## `K_short`

Frozen profiles, in order:

1. baseline, no added host, `h=4`;
2. unseen host `B->A = 1.0`, `h=4`;
3. symmetric unseen host `C<->D = 0.5`, `h=4`;
4. symmetric unseen host `C<->D = 1.0`, `h=4`;
5. symmetric unseen host `C<->D = 2.0`, `h=4`.

Predicted exact partition by blinded slot indices:

`{{0,2,4}, {1,3,5}}`.

Thus the short-profile class sizes are `[3,3]`.

## `K_full`

Append to `K_short`, in order:

6. symmetric unseen host `C<->D = 0.5`, `h=10`;
7. symmetric unseen host `C<->D = 1.0`, `h=10`;
8. symmetric unseen host `C<->D = 2.0`, `h=10`.

Predicted exact partition by blinded slot indices:

`{{0}, {2,4}, {1,3,5}}`.

Thus the full-profile class sizes are `[1,2,3]` after sorting.

## Refinement obligation

`K_full` must strictly refine `K_short`:

- every full-profile class is contained in one short-profile class;
- no pair separated under `K_short` may merge under `K_full`;
- at least one short-profile class must split under `K_full`.

The expected split is the latent-loop member leaving the route-equivalence class while `R_C` and `R_D` remain together.

## Rich-observer identity firewall

Use full A/B/C/D coordinates under contexts `A,B,C,D` with ten continuation steps and no added host.

Every distinct pair that shares a discovered class under either profile family must remain rich-observer distinguishable by more than:

`epsilon_rich = 0.15`.

## Preimplementation numerical cross-check

Using the unchanged verified equations:

- full-family distance `R_C` vs `R_D` = `0`;
- full-family distance `R_C_cut` vs `R_D_cut` = `0`;
- full-family distance `R_C_cut` vs `R_0` = `0`;
- full-family distance `R_C` vs `R_L` ≈ `0.13075034652630502`;
- full-family distance `R_C` vs `R_C_cut` ≈ `0.1595695409356687`;
- short-family distance `R_C` vs `R_C_cut` ≈ `0.10561775346766529`;
- smallest preregistered rich-observer distance among same-class exact-different pairs ≈ `0.16938784150885`.

## PASS

CF-LM-008 PASSES only if:

- the partition algorithm depends only on response-family vectors;
- the short partition is recovered exactly;
- the full partition is recovered exactly;
- `K_full` strictly refines `K_short` with no merge under enrichment;
- exact State identity remains distinct inside every nontrivial class;
- rich observation distinguishes all same-class exact-different members;
- deterministic and numerical regression controls hold.

## FAIL

Any class mismatch, illicit structural inspection, merge under enrichment, identity collapse, or regression failure is recorded as a failed result. The carrier, profile ordering, horizons, observer, or expected partition may not be changed after observing failure except through a versioned successor.

## Claim ceiling

PASS supports behavior-only discovery of finite, profile-relative observational partitions in an external conformance observer.

PASS does not establish semantic equivalence or an endogenous abstraction mechanism inside the language model.
