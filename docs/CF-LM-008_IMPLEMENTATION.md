# CF-LM-008 Implementation Boundary

Status: **Implementation staged; local conformance pending**

Protocol parent: `agent/cf-lm-008-observational-partition-contract` at `6c4675b6c517575c74c4692911f0ab8ced0c83b2`.

## Implementation

`tests/language_observational_partition.rs` implements the preregistered six-State, two-profile-family discovery experiment.

No production model code or parameter is changed.

The partitioning function accepts only continuation-response vectors. It has no access to candidate names, source histories, route kinds, or `Psi`.

## Frozen expected result

Short profile family:

`{{0,2,4}, {1,3,5}}`

Full profile family:

`{{0}, {1,3,5}, {2,4}}`

The richer family must strictly refine the short partition and may not merge a pair separated by the short family.

## Evidence boundary

A local PASS would show that an external observer can recover finite profile-relative observational classes from behavior alone and that profile enrichment refines those classes at the CF-LM-007 latent-consequence boundary.

It would not show that the model internally represents, names, stores, or reasons over an equivalence class. No semantic-equivalence relation is declared by this experiment.

## Local gate

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No frozen carrier, blinded order, profile ordering, host relation, horizon, observer, partition target, or regression value may be changed after a failed gate without a versioned successor.
