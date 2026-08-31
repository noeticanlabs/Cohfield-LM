# CF-LM Corpus Pilot v0.02 — History/Path-Conditioned Continuation

Status: experimental protocol, not a completed capability claim.

## Parent control

v0.02 branches from the exact Navier–Stokes repo-native v0.01 replay. The parent result is retained unchanged as the matched first-order control.

The v0.01 boundary was: increasing exposure from 1 to 4 to 16 epochs did not materially change held-out continuation metrics. Therefore v0.02 changes representational order while holding the visible corpus task and control philosophy fixed.

## Scientific question

Can a persistent relation law conditioned on a short visible-byte history distinguish formula contexts that a first-order predecessor/current relation cannot?

This is not a claim of semantic understanding, theorem proving, or mathematical reasoning.

## Minimal intervention

Parent v0.01 adapts a first-order relation

    psi[b_(t-1), b_t].

v0.02 adds an order-2 path relation

    chi[b_(t-2), b_(t-1), b_t].

Only visible bytes may update chi. Record boundaries reset transient predecessor history exactly as in v0.01. Validation/test targets remain withheld from adaptation.

At continuation time the next field receives both first-order and path-conditioned contributions:

    x' = beta*x + g1*(x psi) + g2*P(x, h; chi) + input.

The implementation must expose g2 and permit g2=0 as an exact path-ablation control.

## Frozen comparisons

1. v0.01 first-order parent.
2. v0.02 true training with path relation enabled.
3. v0.02 shuffled-target training.
4. v0.02 rotated-prompt evaluation.
5. v0.02 answer-boundary-only evaluation.
6. v0.02 path ablation with the same learned first-order state.
7. Untrained v0.02 state.

## Acceptance

A positive result requires reproducible held-out improvement over the matched v0.01 parent and controls. At minimum:

- true pairing advantage must be positive on validation and test;
- prompt-conditioned advantage must be positive on validation and test;
- path ablation must reduce the measured advantage;
- no validation/test adaptation is permitted;
- deterministic replay must reproduce the result and hashes.

Failure of any condition is informative and must not be relabeled as language competence.

## Dataset lock

Use the exact Navier–Stokes CFLM v0.01 packs and hashes inherited from the parent branch. Do not regenerate splits or alter visible task formatting while evaluating this intervention.

## Evidence boundary

A PASS would establish only that short visible history adds causal predictive utility on this finite governed corpus under the specified continuation law. It would not establish semantics, general reasoning, language competence, or generalization outside the frozen task.