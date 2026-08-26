# CF-LM-001 — Implementation Evidence Boundary

Status: **Implementation complete; local conformance pending**

Verified parent evidence:

- CF-ACP-000: `782fc4efcd6df64a2ed06e87cd5cdac1dc01b4df`
- CF-ACP-INFRA-001: `a48207168b7aaa5488b94cfb85ccbce2cf326275`
- CF-ACP-INFRA-002: `b6f3b6a2164eaae17120e107cc03494bb52133cf`

Frozen language contract/protocol branch:

- `agent/cf-lm-000-language-contract`
- protocol head when implementation began: `410c193faceeea57d1b6d22e2fdfb5f8f1ec372b`

## Implemented profile

`src/profiles/language.rs` implements the first language-domain specialization of the existing `AdaptiveContinuationModel` contract.

The implementation contains:

- deterministic four-symbol surface boundary `{A,B,C,D}`;
- fast State role `X_L in R^4`;
- fixed local-condition role `Theta_L = (1,1,1,1)`;
- directed persistent relational configuration `Psi_L in R^(4x4)`;
- frozen exposure adaptation `Psi_(t+1) = (1-rho)Psi_t + eta e_prev e_current^T` with `rho=0.02`, `eta=0.08`;
- frozen finite continuation law `X_(t+1) = 0.50 X_t + 0.50 u_t + 0.20 Psi^T X_t`;
- frozen fresh probes `AC`, `BD`, `CA`, `DB`;
- direct relational replacement capability for the causal intervention.

It contains no learned tokenizer, next-token probability, softmax decoder, neural network, embedding model, attention layer, backpropagation, gradient descent, or external evaluation signal in adaptation.

## Preregistered conformance tests

`tests/language_plasticity.rs` implements the frozen CF-LM-001 controls:

1. deterministic one-hot surface mapping;
2. exact matched symbol counts between `H_A=(ABCD)^32` and `H_B=(ADCB)^32`;
3. fresh-probe verification;
4. nonzero exposure-induced `Psi` difference;
5. pre-intervention response distance greater than frozen `epsilon_R=0.10`;
6. direct `Psi_A -> Psi_B` replacement collapse to `epsilon_floor=1e-12`;
7. identical-history repeat-floor control;
8. no-adaptation collapse control;
9. State-role separation/equalization control.

Independent numerical cross-checking before Rust implementation produced approximately:

- `||Psi_A-Psi_B||_F = 2.6118385827`;
- `D_R(A,B) = 0.2867803345`;
- direct-replacement `D_R = 0`.

Those values are regression checks, not post-result threshold changes.

## Local gate

CF-LM-001 has no PASS/FAIL disposition until all of the following execute on the implementation branch:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Any failure must be classified before changing constants, exposure histories, probe family, response metric, thresholds, or adaptation law.

A successful gate may support only the claim frozen in `CF-LM-001_PROTOCOL.md`: ordered surface-language exposure can create persistent directed relational configuration inside the CF-ACP model that causally alters finite continuation response to fresh surface probes after fast State and local condition are equalized.

It does not establish semantics, grammar, reasoning, generation, conversation, or open-domain natural-language competence.
