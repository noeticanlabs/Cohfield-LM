# GI v0.26 Formal Specification

## Problem

Let the inherited feature grammar be `G_25`.

For source-domain action observations:

`o = (domain, action_id, Delta)`,

first evaluate the best available inherited feature:

`F* = argmax_{F in G_25} J(F)`.

If inherited features fail to jointly achieve the required cross-domain
invariance and within-domain discrimination, invoke observable formation.

## Raw observation carrier

Use raw local transition effect:

`Delta in R^d`.

Normalize:

`q = Delta / ||Delta||_2`.

Zero effects map to zero.

## Candidate learned observable

For candidate category count `k`, learn prototypes:

`P_k = {p_1,...,p_k}`

by deterministic k-means over normalized observed effects.

The anonymous observable is:

`O_k(Delta) = argmin_i ||normalize(Delta)-p_i||^2`.

No semantic label is assigned to `i`.

## Selection

For each candidate `k in {2,...,6}` measure:

- mean pairwise cross-domain Jaccard overlap of anonymous token sets;
- mean within-domain discrimination ratio;
- minimum normalized prototype separation;
- complexity penalty proportional to `k`.

Select the maximum-score codebook.

## Template induction

Successful source traces are rewritten as sequences of learned anonymous
observable IDs.

Repeated token sequences with at least three distinct supporting tasks become
higher-order templates.

## Transfer

The target domain is excluded from observable formation.

Target anonymous actions are mapped through the frozen learned codebook using
their local transition effects.

A transferred template counts as one higher-order planning object and is rolled
out through the target domain's learned transition model before external audit.

## Scientific boundary

The specific prototype vectors and selected category count are learned.

The observable function class is not open-ended: normalization,
nearest-prototype categorization, k-means, k-range, and utility are designed.

Therefore v0.26 is constrained latent-observable formation, not unrestricted
invention of arbitrary sensors, semantics, or representations.
