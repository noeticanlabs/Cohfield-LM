# CF-LM-FM-001 — Lean Proof Obligations

Status: **Frozen proof-obligation map v0.1**

Parent contract: `docs/CF-LM-FM-001.md`.

## A. Universal exact-equivalence layer

### A1 — Profile structure

Formalize a generic profile carrying:

```text
State
Context
Response
respond : State -> Context -> Response
```

No metric is required at this layer.

### A2 — Exact relation

Define:

```text
contextualEquivalent K x y :=
  ∀ k, K.respond x k = K.respond y k
```

### A3 — Reflexivity

Prove for all States:

```text
contextualEquivalent K x x
```

### A4 — Symmetry

Prove:

```text
contextualEquivalent K x y ->
contextualEquivalent K y x
```

### A5 — Transitivity

Prove:

```text
contextualEquivalent K x y ->
contextualEquivalent K y z ->
contextualEquivalent K x z
```

### A6 — Setoid packaging

Construct a Lean `Setoid State` from the exact relation.

This packaging is mathematical convenience only; it does not authorize CohBit identity substitution.

## B. Approximate empirical layer

### B1 — Metric profile

Extend or separately define:

```text
distance : Response -> Response -> ℝ
```

with explicit metric/pseudometric assumptions as needed.

### B2 — Approximate classifier

Define:

```text
approxContextual K ε x y :=
  ∀ k, distance (respond x k) (respond y k) ≤ ε
```

### B3 — Reflexivity

Prove under `0 ≤ ε` and `d(r,r)=0`.

### B4 — Symmetry

Prove under symmetric distance.

### B5 — No unconditional transitivity theorem

Do **not** expose:

```text
approxContextual_trans
```

without stronger premises.

A positive threshold classifier is not generally transitive.

If a later theorem is desired, it must state an appropriate bound, for example composition of tolerances via triangle inequality:

```text
approx ε₁ x y -> approx ε₂ y z -> approx (ε₁ + ε₂) x z
```

under metric assumptions.

This is not the same theorem as transitivity at fixed `ε`.

## C. Composition/congruence layer

### C1 — Partial operation

Represent a declared composition operation explicitly, for example:

```text
compose : Host -> State -> Option State
```

or an equivalent typed partial operation.

### C2 — Definedness preservation is separate

Do not infer that if `compose h x` is defined then `compose h y` is defined from `x ~ y` unless supplied as a premise or proved from a concrete profile.

### C3 — Congruence theorem schema

The generic theorem may take a response-preservation hypothesis such as:

```text
∀ h x y,
  contextualEquivalent K x y ->
  compose h x = some x' ->
  compose h y = some y' ->
  contextualEquivalent K x' y'
```

or a stronger structural law from which it follows.

Do not build congruence into the equivalence definition.

## D. Concrete CF-LM finite-carrier correspondence

A later concrete module may encode the CF-LM-006 carrier:

```text
R_C
R_D
R_L
```

and show exact response equality under an abstracted exact model where possible.

Numerical decimal values from Rust are evidence references, not Lean axioms.

If exact arithmetic encoding of the concrete recurrence is introduced, assumptions and numeric representations must be explicit.

## E. Identity firewall obligations

The Lean documentation/theorem naming must preserve:

```text
contextualEquivalent x y
```

as distinct from Lean equality:

```text
x = y
```

No theorem named or implemented as an identity coercion may be added.

In particular, do not provide a theorem of shape:

```text
contextualEquivalent x y -> x = y
```

unless a separately declared injectivity hypothesis on `respond` is supplied.

## F. Evidence boundary

The formal development must carry a short module-level statement distinguishing:

- definitional/universal proofs;
- model-specific proofs;
- finite Rust evidence;
- unproved semantic claims.

## G. Build gate

The first formalization gate is:

```text
lake build
```

plus any repository-standard Lean test/build command already required by Cohbit-CTRL.

Required quality rule:

```text
no sorry
no hidden axioms
no theorem beyond explicit dependencies
```

## H. Version consequence

Any need to weaken the exact relation, promote the tolerance classifier to an equivalence relation, or broaden congruence beyond declared operations requires a versioned amendment to `CF-LM-FM-001` before proof implementation is changed.
