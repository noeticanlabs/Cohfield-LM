# CF-LM-IC-00 — CohField Language Profile Implementation Contract Root

Status: **Draft v0.1 — staged before language implementation**

Parent profile: `CF-LM-000`

Parent executable evidence:

- CF-ACP-000: `782fc4efcd6df64a2ed06e87cd5cdac1dc01b4df`
- CF-ACP-INFRA-001: `a48207168b7aaa5488b94cfb85ccbce2cf326275`

CF-ACP-INFRA-002 remains pending local disposition.

## 1. Contract purpose

This contract defines the minimum executable obligations for the first language-domain specialization of the CF-ACP adaptive continuation model.

It does not define natural-language understanding, semantic reasoning, text generation, or a final Cohfield-LM architecture.

The first implementation target is only `CF-LM-001 — Language-Induced Relational Continuation Plasticity`.

## 2. Required inherited surface

A conforming language profile MUST implement the existing `AdaptiveContinuationModel` contract without changing the base trait semantics.

It MUST provide typed equivalents of:

1. complete language-domain State `Z_L`;
2. fast condition `X_L`;
3. persistent local condition `Theta_L`;
4. persistent relational configuration `Psi_L`;
5. surface-derived domain input;
6. exposure-derived experience;
7. finite-horizon language-domain evolution;
8. persistent adaptation;
9. continuation observation profile;
10. continuation response.

The language profile MUST remain downstream of the base runtime.

## 3. Surface-representation contract

### IC-LM-00.1 External representation

The implementation MUST declare its surface representation exactly.

For CF-LM-001 the recommended starting representation is a controlled finite character/symbol alphabet encoded deterministically as Unicode scalar values or bytes.

This recommendation does not make characters or bytes learned model tokens.

### IC-LM-00.2 No learned tokenizer

CF-LM-001 MUST NOT use:

- BPE;
- WordPiece;
- sentencepiece;
- learned vocabulary segmentation;
- learned token IDs;
- pretrained lexical embeddings.

### IC-LM-00.3 No neural encoding

The surface-to-domain-input function MUST NOT depend on:

- neural networks;
- pretrained encoders;
- embeddings;
- attention;
- gradient-trained feature extraction.

The mapping must be explicit and deterministic.

## 4. State-role contract

### IC-LM-00.4 Fast-state role

`X_L` MUST be a true fast role: current exposure may change it and a reset/settling procedure must be able to establish a comparison condition in which histories are compared without simply comparing their residual fast activations.

### IC-LM-00.5 Local-condition role

`Theta_L` MUST be separately inspectable from `X_L` and `Psi_L`.

The first experiment MUST either hold `Theta_L` fixed by construction or equalize it to a preregistered tolerance before the causal `Psi_L` comparison.

### IC-LM-00.6 Relational-configuration role

`Psi_L` MUST be persistent across the declared exposure-to-probe boundary and MUST be independently replaceable for the causal intervention test.

A profile that cannot independently intervene on `Psi_L` cannot claim CF-LM-001 causal relational plasticity.

## 5. Exposure contract

### IC-LM-00.7 Ordered exposure

An exposure history is an ordered sequence of surface observations.

The implementation MUST preserve order. A bag-of-symbol counts representation is insufficient for CF-LM-001 unless order also changes the adaptation dynamics.

### IC-LM-00.8 Paired histories

Histories `H_A` and `H_B` MUST use the same:

- alphabet;
- exposure count or declared matched budget;
- initial State profile;
- model parameters;
- adaptation law;
- numerical integration profile.

Their intended difference MUST be the preregistered structural ordering pattern.

### IC-LM-00.9 Evaluation firewall

No continuation-test outcome or downstream external quality label may enter exposure adaptation.

`evaluation signal != adaptation signal`.

## 6. Observation contract

### IC-LM-00.10 Frozen probe family

The continuation probe family MUST be fixed before observing the comparison result.

The same probe instances MUST be applied to the A and B comparison states.

### IC-LM-00.11 Response carrier

The response carrier MUST be numerical and directly comparable without decoding through a language generator.

CF-LM-001 therefore evaluates continuation behavior before natural-language generation exists.

### IC-LM-00.12 Repeat floor

Identical cloned State/profile pairs MUST be observed repeatedly to establish a deterministic or numerical repeat floor `epsilon_floor`.

## 7. Causal plasticity conformance

### IC-LM-00.13 Pre-intervention difference

After comparison-state equalization:

`d(R_A, R_B) > epsilon_R`.

`epsilon_R` MUST be frozen before the result is observed.

### IC-LM-00.14 Direct relational replacement

Construct an intervention state by replacing A's relational configuration with B's:

`z_A[Psi := Psi_B]`.

All other declared comparison roles MUST remain unchanged.

### IC-LM-00.15 Collapse condition

After replacement:

`d(R_intervened, R_B) <= epsilon_floor`.

If the difference does not collapse, CF-LM-001 does not establish that `Psi_L` caused the measured response difference.

## 8. Required negative controls

CF-LM-001 MUST include at least:

1. identical-history control — same exposure twice should not create a reproducible between-history response difference above the repeat floor;
2. exposure-order control — a matched-count order change is required so simple symbol frequency cannot explain the target;
3. direct `Psi` replacement collapse;
4. fresh probe control — at least one frozen probe pattern not present verbatim in either exposure history;
5. no-adaptation control — with persistent adaptation disabled, the target history effect should collapse or materially shrink.

## 9. Initial language claim boundary

A PASS may support:

> The CF-ACP model can carry ordered surface-language history through persistent relational configuration that causally alters future continuation response.

A PASS MUST NOT be reported as evidence of:

- semantic understanding;
- natural-language competence;
- grammar learning;
- reasoning;
- generative language ability.

## 10. Implementation sequence

The first language executable should be developed in this order:

`LM-L0 surface observation`

`-> LM-L1 fast-state dynamics`

`-> LM-L2 exposure adaptation`

`-> LM-L3 comparison-state equalization`

`-> LM-L4 continuation probes`

`-> LM-L5 Psi intervention`

`-> LM-L6 CF-LM-001 conformance disposition`.

No output-generation subsystem is part of CF-LM-001.

## 11. Freeze condition

CF-LM-IC-00 remains draft until:

1. INFRA-002 receives a recorded disposition;
2. `epsilon_floor` and `epsilon_R` are preregistered;
3. controlled language exposure histories and fresh probe family are frozen;
4. a Rust language profile implements the inherited base trait without modifying CF-ACP core semantics;
5. all positive and negative CF-LM-001 controls execute locally;
6. the resulting PASS/FAIL disposition is recorded without post-result threshold tuning.
