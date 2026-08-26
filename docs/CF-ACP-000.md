# CF-ACP-000 — CohField Adaptive Continuation Profile

Status: **Pre-canonical base extraction v0.1**

## 1. Purpose

CF-ACP-000 extracts the domain-neutral computational organism exposed by Infrastructure-Generated CohField Geometry v0.01-v0.10. It is a domain-mathematical profile hosted by CohField. It does not redefine State, Action, Transition, Atomic Transition, CohAtom, CohField, CohBit, or CohTrace.

The infrastructure realization was the experimental laboratory. Conductance, graph Laplacians, electrical flow equations, co-flow matrices, recovery thresholds, and specific probe families are not universal laws of CF-ACP-000.

## 2. Minimum model

A CF-ACP realization is described by

`A_F = <Z, U, E, Pi, Phi, A, O, R>`

with the following mathematical obligations.

### 2.1 State carrier

`Z subseteq S_F`.

A state `z in Z` must contain every domain variable required to determine future evolution under the declared model profile.

### 2.2 State-role projections

The profile declares typed projections

- `pi_X : Z -> X`
- `pi_Theta : Z -> T_Theta`
- `pi_Psi : Z -> C_Psi`

with semantic roles:

- `X` — fast condition;
- `Theta` — persistent local condition;
- `Psi` — persistent relational configuration.

The roles are semantically distinct even when stored inside one State representation. `Psi` is not automatically canonical `FieldConfiguration`.

### 2.3 Domain evolution

For a finite horizon `tau >= 0`,

`Phi_tau : Z x U -> Z`.

`Phi` supplies the domain transition mathematics. CF-ACP-000 does not prescribe ODEs, graph dynamics, neural updates, token prediction, or any other specific evolution law.

### 2.4 Experience adaptation

`A : Z x E -> Z`.

Experience may alter persistent components of State and thereby alter future continuation. The infrastructure update `Psi+ = (1-rho)Psi + eta qq^T` is one experimentally successful specialization and is not universalized by this profile.

### 2.5 Observation profile

An observation profile `O` declares the probe/input family, finite horizon, measured response carrier, weighting where applicable, and environmental conditions needed to interpret the response.

### 2.6 Continuation-response map

`R_O : Z -> Y_O`.

`R_O(z)` is the observable finite continuation behavior of State `z` under observation profile `O`.

The core experimentally supported principle is:

**Fast-state equality does not imply continuation equivalence.**

States may satisfy `pi_X(z_A) approximately pi_X(z_B)` while `R(z_A) != R(z_B)` because persistent local or relational structure differs.

## 3. Causal conformance pattern

A claimed persistent component must be tested by intervention rather than correlation alone. If a profile claims that `Psi` changes continuation, it should support a controlled test of the form

`Psi_A -> Psi_B  ==>  R_A -> R_B`

with other relevant State roles and observation conditions controlled.

This is a conformance-test pattern, not a theorem that every declared `Psi` must influence every response.

## 4. Optional differential geometry

Where `R` is differentiable with respect to a selected coordinate carrier `chi`, define

`J_chi^R = D_chi R`.

Given a positive-semidefinite response weighting `W`, define the pullback form

`G_chi = J^T W J`.

If `J` has full column rank, `G_chi` is positive definite. Otherwise it is positive semidefinite and exposes locally response-invisible directions.

`G` is an induced structure on a declared carrier. It is not CohField itself.

Finite metric length and energy may be defined by

`L_G[gamma] = integral sqrt(gamma_dot^T G gamma_dot) dt`

and

`E_G[gamma] = 1/2 integral gamma_dot^T G gamma_dot dt`.

Metric energy is not physical energy, accounting burden, valuation, admissibility, or authority.

## 5. Traversal and plasticity

Traversal changes local-condition coordinates while relational configuration is held fixed.

Conditional geometric plasticity is demonstrated when, at fixed local condition,

`G_{chi|Psi_A} != G_{chi|Psi_B}`

and controlled replacement of `Psi_A` by `Psi_B` collapses the corresponding response/differential/metric differences.

## 6. Selective functional adaptation

An external evaluation functional may be used experimentally to determine whether experience-induced configuration changes future capability, provided the adaptation/evaluation firewall is preserved:

`U_eval notin Inputs(A)`.

External evaluation is not a CohBit governance decision and is not part of the base adaptation law.

## 7. Optional counterfactual extension

A counterfactual profile declares a perturbation family `P = {p_1, ..., p_M}` and produces independent hypothetical trajectories from cloned reference States.

A domain recovery observable

`r : Gamma -> R_{≥0}`

may be compared with a declared boundary `r_max > 0` using the signed recovery margin

`m_j = 1 - r_j/r_max`.

Binary survival is the threshold projection

`C_j = 1[m_j >= 0]`.

Therefore `m_j -> C_j` is information-losing: the threshold preserves boundary side but discards distance from the boundary.

The v0.10 mean recovery-margin profile is

`Q_rm = (1/M) sum_j m_j`.

Status of `Q_rm`: **experimentally supported optional profile; robustness/domain-of-validity campaign pending v0.11**.

## 8. Governance firewall

CF-ACP-000 structures and measures possible domain continuation. It does not establish later CohBit lifecycle stages.

In particular:

- continuation response != verification;
- geometric distinguishability != truth;
- endogenous score != valuation;
- endogenous score != admissibility;
- adaptive preference != authority;
- simulated continuation != execution;
- realized domain path != CohTrace unless the canonical governance and commitment lifecycle has actually occurred.

## 9. Freeze condition

CF-ACP-000 remains pre-canonical until:

1. CF-ACP-IC-00 is closed;
2. a domain-neutral reference runtime exists;
3. the infrastructure specialization reproduces the established experimental phenomena without infrastructure logic in the core;
4. conformance evidence is recorded;
5. unresolved v0.11 results are incorporated without retroactive tuning.
