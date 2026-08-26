# CF-LM-004 — Implementation and Evidence Boundary

Status: **Implemented; local PASS/FAIL pending**

Protocol parent: `agent/cf-lm-004-consequence-equivalence-contract` at `8cdd864dd523fe025c11c505ac9618b731be6958`.

## Implementation scope

CF-LM-004 adds only `tests/language_consequence_equivalence.rs`.

`CohfieldLanguageModelV1`, CF-ACP core semantics, adaptation law, continuation dynamics, and all previously verified model parameters are unchanged.

The test implementation operationalizes:

- exact-different `A -> C -> B` and `A -> D -> B` learned routes;
- exact matched surface counts;
- direct `A -> B` absence;
- projected A/B consequence observation across four contexts;
- baseline, outgoing-A attenuation, and incoming-B attenuation intervention profiles;
- nondegeneracy of the consequence observer;
- nontrivial shared two-hop A-to-B consequence;
- rich full-coordinate route discrimination;
- deterministic repeat and preregistered numerical cross-checks.

## Evidence ceiling

Until the local gate executes, this branch supports only:

> CF-LM-004 is implemented against a preregistered context-general consequence-equivalence protocol.

A local PASS may support only the claim stated in `CF-LM-004_PROTOCOL.md`.

It must not be described as semantic equivalence, meaning, synonymy, paraphrase understanding, or identity substitution.

## Local gate

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No frozen experimental constant or observer/intervention definition may be changed after a failed gate without a versioned amendment.
