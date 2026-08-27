# CF-LM-IC-09 — Profile-Scoped Internal Equivalence Revision Contract

Status: staged pre-implementation contract v0.1
Parent: verified CF-LM-009 (`4ac241a09bccccb2e91530f91983a9e1a915f736`)

## Purpose

CF-LM-009 established that the language organism can acquire a persistent internal consequence-equivalence relation and use it for later transfer. CF-LM-007 had already established that observational equivalence is profile- and horizon-relative. A bare boolean equivalence memory therefore lacks enough scope information for continued development.

CF-LM-IC-09 requires the next versioned language State to preserve:

1. exposure-derived sequential relations;
2. the currently active internal consequence-equivalence relation;
3. an append-only history of internal consequence-equivalence assessments;
4. the exact assessment profile and measured pairwise consequence distance for each assessment epoch.

These components remain domain-specific State content. They are not new CohBit primitives.

## Firewalls

- internal assessment record != canonical Evidence
- internal assessment record != Verification
- consequence equivalence != semantic equivalence
- consequence equivalence != exact identity
- assessment history != active relation
- relation revision != history erasure
- relation revision != authority

## Required State separation

A conforming versioned relational configuration shall contain typed equivalents of:

```text
sequential
active_consequence_equivalence
assessment_history
active_profile
```

Changing the active relation must not erase prior assessments or sequential learning.

## Assessment profile

Each assessment must bind at minimum:

```text
projection
continuation_steps
epsilon
```

Each pairwise assessment record must bind at minimum:

```text
epoch
left_symbol
right_symbol
profile
measured_distance
equivalent
```

The profile is part of the assessment meaning. Two assessments made under different profiles are distinct records even when they concern the same symbol pair.

## Revision rule

A new assessment epoch recomputes the complete active relation under its declared profile and appends new records. The previous active relation may be replaced, but prior assessment records remain immutable historical State content.

Assessment must be calculated from the underlying sequential substrate without allowing the previously active equivalence relation to self-confirm its own witness measurement.

## Compatibility

- `CohfieldLanguageModelV1` remains unchanged.
- `CohfieldLanguageModelV2` remains unchanged.
- CF-ACP trait semantics remain unchanged.
- V3 is a versioned downstream language-domain extension.

## Claim ceiling

A PASS may support only:

> The Cohfield language organism can bind an internal consequence-equivalence relation to its assessment profile, revise the currently active relation when a different declared profile yields incompatible consequence evidence, preserve the prior assessment history, and make later transfer depend on the currently active relation rather than an erased or universalized equivalence claim.

It does not establish semantic equivalence, truth, verification, admissibility, policy, authority, execution, commitment, or CohTrace substitution.
