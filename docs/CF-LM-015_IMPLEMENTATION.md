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

## Prediction-derivation correction (documented)

The preregistered D-probe trajectory for `cf_lm_015_active_derived_abstraction_mediates_frozen_d_to_a_trajectory` was derived by treating the active derived abstraction as an isolated auxiliary channel `D -> α_CD -> A`. That derivation assumed `x_C == 0` after activation. The full coupled substrate does not satisfy that assumption: A-leg activity generated along `D -> α -> A` is fed back into member C through the substrate route `A -> C` (`seq[A][C] = 0.9840816505055259`), so `x_C` becomes nonzero from the second continuation step onward. That fed-back C activity then re-enters A both directly (learned `C -> A` edge) and through the mean-member activation `a_α = (x_C + x_D)/2`.

The original preregistered trajectory `[0, 0.029847395483642875, 0.029847395483642875, 0.022385546612732156, 0.014923697741821437]` is therefore an incomplete isolated-channel prediction. The corrected frozen trajectory is the full-coupled prediction:

```
[0, 0.029847395483642875, 0.029847395483642875, 0.02357890970562522, 0.017310423927607566]
```

The first three recorded A values coincide because the member-feedback contribution has not yet altered A; the later values differ once `A -> C` feedback makes `x_C` nonzero and that activity re-enters the coupled dynamics.

This is a documented prediction-model correction, not experimental redesign or parameter tuning. The experimental mechanism, all causal controls, formation semantics, identity/provenance separation, learning constants (`ρ = 0.02`, `η = 0.08`), eight `C -> A` events, `W_{α,A} = Ψ[C,A] = 0.5969479096728575`, `Ψ[D,A] = 0`, activation rule, and `selected_profile = None` are unchanged. The surgical ablation (`W_{α,A} ← 0`, keeping `Ψ[C,A]`) remains the decisive causal control.

## Evidence boundary

The correction is present on the implementation branch, but CF-LM-015 is not considered verified until the complete local Rust gate passes after the correction:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Verified parent total: 167 tests. New CF-LM-015 tests: 10. Expected full target: 177/177.

Runtime tests, if successful, remain executable finite-domain evidence rather than formal proof or semantic-concept validation.
