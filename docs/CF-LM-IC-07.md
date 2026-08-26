# CF-LM-IC-07 — Multi-Profile Observational Equivalence Partition Contract

Status: **Pre-canonical downstream language contract v0.1**

Parent evidence: `CF-LM-007` verified at `fafc0dcc980839e60d31a12bc54fe7d0c1c222e0`.

## Purpose

Define the contract for discovering observational-equivalence classes from continuation behavior alone across a frozen family of profiles.

This contract does **not** define semantic equivalence, exact identity, governance equivalence, or an internal learned abstraction mechanism.

## Inherited architecture

Use the unchanged `CohfieldLanguageModelV1` and the existing language State roles:

`z_L = (X_L, Theta_L, Psi_L)`.

No production model parameter or adaptation law may change for this contract.

## Canonical distinction

The tested relation is observer/profile-relative:

`x ≈_K y`

only when the declared response family cannot distinguish `x` and `y` under profile family `K`.

It remains distinct from:

- exact State equality;
- domain semantic equivalence;
- CohAtom identity;
- governance substitution.

## Carrier

The frozen six-State carrier is constructed from already verified learned weights:

- `R_C`: `A -> C -> B`;
- `R_D`: `A -> D -> B`;
- `R_L`: `R_C` plus the independently learned `D -> D` loop from `(D D)^64`;
- `R_C_cut`: `R_C` with only `C -> B` deleted;
- `R_D_cut`: `R_D` with only `D -> B` deleted;
- `R_0`: zero relational core.

All six States keep `X=0` and `Theta=(1,1,1,1)`.

## Response-family map

For a frozen profile family `K`, define the derived response family:

`Resp_K : State -> Vector`.

`Resp_K(z)` is the concatenation of the declared A/B projected continuation responses for every profile in `K`, in frozen order.

The map is an observational derivative. It is not a new CohBit primitive.

## Partition relation

For this finite deterministic experiment, two candidates are placed in the same discovered class iff their complete `Resp_K` vectors are exactly equal.

The partitioning algorithm may inspect only response vectors. It may not inspect:

- candidate names;
- route kind;
- `Psi` entries;
- source histories;
- expected classes.

## Profile enrichment

Two nested profile families are frozen:

- `K_short` — baseline, original unseen `B -> A` host, and cross-relay `C <-> D` hosts, all at four continuation steps;
- `K_full` — every `K_short` profile plus the cross-relay `C <-> D` profiles at ten continuation steps.

Because `K_short` is a strict prefix/subfamily of `K_full`, the experiment must test whether the behavior-induced partition refines rather than being silently redefined.

## Claim ceiling

PASS may establish finite behavior-only recovery of profile-relative observational classes and profile-enrichment refinement on the frozen carrier.

PASS does not establish that `CohfieldLanguageModelV1` internally stores or reasons over those classes. The classifier remains an external conformance observer unless a later experiment explicitly internalizes the relation.
