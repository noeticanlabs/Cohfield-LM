# Cohfield-LM Training v0.01 — Interpretable Relational Learning Contract

Status: experimental training charter. This document defines what may count as learning, how training may alter persistent state, and what evidence is required before a learned structure claim is accepted.

## 1. Purpose

Cohfield-LM Training v0.01 is not a generic next-token training recipe. Its purpose is to make learned change scientifically identifiable.

The central questions are:

1. **What changed?** Which internal distinction or relation became persistently available?
2. **How did it change?** Which observed relational evidence and update rule produced the persistent change?
3. **Why did it persist?** Did that change provide measurable causal utility on held-out computation?

A training run is not considered successful merely because exposure occurred, internal weights changed, or average activation increased.

## 2. Learning definition

A structure is considered learned only when training produces a persistent internal relational change that is:

- attributable to specified training evidence;
- causally consequential for subsequent computation;
- useful on held-out data relative to matched controls;
- degraded by a targeted ablation of the learned mechanism;
- traceable to a reproducible provenance record.

Formally, let persistent relational state be Theta_t. A candidate learned structure DeltaTheta is admissible only if

    DeltaTheta != 0

and there exists a held-out task metric U such that

    U(Theta + DeltaTheta) > U(Theta_control)

under a preregistered comparison, and

    U(Theta + DeltaTheta) > U(Ablate(Theta + DeltaTheta)).

The metric may be task-specific. No single scalar utility is claimed to be universal.

## 3. Training object

Training v0.01 treats the primitive learned object as a typed relational distinction rather than an isolated symbol frequency.

A relational state may be represented abstractly as

    R(i,j) = (type, direction, strength, evidence_status, history).

Examples of distinct relation classes include correlation, dependency, derivation, method use, provenance, candidate cross-reference, and other source-defined edge types. These classes must not be collapsed unless an experiment explicitly tests such a collapse.

## 4. Training flow

The minimal training flow is

    observation -> distinction -> relation -> context -> consequence -> persistence decision.

For observation u_t, operative state Z_t, relational context C_t, and history H_t:

    D_t = Observe(u_t)
    R_t = Relate(D_t, Z_t, H_t)
    Q_t = Contextualize(R_t, C_t)
    Y_t = Consequence(Q_t)
    DeltaTheta_t = Adapt(R_t, C_t, Y_t, evidence_t)

Persistent update is permitted only through the declared adaptation law.

## 5. Why a relation strengthens

A relation must not strengthen solely because it appeared frequently unless frequency is the explicit hypothesis under test.

The general training form is

    DeltaTheta_ij = eta * E_ij * C_t * G_t - rho * Theta_ij,

where:

- E_ij is observed relational evidence;
- C_t is the context-dependent eligibility or selection term;
- G_t is a measured task consequence, gain, or other declared learning signal;
- rho is the persistence-decay or regularization term.

This equation is a training contract, not a claim that all Cohfield-LM profiles must use multiplication or a universal scalar reward. Concrete profiles may instantiate a different bounded rule if they preserve the same evidence/context/consequence separation.

## 6. What Training v0.01 must record

Every accepted training run must emit a receipt sufficient to answer:

- source dataset identity and hashes;
- exact train/validation/test split identity;
- model/runtime source identity;
- initial persistent-state identity;
- update-law identity and parameters;
- number and class of persistent relations changed;
- relation-type distribution before and after training;
- held-out metrics;
- matched control metrics;
- ablation metrics;
- deterministic replay status;
- claim boundary.

## 7. Required controls

At minimum a learning claim requires:

1. untrained control;
2. matched exposure control with the target relation disrupted or shuffled;
3. context disruption or wrong-context control when context is part of the hypothesis;
4. targeted ablation of the proposed learned mechanism;
5. untouched held-out evaluation;
6. deterministic replay.

Where relation types are involved, a wrong-edge-type control should be added whenever feasible.

## 8. Training levels

Training v0.01 defines the following evidence ladder. These are capability tests, not product maturity levels.

- L1 Distinction learning: different observations produce persistently distinguishable internal states.
- L2 Relation learning: learned state preserves directional or otherwise operational relationships between distinctions.
- L3 Typed relation learning: different source-defined relation classes remain internally distinguishable.
- L4 Context-conditioned relational selection: the same local distinction produces different consequences under different relational contexts.
- L5 Multi-hop relational composition: held-out consequences depend on composing more than one learned relation.
- L6 Counterfactual relation testing: changing or removing a relation produces the predicted consequence difference.
- L7 Utility-governed persistence: learned structures persist or decay according to measured downstream utility under a declared governance rule.

A later level does not retroactively prove earlier levels unless its experiment explicitly contains the required controls.

## 9. v0.01 first experimental target

The first Training v0.01 experiment should test contextual relational selection using a frozen structured mathematical graph.

Construct matched cases where a source distinction has multiple legitimate outgoing relations. The correct held-out continuation must depend on surrounding relational context rather than only the local source identity.

Required condition:

    consequence(A, C1) != consequence(A, C2)

for matched source A and different contexts C1 and C2.

The experiment must include:

- true context;
- shuffled context;
- wrong relation type;
- context ablation;
- relation ablation;
- untrained control.

A PASS requires held-out context-sensitive discrimination that is lost under the relevant ablation.

## 10. Relationship to corpus pilots v0.01-v0.03

The earlier corpus pilots remain evidence about representational limits and history mechanisms:

- first-order exposure alone was insufficient;
- explicit short history was causally consequential but did not pass the full paired discrimination gate;
- compressed trajectory history was also consequential but weaker under the frozen task.

Training v0.01 therefore does not simply add more history. It changes the scientific target from exposure-based continuation to identifiable relational learning.

## 11. Claim boundary

A PASS under Training v0.01 may support a claim that Cohfield-LM acquired and used a specific relational distinction under a controlled task.

It does not by itself establish semantic understanding, mathematical reasoning, theorem proving, language competence, or general intelligence.

## Frozen principle

> Cohfield-LM training should optimize for identifiable relational change rather than mere exposure. A learned structure is accepted only when its origin, persistence, causal consequence, held-out utility, and ablation dependence are all measurable.
