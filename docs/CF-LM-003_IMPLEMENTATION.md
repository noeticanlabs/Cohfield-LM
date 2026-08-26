# CF-LM-003 — Implementation and Evidence Boundary

Status: **Implementation staged; local conformance pending**

Protocol parent:

- branch: `agent/cf-lm-003-observer-equivalence-contract`
- commit: `ad1be9a6c37b41f388786c8e77f318358a2a6e0d`

Verified executable parent:

- CF-LM-002: `a0c5afe8189b3d42128e72e375ab3b2f2100fb91`

## Implementation scope

CF-LM-003 adds no production model code.

It reuses `CohfieldLanguageModelV1` unchanged and adds only `tests/language_observer_equivalence.rs` to execute the preregistered observer-relative equivalence protocol.

## Frozen comparison

Histories:

`H_CD = (C D)^64`

`H_DC = (D C)^64`.

Comparison equalization:

`X_CD = X_DC = 0`

`Theta_CD = Theta_DC = (1,1,1,1)`.

Persistent relational configurations remain exposure-derived.

## Frozen observer profiles

Restricted observer:

`AB, BA`

with four autonomous continuation steps.

Enriched observer:

`AB, BA, CD, DC`

with the same continuation depth and full response carrier.

## Frozen PASS thresholds

`epsilon_state = 0.05`

`epsilon_floor = 1e-12`

`epsilon_discrim = 0.01`.

The preregistered target is:

`||Psi_CD - Psi_DC||_F > epsilon_state`

`D_restricted <= epsilon_floor`

`D_enriched > epsilon_discrim`.

## Preimplementation numerical cross-check

The frozen equations predict approximately:

`||Psi_CD - Psi_DC||_F = 0.061531831442227035`

`D_restricted = 0`

`D_enriched = 0.01652979019225732`.

Regression checks against these decimal predictions use `1e-9` numerical tolerance. That regression tolerance is not a PASS threshold and was fixed before local Rust execution.

## Claim ceiling

A PASS supports observer-relative continuation equivalence only.

It does not establish semantic equivalence, same denotation, paraphrase, substitutability, or natural-language understanding.

## Local evidence gate

Run:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No history, observer profile, PASS threshold, model parameter, continuation depth, or response metric may change after a failed gate without a versioned amendment.
