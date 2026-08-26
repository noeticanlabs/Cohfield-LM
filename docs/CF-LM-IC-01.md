# CF-LM-IC-01 — Compositional Continuation Contract

Status: **Draft v0.1 — extension of CF-LM-IC-00**

Parent evidence:

- CF-LM-001 verified local evidence: `f52641e68f34377e40aab7fc1be4293dcf113e93`

Parent contract:

- `CF-LM-IC-00`

This document extends the existing language implementation contract. It does not redefine CF-LM-000, CF-ACP, State, Action, Transition, CohAtom, CohField, CohBit, CohTrace, or semantic equivalence.

## 1. Purpose

CF-LM-IC-01 defines the minimum executable obligations for `CF-LM-002 — Two-Hop Compositional Continuation`.

The target is narrower than semantics:

> determine whether two learned directed relations can compose into a held-out finite continuation consequence when the direct relation is absent.

This is a domain-level continuation/composition test.

It is not a claim of semantic equivalence, grammar induction, reasoning, or natural-language understanding.

## 2. Canonical boundary

The Mathematical Spine permits ordered path composition and domain-declared path equivalence while preserving:

`path identity != endpoint equality != observational equivalence != semantic equivalence`.

CF-LM-002 therefore tests only a finite compositional continuation consequence.

No result from CF-LM-002 may be reported as semantic equivalence.

## 3. Inherited model

CF-LM-002 MUST use the already verified `CohfieldLanguageModelV1` parameterization unchanged:

- `beta = 0.50`
- `input_gain = 0.50`
- `relational_gain = 0.20`
- `psi_decay = 0.02`
- `psi_gain = 0.08`
- `Theta_L = (1,1,1,1)`

No neural component, token predictor, embedding model, softmax, attention, backpropagation, or evaluation-fed adaptation may be introduced.

## 4. Composition target

The learned relational configuration MUST contain a two-edge directed path:

`A -> B -> C`

while the direct learned relation:

`A -> C`

remains absent.

The experiment MUST then probe from `A` under zero external continuation input and test whether activity reaches `C` at the exact depth predicted by two relational transitions.

## 5. Matched structural control

A control exposure MUST preserve the same surface-symbol counts while breaking the `B -> C` bridge.

The control MUST retain the first hop `A -> B` so failure to reach `C` cannot be attributed to loss of the source-to-bridge relation.

## 6. Direct-edge exclusion

The implementation MUST verify before the target measurement that:

`Psi[A][C] <= epsilon_floor`

for both target and control states.

If a direct `A -> C` edge exists above floor, CF-LM-002 cannot establish two-hop composition under this protocol.

## 7. Probe-depth contract

Starting from an equalized comparison state:

1. apply one surface input `A`;
2. apply one zero-input continuation step;
3. apply a second zero-input continuation step.

Let the resulting states be:

`x^(0), x^(1), x^(2)`

where `x^(0)` is after the `A` input.

The two-hop target observable is:

`C_2 = x_C^(2)`.

The first-hop observable is:

`B_1 = x_B^(1)`.

## 8. Causal bridge intervention

The implementation MUST construct a surgical intervention state from the target state by replacing only:

`Psi[B][C] := 0`.

All other state components and relational entries MUST remain unchanged.

The experiment must demonstrate:

- first-hop `B_1` survives the intervention;
- two-hop `C_2` collapses to the repeat floor.

This is the direct causal test that the learned bridge is necessary for the measured two-hop effect.

## 9. Required controls

CF-LM-002 MUST include:

1. matched-count verification;
2. direct `A -> C` absence;
3. intact-chain two-hop target;
4. broken-bridge matched-count control;
5. surgical `B -> C` removal;
6. first-hop preservation after surgical bridge removal;
7. no-adaptation control;
8. deterministic repeat control.

## 10. Claim boundary

A PASS may support only:

> The verified CF-LM adaptive continuation model can compose two learned directed relational steps into a held-out finite continuation consequence at the expected continuation depth, without a directly learned source-to-target relation.

A PASS MUST NOT be reported as evidence of:

- semantic understanding;
- semantic equivalence;
- logical reasoning;
- grammar induction;
- symbolic theorem proving;
- natural-language competence.

## 11. Freeze condition

CF-LM-IC-01 remains draft until:

1. CF-LM-002 protocol values are frozen before implementation;
2. implementation uses unchanged CF-LM-001 model parameters;
3. all required controls execute locally;
4. PASS/FAIL is recorded without post-result threshold tuning.
