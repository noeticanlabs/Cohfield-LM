# GI Version Status

## Frozen completed baseline

- GI v0.25 — completed experimental release outside the repository and current capability baseline.

## Next release

- GI v0.26 — **not implemented yet**.

The intended v0.26 question is whether GI can form a useful primitive observable beyond merely selecting or composing complete representational features supplied by the designer.

## v0.26 repository gate

GI v0.26 must not be marked complete until `agent/gi` contains its full self-contained source and evidence package.

Required repository contents:

1. complete source modules, including inherited runtime dependencies;
2. v0.26 primitive-observable-formation implementation;
3. experiment runner;
4. acceptance tests;
5. formal experiment specification;
6. release notes and README;
7. frozen results and validation records;
8. explicit negative controls and falsification tests;
9. information-boundary audit;
10. reproducible command(s) for executing the evidence suite.

This gate is intentionally stronger than preserving only a patch or release archive: the branch itself must be sufficient to inspect and reproduce the v0.26 implementation.
