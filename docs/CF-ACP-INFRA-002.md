# CF-ACP-INFRA-002 — Selective Adaptation, Retention, and Counterfactual Result Closure

Status: **Draft / local conformance pending**

Parent evidence: `CF-ACP-INFRA-001` local gate PASS at `a48207168b7aaa5488b94cfb85ccbce2cf326275`.

## 1. Purpose

CF-ACP-INFRA-002 carries the infrastructure specialization from the v0.01-v0.06 causal/geometry reproduction into the v0.07-v0.10 selective-adaptation and counterfactual-retention results.

This milestone preserves a strict distinction between:

1. **mechanism replay** — recomputing an experimental outcome from the original frozen inputs and laws; and
2. **result-record conformance** — preserving reported experimental outcomes and verifying downstream arithmetic/ordering without pretending the missing raw ledger has been reconstructed.

The current repository supports exact executable reconstruction of the retention mathematics and result-record conformance. It does **not** currently contain enough raw material to claim exact historical replay of every v0.07-v0.10 experiment.

## 2. Canonical firewalls

This milestone does not redefine CohBit primitives.

The following remain distinct:

- external task utility != Coh valuation;
- endogenous burden != utility;
- counterfactual survival != admissibility;
- recovery-margin score != policy;
- retention preference != authority;
- simulated future != execution;
- persistence != commitment.

No external evaluation score is an input to the retention utilities implemented here.

## 3. v0.07 — selective functional adaptation

Frozen reported primary record:

- baseline utility: `0.186150`
- useful: `0.221769`
- neutral: `0.189324`
- harmful: `0.160680`

Ordering:

`U_useful > U_neutral > U_harmful`

with useful above baseline and harmful below baseline.

The reconstructed post-history infrastructure matrices already present in INFRA-001 have Frobenius norms matching the reported useful and harmful initial v0.08 norms:

- useful approximately `2.1867`
- harmful approximately `2.1810`

The neutral matrix and the full original v0.07 external-evaluator definition are not currently present in the repository/source set. Therefore this milestone preserves the v0.07 result as **result-record conformance**, not exact evaluator replay.

## 4. v0.08 — endogenous persistence regulation / selective retention FAIL

Frozen fixed forgetting baseline:

`rho_base = 0.035`

for 30 steps gives

`(1-rho_base)^30 = 0.343415...`

Reported endogenous retention ratios:

- useful: `0.372346`
- neutral: `0.375176`
- harmful: `0.374891`

All exceed fixed forgetting, establishing persistence modulation.

But the ordering is

`neutral > harmful > useful`,

so the preregistered usefulness-aligned selective-retention hypothesis remains **FAIL**.

INFRA-002 intentionally locks this negative result. No coefficient retuning is permitted to make v0.08 appear successful.

The original burden-to-forgetting mapping coefficients are not currently available in the repository/source set, so exact historical burden-law replay is not claimed.

## 5. v0.09 — binary counterfactual survival

Frozen reported result:

`18/18 > 16/18 = 16/18`

for useful, neutral, harmful respectively.

Interpretation:

- useful separated;
- neutral and harmful remained indistinguishable under threshold survival counting.

The original frozen perturbation generator and rollout ledger are not currently available in this repository/source set. Therefore the survival counts are preserved as result-record evidence, not regenerated here.

## 6. v0.10 — signed recovery-margin discrimination

Frozen endogenous score:

`m_j = 1 - r_j/r_max`, with `r_max = 0.20`.

Frozen reported means:

- `Q_useful = 0.411920`
- `Q_neutral = 0.407113`
- `Q_harmful = 0.389890`

Therefore:

`Q_useful > Q_neutral > Q_harmful`.

The useful-neutral gap is:

`0.004807`.

Binary survival is the threshold projection

`C_j = 1[m_j >= 0]`,

so thresholding discards distance-to-boundary information already preserved by the signed margin.

The raw per-rollout margins are not currently present in this repository/source set. Exact re-aggregation from the historical rollout ledger is therefore pending recovery of those records.

## 7. v0.10 retention reconstruction

The reported 30-step retention values are:

- useful: `0.5455`
- neutral: `0.4461`
- harmful: `0.2146`

These are exactly consistent to reported rounding with the declared affine score-to-forgetting reconstruction:

- highest score -> forgetting rate `0.02`
- lowest score -> forgetting rate `0.05`
- intermediate scores -> linear interpolation

followed by

`Retain(Q) = (1-rho(Q))^30`.

This reconstruction is implemented by `AffineForgettingProfile` in `src/profiles/infrastructure_selection.rs`.

It is a downstream infrastructure profile, not a universal CF-ACP rule and not CohBit valuation.

## 8. Evidence status

### Executable reconstruction supported now

- fixed multiplicative forgetting;
- relational-configuration norm scaling under forgetting;
- affine v0.10 score-to-forgetting mapping;
- v0.10 reported 30-step retention values;
- fail-closed score-domain behavior.

### Result-record conformance supported now

- v0.07 external utility ordering;
- v0.08 persistence-modulation success and selective-retention failure;
- v0.09 binary survival count ordering;
- v0.10 recovery-margin three-way ordering.

### Historical replay blocked by missing raw evidence

- full neutral post-history `Psi` matrix;
- exact v0.07 task utility/evaluator formula and holdout ledger;
- exact v0.08 burden-to-forgetting mapping coefficients/ledger;
- frozen v0.09 perturbation-generation specification and rollouts;
- raw v0.10 per-rollout recovery margins.

These gaps must not be silently filled by reverse engineering or post-hoc tuning.

## 9. Local conformance gate

Run from `agent/cf-acp-infra-002`:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No experimental ordering, tolerance, or negative classification may be altered after seeing a failure without an explicit reconstruction amendment.

## 10. Exit condition

INFRA-002 may close its current scope when:

1. the local Rust gate passes;
2. the v0.08 failure remains preserved;
3. v0.10 retention reconstruction matches the frozen report;
4. missing historical replay inputs remain explicitly recorded rather than inferred.

Full v0.07-v0.10 historical replay remains a separate evidence-recovery target unless the missing raw records are supplied.
