"""GI — Grid Intelligence v0.26.

Residual-derived latent observable formation.

Milestone:
GI v0.25 could synthesize composite features from a supplied atomic feature
library. GI v0.26 adds a different path: when the inherited feature grammar
cannot preserve the distinctions required by a new experience ecology, GI
constructs an anonymous latent observable directly from raw transition effects.

The new observable is a learned directional codebook. Its categories and
prototype vectors are inferred from experience rather than pre-enumerated as
AXIS, SIGN, magnitude, or named symbolic features.

Boundary:
the codebook-learning algorithm, L2 normalization, candidate cluster-count
range, distance measure, and selection objective are still designed. This is
data-driven formation of a new observable inside a declared function class,
not unrestricted invention of arbitrary senses or semantics.
"""
__version__="0.26"
