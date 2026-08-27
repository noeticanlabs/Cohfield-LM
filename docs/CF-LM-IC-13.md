# CF-LM-IC-13 — Consequence-Grounded Abstraction Applicability Contract

Status: preregistered experimental implementation contract
Parent evidence: CF-LM-013 verified at `c6472bf8b15408cc0adadc4f781422996127f582`
Target: CF-LM-014

## 1. Purpose

CF-LM-013 established supervised context-to-abstraction applicability learning: a profile identity was supplied during applicability acquisition, and later held-out contexts were classified from the learned applicability structure.

CF-LM-014 removes that profile label from acquisition. The language-domain organism receives a recognized context and an observed continuation consequence. It must compare that observed consequence against counterfactual predictions generated under each already-assessed abstraction profile, infer which profile best explains what actually happened, and bind the inferred profile to the context for later held-out applicability inference.

This contract does not define semantic truth, utility, reward, valuation, admissibility, policy, authority, execution, commitment, or CohTrace substitution.

## 2. Canonical firewalls

The following distinctions are REQUIRED:

- observed domain consequence != canonical CohBit Evidence;
- observed domain consequence != Verification;
- prediction error != valuation;
- prediction error != admissibility;
- inferred applicability != semantic truth;
- inferred applicability != identity;
- context recognition != consequence matching;
- consequence matching != later applicability inference;
- applicability inference != authority.

The observed consequence is an exogenous domain observation supplied to the adaptive language model. The model may use it as experience. It MUST NOT represent that observation as a verified claim or governance decision.

## 3. Versioning boundary

CF-LM-014 MUST be implemented as an additive language-domain version after V6. V1-V6 and CF-ACP semantics MUST remain unchanged.

The new version MAY preserve prior V6 State, including supervised applicability history, but CF-LM-014 consequence-grounded inference MUST use its own provenance-preserving outcome-applicability history rather than silently rewriting supervised records.

## 4. Fixed candidate abstraction set

The experiment reuses the already-assessed profiles:

- `P_AB = <[A,B], continuation_steps=4, epsilon=1e-12>`; C/D equivalent.
- `P_BC = <[B,C], continuation_steps=4, epsilon=1e-12>`; no nontrivial pair equivalent.

CF-LM-014 does not discover new abstraction profiles. It discovers which existing assessed abstraction best explains an observed consequence.

## 5. Frozen sequential substrate

The same combined route substrate and eight isolated `C->A` teaching episodes are reused. Teaching occurs while no profile is selected.

Frozen sequential expectations:

- `sequential[C][A] = 0.5579844028434426 +/- 1e-9`;
- `sequential[D][A] <= 1e-12`.

## 6. Consequence observation type

One consequence observation is the five-point A-coordinate trajectory produced by a fresh D probe:

`Y = [A_0, A_1, A_2, A_3, A_4]`.

Frozen reference observations:

`Y_TRANSFER = [
    0.0,
    0.0,
    0.011159688056868854,
    0.01673953208530328,
    0.017363331386570834,
]`

`Y_ZERO = [0.0, 0.0, 0.0, 0.0, 0.0]`.

Their Euclidean separation is preregistered as approximately:

`0.026575098283946105`.

## 7. Counterfactual prediction rule

For every distinct already-assessed profile `P`, the organism MUST:

1. clone the current language State;
2. set the clone's selected abstraction profile to `P` only inside the counterfactual witness;
3. equalize fast/local State exactly as in prior transfer probes;
4. drive `D` once;
5. continue four zero-input steps;
6. record the predicted five-point A-coordinate trajectory `Pred(P)`;
7. compute `error(P) = ||Pred(P) - Y_observed||_2`.

This calculation MUST NOT mutate the actual selected profile, sequential substrate, assessment history, or context history.

## 8. Outcome-match decision rule

Frozen thresholds:

- maximum winning prediction error: `epsilon_outcome = 0.020`;
- minimum error margin: `delta_outcome = 0.010`, required strictly.

A consequence-grounded applicability episode is accepted only when:

- the minimum candidate error is <= `epsilon_outcome`;
- exactly one candidate has the minimum error;
- `runner_up_error - winner_error > delta_outcome`.

Otherwise acquisition fails closed.

## 9. Frozen consequence-grounded acquisition set

The context mapping intentionally remains opposite to CF-LM-012's projection-overlap heuristic.

No profile identity is supplied in these acquisition experiences.

- `T_C1=[C,C,C,D]` with observed `Y_TRANSFER`;
- `T_C2=[C,C,D,D]` with observed `Y_TRANSFER`;
- `T_A1=[A,A,A,D]` with observed `Y_ZERO`;
- `T_A2=[A,A,D,D]` with observed `Y_ZERO`.

The organism MUST infer:

- `T_C1,T_C2 -> P_AB` from consequence prediction match;
- `T_A1,T_A2 -> P_BC` from consequence prediction match.

The acquisition API MUST NOT contain a profile parameter.

## 10. History-derived applicability structure

Each accepted outcome-applicability record MUST bind:

- its own epoch;
- context epoch;
- recognized context activity;
- observed consequence trajectory;
- candidate profile prediction errors;
- inferred winning profile.

Profile applicability prototypes MUST be derived from this append-only history and MUST NOT become an independently mutable second source of truth.

Expected derived prototypes remain:

- `mu_AB=[0,0,0.625,0.375]`;
- `mu_BC=[0.625,0,0,0.375]`.

## 11. Held-out applicability inference

Held-out inference receives only recognized context and uses prototypes derived exclusively from consequence-grounded applicability history.

Frozen context-distance rule and thresholds remain:

- Euclidean context distance;
- maximum applicability distance `0.50`;
- winning margin strictly greater than `0.25`.

Frozen held-out contexts:

- `K_C=[B,C,C,D]` -> `P_AB`;
- `K_A=[A,A,B,D]` -> `P_BC`.

These remain inversion controls against CF-LM-012's old projection heuristic.

## 12. Causal transfer requirement

After consequence-grounded applicability acquisition:

- `K_C -> inferred P_AB -> D probe A_2 = 0.011159688056868854 +/- 1e-9`;
- `K_A -> inferred P_BC -> |D probe A_2| <= 1e-12`;
- returning to `K_C` restores the exact original transfer trajectory without reassessment, outcome-applicability retraining, or sequential relearning.

## 13. Negative controls

The implementation MUST include at least:

1. no outcome-applicability history -> held-out inference fails closed;
2. outcome observation midway between `Y_TRANSFER` and `Y_ZERO` -> ambiguous outcome match;
3. unsupported far observation -> unsupported outcome match;
4. outcome acquisition does not select a runtime profile;
5. outcome acquisition does not mutate sequential relations or assessment history;
6. candidate predictions are generated without mutating actual State;
7. consequence-grounded history preserves context provenance;
8. deterministic replay.

## 14. Claim ceiling

A PASS may support only the bounded claim:

> On the frozen finite language-domain carrier and candidate profile set, the organism can infer context-to-abstraction applicability from observed continuation consequences without receiving an abstraction-profile label during acquisition, generalize the resulting applicability structure to held-out contexts, and make later continuation causally follow the inferred abstraction.

A PASS does not establish semantic understanding, autonomous abstraction invention, general reinforcement learning, universal model selection, general intelligence, or governance authority.
