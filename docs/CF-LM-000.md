# CF-LM-000 — CohField Language Domain Profile

Status: **Preregistered language-domain profile v0.1 — frozen before implementation; dependency evidence updated after INFRA-002 closure**

Parent evidence:

- CF-ACP-000 base conformance: `782fc4efcd6df64a2ed06e87cd5cdac1dc01b4df`
- CF-ACP-INFRA-001 reconstruction conformance: `a48207168b7aaa5488b94cfb85ccbce2cf326275`
- CF-ACP-INFRA-002 selective-retention closure: `b6f3b6a2164eaae17120e107cc03494bb52133cf`

## 1. Purpose

CF-LM-000 instantiates the existing CF-ACP adaptive continuation model over a language domain. It does not define a second language architecture and does not redefine State, Action, Transition, Atomic Transition, CohAtom, CohField, CohBit, or CohTrace.

The language model inherits the domain-neutral CF-ACP structure:

`L_F = <Z_L, U_L, E_L, Pi_L, Phi_L, A_L, O_L, R_L>`

with the same semantic roles:

- `X_L = pi_X(z)` — fast language-domain condition;
- `Theta_L = pi_Theta(z)` — persistent local condition;
- `Psi_L = pi_Psi(z)` — persistent relational configuration;
- `Phi_L` — finite-horizon language-domain evolution;
- `A_L` — experience-induced persistent adaptation;
- `O_L` — declared language continuation observation profile;
- `R_L` — finite continuation-response map.

The purpose of the first language program is to determine whether language exposure can inhabit this existing model directly.

## 2. Architectural boundary

CF-LM-000 MUST NOT make any of the following the computational foundation of the model:

- learned tokenization;
- token IDs as the authoritative internal state;
- next-token probability;
- softmax decoding;
- neural networks;
- backpropagation;
- gradient descent;
- embedding vectors as the authoritative semantic substrate;
- Transformer attention;
- an independently authoritative semantic graph.

External text is a boundary representation. An implementation may read UTF-8 bytes, Unicode scalar values, characters, or another explicitly declared surface representation, but such units are observations presented to the model, not the learned predictive ontology of the model.

`surface representation != model substrate`.

## 3. Language-domain State

A language-domain State is

`z_L in Z_L subseteq S_F`.

It MUST contain every variable required to determine future language-domain evolution under the active profile.

The state-role interpretation is inherited rather than redefined.

### 3.1 Fast condition `X_L`

`X_L` records rapidly changing language-domain activity induced by current surface observations and internal continuation dynamics.

It is not a token sequence or context-window definition.

### 3.2 Persistent local condition `Theta_L`

`Theta_L` records persistent condition associated with individual active language-domain channels/components.

CF-LM-000 does not yet canonically identify those components with words, morphemes, concepts, grammar rules, or any other conventional NLP unit.

### 3.3 Persistent relational configuration `Psi_L`

`Psi_L` records persistent organization among language-domain channels/components.

The first language hypothesis is that structured exposure can produce persistent `Psi_L` differences that alter later continuation response even after fast state and local condition are controlled.

`Psi_L` is not automatically canonical `FieldConfiguration`.

## 4. Surface input boundary

Let

`U_surface`

be the external language-observation carrier.

A surface observation enters the active language profile as domain input

`u_L in U_L`.

The surface-to-input boundary MUST be deterministic under a declared representation profile for CF-LM-001. It MUST NOT call a neural encoder or learned tokenizer.

CF-LM-001 should begin with a deliberately small controlled symbol/character language so that causal structure is measurable before any natural-language capability claim is attempted.

## 5. Language exposure

A language exposure is a bounded ordered observation history

`h = (u_1, u_2, ..., u_T)`.

Exposure drives fast evolution through `Phi_L` and may generate experience supplied to `A_L`.

The adaptation path MUST preserve the experimental firewall:

`external evaluation notin Inputs(A_L)`.

No correct-answer label, utility score, holdout score, or downstream language-evaluation result may be fed into adaptation in CF-LM-001.

## 6. Language continuation response

For a frozen observation profile `O_L`, define

`R_L,O : Z_L -> Y_L,O`.

`R_L,O(z)` is the measured finite continuation behavior of the current language-domain State under the declared probe family.

CF-LM-001 does not require `Y_L,O` to be text or a probability distribution. The first experiment should measure the model's internal continuation response directly.

Thus:

`language continuation response != next-token distribution`.

## 7. CF-LM-001 — Language-Induced Relational Continuation Plasticity

### 7.1 Primary question

Does structured language exposure causally alter persistent relational configuration such that future continuation response changes after fast-state and local-condition equality are restored?

### 7.2 Experimental structure

Construct at least two controlled exposure histories:

`H_A -> z_A`

`H_B -> z_B`

with different ordered surface structure but identical model parameters and adaptation law.

After exposure, establish a comparison state satisfying, to declared tolerance,

`X_A ~= X_B`

and

`Theta_A ~= Theta_B`.

Permit

`Psi_A != Psi_B`.

Apply the exact same frozen continuation probe family to both states.

Measure

`D_R = d(R_L(z_A), R_L(z_B))`.

### 7.3 Causal intervention

Perform direct relational replacement:

`Psi_A -> Psi_B`

while preserving the comparison values of `X` and `Theta`.

Recompute response.

The causal plasticity target is:

`D_R(before) > epsilon_R`

and

`D_R(after Psi replacement) <= epsilon_floor`.

The tolerances MUST be preregistered before local execution.

### 7.4 Null result

If structured exposure produces no reproducible `Psi` differentiation, or if replacing `Psi` does not collapse the response difference, CF-LM-001 FAILS.

The profile MUST NOT be repaired after seeing language-evaluation outcomes without a versioned amendment.

## 8. What CF-LM-001 can establish

A PASS would support only:

> Structured surface-language exposure can inhabit the CF-ACP adaptive continuation model and causally alter future language-domain continuation through persistent relational configuration.

It would NOT yet establish:

- natural-language understanding;
- semantic meaning;
- grammar induction;
- reasoning;
- generation;
- conversation;
- open-domain language competence;
- superiority to neural language models.

Those require later experiments.

## 9. Governance boundary

CF-LM-000 remains domain mathematics for possible continuation.

A continuation response, learned configuration, geometric distinction, or endogenous score is not automatically verified, admissible, authorized, executed, committed, receipted, or a CohTrace member.

Any later language candidate that may become authoritative must still enter the canonical CohBit lifecycle with exact Source, Action, Target, Boundary, and Semantics.

## 10. Dependency gate

CF-LM-000 is now grounded on verified CF-ACP-000, INFRA-001, and INFRA-002 evidence.

Language implementation MUST NOT be classified as conformance-verified until:

1. CF-LM-IC-00 is closed for CF-LM-001;
2. CF-LM-001 local tests pass without post-result tolerance tuning;
3. the CF-LM-001 PASS/FAIL disposition is recorded separately from upstream infrastructure evidence.

This preserves:

`base evidence != infrastructure result record != language evidence`.
