# CF-LM-IC-03 — Context-General Consequence Equivalence Contract

Status: **Draft v0.1 — preregistered before CF-LM-004 implementation**

Parent evidence:

- CF-LM-001 ordered-exposure plasticity: `f52641e68f34377e40aab7fc1be4293dcf113e93`
- CF-LM-002 two-hop composition: `a0c5afe8189b3d42128e72e375ab3b2f2100fb91`
- CF-LM-003 observer-relative equivalence: `c4d94cf480c745fc378dbb7de7f447b42eb163d5`

## 1. Purpose

CF-LM-IC-03 strengthens observer-relative equivalence into a context-general consequence-equivalence test without defining semantic equivalence.

The contract tests whether two exact-different learned relational States can preserve the same declared external continuation consequences across multiple shared contexts and shared interventions while remaining distinguishable to a richer observer.

This extends existing domain-relative observational/effect-equivalence machinery. It does not create a new primitive and does not grant identity substitution.

## 2. Canonical firewall

The implementation MUST preserve:

`exact equality != observer equivalence != consequence equivalence != semantic equivalence`.

A PASS MUST NOT be interpreted as proof that the two States have the same meaning.

No consequence-equivalence relation may replace exact State, CohAtom, transition, commitment, receipt, or CohTrace identity.

## 3. Different internal path requirement

The two target histories MUST generate exact-different `Psi_L` configurations representing different internal two-hop routes from `A` toward `B`.

For CF-LM-004 the intended route pair is:

`A -> C -> B`

versus

`A -> D -> B`.

Direct `A -> B` adaptation MUST remain absent to the declared numerical floor.

## 4. Matched exposure requirement

The paired histories MUST have identical per-symbol exposure counts and use identical model parameters, initial State, adaptation law, and exposure budget.

Simple frequency difference must not explain any result.

## 5. Consequence observer

Define a declared observer `O_AB` whose response projection retains only coordinates `A` and `B` while the underlying model State and dynamics retain all four coordinates.

The observer may vary shared surface contexts, but must apply the same context family to both States.

Observer projection is an experimental observation choice, not a mutation or compression of the State.

## 6. Context family

The context family MUST contain multiple non-identical shared probes. CF-LM-004 freezes:

- `A`
- `B`
- `AB`
- `BA`

Each context is followed by four autonomous zero-input continuation steps.

The projected response records `A/B` coordinates after every driven and autonomous step.

## 7. Shared intervention family

The equivalence claim MUST survive more than the unperturbed configuration.

CF-LM-004 freezes three intervention profiles applied identically to both exact-different States:

1. identity intervention;
2. halve every outgoing relational coefficient from `A` (`Psi[A][*] *= 0.5`);
3. halve every incoming relational coefficient to `B` (`Psi[*][B] *= 0.5`).

These interventions intentionally affect the two-hop routes while preserving the `C <-> D` symmetry of the paired States.

## 8. Nondegeneracy requirement

The consequence observer MUST be responsive to the declared interventions. A pair of States is not considered context-general equivalent merely because the observed carrier is always zero or insensitive.

At least one frozen intervention must change the projected consequence-family response by more than the preregistered nondegeneracy threshold when compared with baseline within each State.

## 9. Consequence-equivalence requirement

For every frozen intervention profile `I_k`, let `R_AB(I_k,z)` be the concatenated projected A/B consequence response across the full frozen context family.

Require:

`d(R_AB(I_k,z_C), R_AB(I_k,z_D)) <= epsilon_floor`

for every frozen intervention profile.

This establishes equivalence only for the declared consequence family.

## 10. Rich-observer discrimination

A richer observer retaining all four coordinates under at least one frozen context MUST distinguish the same State pair above a preregistered threshold.

This prevents consequence equivalence from being mistaken for exact State equality.

## 11. Claim ceiling

A PASS may support only:

> Two exact-different learned relational paths can preserve the same declared A/B continuation consequences across multiple shared contexts and shared path-sensitive interventions while remaining distinguishable under a richer observer.

A PASS does not establish:

- semantic equivalence;
- lexical synonymy;
- paraphrase equivalence;
- natural-language meaning;
- grammar induction;
- reasoning;
- substitution rights in governed identity.

## 12. Freeze rule

No frozen history, context, observer projection, intervention, threshold, response metric, continuation depth, or model parameter may change after observing a failed local gate without a versioned amendment.
