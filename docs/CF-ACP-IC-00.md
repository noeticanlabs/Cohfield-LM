# CF-ACP-IC-00 — Adaptive Continuation Implementation Contract Root

Status: **Draft v0.1**

## 1. Contract purpose

This contract defines the minimum executable obligations for a domain-neutral implementation of CF-ACP-000. It does not certify a CohBit Primitive implementation, a governed runtime, or a language model.

## 2. Required executable surface

A conforming base runtime MUST provide typed equivalents of:

1. complete domain `State`;
2. fast-state projection `X`;
3. persistent local-condition projection `Theta`;
4. persistent relational-configuration projection `Psi`;
5. domain input;
6. experience input;
7. finite-horizon evolution;
8. experience adaptation;
9. observation profile;
10. continuation response.

The Rust reference trait is `AdaptiveContinuationModel`.

## 3. Core operation contracts

### IC-00.1 Role projection

Input: `State`.

Output: typed `StateRoles<X, Theta, Psi>`.

Requirement: all three roles MUST be derived from the same complete State instance. Implementations MUST NOT imply that `Psi` is outside State when future evolution depends on it.

### IC-00.2 Evolve

Input: `State`, `Input`, finite horizon.

Output: new `State` or explicit error.

Requirements:

- no silent mutation of the source State through a supposedly pure rollout interface;
- invalid horizons MUST fail explicitly;
- domain evolution law MUST be supplied by the domain profile, not hard-coded into the base;
- independent probe/counterfactual rollouts MUST be able to begin from cloned/equivalent reference States.

### IC-00.3 Adapt

Input: `State`, `Experience`.

Output: new `State` or explicit error.

Requirements:

- adaptation MUST be represented as a State change when it changes future continuation;
- adaptation signals and external evaluation signals MUST be separately typed/declared when an experiment requires that firewall;
- no universal adaptation law is imposed by this contract.

### IC-00.4 Observe

Input: `State`, `ObservationProfile`.

Output: continuation `Response` or explicit error.

Requirements:

- the observation profile MUST carry enough information to interpret the response;
- observation MUST NOT be named or treated as verification, admissibility, authority, execution, or commitment;
- repeated deterministic observation of identical State/profile pairs SHOULD be able to establish a numerical repeat floor.

## 4. Optional geometry contract

A profile claiming differential geometry MUST expose a response Jacobian with declared row/column semantics.

For Jacobian `J` and response weighting `W`, the base utility computes

`G = J^T W J`.

The implementation MUST reject inconsistent dimensions rather than inventing implicit padding/truncation.

Claims of positive definiteness require a rank/eigenvalue argument appropriate to the domain. A nonconstant metric or nonzero Christoffel symbols MUST NOT be reported as proof of intrinsic curvature.

## 5. Optional counterfactual contract

A profile claiming counterfactual evaluation MUST provide:

- a perturbation type;
- a rollout from a reference State;
- a trajectory type;
- a declared recovery observable.

Counterfactual ensemble comparisons MUST use paired perturbations across compared configurations unless a different sampling design is explicitly justified.

### IC-00.CF1 Recovery boundary

The helper

`m = 1 - r/r_max`

MUST fail closed for non-finite inputs or `r_max <= 0`.

### IC-00.CF2 Threshold projection

`C = 1[m >= 0]`.

The implementation and documentation MUST preserve that thresholding is lossy with respect to signed margin.

### IC-00.CF3 Mean recovery margin

`Q_rm = mean(m_j)`.

An empty or non-finite sample MUST fail explicitly.

`Q_rm` is an optional experimental profile pending v0.11 robustness/domain-of-validity results.

## 6. Domain-neutrality requirements

The core crate MUST NOT depend on or contain assumptions specific to:

- electrical conductance;
- graph Laplacians;
- a fixed node/edge count;
- co-flow outer-product adaptation;
- natural-language tokens;
- token probabilities;
- neural networks;
- embeddings;
- Transformer attention;
- a universal reward/value function.

Those belong only in downstream domain profiles if used at all.

## 7. CohBit boundary

This runtime supplies domain mathematics for possible continuation. It MUST NOT skip the canonical architectural spine.

A response, score, metric, or selected candidate is not automatically a CohAtom, verified result, admissible transition, authorized transition, execution, commitment, receipt, or CohTrace member.

A downstream CohBit integration MUST still provide exact Source, Action, Target, Boundary, and Semantics for any candidate transition and pass the canonical governance lifecycle before authoritative realization.

## 8. Required base tests

Before IC-00 can close, CI MUST demonstrate at minimum:

1. signed margin preserves distinctions lost by binary thresholding;
2. invalid recovery boundaries fail closed;
3. empty/non-finite recovery-margin samples fail closed;
4. identity-weight pullback matches `J^T J` on a known matrix;
5. inconsistent pullback dimensions fail closed;
6. a toy domain can implement `AdaptiveContinuationModel` without infrastructure-specific types;
7. cloned identical State/profile observations are deterministic under a deterministic toy domain;
8. changing a persistent role in a toy domain can causally change continuation while fast-state projection is held fixed.

## 9. Closure gate

`CF-ACP-IC-00` may be marked closed only when the specification, Rust interface, tests, and recorded behavior agree. Contract closure alone does not freeze CF-ACP-000 or establish a language model.
