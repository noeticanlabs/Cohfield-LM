# CF-LM-002 — Two-Hop Compositional Continuation

Status: **Preregistered implementation protocol v0.1**

Parent evidence:

- CF-LM-001 verified local evidence: `f52641e68f34377e40aab7fc1be4293dcf113e93`

Parent contract:

- `CF-LM-IC-01`

No CF-LM-002 Rust implementation existed when this protocol was frozen.

## 1. Scientific question

Can the verified CF-LM adaptive continuation model compose two learned directed relations into a held-out finite continuation consequence without learning the direct source-to-target relation?

Target chain:

`A -> B -> C`

Held-out direct relation:

`A -> C`.

## 2. Claim boundary

This experiment tests two-hop compositional continuation only.

It does not test semantic equivalence, semantic meaning, grammar, reasoning, generation, or natural-language competence.

## 3. Inherited model

Use `CohfieldLanguageModelV1` unchanged:

`Psi_(t+1) = 0.98 Psi_t + 0.08 e_prev e_current^T`

`X_(t+1) = 0.50 X_t + 0.50 u_t + 0.20 Psi^T X_t`.

Use the same initial State:

`X_0 = 0`

`Theta_0 = (1,1,1,1)`

`Psi_0 = 0`.

## 4. Frozen exposure histories

Use exactly 192 surface observations per history.

Target chain history:

`H_chain = (A B D B C D)^32`.

Broken-bridge control history:

`H_break = (A B D C B D)^32`.

Both histories contain exactly:

- `A = 32`
- `B = 64`
- `C = 32`
- `D = 64`.

Thus total exposure and per-symbol frequency are matched.

`H_chain` contains learned adjacency `A -> B` and `B -> C`.

`H_break` retains `A -> B` but does not contain `B -> C`.

Neither history contains adjacent `A -> C`.

## 5. Comparison-state equalization

After exposure, construct comparison states with:

`X = 0`

`Theta = (1,1,1,1)`

while preserving each exposure-derived `Psi`.

## 6. Direct-edge exclusion

Before computing the compositional target, require:

`abs(Psi_chain[A][C]) <= epsilon_floor`

and

`abs(Psi_break[A][C]) <= epsilon_floor`.

Freeze:

`epsilon_floor = 1e-12`.

If either direct edge exceeds floor, CF-LM-002 FAILS under this protocol.

## 7. Frozen probe

Starting from an equalized state:

1. apply input `A` and record `x^(0)`;
2. apply zero input and record `x^(1)`;
3. apply zero input and record `x^(2)`.

Define:

`B_1 = x_B^(1)`

and

`C_2 = x_C^(2)`.

This timing is deliberate:

- one relational continuation step carries `A` activity toward `B`;
- the second can carry that activity from `B` toward `C`.

## 8. Frozen thresholds

Freeze:

`epsilon_comp = 0.005`

`epsilon_first_hop = 0.05`

`epsilon_floor = 1e-12`.

Required target-chain conditions:

`B_1(chain) > 0.05`

`C_2(chain) > 0.005`.

Required broken-bridge condition:

`C_2(break) <= 1e-12`.

## 9. Surgical bridge intervention

Construct:

`z_surgical = z_chain`

except:

`Psi_surgical[B][C] = 0`.

No other entry may be modified.

Require:

`B_1(surgical) > 0.05`

and

`C_2(surgical) <= 1e-12`.

This shows removal of the second edge destroys the two-hop consequence while preserving the first hop.

## 10. No-adaptation control

Repeat both histories using `CohfieldLanguageModelV1::without_adaptation()`.

After equalization and the same `A` probe, require:

`C_2(no-adapt chain) <= epsilon_floor`

and

`C_2(no-adapt break) <= epsilon_floor`.

## 11. Deterministic repeat

Two independent target-history runs from the same initial State MUST produce identical `Psi`, `B_1`, and `C_2` to numerical floor.

## 12. Preregistered analytical/numerical cross-check

Before Rust CF-LM-002 implementation, the frozen equations predict approximately:

`Psi_chain[A][B] = 0.6330194459`

`Psi_chain[B][C] = 0.6725720638`

`Psi_chain[A][C] = 0`

`B_1(chain) = 0.0633019446`

`C_2(chain) = 0.0085150239`

and for the matched broken-bridge history:

`B_1(break) = 0.0633019446`

`C_2(break) = 0`.

After surgical removal of `Psi[B][C]` from the target state, the predicted values are:

`B_1(surgical) = 0.0633019446`

`C_2(surgical) = 0`.

These cross-check values do not replace the local Rust gate.

## 13. PASS rule

CF-LM-002 passes only if all are true:

1. per-symbol counts are matched exactly;
2. `A -> C` direct relational configuration is absent to floor in both histories;
3. target history has `B_1 > 0.05`;
4. target history has `C_2 > 0.005`;
5. broken-bridge control has `C_2 <= 1e-12`;
6. surgical `B -> C` removal preserves `B_1 > 0.05`;
7. surgical `B -> C` removal collapses `C_2 <= 1e-12`;
8. no-adaptation controls remain at floor for `C_2`;
9. deterministic repeat holds.

Any failed required condition yields CF-LM-002 FAIL.

No history, parameter, threshold, probe depth, or observable may be changed after observing a failure without a versioned amendment.

## 14. Interpretation of PASS

A PASS supports only:

> Two learned directed relations in the CF-LM persistent relational configuration can compose into a held-out two-hop continuation consequence at the expected continuation depth, without a directly learned source-to-target relation.

This is compositional continuation evidence.

It is not semantic-equivalence evidence.

## 15. Next experiment boundary

Only after CF-LM-002 disposition should a later experiment test whether distinct surface paths can become observationally equivalent under a declared continuation profile, while preserving the canonical distinction:

`exact identity != observational equivalence != semantic equivalence`.
