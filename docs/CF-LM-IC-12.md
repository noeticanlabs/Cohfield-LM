# CF-LM-IC-12 — Experience-Learned Context-to-Abstraction Applicability

Status: preregistered contract for CF-LM-013. No executable PASS is claimed by this document.

## 1. Purpose

CF-LM-012 demonstrated that an organism can recognize a surface context and infer which already-assessed abstraction profile to use without receiving that profile's identity. Its inference rule, however, was still supplied by the designer:

\[
S(P\mid K)=\sum_{s\in projection(P)} c_s(K).
\]

CF-LM-013 tests the next narrower capability: whether context-to-profile applicability can itself be acquired from experience and generalized to held-out contexts.

## 2. Architectural boundary

This contract introduces no CohBit primitive and does not redefine State, Action, Transition, Atomic Transition, CohAtom, CohField, CohBit, or CohTrace.

The new learned applicability structure is language-domain State. It is not canonical Evidence, Verification, semantic truth, policy, admissibility, authority, execution, commitment, or trace.

The contract preserves:

\[
\text{equivalence assessment}
\neq
\text{context recognition}
\neq
\text{applicability learning}
\neq
\text{profile selection}.
\]

CF-LM-013 is explicitly a supervised applicability-learning experiment. The applicability experience supplies a profile identity during acquisition. The held-out inference request supplies no profile identity. This experiment does not claim autonomous consequence-grounded discovery of applicability.

## 3. Parent evidence

Verified parent:

`CF-LM-012` evidence commit `157c2aad9111eb3c83e812643431b4e54fb60508`, 147/147 local tests.

V1-V5 and CF-ACP semantics remain unchanged.

## 4. Frozen assessed abstraction profiles

Reuse exactly:

\[
P_{AB}=\langle[A,B],4,10^{-12}\rangle
\]

with only `C/D` consequence-equivalent, and:

\[
P_{BC}=\langle[B,C],4,10^{-12}\rangle
\]

with no nontrivial consequence-equivalent pair.

Assessment history remains the source of truth for each profile's equivalence relation.

## 5. Context representation

For non-empty surface cue

\[
K=[s_1,\ldots,s_n],
\]

recognition produces normalized symbol activity

\[
c_i(K)=\frac{\#\{s_j=i\}}{n}.
\]

This is the same representation already verified in CF-LM-012.

## 6. Learned applicability experience

A supervised applicability episode binds the currently recognized context to one already-assessed profile. Each episode appends an immutable language-State record containing at least:

- applicability epoch;
- context epoch;
- profile;
- context activity used for learning.

Applicability acquisition must not:

- mutate X;
- mutate Theta;
- mutate sequential relations;
- mutate abstraction assessment history;
- select a profile.

## 7. Frozen acquisition set

The training set is intentionally chosen so the learned mapping contradicts the old CF-LM-012 projection-overlap heuristic.

For `P_AB`:

- `T_AB1 = [C,C,C,D]` -> `[0,0,0.75,0.25]`;
- `T_AB2 = [C,C,D,D]` -> `[0,0,0.50,0.50]`.

For `P_BC`:

- `T_BC1 = [A,A,A,D]` -> `[0.75,0,0,0.25]`;
- `T_BC2 = [A,A,D,D]` -> `[0.50,0,0,0.50]`.

No held-out context may participate in acquisition.

## 8. Derived applicability prototype

For profile `P`, derive its applicability prototype only from applicability history:

\[
\mu_P=\frac{1}{N_P}\sum_{i=1}^{N_P} c(K_i).
\]

Frozen prototypes are therefore:

\[
\mu_{AB}=[0,0,0.625,0.375]
\]

and

\[
\mu_{BC}=[0.625,0,0,0.375].
\]

The prototype is derived structure, not a second mutable source of truth.

## 9. Learned inference rule

For current recognized context `K`, compute for every profile represented in applicability history:

\[
d(P\mid K)=\lVert c(K)-\mu_P\rVert_2.
\]

The inference request carries no profile identity.

Frozen decision thresholds:

\[
d_{max}=0.50,
\]

\[
\Delta_{min}=0.25.
\]

The profile with unique minimum distance may be selected only when:

\[
d_{min}\le d_{max}
\]

and

\[
d_{runnerup}-d_{min}>\Delta_{min}.
\]

Ties, insufficient margin, unsupported contexts, or absence of applicability experience fail closed.

The learned inference implementation must not read `profile.projection` when computing applicability distance.

## 10. Held-out generalization contexts

### K_C

`[B,C,C,D]` -> `[0,0.25,0.50,0.25]`.

Frozen distances:

\[
d(P_{AB}\mid K_C)=0.30618621784789724,
\]

\[
d(P_{BC}\mid K_C)=0.8477912478906585.
\]

Expected learned selection: `P_AB`.

The old CF-LM-012 rule would score this context `P_AB=0.25`, `P_BC=0.75` and therefore choose the opposite profile. This inversion is a required control.

### K_A

`[A,A,B,D]` -> `[0.50,0.25,0,0.25]`.

Frozen distances:

\[
d(P_{AB}\mid K_A)=0.8477912478906585,
\]

\[
d(P_{BC}\mid K_A)=0.30618621784789724.
\]

Expected learned selection: `P_BC`.

The old CF-LM-012 rule would choose `P_AB`; the learned result must therefore again invert the prior designer rule.

## 11. Fail-closed contexts

Ambiguous midpoint:

\[
K_{tie}=[A^5,C^5,D^6]
\]

with activity `[0.3125,0,0.3125,0.375]` and equal frozen distances:

\[
0.4419417382415922
\]

to both prototypes. It must fail as ambiguous.

Unsupported context:

`K_none=[B,B]` -> `[0,1,0,0]` with frozen distance

\[
1.2374368670764582
\]

to both prototypes. It must fail as unsupported.

## 12. Frozen transfer consequence

As in CF-LM-012, teach only `C->A` for eight isolated episodes while no profile is selected.

Frozen learned sequential relation:

\[
\Psi[C,A]=0.5579844028434426,
\]

with

\[
\Psi[D,A]=0.
\]

Then:

- held-out `K_C` must infer learned `P_AB`, and a D probe must produce `A_step2 = 0.011159688056868854 +/- 1e-9`;
- held-out `K_A` must infer learned `P_BC`, and D-probe A consequence must remain at floor `<=1e-12`;
- `K_C` again must restore the identical original transfer trajectory without reassessment, applicability retraining, or sequential relearning.

## 13. Claim ceiling

PASS may support only:

> Under the frozen finite language-domain carrier, supervised context-to-profile applicability experience can be stored as persistent State, generalized to held-out contexts through a learned prototype relation, override the prior designer-supplied projection heuristic, and causally control which stored abstraction participates in later continuation.

PASS does not establish:

- semantic understanding;
- autonomous discovery of the correct applicability relation;
- consequence-grounded self-supervision;
- universal contextual generalization;
- policy, authority, execution, commitment, or CohTrace rights.

## 14. Freeze rule

After the first executable gate begins, do not change without versioned amendment:

- P_AB / P_BC definitions;
- acquisition cues and profile bindings;
- held-out cues;
- context normalization;
- prototype mean rule;
- Euclidean distance;
- maximum distance 0.50;
- minimum margin 0.25;
- transfer training schedule;
- transfer thresholds and regression values;
- V1-V5 parameters.
