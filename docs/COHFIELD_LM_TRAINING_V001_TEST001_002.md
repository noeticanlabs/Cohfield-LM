# Cohfield-LM Training v0.01 — Test 001 / 002

Status: frozen pre-runtime curriculum audit.

## Purpose

Test 001 established a graph-conditioned curriculum in which the same source node is paired with two distinct typed relational contexts and two distinct consequences. Step 002 audits whether the frozen split is actually learnable without target leakage or an impossible closed-world requirement before any runtime is allowed to claim success.

## Frozen curriculum identity

The local Test 001 curriculum contains:

- 2,836 eligible source records;
- 5,672 source/context -> target examples;
- train: 2,270 sources / 4,540 examples;
- validation: 279 sources / 558 examples;
- test: 287 sources / 574 examples;
- source-disjoint train/validation/test splits.

Positive training edges are restricted to source-defined `explicit_or_provenance` and `other_structured` relations. Candidate/inferred and correlation/cross-reference edges are excluded from positive targets for this test.

## Target-support audit

Because held-out source identities are disjoint from training, exact target-ID prediction is only a meaningful closed-world test when the held-out target identity has appeared in the training target vocabulary.

Audit results:

- validation target examples supported by the training target vocabulary: 534 / 558 = 95.6989%;
- validation unique target IDs: 171; unique target IDs seen in training: 147;
- test target examples supported by the training target vocabulary: 545 / 574 = 94.9477%;
- test unique target IDs: 175; unique target IDs seen in training: 146.

All held-out context edge types are represented in training.

Therefore approximately 4-5% of held-out examples contain target identities that a closed-vocabulary learner could not have acquired from the training target set. Those examples must not be silently counted as ordinary classification failures.

## Evaluation contract

Test 001 will report two separate evaluation surfaces.

### A. Supported-target discrimination

Primary learning metric. Evaluate only examples whose exact target ID is present in the training target vocabulary.

Required metrics:

- exact target top-1;
- exact target mean rank;
- paired-source success: both contexts for the same held-out source resolve to their respective targets;
- true-context vs shuffled-context delta;
- true-context vs wrong-edge-type delta;
- context-ablation delta;
- relation-ablation delta;
- untrained delta.

A learning PASS requires positive context-sensitive held-out discrimination on both validation and test and degradation under the targeted context/relation ablations.

### B. Open-target surface

Examples whose target ID is absent from the training target vocabulary are reported separately.

The runtime may emit `unseen_target` / defer for these examples. Test 001 does not require invention of an unseen target identity. If a later experiment supplies compositional target features that permit zero-shot target construction, that must be defined as a new experimental capability rather than retroactively credited here.

## Context-support audit

Every held-out context edge type occurs in training. This means a failure on supported-target examples cannot be explained merely by an unseen relation-type token.

However, this does not prove that relation types are sufficiently informative. A model that succeeds only because a relation type maps almost deterministically to one target will fail the matched source/context controls and paired-source analysis if those controls are constructed correctly.

## Anti-shortcut requirements

Before execution, the runtime must measure and publish:

1. target frequency by context edge type;
2. majority-target baseline per context edge type;
3. source-type + context-type majority baseline;
4. context-only baseline;
5. source-only baseline;
6. shuffled-context baseline.

The learned model must outperform these shortcut baselines on the preregistered supported-target surface. Otherwise the result is evidence of corpus regularity exploitation, not context-conditioned relational learning.

## Learning receipt requirements

For every persistent relation used by the runtime, the receipt must make it possible to recover:

- originating training evidence class;
- source/context feature responsible for eligibility;
- update count or accumulated evidence;
- final persistent strength;
- whether the relation was used in held-out consequence resolution;
- whether ablation of that relation changed the result.

## Claim boundary

A PASS may support only the claim that the runtime acquired a context-sensitive relational distinction that transfers to held-out source identities when the target identity is within the learned target vocabulary.

It does not establish zero-shot target invention, semantic mathematical understanding, theorem proving, or general reasoning.

## Frozen 002 conclusion

Test 001 is executable, but exact-target evaluation must be split into a supported-target learning surface and an open-target surface. This prevents approximately 4-5% unseen-target cases from contaminating the interpretation and adds shortcut baselines required to distinguish relational learning from simple context-frequency lookup.
