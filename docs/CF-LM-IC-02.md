# CF-LM-IC-02 — Observer-Relative Continuation Equivalence Contract

Status: **Draft v0.1 — staged before CF-LM-003 implementation**

Parent contracts:

- `CF-LM-IC-00`
- `CF-LM-IC-01`

Parent executable evidence:

- CF-LM-001 ordered-exposure plasticity: `f52641e68f34377e40aab7fc1be4293dcf113e93`
- CF-LM-002 two-hop composition: `a0c5afe8189b3d42128e72e375ab3b2f2100fb91`

## 1. Purpose

This contract defines the first language-domain test of the canonical distinction among exact State identity, observer-relative continuation equivalence, and semantic equivalence.

It introduces no new primitive and does not redefine State or semantic equivalence.

The contract asks only whether two exact-different language States can be indistinguishable through one declared continuation-observation profile while remaining distinguishable through a strictly richer declared observer.

## 2. Canonical non-collapse

For language-domain States `z1` and `z2`, CF-LM-003 MUST preserve:

`z1 = z2`

as distinct from:

`z1 ~=_O z2`

and both distinct from any later domain-declared semantic equivalence relation.

A CF-LM-003 PASS MUST NOT be reported as semantic equivalence.

## 3. Exact-difference obligation

The two comparison States MUST differ in an inspectable identity-bearing model component after fast-state and local-condition equalization.

For CF-LM-003 this component is `Psi_L`.

Require:

`||Psi_1 - Psi_2||_F > epsilon_state`.

The threshold MUST be frozen before implementation execution.

## 4. Observer profile

An observer profile is the declared continuation interface used to interrogate a State.

For CF-LM-003 it is defined by:

- a frozen probe family;
- the existing continuation horizon/step count;
- the existing full numerical response carrier;
- Euclidean response distance.

Observer equivalence is therefore profile-relative:

`z1 ~=_O z2 iff d(R_O(z1), R_O(z2)) <= epsilon_floor`.

This relation is not a new State identity relation and cannot substitute for exact equality in CohBit identity, adjacency, authority, commitment, or trace continuity.

## 5. Observer refinement

CF-LM-003 MUST include two frozen observer profiles:

- `O_restricted`;
- `O_enriched`.

`O_enriched` MUST strictly add probe access beyond `O_restricted` while preserving the same model, State pair, response representation, distance metric, and continuation depth.

The target pattern is:

`z1 ~=_(O_restricted) z2`

while:

`z1 !~=_(O_enriched) z2`.

This establishes profile relativity rather than universal equivalence.

## 6. Exposure-origin obligation

The comparison States MUST arise from ordered surface exposure under the existing CF-LM adaptation law.

Their histories MUST have matched per-symbol counts so that raw symbol frequency does not explain exact State difference.

No external evaluation signal may enter adaptation.

## 7. Comparison-state equalization

Before either observer is applied:

- `X_1 = X_2 = 0`;
- `Theta_1 = Theta_2 = (1,1,1,1)`;
- `Psi_1` and `Psi_2` remain exposure-derived and unmodified.

Thus the exact difference resides in persistent relational configuration.

## 8. Required conformance controls

CF-LM-003 MUST include at least:

1. matched exposure counts;
2. exact `Psi` difference above the frozen State threshold;
3. restricted-observer response distance at numerical floor;
4. enriched-observer response distance above the frozen discrimination threshold;
5. deterministic repeat for each observer;
6. direct confirmation that observer enrichment changes only the probe family;
7. no claim of semantic equivalence.

## 9. Claim ceiling

A PASS may support only:

> Two exact-different CF-LM language States can be continuation-equivalent relative to one declared observer and distinguishable relative to a richer observer.

A PASS does NOT establish:

- semantic equivalence;
- paraphrase;
- synonymy;
- same denotation;
- same governed meaning;
- substitutability in CohBit identity or composition;
- natural-language understanding.

## 10. Freeze condition

CF-LM-IC-02 remains draft until:

1. histories are frozen;
2. both observer profiles are frozen;
3. `epsilon_state`, `epsilon_floor`, and `epsilon_discrim` are frozen;
4. the experiment runs locally without post-result tuning;
5. PASS/FAIL is recorded.
