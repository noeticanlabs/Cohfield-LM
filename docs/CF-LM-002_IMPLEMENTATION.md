# CF-LM-002 — Implementation and Evidence Boundary

Status: **Implementation staged; local disposition pending**

Parent verified evidence:

- CF-LM-001: `f52641e68f34377e40aab7fc1be4293dcf113e93`

Preregistered contract/protocol branch:

- `agent/cf-lm-002-composition-contract`
- protocol frozen before implementation

## 1. Implementation scope

CF-LM-002 adds no new model primitive and changes no `CohfieldLanguageModelV1` parameter.

The experiment is implemented entirely as downstream conformance tests using the already verified model surface:

- ordered exposure through `expose`;
- comparison-state equalization through `LanguageState::equalized_from`;
- finite continuation through `AdaptiveContinuationModel::evolve`;
- direct inspection/intervention on persistent relational configuration `Psi_L`.

## 2. Frozen experiment

Target history:

`(A B D B C D)^32`

Matched-count broken-bridge history:

`(A B D C B D)^32`.

Both preserve the same per-symbol counts.

The target requires learned:

`A -> B -> C`

while direct:

`A -> C`

must remain absent to `1e-12`.

Probe:

`A -> zero -> zero`.

Observable:

- `B_1` after the first zero-input continuation step;
- `C_2` after the second zero-input continuation step.

Frozen thresholds:

- `B_1 > 0.05`
- `C_2(chain) > 0.005`
- `C_2(break) <= 1e-12`.

Surgical causal intervention:

`Psi[B][C] := 0`

with all other entries preserved.

Required result:

- `B_1` remains above `0.05`;
- `C_2` collapses to `<= 1e-12`.

## 3. Evidence boundary

No CF-LM-002 PASS is claimed by this document or implementation commit.

PASS/FAIL requires the local gate:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No frozen history, model parameter, threshold, probe depth, observable, or intervention may be changed after a failed gate without a versioned amendment.

## 4. Claim boundary

A PASS would demonstrate two-hop compositional continuation in the learned relational configuration.

It would not establish semantic equivalence, semantic understanding, grammar, reasoning, or natural-language competence.
