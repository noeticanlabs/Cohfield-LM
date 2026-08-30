# GI Runtime Kernel v0.02 — Pilot Evidence Ledger

## Pilot 001 — Dynamic Primitive Population Invariant

Result: **1/7 PASS — GAP CONFIRMED**.

The inherited fixed-N runtime had no population mutation API, did not resize X/Theta/H after changing N metadata, had no topology node-resize path, no Noetican Z/Y remap, no delay-buffer population resize, and no structural-lineage checkpoint field. The no-resize control remained healthy for 100 ticks with max |X| = 0.9800501522191546.

Conclusion: N was configuration metadata rather than dynamical runtime state.

## Pilot 002 — Identity-Preserving Population Transition

Result: **10/10 PASS**.

Birth preserved original persistent identities [0,1,2,3] and created p4. Retirement of p1 preserved survivors [0,2,3,4]. X, Theta, H, u_external, u_internal, delay history, Z and Y resized coherently. Structural lineage recorded BIRTH and RETIRE events. Checkpoint/restart was byte-exact with post-resume drift 0.0. Deterministic structural replay was byte-identical.

Conclusion: identity-preserving birth and retirement were demonstrated in the tested runtime lineage.

## Pilot 003 — Dynamics-Generated Structural Proposal

Result: **10/10 PASS**.

A reference activity pressure derived from fast state, persistent condition and history generated birth/retirement proposals without fixed semantic node identities. A localized field [10,1,1,1] selected p0 for birth; moving the hotspot to [1,1,1,10] selected p3. Underuse [1,1,1,0.001] selected p3 for retirement; moving underuse selected the corresponding new target. Uniform-field and pressure-ablation controls suppressed birth. Population bounds, causal lineage, restart and deterministic replay passed.

Conclusion: runtime state can determine which structural transition is proposed under the reference policy.

## Pilot 004 — Utility-Governed Structural Survival

Result: **8/8 PASS after repairing the experimental design**.

The first design falsely rewarded isolated/harmful births because transient state splitting alone lowered the short-horizon burden. The repaired experiment required a useful birth to share continuing load and extended the evaluation horizon. No acceptance threshold was tuned around the initial result.

Final results:

- useful stressed-source birth: baseline cost 1.7308444859367218, trial cost 1.1935078451254955, utility gain +0.5373366408112263 -> COMMIT;
- isolated birth: utility gain -0.02533991969771554 -> RETIRE/REJECT;
- harmful over-coupled birth: utility gain -0.03425219470942076 -> RETIRE/REJECT;
- removing the hotspot changed gain from +0.5373366408112263 to -0.052165838252896;
- proposing birth from the wrong source produced gain -0.2534268365705221;
- complexity cost reduced structural gain;
- deterministic utility replay was byte-equivalent;
- proposal and persistence were explicitly separated as BIRTH_PROPOSAL -> COMMIT or RETIRE/REJECT.

Conclusion: in the bounded reference testbed, structural persistence can be governed by measured future consequence rather than by birth pressure alone.

## Evidence boundary

The v0.02 pilots were produced across an evolving sandbox/runtime lineage. Pilot 004 is preserved as a self-contained executable harness in this repository. The result ledger for Pilots 001–003 is historical evidence, but this integration does not claim that all four pilots were replayed against one byte-identical consolidated v0.02 implementation.

Release-complete v0.02 therefore requires a single self-contained runtime implementation plus clean replay of all applicable acceptance and falsification tests against that exact source identity.
