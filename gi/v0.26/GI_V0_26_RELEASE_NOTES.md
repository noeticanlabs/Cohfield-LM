# GI — Grid Intelligence v0.26 Release Notes

## Release status

All acceptance tests pass: **True**

## Cognitive milestone

GI v0.26 adds a residual-driven representation path.

When the inherited v0.25 feature grammar cannot jointly preserve the
cross-domain invariance and action discrimination required by a new experience
ecology, GI forms an anonymous learned observable directly from raw transition
effects.

## Stress condition

The source ecology contains four multi-axis action directions. Every action
changes both coordinates, and the two source domains use different raw
magnitudes.

This makes the inherited v0.25 feature grammar inadequate:

- ACTIVE_AXIS collapses because there is no single changed coordinate;
- SIGN collapses because the single-axis sign assumption is not satisfied;
- SUPPORT_MASK is identical for all actions;
- RAW_DELTA is too magnitude-specific for cross-domain matching;
- magnitude alone does not preserve direction.

Inherited grammar best discrimination:
0.25

## Learned observable

v0.26 normalizes raw transition effects and learns an anonymous prototype
codebook.

Selected category count:

`k = 4`

Learned prototypes:

`[[-0.7071067811865475, -0.7071067811865475], [-0.7071067811865475, 0.7071067811865475], [0.7071067811865475, -0.7071067811865475], [0.7071067811865475, 0.7071067811865475]]`

No directional semantic names are supplied to these categories.

Both source domains map their four anonymous actions to the same four learned
category IDs despite different raw magnitudes.

## Cross-domain transfer

A two-step structural template learned through the new observable transfers to
an unseen target domain excluded from observable formation.

Transferred plan:

`[0, 1]`

External verification:

True

Held-out transfer:

4/4

## Controls

- Shuffling a prototype identity actually used by the transferred template
  breaks verification.
- A different cardinal-direction experience ecology produces a different
  learned prototype set.
- Corrupting the unseen target transition model causes external verification
  failure.
- A cross-domain one-off outlier does not force unnecessary codebook growth.

## Scientific correction during development

The initial development run exposed two weak controls:

1. the first shuffled-codebook control accidentally swapped categories not used
   by the tested transfer template;
2. the initial codebook complexity coefficient (`0.06 * k`) slightly favored a
   fifth category when a matched low-support outlier was injected.

Before release freeze:
- the shuffle control was corrected to perturb a category actually used by the
  transferred template;
- the explicit complexity coefficient was changed to `0.075 * k`, which keeps
  the four-category model under the outlier control while preserving the main
  four-category result.

No target task, target plan, transfer answer, or semantic label was introduced
by this correction.

## Scientific boundary

The specific prototypes and selected category count are learned.

The following are still designed:

- access to raw transition-effect vectors;
- L2 normalization;
- nearest-prototype categorical observation class;
- deterministic k-means;
- candidate k range 2..6;
- observable utility function;
- template support threshold.

Therefore the strongest defensible claim is:

**GI v0.26 demonstrates constrained, data-driven formation of a new anonymous
latent observable inside a declared unsupervised codebook class, triggered by a
failure mode of its inherited representation grammar.**

It does not establish arbitrary new-sensor invention, semantic feature
formation, general reasoning, or general intelligence.
