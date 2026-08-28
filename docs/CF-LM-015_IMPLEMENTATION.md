# CF-LM-015 Implementation Boundary

## Branch

`agent/cf-lm-015-derived-abstraction-impl`

Protocol parent: `agent/cf-lm-015-derived-abstraction-contract` at `d7379398a5d8dc22b1c22ea9baf50ebf4e8f6312`.

## Added implementation

- `src/profiles/language_v8.rs`
- `src/profiles/mod.rs` export
- `tests/language_derived_abstraction.rs`

## V8 architecture

V8 nests the complete V7 relational configuration unchanged as `parent` and adds only:

- stable `derived_abstractions` identities;
- append-only `abstraction_formation_history`;
- mutable `abstraction_relations` from derived abstractions to surface symbols;
- one explicitly active derived abstraction for the CF-LM-015 causal test.

A derived abstraction identity contains only the assessment profile and member set. Formation epoch/source-assessment epoch remain separate provenance records.

## Formation

`FormDerivedAbstractions(profile)` receives a profile but no member set. It reconstructs the latest executable equivalence relation for that assessed profile, verifies symmetry/transitivity with implicit reflexivity, derives nontrivial classes, and creates stable abstraction identities.

Re-running against the same assessment is idempotent. Re-running after a later same-profile assessment preserves the same abstraction identity and appends new provenance.

## Learning

Parent sequential adaptation remains unchanged. V8 additionally applies the same decay/gain constants to abstraction-to-symbol relations when a sequential predecessor belongs to a derived abstraction.

Eight direct C->A adaptation events are preregistered to yield both direct Psi[C,A] and derived `{C,D}->A` relation weight `0.5969479096728575`, with direct Psi[D,A]=0.

## Continuation

V8 one-step continuation first runs verified V7 one-step dynamics, then adds abstraction-mediated contribution from the explicitly active derived abstraction using mean member activity. The CF-LM-015 probe requires the V7 parent selected profile to remain None, so pairwise equivalence coupling cannot explain transfer.

## Evidence boundary

No local Rust gate has been run in this environment. No CF-LM-015 PASS is claimed until:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

passes on the frozen branch.

Verified parent total: 167 tests. New CF-LM-015 tests: 10. Expected full target: 177/177.

Runtime tests, if successful, remain executable finite-domain evidence rather than formal proof or semantic-concept validation.
