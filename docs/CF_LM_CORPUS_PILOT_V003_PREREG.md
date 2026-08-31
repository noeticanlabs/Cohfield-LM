# CF-LM Corpus Pilot v0.03 — Trajectory-Trace Conditioning

Status: preregistered successor; no result claim.

## Motivation

v0.01 is first-order predecessor/current adaptation. v0.02 adds an explicit order-2 path relation. v0.03 tests whether a compressed evolving history state can carry useful context without enumerating longer discrete byte paths.

## Scientific question

Can a bounded trajectory trace of prior visible activity causally improve held-out continuation beyond both the first-order parent and the fixed order-2 path mechanism?

## Minimal intervention

Introduce a transient trajectory trace h_t over the visible byte state:

    h_(t+1) = lambda_h * h_t + (1 - lambda_h) * x_t

with 0 <= lambda_h < 1.

Add a persistent trace-to-successor relation omega learned only from training-visible transitions:

    omega <- decay(omega) + eta * outer(h_t, e(b_(t+1))).

Continuation becomes:

    x_(t+1) = beta*x_t + g1*Psi(x_t) + g_h*(h_t Omega) + u_t.

The trace resets at record boundaries. It is not copied across examples. No tokenizer, embedding model, hidden teacher representation, target leakage, or validation/test adaptation is allowed.

## Matched controls

1. v0.01 first-order parent.
2. v0.02 fixed order-2 path parent.
3. v0.03 trajectory-trace enabled.
4. v0.03 trace contribution ablated at evaluation with learned first-order state unchanged.
5. v0.03 history trace time-shuffled during training while preserving visible byte marginals.
6. shuffled training targets.
7. rotated holdout prompts.
8. boundary-only prompts.
9. untrained state.

## Acceptance

A positive v0.03 result requires all of the following on the frozen validation and test splits:

- positive true-vs-shuffled pairing advantage;
- positive actual-vs-rotated-prompt advantage;
- positive actual-vs-boundary-only advantage;
- measurable degradation under trace ablation;
- measurable degradation under history time-shuffle;
- reproducible advantage over the exact v0.01 baseline;
- comparison with v0.02 reported whether v0.02 passes or fails;
- no validation/test adaptation and deterministic replay.

## Complexity boundary

v0.03 must remain bounded. The trace dimension is the existing 256 visible-state channels; it must not create one state per observed sequence. Sparse storage or lazy decay may be used for Omega, but such implementation changes must preserve the declared learning law.

## Claim ceiling

A PASS would establish only that compressed visible trajectory history provides causal predictive utility for this finite governed corpus. It would not establish semantic understanding, mathematical reasoning, language competence, or general intelligence.

## Dependency rule

v0.03 may be implemented while v0.02 is unresolved, but no comparative conclusion may be drawn until the v0.02 execution gate has been completed and recorded.