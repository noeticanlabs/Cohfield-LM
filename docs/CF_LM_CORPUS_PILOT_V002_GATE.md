# CF-LM Corpus Pilot v0.02 Execution Gate

v0.02 is an unresolved experimental parent for v0.03.

Before any v0.03 result may be interpreted, v0.02 must be wired into the crate, compiled, tested, and run against the exact frozen Navier–Stokes CFLM packs inherited from v0.01.

Required evidence:

- exact dataset-pack hashes unchanged from v0.01;
- true, shuffled-target, rotated-prompt, boundary-only, path-ablation, and untrained controls;
- validation and test results;
- deterministic replay;
- no validation/test adaptation;
- exact source commit and CI receipt.

A v0.03 branch may be prepared before this gate completes, but it is preregistered only and must not be described as evidence superseding v0.02.