# CF-LM Teacher Bridge v0.02 — Withheld-Combination Composition

## Status

**Disposition: PASS**

Implementation branch: `agent/cf-lm-teacher-bridge-v002`

Purpose: with the v0.01 bridge mechanics unchanged (no new adaptation law), teach a branching curriculum of local experiences such that CF-LM resolves multiple entire two-hop combinations it was never shown, with causal double-dissociation controls localizing each withheld combination to its own learned route.

This is a withheld-combination **composition** result, not abstract transfer. Both CI gates (new-experiment gate + full inherited regression gate) must pass on the frozen branch head for the disposition to stand.

## Experimental question

Can an LLM-authored branching curriculum teach only local experiences such that CF-LM resolves entire two-hop combinations it was never shown, while structurally underivable pairs stay silent, and does each withheld combination depend causally on its own learned route?

## Relation to v0.01

The v0.01 bridge demonstrated three-hop composition of a single taught chain (A->B->C->D). v0.02 reuses the identical verified bridge mechanics (`CfLmTeacherBridgeV001`) with no new adaptation law and adds:

- a branching curriculum;
- withheld two-hop combinations that are never exposed as episodes;
- a structurally underivable negative family;
- a double-dissociation ablation localizing each withheld combination to its own route.

## Frozen LLM-authored branching curriculum

Episodes (each exposed as an isolated two-symbol experience, 64 epochs):

- `A -> B`
- `B -> C`
- `B -> D`

The v0.01 edge `C -> D` is deliberately absent. No episode ever targets `A`. The full sequences `A -> C` and `A -> D` are never exposed as training episodes.

## Frozen persistent relations after training

- `Psi_AB = 0.6461059481081141`
- `Psi_BC = 0.6727467181467244`
- `Psi_BD = 0.7004859622518996`
- `Psi_AC = 0`
- `Psi_AD = 0`
- `Psi_CD = 0`
- `Psi_CA = Psi_CB = Psi_BA = 0` (no taught episode targets A; C has no outgoing relation)

`Psi_AB` and `Psi_BC` reproduce the v0.01 values exactly, as expected from identical episode slots; `Psi_BD` occupies the third slot value.

## Teacher-off probe

Teacher removed. Equalize X=0, Theta=1, drive A once, then two zero-input continuation steps:

```
step 0: A=0.5                 B=0                  C=0                        D=0
step 1: A=0.25                B=0.06461059481081141 C=0                       D=0
step 2: A=0.125               B=0.06461059481081141 C=0.008693313123296232    D=0.009051762935543765
```

Both withheld two-hop combinations activate despite zero direct `A->C` and `A->D` relations.

## Controls

### Structurally underivable family

Probing from C: `Psi` into A is zero from every source, C has no outgoing learned relation, and A/B/D remain <= 1e-12 at every probe step. Probing from D or B into A likewise stays silent.

### No adaptation

With `psi_gain = 0`, the same curriculum yields no persistent relations and both withheld activations remain <= 1e-12.

### Double-dissociation surgical ablation

- Ablating only `Psi_BC`: the withheld `A -> C` activation collapses to <= 1e-12 while `A -> D` survives at 0.009051762935543765.
- Ablating only `Psi_BD`: the withheld `A -> D` activation collapses to <= 1e-12 while `A -> C` survives at 0.008693313123296232.

The full run tuple is asserted deterministic across repeated executions.

### Teacher-off purity

The teacher-off examination does not mutate persistent `Psi` (asserted by state equality).

## Inherited-surface restoration

The v0.01 branch head (`91e7aa6`) silently modified two inherited CF-LM-015 research files: it reverted the frozen-trajectory expectation in `tests/language_derived_abstraction.rs` to the superseded isolated-substrate values and deleted the documented prediction-derivation correction from `docs/CF-LM-015_IMPLEMENTATION.md`. Because the coupled V8 dynamics are unchanged, that reversion broke the CF-LM-015 frozen-trajectory regression (9/10 on the v0.01 head), which the v0.01 branch-scoped CI did not execute.

This branch restores both files byte-identically to `agent/cf-lm-015-derived-abstraction-impl`. With the restoration, the full local suite passes, including the CF-LM-015 conformance tests (10/10). The remaining `language_v8.rs` differences from CF-LM-015 (rustfmt reflow plus a behavior-equivalent manual `Default` impl) are retained unchanged from v0.01.

## Evidence boundary

Local Rust gate on this machine (rustc 1.94.1):

```text
cargo test --test teacher_bridge_v002  ->  6/6 PASS
```

The branch-scoped CI gate (`.github/workflows/teacher-bridge-ci.yml`) now formats and runs both bridge surfaces and triggers on `agent/cf-lm-teacher-bridge-v002`. No CI PASS is claimed until the pushed branch-head run completes.

## Claim ceiling

v0.02 is a withheld-combination composition result: combinations absent from training resolve through composition of locally taught relations, with route-specific causal dependence. It does not demonstrate grammar induction, semantic understanding, abstract rule learning, or natural-language competence.
