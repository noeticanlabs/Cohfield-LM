# CF-ACP-INFRA-001 — Infrastructure Reproduction Profile

Status: **Draft reconstruction / local conformance pending**

## 1. Purpose

CF-ACP-INFRA-001 is the first downstream domain profile implemented against the domain-neutral `AdaptiveContinuationModel` contract.

Its purpose is to determine whether the extracted CF-ACP-000 runtime can reproduce the causal, differential, geometric, trajectory, traversal, and relational-plasticity phenomena reported by Infrastructure-Generated CohField Geometry v0.01-v0.06 without placing infrastructure assumptions in the core runtime.

This profile is downstream domain mathematics. It does not redefine CohField, State, CohAtom, CohBit, or CohTrace.

## 2. Reconstruction boundary

The repository currently contains the reported post-history infrastructure states and result values from the experiment record, but not the complete raw parameter ledger used to generate every original history.

Therefore INFRA-001 deliberately separates two claims.

### Supported reconstruction target

The profile reconstructs the reported three-edge dynamical specialization from the controlled post-history states:

- edge order `(e01, e12, e02)`;
- fast State `X in R^3`;
- persistent local condition `Theta in R^3`;
- persistent relational configuration `Psi in R^(3x3)`;
- effective edge response `H(Theta, Psi) = diag(Theta) + alpha_Psi Psi`;
- node dynamics `dX/dt = u - C H C^T X - lambda X`;
- finite-horizon continuation response measured as `(X(T)-X(0))/T` over a balanced probe family.

The v0.01 reconstruction uses the reported endpoint conditions

`Theta_A = (2.7727, 1.6468, 1.6468)`

and

`Theta_B = (1.6468, 1.6468, 2.7727)`.

The reconstruction profile uses `lambda = 0.5` and balanced zero-sum probes of magnitude `0.6`. These values reproduce the reported v0.01 finite-horizon deformation curve to close numerical agreement and reproduce the reported relative metric-state dependence to the stated experimental precision.

### Not yet claimed

INFRA-001 does not yet claim exact reproduction of the original history-generation process that produced `Theta_A`, `Theta_B`, or the v0.06 learned `Psi` matrices. The full original history/adaptation parameter ledger must be imported or independently reconstructed before that stronger claim can be made.

The reported post-history states are used as controlled initial conditions for causal reproduction tests.

## 3. Domain State

The complete profile State is

`z = (X, Theta, Psi)`.

The role projections are:

- `X` — fast node condition;
- `Theta` — persistent condition of the three individual channels;
- `Psi` — persistent relational organization among channels.

All three remain components of complete State because each may affect future continuation.

## 4. Profile dynamics

For oriented edge differences

`d = C^T X`,

define

`H(Theta, Psi) = diag(Theta) + alpha_Psi Psi`.

Edge flow is

`J = H d`,

and node evolution is

`dX/dt = u - C J - lambda X`.

The reference implementation uses deterministic fourth-order Runge-Kutta integration for finite-horizon rollouts. `Theta` and `Psi` remain fixed during one fast probe rollout; persistent adaptation is represented separately through the profile's `adapt` operation.

This preserves the fast/persistent separation used in the experiments.

## 5. INFRA-001 conformance targets

### T1 — v0.01 finite-horizon causal deformation

At equal fast State and different `Theta`, the reported horizon sweep should be reproduced in shape and close scale:

| Horizon | Reported D_dyn |
| ---: | ---: |
| 0.02 | 0.01478 |
| 0.05 | 0.03258 |
| 0.10 | 0.05311 |
| 0.25 | 0.0748336 |
| 0.50 | 0.06577 |

Acceptance: every reconstructed value lies within `0.0025` absolute distance of the reported value, the response grows through `T=0.25`, and the `T=0.50` response falls below the `T=0.25` peak.

### T2 — v0.01 direct intervention

Replacing only `Theta_A` by `Theta_B` at controlled fast State must collapse the response difference to deterministic numerical floor.

### T3 — v0.03 pullback metric

At the tested midpoint, the finite-difference response Jacobian must induce a positive-definite `G = J^T J` and predict a nearby finite response displacement with less than one-percent relative error for the frozen local perturbation used by the test.

### T4 — v0.03 metric-state dependence

Moving along the reported v0.01 infrastructure contrast must reproduce the reported relative Frobenius changes approximately:

- quarter displacement: about `11.4%`;
- half displacement: about `23.4%`;
- three-quarter displacement: about `36.4%`.

### T5 — v0.04 finite path distinction

Two different infrastructure-coordinate paths with identical endpoints must produce different induced metric lengths.

This tests finite trajectory geometry only. It is not a curvature claim and does not independently prove geodesic optimality.

### T6 — v0.06 conditional geometric plasticity

At identical `Theta` and different reported relational configurations `Psi_A` and `Psi_B`:

- continuation responses must differ;
- response Jacobians must differ;
- induced metrics must differ;
- direct replacement `Psi_A -> Psi_B` must collapse the response and Jacobian differences.

### T7 — deterministic cloned-state observation

Repeated observation from cloned identical States and identical profiles must produce identical response records and must not mutate the source State.

## 6. Architectural firewall

INFRA-001 is a domain profile only.

The following remain outside the CF-ACP-000 core:

- three-node topology;
- incidence orientation;
- conductance-like `Theta`;
- relational edge matrix `Psi`;
- RK4 integration choice;
- balanced probe amplitudes;
- infrastructure-specific response interpretation.

Likewise, a reproduced response or metric does not establish verification, admissibility, authority, execution, commitment, or CohTrace membership.

## 7. Closure condition

INFRA-001 may be recorded as first infrastructure conformance evidence only after the stacked branch passes locally:

```text
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

and all `tests/infrastructure_profile.rs` tests pass without weakening their preregistered tolerances after observing failures.

If a reconstruction target fails, the failure must be recorded before any parameter amendment. A parameter change intended to improve agreement becomes a new version of the reconstruction profile rather than a silent rewrite.
