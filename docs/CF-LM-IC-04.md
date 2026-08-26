# CF-LM-IC-04 — Contextual Substitution Preservation Contract

Status: **Draft v0.1 — preregistered before CF-LM-005 implementation**

Parent evidence:

- CF-LM-001 ordered-exposure plasticity: `f52641e68f34377e40aab7fc1be4293dcf113e93`
- CF-LM-002 two-hop composition: `a0c5afe8189b3d42128e72e375ab3b2f2100fb91`
- CF-LM-003 observer-relative equivalence: `c4d94cf480c745fc378dbb7de7f447b42eb163d5`
- CF-LM-004 context-general consequence equivalence: `1cf39d937ad2139c02aefbeaca19b2754a5ea0a1`

Parent contract family: `CF-LM-IC-00` through `CF-LM-IC-03`.

## 1. Purpose

CF-LM-IC-04 defines the minimum executable obligations for testing whether the consequence-equivalent route pair established by CF-LM-004 may be **explicitly substituted inside a new declared composition context** while preserving the declared external continuation consequence.

This contract does not define semantic equivalence and does not grant general substitution rights.

The canonical firewall is:

`explicit contextual substitution != exact identity != semantic equivalence != governance authority`.

## 2. Canonical basis

CohBit Primitive permits State, Action, and path substitution only under an explicit relation appropriate to the operation. Equivalent objects do not silently become identical, and substitution must preserve the exact identities of the objects being related.

CF-LM-005 therefore tests one narrow operational relation:

`Substitutable_K(route_C, route_D; host_family, observer)`.

The relation is provisional and experiment-local.

## 3. Source route pair

Reuse the two verified CF-LM-004 exposure histories:

`H_C = (A C B D)^64`

`H_D = (A D B C)^64`.

From the resulting learned States, extract only the two-hop route cores:

`route_C = { Psi[A][C], Psi[C][B] }`

`route_D = { Psi[A][D], Psi[D][B] }`.

No route magnitude may be manually equalized. The extracted weights must be those produced by the frozen CF-LM-001 adaptation law.

The extracted route cores remain exact-different structures.

## 4. Unseen host composition

Define a host relation absent from both original exposure histories:

`B -> A`.

For host strength `w`, construct a fresh equalized host State with:

- `X = 0`;
- `Theta = (1,1,1,1)`;
- exactly one inserted route core;
- common host edge `Psi[B][A] = w`;
- all other `Psi` entries zero.

Freeze the host family:

`W = {0.5, 1.0, 2.0}`.

Thus the two larger compositions are:

`B -> A -> C -> B -> A ...`

and

`B -> A -> D -> B -> A ...`.

The host relation is deliberately not learned in `H_C` or `H_D`; it is a declared composition context used only for substitution testing.

## 5. Two-sided context family

Use two frozen driven contexts:

1. `A` — tests the learned route followed by the common host edge;
2. `B` — tests the common host edge followed by the learned route.

For each context:

- initialize from the fresh host State;
- apply the one surface input;
- record the resulting State;
- apply four zero-input continuation steps, recording each State.

The substitution observer projects every recorded State onto the `A/B` coordinates only.

Concatenate both contexts into one projected consequence response.

## 6. Exact-difference requirement

For every `w in W`, the route-host States must remain exact-different in relational configuration.

Freeze:

`epsilon_state = 1.9`.

Require:

`||Psi_host_C(w) - Psi_host_D(w)||_F > 1.9`.

This prevents substitution preservation from being reported as State identity.

## 7. Contextual substitution preservation

Freeze:

`epsilon_floor = 1e-12`.

For every `w in W`, require:

`D_AB(host_C(w), host_D(w)) <= epsilon_floor`.

This tests preservation across a family of previously unseen larger compositions, not only one host weight.

## 8. Explicit substitution operation

At `w = 1.0`, construct an explicit substitution transform:

`Sub_C_to_D`.

It must:

1. remove `A -> C` and `C -> B` from the C-host;
2. insert the learned `A -> D` and `D -> B` route weights;
3. preserve the common host edge `B -> A` exactly;
4. preserve `X` and `Theta` exactly;
5. modify no other `Psi` entry.

The resulting State must be exact-equal to the independently constructed D-host State.

The projected consequence response must remain within `epsilon_floor` before versus after the full substitution.

This is explicit substitution; no identity merge occurs.

## 9. Partial-substitution negative controls

A correct substitution relation must not bless arbitrary partial rewrites.

Construct two broken hybrids at `w = 1.0`:

- first-hop-only substitution: replace `A -> C` with `A -> D` but leave `C -> B`;
- second-hop-only substitution: replace `C -> B` with `D -> B` but leave `A -> C`.

Freeze:

`epsilon_break = 0.045`.

Require each broken hybrid to change the projected consequence response by more than `0.045` relative to the intact C-host.

Thus:

`full route substitution may preserve consequence`

while

`partial route substitution must not`.

## 10. Route-use nondegeneracy

At `w = 1.0`, delete only the second route edge from the intact C-host.

Require the projected consequence response to change by more than:

`epsilon_break = 0.045`.

This establishes that the host observer is actually sensitive to use of the inserted route.

## 11. Rich-observer identity control

The projected A/B observer may regard the two host States as consequence-equivalent while a richer observer still distinguishes their internal route realization.

Use the same `A` and `B` contexts and continuation depth, but retain all four coordinates.

Freeze:

`epsilon_rich = 0.23`.

At `w = 1.0`, require:

`D_full(host_C, host_D) > 0.23`.

Therefore contextual substitution preservation does not erase internal path identity.

## 12. Frozen numerical cross-check

Before Rust CF-LM-005 implementation, the existing verified equations predict approximately:

`||Psi_host_C - Psi_host_D||_F = 1.988348028216815`

at every host strength in `W`.

At `w = 1.0`:

`D_AB(full substitution pair) = 0`

`D_full = 0.24267014285915262`

`D_AB(second-edge cut) = 0.048012141014796256`

`D_AB(first-hop-only hybrid) = 0.048012141014796256`

`D_AB(second-hop-only hybrid) = 0.048012141014796256`.

Regression checks may use `1e-9` around these reported decimal predictions. The actual PASS thresholds above remain frozen separately.

## 13. PASS claim boundary

A PASS may support only:

> The CF-LM consequence-equivalent route pair can be explicitly substituted inside the declared unseen host-composition family while preserving the declared external A/B continuation consequence, and incomplete substitution fails.

A PASS does not establish:

- universal congruence;
- semantic equivalence;
- lexical synonymy;
- logical substitutivity;
- grammatical interchangeability;
- CohAtom identity substitution;
- trace substitution;
- authority to replace committed objects.

## 14. Failure discipline

If any required test fails, CF-LM-005 is FAIL under this contract.

No frozen history, route extraction rule, host edge, host weight, context family, observer projection, threshold, continuation depth, response metric, or model parameter may be changed after observing the failed result without a versioned amendment.