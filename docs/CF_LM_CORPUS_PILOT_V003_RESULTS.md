# CF-LM Corpus Pilots v0.02-v0.03 — Navier-Stokes Results

Status: **EXECUTED — PREREGISTERED POSITIVE GATE NOT MET**

Workflow: `CF-LM Corpus Pilots v0.02-v0.03 CI`
Run: `33416826462`
Head at execution: `335f6d5a1e9b8f7630f79a0e05ad19f8a6454def`
Artifact digest: `sha256:9f3e2728caa875e45df311c84a4d2f7a98931c7a2610ebf4dbf6400a1cac8ce5`

The exact frozen Navier-Stokes CFLM packs inherited from v0.01 were recovered and hash-verified before execution:

- train: `4d0ec2bea1b856fe513f1af5469f0757d12245a1e5fb163693d275420e872fe5`
- validation: `e97b96d4bb26044266759f9797eac0facfe033f976fd095e06600b295a81568b`
- test: `8324b3f9aa978f7959f4363ece7378480d2ba9c87e53382c236e649b78d06b64`

## v0.02 — explicit order-2 path history

At epoch 1:

| Metric | Validation | Test |
|---|---:|---:|
| Mean correct activation | 0.00923958476906277 | 0.01021100110674532 |
| Mean rank | 20.34782608695652 | 19.63888888888889 |
| Top-1/tied | 0.0 | 0.0 |
| True vs shuffled pairing delta | -0.00018362304133242 | +0.00215511243201979 |
| Prompt-pair delta | +0.00002355257721440 | +0.00001908181114909 |
| Boundary delta | +0.00000324022788244 | +0.00000330929581297 |
| Path ablation delta | +0.00426971425756333 | +0.00503890505028691 |

The path mechanism is causally consequential: disabling it reduces target activation on both validation and test. However, the preregistered positive gate required true-vs-shuffled pairing advantage to be positive on both validation and test. Validation is negative, so v0.02 does not PASS.

## v0.03 — compressed trajectory trace

At epoch 1:

| Metric | Validation | Test |
|---|---:|---:|
| Mean correct activation | 0.00694677803172599 | 0.00728484441220099 |
| Mean rank | 23.13043478260870 | 22.36111111111111 |
| Top-1/tied | 0.0 | 0.0 |
| True vs shuffled pairing delta | -0.00107359984379629 | +0.00000962946927951 |
| Prompt-pair delta | +0.00006921958182129 | +0.00007817447801980 |
| Boundary delta | +0.00032870035697410 | +0.00034042360054103 |
| Trace ablation delta | +0.00197690752022655 | +0.00211274835574258 |

The trajectory trace is also causally consequential, but it fails the same preregistered pairing criterion on validation and provides almost no true-vs-shuffled pairing advantage on test. v0.03 therefore does not PASS.

## Exposure sweep

Epochs 1, 4, and 16 produced the same reported holdout metrics for both mechanisms to printed precision. More exposure again does not repair the conditional-discrimination boundary.

## Comparative interpretation

The experiment separates two findings that should not be conflated:

1. **History is computationally consequential.** Both v0.02 path ablation and v0.03 trace ablation measurably reduce target activation.
2. **History alone is not sufficient for robust conditional discrimination under the present learning/evaluation law.** Neither mechanism achieves positive true-vs-shuffled pairing advantage on both frozen holdouts, and neither achieves nonzero Top-1 performance.

On this finite task, explicit order-2 paths outperform the compressed trajectory trace on mean rank and test pairing advantage. This is comparative evidence for this benchmark only; it does not establish that explicit paths are generally superior.

## Claim ceiling

These results do not establish semantic understanding, formula comprehension, mathematical reasoning, language competence, or generalization beyond the frozen Navier-Stokes classification task. The supported result is narrower: short-history mechanisms are causally active, but the current adaptation law still fails the preregistered paired-discrimination gate.
