# GI — Grid Intelligence v0.26

## Milestone: residual-derived latent observable formation

GI v0.25 could build a better feature by composing atomic observables supplied
in advance.

v0.26 asks a harder question:

> When the inherited feature grammar cannot preserve the distinctions needed by
> a new experience ecology, can GI form a new anonymous observable directly
> from raw transition effects?

## Stress ecology

The source domains use four multi-axis action directions:

- northeast,
- southeast,
- southwest,
- northwest,

with different raw magnitudes between domains.

This breaks the main assumptions of the inherited v0.25 atoms:

- `ACTIVE_AXIS` collapses because every action changes both coordinates;
- `SIGN` collapses because no single active coordinate exists;
- `SUPPORT_MASK` is identical for every action;
- `RAW_DELTA` is too literal to match across domains with different magnitudes;
- `L1_MAG` does not preserve direction.

## New formation path

GI receives raw transition-effect vectors.

It normalizes them and fits anonymous directional prototype codebooks for
candidate category counts.

The codebook is selected by a designed utility balancing:

- cross-domain token overlap;
- within-domain action discrimination;
- geometric separation;
- codebook complexity.

The specific prototype directions and selected category count are learned from
experience.

## Result

GI selects four anonymous categories corresponding to the four recurrent
directional structures.

The category labels themselves have no semantics.

A two-step template induced through this learned observable transfers into an
unseen target domain whose raw action magnitudes differ from both source
domains.

Held-out transfer: 4/4.

## Boundary

This is not unrestricted sensory invention.

The following remain designed:

- access to raw transition vectors;
- L2 normalization;
- nearest-prototype observation class;
- deterministic k-means;
- candidate cluster-count range;
- codebook utility.

The defensible result is:

**GI v0.26 demonstrates data-driven formation of a new anonymous latent
observable inside a declared unsupervised codebook class, triggered by failure
of its inherited representational grammar.**
