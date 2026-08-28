# CF-LM Teacher Bridge v0.03

## Disposition

**BOUNDARY PASS — structural transfer remains unproven.**

Branch: `agent/cf-lm-teacher-bridge-v003`

v0.03 expands the Teacher Bridge to a bridge-scoped nine-symbol synthetic surface and separates three mechanisms under the same visible curriculum.

## Frozen curriculum

The teacher exposes only:

- `A1 -> B1`
- `A2 -> B2`
- `A3 -> B3`
- `B1 -> C1`
- `B2 -> C2`

for 64 epochs.

`B3 -> C3` is withheld. The direct persistent relation `Psi[B3,C3]` remains zero in every non-direct arm.

## v0.03b — plain composition boundary

Teacher-off probe begins at `A3`.

Plain dynamics reach the taught `B3` state, but all `C1`, `C2`, and `C3` activations remain at the numerical floor because `B3` has no learned outgoing C edge.

This is the intended null result:

> Composition of learned edges does not imply inference of an unlearned edge.

The direct-teaching positive control adds `B3 -> C3`; the same nine-symbol Plain substrate then learns a nonzero direct relation and propagates `C3` teacher-off. Therefore the null result is not a surface-capacity failure.

## v0.03c — CF-LM-015-style member abstraction boundary

The MemberAbstraction arm forms a B-family abstraction and uses mean member activity with learned abstraction-to-symbol relations.

From held-out member `B3`, teacher-off continuation activates `C1` and `C2`, because those targets were previously learned from other B-family members. `C3` remains exactly silent because no abstraction-to-`C3` relation was learned.

Frozen step-3 values:

- `C1 = 0.0016942511181904604`
- `C2 = 0.0017641098690029781`
- `C3 = 0.0`

A surgical ablation of only the abstraction-to-`C1` weight collapses `C1` while preserving `C2` and leaving ordinary `Psi` unchanged.

This is a genuine abstraction-mediated transfer effect, but it is transfer from an unseen **source member** to already learned targets. It is not yet the requested unseen aligned pair `B3 -> C3`.

## Exploratory Target pool

The Target mechanism adds a theorized pooled B-family -> C-family path that is explicitly not the frozen CF-LM-015 primitive.

It produces a nonzero `C3` response without a direct `Psi[B3,C3]` edge. However, the response is exactly equal across the entire C family:

- `C1 = 0.003458360987193436`
- `C2 = 0.003458360987193436`
- `C3 = 0.003458360987193436`

This equality is now an executable control in the test suite.

Therefore this arm demonstrates **pooled family activation**, not specific structural inference that `B3` should map to `C3`. It must not be used as evidence of matched structural generalization.

Zeroing only `w_pool_c` collapses the pooled C response while preserving `Psi` and the member-abstraction weights.

## Frozen diagnostics

Selected trained-state values:

- `Psi[A3,B3] = 0.40338056149374846`
- `Psi[B1,C1] = 0.42001307943955485`
- `Psi[B2,C2] = 0.437331402998287`
- `Psi[B3,C3] = 0.0`
- `w_pool_c = 0.8573444824378412`

Plain teacher-off `B3` activation at step 1:

- `0.04033805614937485`

All values above are frozen into `tests/teacher_bridge_v003.rs` as regression assertions rather than diagnostic printouts.

## Test-structure repair

An earlier local version of `tests/teacher_bridge_v003.rs` contained a nested `#[test] fn diag_exact()` item and could emit `cannot test inner items`. The repaired file contains only top-level tests. The frozen diagnostic is now an ordinary discoverable test named:

`frozen_exact_diagnostics_are_stable`

CI explicitly runs that test with `--exact` so a future nesting/truncation regression cannot silently hide it.

## CI discipline

The Teacher Bridge workflow now triggers on the v0.03 branch and contains two gates:

1. new-experiment gate — rustfmt plus v0.01/v0.02/v0.03 bridge tests and an explicit v0.03 diagnostic-discovery check;
2. inherited-regression gate — `cargo test --all-targets`.

The repaired v0.03 gate executed successfully with:

- v0.01: `4/4`
- v0.02: `6/6`
- v0.03: `10/10`
- explicit frozen diagnostic: `1/1`
- inherited full suite: PASS

GitHub Actions run: `33166350973`.

## Claim ceiling

v0.03 supports three bounded claims:

1. plain learned-edge composition does not infer a withheld relation;
2. a CF-LM-015-style member abstraction can causally transfer an unseen source member into already learned target consequences;
3. naive target-family pooling can make a withheld target nonzero, but does not select the correct aligned target and therefore does not establish structural generalization.

It does not establish semantic abstraction, grammar induction, natural-language competence, or general reasoning.

## Next experiment

The next experiment should attack the missing capability directly: **role-preserving relational binding**.

The system needs a learned structure capable of preserving the correspondence

`A_i -> B_i -> C_i`

across `i`, rather than either memorizing individual edges or broadcasting to every member of a target family.

A meaningful next positive result would require `B3 -> C3` to become teacher-off positive while `B3 -> C1` and `B3 -> C2` remain suppressed, with `Psi[B3,C3] = 0` and a surgical binding-ablation collapsing only the inferred aligned continuation.
