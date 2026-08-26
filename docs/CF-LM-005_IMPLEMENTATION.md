# CF-LM-005 — Contextual Substitution Preservation Implementation

Status: **Implemented — local conformance disposition pending**

Protocol parent:

- `CF-LM-IC-04`
- `CF-LM-005_PROTOCOL.md`
- contract head `3a190db46fe4d926cfed5b75da7c2ba9765bfb71`

Verified executable parent:

- CF-LM-004 evidence `1cf39d937ad2139c02aefbeaca19b2754a5ea0a1`

## Implementation scope

CF-LM-005 adds no production model code.

The experiment is implemented entirely in:

`tests/language_contextual_substitution.rs`.

It reuses the unchanged:

`CohfieldLanguageModelV1`.

The test implementation performs only downstream experimental operations:

1. reproduce the frozen CF-LM-004 source histories;
2. extract the two learned route cores without renormalizing weights;
3. insert each route into the same fresh host family `Psi[B][A] = w` for `w in {0.5,1.0,2.0}`;
4. measure the frozen two-sided A/B projected consequence family;
5. perform the explicit whole-route substitution;
6. perform two deliberately incomplete substitutions;
7. perform a route-cut nondegeneracy control;
8. compare the same host pair under a full-coordinate observer;
9. regression-check the preregistered numerical record.

## Evidence boundary

At this commit, CF-LM-005 is **implemented but not yet verified**.

No PASS is claimed until the local gate executes:

```text
cargo fmt --all
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

No frozen history, route extraction rule, host edge, host weight, context family, observer projection, threshold, continuation depth, response metric, or model parameter may change after a failed gate without a versioned amendment.

## Claim boundary

Even after a PASS, CF-LM-005 would establish only operation-specific contextual substitution preservation for the declared host family and consequence observer.

It would not establish semantic equivalence, universal congruence, identity substitution, CohAtom substitution, governance substitution, or trace substitution.