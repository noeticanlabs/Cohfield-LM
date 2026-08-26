# CF-LM-IC-05 — Algebraic Closure of Contextual Consequence Equivalence

Status: **Experimental contract v0.1 — preregistered before CF-LM-006 implementation**

Parent evidence: CF-LM-005 `cbb42a50c472f93ab7cef02ea86d6e2e7b451cee`.

## Purpose

CF-LM-006 asks whether the contextual-consequence relation earned by CF-LM-004/005 satisfies the executable algebraic obligations of an equivalence relation over a declared finite carrier while remaining distinct from exact identity.

This contract does not define semantic equivalence and does not modify State, Action, Transition, Atomic Transition, CohAtom, CohField, CohBit, or CohTrace.

## Canonical basis

The Mathematical Spine permits a domain-relative equivalence relation and states that an equivalence relation should ordinarily satisfy reflexivity, symmetry, and transitivity. It also requires substitution rights to be declared explicitly rather than inferred from equivalence.

CF-LM-006 therefore defines only an experimental relation `~_K`, **contextual consequence equivalence under profile K**.

`~_K != exact equality`.

`~_K != semantic equivalence`.

## Frozen carrier

The carrier contains three exact-different relational cores:

- `R_C`: learned `A -> C -> B` route;
- `R_D`: learned `A -> D -> B` route;
- `R_L`: `R_C` plus an independently learned latent `D -> D` loop.

`R_C` and `R_D` use the unchanged learned route weights extracted from:

`H_C = (A C B D)^64`

`H_D = (A D B C)^64`.

The latent loop is learned independently from:

`H_loop = (D D)^64`.

Only the learned `Psi[D][D]` component is composed into `R_L`. No weight may be normalized or tuned.

## Exact-difference requirement

At host strength `w=1.0`, every distinct carrier pair must satisfy:

`D_Psi(R_i,R_j) > 0.70`.

## Declared composition profile K

For each core construct hosts by adding the common relation:

`B -> A`

at strengths:

`W = {0.5, 1.0, 2.0}`.

For each host apply contexts `A` and `B`, followed by four zero-input continuation steps, and project the full dynamics onto A/B coordinates.

Let the concatenated projected response be `R_K(core;w)`.

Define:

`p ~_K q`

iff for every `w in W`:

`d(R_K(p;w),R_K(q;w)) <= 1e-12`.

This relation is finite-profile executable evidence only.

## Required algebraic obligations

### Reflexivity

For every carrier member `r`, require `r ~_K r` using the same relation evaluator used for distinct pairs.

### Symmetry

For every distinct pair `p,q`, execute and require both `p ~_K q` and `q ~_K p`.

### Nontrivial transitivity

Require:

`R_C ~_K R_D`

and

`R_D ~_K R_L`,

then independently evaluate and require:

`R_C ~_K R_L`.

`R_L` must remain exact-different from both prior members.

### Composition-profile closure

Relation membership must hold separately at all three host strengths. A single aggregate equality is insufficient.

This is not universal congruence.

## Rich-observer identity firewall

Use rich contexts `A,B,C,D`, record all four coordinates, and run the same four continuation steps.

Every distinct carrier pair must satisfy:

`D_rich > 0.13`.

Thus relation membership must not erase internal State identity.

## Counterexample requirement

Construct `R_break` by deleting only `Psi[C][B]` from `R_C`.

`R_break` must not be related to any carrier member under `~_K`.

At `w=1.0`, projected consequence distance from every carrier member must exceed:

`epsilon_break = 0.045`.

This prevents the relation from degenerating into a universal relation.

## Claim ceiling

A PASS supports only:

> On the frozen three-State carrier and declared finite profile K, contextual consequence equivalence behaves as an executable equivalence relation and remains preserved across the tested host-composition family while exact relational identity remains distinct.

A PASS does not establish semantic equivalence, universal congruence, linguistic synonymy, denotational equality, universal substitution, CohAtom identity equivalence, governance status, or CohTrace equivalence.

## Evidence class

Rust tests provide runtime conformance evidence, not formal proof over an unbounded carrier.

`runtime test != formal proof`.

## Freeze discipline

After first local execution begins, no change is permitted without versioned amendment to: carrier construction, source histories, latent-loop extraction, host relation, host strengths, contexts, projection, continuation depth, metric, thresholds, or model parameters.
