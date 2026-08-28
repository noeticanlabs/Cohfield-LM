# CF-LM-015 — Derived Abstraction Object Formation and Member-Mediated Reuse

## Experimental question

Can the verified Cohfield language organism construct a new internal abstraction object from its own consequence-equivalence assessment, preserve that object's identity separately from formation provenance, learn a relation to the abstraction through experience with one member, and later use the abstraction to alter continuation from another member?

## Verified parent

CF-LM-014 evidence head:

`97b3792218966f063d7dd31574842f0b30f1e0ed`

Verified parent suite: 167 tests.

## Frozen source substrate

Use the established route substrate containing:

- A->C = 0.9840816505055259
- C->B = 1.0041649494954346
- A->D = 0.9840816505055259
- D->B = 1.0041649494954346

with X=0 and Theta=1.

## Frozen profiles

`P_AB = <projection=[A,B], continuation_steps=4, epsilon=1e-12>`

`P_BC = <projection=[B,C], continuation_steps=4, epsilon=1e-12>`

Under P_AB, only C/D is nontrivially equivalent.

Under P_BC, no nontrivial pair is equivalent.

## Formation protocol

1. Assess P_AB.
2. Invoke `FormDerivedAbstractions(P_AB)` without supplying a member set.
3. Require exactly one derived abstraction with identity:

`<P_AB, members=[false,false,true,true]>`.

4. Require source assessment epoch = 1 in formation provenance.
5. Require no mutation of X, Theta, sequential relations, assessment history, or selected profile.

### Identity/provenance control

Repeating formation against the same assessment must create neither a duplicate abstraction object nor duplicate formation provenance.

After a second P_AB assessment, repeating formation must:

- preserve the same abstraction identity;
- preserve one abstraction object;
- append a second formation-provenance record bound to the new assessment epoch.

### Negative formation control

Assess P_BC and invoke `FormDerivedAbstractions(P_BC)`.

Required: `NoNontrivialAbstraction` and no successor State.

## Frozen abstraction relation acquisition

From the P_AB-derived abstraction State, apply eight direct sequential adaptation experiences:

`predecessor=C, current=A`

No exposure helper is used; exactly eight adaptation events occur.

The abstraction relation recurrence is:

`w_{n+1}=0.98*w_n+0.08`, `w_0=0`.

Frozen result after eight events:

`w_abs({C,D}->A)=0.5969479096728575`.

The ordinary sequential relation must simultaneously satisfy:

`Psi[C,A]=0.5969479096728575`

and

`Psi[D,A]=0`.

## Causal abstraction probe

Activate only the derived abstraction `{C,D}` while requiring parent `selected_profile=None`.

Fresh probe:

1. equalize X=0, Theta=1;
2. drive D once;
3. run four zero-input continuation steps;
4. record A after the drive and after each zero step.

Frozen trajectory:

`[0.0, 0.029847395483642875, 0.029847395483642875, 0.022385546612732156, 0.014923697741821437]`

Regression tolerance: `1e-9`.

## Controls

### No abstraction

Apply the exact same eight C->A adaptation events to a migrated V8 State in which no derived abstraction was formed. D-probe A must remain <=1e-12.

### No abstraction relation

Form and activate `{C,D}` but do not run C->A adaptation. D-probe A must remain <=1e-12.

### Surgical abstraction-relation ablation

After successful acquisition, set only the learned `{C,D}->A` abstraction relation weight to zero.

Require:

- derived abstraction object unchanged;
- formation provenance unchanged;
- direct Psi[C,A] unchanged at 0.5969479096728575;
- Psi[D,A] remains zero;
- parent selected_profile remains None;
- D-probe A collapses to <=1e-12.

## PASS requirements

All preregistered values and controls must hold. Rust gate:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Ten new CF-LM-015 tests are planned. Expected full target if clean: `177/177`.

## Claim ceiling

This experiment tests derived abstraction-object formation and reuse in the frozen four-symbol language domain. It does not test autonomous selection among multiple invented abstractions, hierarchy, semantic truth, unconstrained concept generation, or open-domain language understanding.
