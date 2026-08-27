# CF-LM-010 — Profile-Scoped Internal Equivalence Revision

Status: preregistered protocol v0.1
Parent: CF-LM-IC-09

## Question

Can a versioned Cohfield language State preserve the profile and measured consequence geometry that produced an internal equivalence relation, revise the active relation when another declared observer profile contradicts it, retain both assessments, and make future transfer follow only the currently active relation?

## Frozen source State

Reuse the verified CF-LM-009 source substrate containing both route cores:

```text
A -> C = 0.9840816505055259
C -> B = 1.0041649494954346
A -> D = 0.9840816505055259
D -> B = 1.0041649494954346
```

Initial fast State is zero, Theta is `[1,1,1,1]`, active consequence-equivalence is empty, and assessment history is empty.

## Frozen assessment profiles

### P_AB

```text
projection = [A,B]
continuation_steps = 4
epsilon = 1e-12
```

Preregistered pairwise distances:

```text
A/B = 0.8084614995832016
A/C = 0.5891841588041229
A/D = 0.5891841588041229
B/C = 0.5229752821187045
B/D = 0.5229752821187045
C/D = 0.0
```

Therefore the only nontrivial active relation after the P_AB assessment is symmetric `C~D`.

### P_BC

```text
projection = [B,C]
continuation_steps = 4
epsilon = 1e-12
```

Preregistered pairwise distances:

```text
A/B = 0.5897786901468243
A/C = 0.5369055299673055
A/D = 0.20338849532042508
B/C = 0.7787881343517881
B/D = 0.5229752821187045
C/D = 0.5770682910193559
```

No nontrivial pair is equivalent under P_BC.

## Assessment history

Assessment epoch 1 applies P_AB and appends six pairwise assessment records.

Assessment epoch 2 applies P_BC and appends another six records.

Required after epoch 2:

```text
assessment_history.len() = 12
active_profile = P_BC
active nontrivial equivalence pairs = empty
```

Epoch-1 records remain unchanged and retrievable, including the original C/D zero-distance equivalent result.

## Frozen transfer attack

Use eight isolated `[C,A]` teaching episodes exactly as CF-LM-009.

Predicted sequential relation:

```text
sequential[C][A] = 0.5579844028434426
```

### Branch A — P_AB active

Assess P_AB, then teach C->A, then probe D for four continuation steps.

Require:

```text
A_step2(D) > 0.01
A_step2(D) = 0.011159688056868854 +/- 1e-9
sequential[D][A] <= 1e-12
```

### Branch B — P_BC revision active

Assess P_AB, then P_BC, then teach the identical C->A episodes, then probe D.

Require:

```text
A_step2(D) <= 1e-12
sequential[C][A] remains 0.5579844028434426 +/- 1e-9
sequential[D][A] <= 1e-12
```

The transfer difference must therefore be caused by active equivalence revision, not by erasing C->A learning.

### Branch C — reacquisition

From the pre-teaching State after P_AB then P_BC, assess P_AB again as epoch 3, then teach C->A and probe D.

Require:

```text
assessment_history.len() = 18
active_profile = P_AB
C~D active again
A_step2(D) = 0.011159688056868854 +/- 1e-9
```

Historical P_BC records remain present.

## Required controls

1. V2 -> V3 migration preserves fast State, Theta, sequential relations, and starts with empty V3 assessment history unless explicitly migrated.
2. P_AB assessment discovers only C/D.
3. P_BC assessment discovers no nontrivial pair.
4. Each epoch appends exactly six pairwise records.
5. Previous records are not mutated by later assessments.
6. Assessment calculation ignores the currently active equivalence relation for witness measurement.
7. P_AB-active transfer reproduces CF-LM-009.
8. P_BC revision collapses transfer without erasing C->A sequential learning.
9. P_AB reacquisition restores transfer while preserving all prior history.
10. Construction and assessment are deterministic.

## Frozen thresholds

```text
epsilon_floor = 1e-12
epsilon_transfer = 0.01
regression_tolerance = 1e-9
teaching_episodes = 8
```

## PASS / FAIL

All required controls must pass. Any failure is CF-LM-010 FAIL under this protocol. No profile, threshold, source weight, teaching schedule, pairwise regression, or revision rule may change after execution without a versioned successor.

## Claim ceiling

PASS supports profile-scoped, revision-capable internal consequence-equivalence memory on the declared finite language system. It does not establish semantic equivalence or any governance-layer permission.
