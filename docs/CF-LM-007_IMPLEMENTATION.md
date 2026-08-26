# CF-LM-007 — Implementation and Evidence Boundary

Status: **Implementation staged; executable result pending local gate**

Protocol parent: `docs/CF-LM-007_PROTOCOL.md`

Contract parent: `docs/CF-LM-IC-06.md`

Verified upstream evidence: `CF-LM-006` at `edee108eb470913e7dab43f83dec91e1115f4650`.

## Implementation scope

`CF-LM-007` adds only downstream conformance tests in:

`tests/language_profile_transfer_boundary.rs`.

No production model code, adaptation law, State representation, profile trait, or model parameter is changed.

The tests reconstruct the same frozen three-State carrier used by `CF-LM-006`, add the preregistered symmetric unseen `C <-> D` transfer-host edges, and compare projected A/B consequence responses at the frozen 4-, 5-, and 10-step horizons.

## Evidence boundary

Before the local Rust gate, the repository may claim only:

- the transfer/boundary protocol was frozen before implementation;
- independent numerical cross-checks support the expected target pattern;
- the implementation encodes the frozen test profile without changing `CohfieldLanguageModelV1`.

It may **not** claim `CF-LM-007 PASS` until all local format, lint, and test gates pass.

## Required gate

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

If any test fails, the failure must be classified before changing the carrier, source histories, cross-relay edges, host strengths, horizons, observer projection, thresholds, response metric, or model parameters.

## Claim ceiling after PASS

A successful gate would establish finite executable evidence that the previously demonstrated contextual-consequence equivalence is not profile-independent: it transfers under the new host at the original horizon, but longer continuation exposes the independently learned latent-loop member while preserving the whole-route C/D equivalence.

This remains observational/consequence evidence. It is not semantic equivalence and grants no new CohBit identity or governance substitution rights.
