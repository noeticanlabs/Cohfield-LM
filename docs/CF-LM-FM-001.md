# CF-LM-FM-001 — Contextual Consequence Equivalence Formalization Contract

Status: **Pre-canonical formalization contract v0.1**

Verified experimental parent: `CF-LM-006` evidence commit `edee108eb470913e7dab43f83dec91e1115f4650`.

## 1. Purpose

Freeze the mathematical object to be formalized after CF-LM-001 through CF-LM-006.

This contract does **not** amend CohBit primitives and does **not** declare semantic equivalence. It defines a language-domain relation over continuation consequences and separates exact mathematical equivalence from the tolerance-based empirical classifier used in Rust.

## 2. Canonical boundary

The Mathematical Spine distinguishes exact equality, domain semantic equivalence, and observational equivalence. It also requires explicit substitution and profile-bound composition semantics.

Accordingly:

```text
ContextualConsequenceEquivalent_K ≠ exact State equality
ContextualConsequenceEquivalent_K ≠ domain semantic equivalence
ContextualConsequenceEquivalent_K ≠ CohAtom identity
ContextualConsequenceEquivalent_K ≠ governance substitution permission
```

No identity-bearing object is merged by this relation.

## 3. Mathematical carriers

Let:

- `S_L` be the declared language-domain State carrier;
- `W_K` be the host/composition-profile carrier;
- `Y_K` be the declared consequence-response carrier.

For the CF-LM-006 profile:

```text
W_K = {0.5, 1.0, 2.0}
Y_K = finite A/B consequence-response vectors
```

The formal definition is intentionally generic over `S_L`, `W_K`, and `Y_K`.

## 4. Response map

Freeze a profile-indexed response map:

\[
R_K : S_L \times W_K \to Y_K.
\]

Semantic interpretation:

`R_K(x,w)` is the declared consequence observation produced by State `x` under profile member `w`.

Architectural purpose:

`R_K` is an observation/consequence map only. It does not verify truth, decide admissibility, grant authority, execute, commit, or create trace identity.

## 5. Exact contextual consequence equivalence

Define:

\[
\boxed{
x \sim_K y
\iff
\forall w\in W_K,\;R_K(x,w)=R_K(y,w).
}
\]

This is the mathematical relation Lean should formalize as the primary object.

### Required universal laws

For arbitrary `S_L`, `W_K`, `Y_K`, and total `R_K`, prove:

\[
\forall x,\;x\sim_Kx,
\]

\[
x\sim_Ky\Rightarrow y\sim_Kx,
\]

\[
x\sim_Ky\land y\sim_Kz\Rightarrow x\sim_Kz.
\]

These laws follow from equality of the response function, not from CF-LM-006 finite-case enumeration.

## 6. Empirical tolerance classifier

Rust used the classifier:

\[
\boxed{
x\sim_{K,\varepsilon}y
\iff
\forall w\in W_K,\;d(R_K(x,w),R_K(y,w))\le\varepsilon
}
\]

with:

\[
\varepsilon=10^{-12}.
\]

This relation is retained as an **empirical classifier**, not promoted to a universal equivalence relation.

### Mathematical firewall

For positive `ε`, metric closeness is generally not transitive.

Therefore Lean must **not** prove or assume universal transitivity of `~_{K,ε}` without additional hypotheses.

CF-LM-006 establishes only that the frozen three-State carrier happened to satisfy reflexivity, symmetry, and transitivity under this classifier.

## 7. Exact identity separation

The formalization must preserve:

\[
x\sim_Ky\not\Rightarrow x=y.
\]

The relation classifies consequences under profile `K`; it does not erase internal State identity.

A model may therefore satisfy:

\[
x\neq y
\quad\land\quad
x\sim_Ky.
\]

## 8. Profile-bound congruence

Let `C` be a declared partial composition/host operation:

\[
C : H \times S_L \rightharpoonup S_L.
\]

A congruence claim is not automatic.

For an explicitly declared composition class `H_K`, the desired preservation obligation is:

\[
\boxed{
x\sim_Ky
\land C(h,x)\downarrow
\land C(h,y)\downarrow
\Rightarrow
C(h,x)\sim_KC(h,y).
}
\]

This theorem may be proved only from explicit assumptions about `R_K` and `C` or for a declared concrete model. It is not built into the definition of `~_K`.

## 9. Substitution boundary

CF-LM-005 experimentally established whole-route contextual substitution for one frozen composition family and rejected partial substitutions.

The formalization therefore distinguishes:

```text
relation membership
≠
substitution permission
```

Any substitution theorem must name:

- the operation being preserved;
- the composition domain;
- the response/consequence profile;
- the required premises.

No theorem may infer primitive identity substitution or governance substitution from `~_K` alone.

## 10. Rust evidence versus formal theorem

Rust has established finite executable evidence for:

- one concrete language dynamical model;
- one three-State carrier;
- one finite host family;
- one A/B consequence projection;
- one tolerance classifier;
- one broken-route counterexample;
- one contextual substitution family.

Lean is tasked with separating and proving:

1. universal algebraic properties that follow definitionally from exact response equality;
2. model-specific theorems about declared composition/substitution operations;
3. finite concrete carrier theorems corresponding to CF-LM-006 where useful.

Lean must not convert numerical experiment results into universal assumptions.

## 11. Required proof artifacts

The first Lean formalization should expose at least:

```text
ContextualProfile
contextualEquivalent
contextualEquivalent_refl
contextualEquivalent_symm
contextualEquivalent_trans
ContextualEquivalentSetoid
approxContextual
approxContextual_refl
approxContextual_symm
```

and must deliberately omit a universal `approxContextual_trans` theorem unless its required additional assumptions are explicit.

## 12. Compatibility

Affected layer: downstream `Cohfield-LM` mathematical/formal interface only.

Upstream CohBit primitives: unchanged.

CF-ACP: unchanged.

CF-LM-001 through CF-LM-006 evidence: unchanged.

Downstream consequence: Cohbit-CTRL may encode this contract and prove the declared obligations without redefining the language model.

## 13. Claim ceiling

Successful formalization supports:

> Exact profile-indexed continuation-response equality defines a mathematical equivalence relation, while the finite tolerance classifier remains an empirical approximation unless stronger hypotheses are supplied.

It does not establish natural-language semantics, universal contextual equivalence, or governance-level interchangeability.
