# CF-LM-IC-11 — Context-Conditioned Internal Abstraction Selection Contract

Status: Experimental implementation contract

Experiment: CF-LM-012 — Endogenous Context-Conditioned Abstraction Selection

Parent evidence: CF-LM-011 verified at `955b33257707c7f448b56ecdf71767b28bdcf927`.

## 1. Purpose

CF-LM-011 established that multiple incompatible profile-scoped internal consequence-equivalence assessments may coexist and may be reversibly selected without reassessment. CF-LM-012 removes the answer-specific profile identifier from the selection request.

The experiment asks whether the language organism can:

1. recognize a declared surface context;
2. compare that recognized context against all already-assessed internal consequence profiles using one generic compatibility rule;
3. infer the uniquely best-supported assessed profile without being supplied its identity;
4. make future continuation use the inferred profile;
5. fail closed when the context is unsupported or does not uniquely determine a profile.

This is a domain-specific extension of the existing language State. It does not add a CohBit primitive.

## 2. Canonical firewalls

CF-LM-012 MUST preserve:

```text
context recognition != abstraction selection
context compatibility != semantic truth
inferred profile != policy
inferred profile != authority
internal assessment record != canonical Evidence
observational equivalence != exact identity
```

The applicable profile changes the interpretation of an internal equivalence relation, so its context and selection provenance MUST remain explicit.

## 3. Versioned State

CF-LM-012 MAY introduce `LanguageStateV5` as an additive version over verified V4.

V5 MUST preserve V4:

- fast State `x`;
- local condition `theta`;
- sequential relational configuration;
- selected profile, if one exists at migration;
- append-only consequence-equivalence assessment history.

V5 MAY add domain-specific context-recognition and context-selection records inside the language State.

V1-V4 and CF-ACP semantics MUST NOT be modified.

## 4. Context recognition

A context cue is a non-empty ordered surface-symbol sequence. Recognition MUST derive a normalized activity signature:

\[
c_i = \frac{\#\{s\in cue:s=i\}}{|cue|}.
\]

Recognition MUST:

- append a context-recognition record;
- identify that record as the current recognized context;
- preserve sequential relations;
- preserve assessment history;
- preserve the currently selected profile.

Recognition alone MUST NOT select an abstraction profile.

## 5. Generic profile compatibility

For an already-assessed profile `P` with declared two-symbol projection `projection(P)`, define:

\[
S(P\mid c)=\sum_{s\in projection(P)}c_s.
\]

The rule MUST be applied uniformly to every already-assessed profile. It MUST NOT contain answer-specific branches such as `if cue contains A then choose P_AB`.

The frozen selection parameters are:

```text
minimum supported score = 0.50
minimum winning margin  = 0.25
```

A profile may be selected only when:

1. it has a complete stored assessment;
2. its score is at least `0.50`;
3. it is the unique highest-scoring assessed profile;
4. its score exceeds the runner-up by more than `0.25`.

Otherwise inference MUST fail closed.

## 6. Frozen assessed profiles

Reuse exactly:

```text
P_AB = <projection=[A,B], continuation_steps=4, epsilon=1e-12>
P_BC = <projection=[B,C], continuation_steps=4, epsilon=1e-12>
```

Their verified incompatible dispositions remain:

```text
P_AB: only C/D equivalent
P_BC: no nontrivial equivalent pair
```

Both profiles MUST be assessed before contextual inference.

## 7. Frozen context cues

The positive cues are deliberately not identical to either profile projection:

```text
K_AB = [A, A, B, D]
K_BC = [B, C, C, D]
```

Their normalized activity signatures are:

```text
K_AB = [0.50, 0.25, 0.00, 0.25]
K_BC = [0.00, 0.25, 0.50, 0.25]
```

Therefore the preregistered profile scores are:

```text
K_AB: S(P_AB)=0.75, S(P_BC)=0.25
K_BC: S(P_AB)=0.25, S(P_BC)=0.75
```

The inferred profiles MUST therefore be `P_AB` and `P_BC`, respectively, without either profile identity being passed to the inference operation.

## 8. Fail-closed contexts

Freeze:

```text
K_tie = [B, D]
K_none = [D, D]
```

For `K_tie`:

```text
S(P_AB)=0.50
S(P_BC)=0.50
```

Inference MUST fail as ambiguous and MUST NOT mutate selected profile, assessment history, sequential learning, or recognized context history.

For `K_none`:

```text
S(P_AB)=0
S(P_BC)=0
```

Inference MUST fail as unsupported and MUST NOT mutate State.

## 9. Frozen transfer experiment

Before context inference, teach only `C->A` for the same eight isolated episodes used by CF-LM-009 through CF-LM-011.

Freeze the existing sequential regression:

\[
\Psi[C,A]=0.5579844028434426.
\]

No direct `D->A` sequential relation may be learned.

After `K_AB` recognition and generic inference, a fresh D probe MUST reproduce:

\[
A_2(D)=0.011159688056868854\pm10^{-9}.
\]

After `K_BC` recognition and generic inference, the same D probe MUST remain at floor:

\[
|A_2(D)|\le10^{-12}.
\]

After recognizing `K_AB` again and re-running generic inference, the original D-probe trajectory MUST be restored without profile reassessment or additional C->A learning.

## 10. Provenance boundary

V5 SHOULD preserve append-only domain records for:

- context recognition;
- profile candidate scores;
- inferred profile selection.

These records are model State and provenance-like internal history. They are NOT canonical CohBit Evidence, Verification, Policy, Authority, Receipt, or CohTrace.

## 11. Claim ceiling

A PASS may support only:

> Given multiple previously assessed internal abstraction profiles, the organism can derive a context signature from surface context, apply a generic compatibility rule over the assessed profiles, infer a uniquely supported profile without receiving its identity, and make subsequent continuation follow that inferred abstraction; ambiguous or unsupported contexts fail closed.

A PASS MUST NOT be described as:

- semantic understanding;
- general context understanding;
- autonomous policy selection;
- authority;
- universal language intelligence;
- proof of human-like meaning.

## 12. Amendment classification

Affected section: CF-LM language-domain implementation only.

Reason: CF-LM-011 still required an external caller to name the selected profile.

Compatibility: additive V5; V1-V4 and CF-ACP remain unchanged.

Architectural consequence: context recognition and abstraction selection become distinct internal operations, and profile identity is no longer supplied by the caller during contextual inference.
