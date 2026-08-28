# CF-LM Teacher Bridge v0.04

## Status

**PASS — bounded role-preserving structural binding**

Branch: `agent/cf-lm-teacher-bridge-v004`

v0.04 follows the v0.03 boundary result. v0.03 showed that plain composition cannot infer a missing edge, CF-LM-015-style member abstraction can carry learned member consequences to another member, and a pooled target mechanism reaches the withheld target only by broadcasting nonspecifically across the whole target family.

v0.04 therefore asks a narrower question:

> Can CF-LM preserve which target corresponds to an active source and use that learned structural correspondence to cross a withheld relation without storing the withheld direct edge?

## Frozen visible curriculum

The teacher supplies these isolated visible pair episodes for 64 epochs:

```text
A1 -> B1
A2 -> B2
A3 -> B3

A1 -> C1
A2 -> C2
A3 -> C3

B1 -> C1
B2 -> C2
```

The test relation is deliberately withheld:

```text
B3 -> C3
```

`C3` is therefore not an unseen symbol. It is visible only through the independent anchor `A3 -> C3`. The experiment tests transfer of a relation to a structurally identified target, not invention of an unseen target identity.

## Mechanism

v0.04 keeps the frozen v0.03 plain relational substrate for `Psi` learning. It does not add a new update rule to `Psi`.

Known source and target families (`B_FAMILY`, `C_FAMILY`) remain designer-supplied in this bounded experiment. Pair identity is not supplied.

For every candidate B/C pair, v0.04 derives an affinity from the cosine overlap of their learned incoming-relation signatures:

```text
affinity(b,c) = cosine(Psi[:,b], Psi[:,c])
```

The global B->C schema strength is derived from the two visible B->C examples:

```text
binding_gain = mean(Psi[B1,C1], Psi[B2,C2])
```

During teacher-off continuation, the structural-binding arm adds:

```text
x_next[c] += relational_gain * binding_gain * affinity(b,c) * x[b]
```

Thus v0.04 requires two independently learned ingredients:

1. structural correspondence identifying which C target matches a B source;
2. visible evidence that a B->C relation exists at all.

## Frozen numerical diagnostics

After the frozen curriculum:

```text
Psi[A1,B1] = 0.21828695760528818
Psi[A2,B2] = 0.22728754436202436
Psi[A3,B3] = 0.2366592506893215

Psi[A1,C1] = 0.2464173788935043
Psi[A2,C2] = 0.25657786223813445
Psi[A3,C3] = 0.26715729096015667

Psi[B1,C1] = 0.27817293935876375
Psi[B2,C2] = 0.2896427940012118
Psi[B3,C3] = 0.0

binding_gain = 0.2839078666799878
```

The learned B/C affinity matrix is diagonal in the frozen case:

```text
B1 -> C1 = 0.6630889757074974
B2 -> C2 = 0.6630889757074975
B3 -> C3 = 1.0
cross-slot affinities = 0
```

## Teacher-off result

Starting from visible `B3`, with no teacher input, correction, target label, or adaptation after the initial probe:

```text
C1 = 0
C2 = 0

C3 step 1 = 0.028390786667998782
C3 step 2 = 0.028390786667998782
C3 step 3 = 0.021293090000999087
```

The withheld direct edge remains exactly:

```text
Psi[B3,C3] = 0
```

The plain matched arm remains silent on all C targets.

## Causal controls

The v0.04 gate includes all of the following:

- plain v0.04 runtime is exactly equal to the frozen v0.03 plain dynamics;
- plain composition cannot cross the withheld `B3 -> C3` relation;
- matched Plain and StructuralBinding arms share the same trained state;
- zeroing only `binding_gain` collapses `C3` without altering `Psi` or affinity;
- zeroing only `affinity[B3,C3]` collapses `C3` without altering `Psi` or schema gain;
- swapping the B3 affinity from C3 to C2 moves the response to C2;
- structural anchors without visible B->C examples produce affinity but no schema gain and no transfer;
- visible B->C schema examples without the `A3 -> C3` target anchor produce schema gain but no B3/C3 affinity and no C3 transfer;
- teacher-off probes do not mutate persistent state;
- repeated training/probing is deterministic;
- exact numerical diagnostics are frozen as an independently discoverable test.

## CI gate

GitHub Actions runs both the new experiment gate and the full inherited regression gate.

At the first green v0.04 implementation head (`51ca37b3e9e14401bab70fa11a96fbde021fead8`):

```text
teacher_bridge_v001: 4/4
teacher_bridge_v002: 6/6
teacher_bridge_v003: 10/10
teacher_bridge_v004: 11/11
v0.03 exact diagnostic: PASS
v0.04 exact diagnostic: PASS
full cargo test --all-targets: PASS
rustfmt bridge surface: PASS
```

GitHub Actions run: `33170700658`.

## Claim ceiling

v0.04 supports this bounded claim:

> Given designer-supplied source and target families, CF-LM can combine a learned global relation schema with learned incoming-relation correspondence to select a structurally matched held-out target (`C3`) from `B3`, while the direct persistent edge `Psi[B3,C3]` remains zero.

This is stronger than the v0.03 nonspecific target pool because the response is selective and its target can be moved causally by swapping only the learned correspondence state.

It does **not** establish:

- autonomous discovery of source/target families;
- invention of unseen symbols;
- natural-language semantics;
- grammar induction;
- unrestricted analogy;
- general abstract reasoning;
- open-domain language competence.

## Next experiment

v0.05 should remove the designer-supplied B/C family labels. The next question is whether CF-LM can derive the relevant role sets themselves from experience and then use the same correspondence-plus-schema principle on a held-out relation without being told which symbols belong to the source and target families.
