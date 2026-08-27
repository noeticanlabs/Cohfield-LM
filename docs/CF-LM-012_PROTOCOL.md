# CF-LM-012 — Endogenous Context-Conditioned Abstraction Selection

Status: Preregistered experiment

Parent contract: `CF-LM-IC-11`

Parent evidence: CF-LM-011 verified at `955b33257707c7f448b56ecdf71767b28bdcf927`.

## 1. Research question

Can the organism infer which already-assessed internal consequence-equivalence profile applies from current surface context, without the caller naming that profile, and make future continuation use the inferred abstraction while ambiguous or unsupported contexts fail closed?

## 2. Frozen model boundary

- V1-V4 remain unchanged.
- CF-ACP remains unchanged.
- A versioned V5 language profile is permitted.
- Existing V4 assessment history remains the source of truth for each profile's equivalence relation.
- Context recognition and profile inference are distinct State operations.

## 3. Frozen source State

Reconstruct the verified route substrate exactly as in CF-LM-009 through CF-LM-011:

```text
A->C = 0.9840816505055259
C->B = 1.0041649494954346
A->D = 0.9840816505055259
D->B = 1.0041649494954346
```

Migrate through V4 into V5 without altering X, Theta, sequential relations, selected profile, or assessment history.

## 4. Frozen assessed profiles

Assess both profiles before context inference:

```text
P_AB = <projection=[A,B], h=4, epsilon=1e-12>
P_BC = <projection=[B,C], h=4, epsilon=1e-12>
```

Expected dispositions:

```text
P_AB -> only C/D equivalent
P_BC -> no nontrivial equivalent pair
```

No profile is selected by assessment.

## 5. Frozen context-recognition rule

For cue `K = [s_1,...,s_n]`, define normalized surface activity:

\[
c_i(K)=\frac{1}{n}\sum_{j=1}^n 1[s_j=i].
\]

Recognition appends one immutable context record and sets the current recognized-context reference. It does not select a profile.

## 6. Frozen compatibility rule

For each distinct assessed profile `P`:

\[
S(P\mid K)=\sum_{s\in projection(P)} c_s(K).
\]

Inference evaluates every assessed profile using this same rule.

Frozen thresholds:

```text
MIN_CONTEXT_SCORE  = 0.50
MIN_CONTEXT_MARGIN = 0.25
```

Selection succeeds only if the top score is at least `MIN_CONTEXT_SCORE` and exceeds every other assessed profile by more than `MIN_CONTEXT_MARGIN`.

The inference request carries no profile identity.

## 7. Frozen positive contexts

### K_AB

```text
[A, A, B, D]
```

Expected activity:

```text
[A,B,C,D] = [0.50,0.25,0.00,0.25]
```

Expected scores:

```text
P_AB = 0.75
P_BC = 0.25
```

Expected inferred profile: `P_AB`.

### K_BC

```text
[B, C, C, D]
```

Expected activity:

```text
[A,B,C,D] = [0.00,0.25,0.50,0.25]
```

Expected scores:

```text
P_AB = 0.25
P_BC = 0.75
```

Expected inferred profile: `P_BC`.

## 8. Frozen fail-closed contexts

### K_tie

```text
[B, D]
```

Expected scores:

```text
P_AB = 0.50
P_BC = 0.50
```

Expected result: ambiguous-context failure; State unchanged by inference.

### K_none

```text
[D, D]
```

Expected scores:

```text
P_AB = 0.00
P_BC = 0.00
```

Expected result: unsupported-context failure; State unchanged by inference.

## 9. Frozen teaching and probe

From the assessed-but-unselected V5 State, teach exactly eight isolated `[C,A]` episodes.

Expected sequential regression:

```text
sequential[C][A] = 0.5579844028434426 +/- 1e-9
sequential[D][A] <= 1e-12
```

Then run context recognition + generic inference, followed by a fresh D probe with four continuation steps.

Expected results:

```text
K_AB -> inferred P_AB -> A_step2(D) = 0.011159688056868854 +/- 1e-9
K_BC -> inferred P_BC -> |A_step2(D)| <= 1e-12
K_AB again -> inferred P_AB -> exact original D-probe trajectory restored
```

No profile reassessment and no additional C->A learning may occur during the context-switch cycle.

## 10. Frozen provenance requirements

The implementation must retain append-only records sufficient to reconstruct:

- each recognized context cue;
- its normalized activity signature;
- every assessed-profile score considered during inference;
- the selected profile when inference succeeds.

Recognition history and selection history are domain State, not CohBit governance evidence.

## 11. Frozen conformance tests

The implementation must include tests for at least:

1. V4->V5 migration preserves substrate, assessment history, and selection while initializing empty context history.
2. K_AB recognition produces the exact normalized signature without changing selected profile.
3. K_AB inference considers all assessed profiles and selects P_AB from scores 0.75/0.25.
4. K_BC recognition/inference selects P_BC from scores 0.25/0.75.
5. K_tie fails closed as ambiguous with inference-state nonmutation.
6. K_none fails closed as unsupported with inference-state nonmutation.
7. K_AB inferred selection enables the frozen D-probe transfer after C->A teaching.
8. K_BC inferred selection collapses transfer without reassessment or sequential-learning loss.
9. K_AB -> K_BC -> K_AB context cycle restores the identical transfer trajectory with assessment history unchanged.
10. Context recognition/selection provenance and the complete cycle are deterministic.

## 12. Failure discipline

After a scientific or numerical failure, do not change:

```text
P_AB
P_BC
K_AB
K_BC
K_tie
K_none
normalization rule
compatibility score
MIN_CONTEXT_SCORE
MIN_CONTEXT_MARGIN
eight teaching episodes
probe symbol D
four continuation steps
transfer threshold
regression values
V1-V4 parameters
```

Mechanical formatting, typing, or lint corrections are permitted only when they do not alter frozen semantics.

## 13. PASS claim ceiling

A PASS supports only:

> The organism can infer a previously assessed abstraction profile from current surface-context compatibility without receiving the profile identity, and continuation follows that inferred profile; ambiguous or unsupported context fails closed.

It does not establish semantic understanding, general contextual reasoning, governance authority, or general intelligence.
