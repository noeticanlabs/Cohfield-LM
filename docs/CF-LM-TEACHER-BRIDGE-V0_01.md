# CF-LM Teacher Bridge v0.01

## Status

Implementation branch: `agent/cf-lm-teacher-bridge-v001`

Purpose: establish a clean external-LLM-teacher boundary for Cohfield-LM without importing teacher weights, embeddings, logits, hidden states, or chain-of-thought into CF-LM.

The teacher supplies only a frozen visible curriculum. CF-LM updates its existing persistent relational configuration through its ordinary `evolve` and `adapt` interfaces. Evaluation is teacher-off.

## Scientific question

Can an LLM-authored curriculum teach only local language relations while CF-LM later composes those learned relations into a held-out longer continuation after the teacher has been removed?

This is a plumbing and compositional-baseline experiment. It is not a claim of semantics, grammar, open-domain language learning, or abstract rule induction.

## Frozen teacher curriculum

The v0.01 curriculum contains only three two-symbol episodes:

- `A -> B`
- `B -> C`
- `C -> D`

Each episode is repeated for 64 epochs. The full sequence `A -> B -> C -> D` is never presented as one training episode.

Between episodes, fast state `X` and local condition `Theta` are equalized while persistent relational configuration `Psi` survives. Each visible pair still passes through the existing language input/evolution boundary before the existing sequential adaptation event is applied.

## Teacher-off test

After training, the teacher is removed. CF-LM is reset only in fast/local state, receives a single visible `A`, and then receives three zero-input continuation steps.

The held-out causal target is activation of `D` at continuation step 3 despite no direct learned `A -> D` relation.

## Controls

1. **No adaptation**: identical curriculum with `psi_gain = 0` must produce zero `D` activation.
2. **Surgical middle-relation ablation**: after normal training, setting only `Psi[B,C] = 0` must collapse held-out `A -> ... -> D` activation.
3. **Teacher-off nonmutation**: evaluation must not change persistent `Psi`.
4. **No direct shortcut**: `Psi[A,D]` must remain zero after training.

## Independent numerical cross-check

The exact CF-LM V1 equations and frozen constants were reproduced independently before the Rust gate.

After 64 epochs, expected persistent relations are approximately:

- `Psi[A,B] = 0.6461059481081141`
- `Psi[B,C] = 0.6727467181467244`
- `Psi[C,D] = 0.7004859622518996`
- `Psi[A,D] = 0.0`

Teacher-off trajectory from `A`:

```text
step 0: [0.5,    0.0,                 0.0,                  0.0]
step 1: [0.25,   0.06461059481081141, 0.0,                  0.0]
step 2: [0.125,  0.06461059481081141, 0.008693313123296232, 0.0]
step 3: [0.0625, 0.04845794610810856, 0.013039969684944348, 0.0012179087616658458]
```

The held-out `D` activation at step 3 is therefore expected to be:

`0.0012179087616658458`

No-adaptation and middle-relation-ablation controls both predict exactly zero `D` activation.

## Claim ceiling

A clean Rust gate supports only this claim:

> An LLM-authored curriculum can be delivered through the CF-LM language boundary to create persistent local relations that are later composed by the existing CF-LM continuation dynamics into a held-out multi-hop consequence after the teacher is removed.

It does **not** establish:

- natural-language competence;
- semantic understanding;
- grammar induction;
- autonomous abstraction;
- general rule learning;
- learning from unrestricted LLM conversation;
- equivalence to neural distillation.

## Next experiment

v0.02 should replace the fixed four-symbol surface with a larger teacher-authored synthetic micro-language and withhold entire surface combinations. The target should be transfer across a structural relation rather than only multi-hop composition of explicitly learned local edges.

## Gate

The repository branch includes a dedicated GitHub Actions workflow running:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No executable PASS should be claimed until that gate completes successfully.
