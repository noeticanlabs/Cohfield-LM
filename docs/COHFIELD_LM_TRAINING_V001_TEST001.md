# Cohfield-LM Training v0.01 — Test 001

## Title

Graph-Conditioned Contextual Relational Selection

## Status

Protocol and deterministic curriculum construction frozen. Model execution pending.

## Source

Complete Mathematical Graph v0.03 (`complete.mathematical_graph.v003`), created 2026-08-31.

The source graph contains 5,999 nodes, 47,164 edges, 38 node types, and 50 edge types. Its scientific boundary explicitly distinguishes correlation from dependency, dependency from proof, simulation from analytic closure, recurrence from validity, academic classification from theorem certification, and candidate edges from audited mathematical relations.

## Question

Can Cohfield-LM learn that the consequence of a source distinction changes with typed relational context?

The minimal target structure is:

    same source + context A -> target A
    same source + context B -> target B

A positive result therefore requires more than source memorization. The same source identity must support different continuations under different graph contexts.

## Curriculum construction

Only graph edges in the source schema's `explicit_or_provenance` or `other_structured` classes are eligible as positive targets in Test 001.

`candidate_or_inferred` and `correlation_or_crossreference` edges are excluded from positive training targets in this first test. They may later appear as negative or epistemic controls, but are not silently promoted to established relations.

For a source node to enter the curriculum:

1. it must have at least two distinct eligible outgoing edge types;
2. each selected edge type must have exactly one outgoing target from that source, eliminating target ambiguity;
3. exactly two typed contexts are retained per source for the v0.01 matched experiment;
4. sources are assigned to train/validation/test by a deterministic SHA-256 source-ID split so a source node cannot appear in more than one split.

The frozen local build produced:

- 2,836 eligible source records;
- 5,672 source/context -> target examples;
- train: 2,270 sources / 4,540 examples;
- validation: 279 sources / 558 examples;
- test: 287 sources / 574 examples;
- 2,393 explicit/provenance relation examples;
- 3,279 other-structured relation examples.

## Learning object

Test 001 does not define learning as raw exposure or weight movement.

For source representation `R_s`, context representation `C_r`, persistent relational state `Theta`, and resolved consequence `Y`, the intended mechanism is:

    Y = F(R_s, C_r, Theta, H)

with the requirement that for at least some held-out sources:

    F(R_s, C_a, Theta, H) != F(R_s, C_b, Theta, H).

The learned distinction must disappear or materially degrade when contextual relational state is ablated.

## Controls

The matched evaluation must include:

1. true typed context;
2. shuffled context labels while holding sources and targets fixed;
3. wrong edge-type context sampled from another valid relation class;
4. context ablation;
5. learned-relation ablation;
6. source-only baseline;
7. untrained model;
8. held-out source nodes with no source identity leakage from training.

## Measurements

At minimum record:

- correct-target activation/rank;
- true-vs-shuffled context delta;
- true-vs-wrong-context delta;
- context-ablation delta;
- relation-ablation delta;
- paired source discrimination rate: fraction of held-out sources for which the two contexts select their respective distinct targets;
- learned relation count and persistence;
- deterministic state/checkpoint digest;
- per-example provenance back to graph node and edge IDs.

## PASS gate

Test 001 passes only if all of the following hold on held-out data:

- true context outperforms shuffled context;
- true context outperforms wrong edge-type context;
- context ablation causes a measurable loss;
- learned-relation ablation causes a measurable loss;
- paired source discrimination exceeds the source-only and untrained controls;
- results reproduce deterministically under the frozen curriculum and implementation.

No training-set-only improvement can satisfy the gate.

## Interpretation boundary

A PASS establishes only that Cohfield-LM acquired a persistent, causally useful typed relational distinction that changes consequence selection for held-out graph sources.

It does not establish mathematical understanding, theorem proving, semantic comprehension, proof validity, or general reasoning competence.

A FAIL is also informative. It means the current architecture does not yet convert typed graph context into sufficiently discriminative persistent relational state under this controlled task.