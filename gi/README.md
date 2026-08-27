# GI — Grid Intelligence

This directory is the canonical GI development area on `agent/gi`.

## Current completed experimental lineage

GI releases completed outside this repository currently run from v0.01 through v0.25.

GI v0.26 is the next research release and is **not yet implemented**. Therefore this branch must not represent v0.26 as complete until its full implementation is committed here.

## Source-completeness rule

A GI release is not considered repository-complete until the branch contains, at minimum:

- the complete executable source tree for that release;
- every inherited source module needed to run it without relying on an unavailable prior release archive;
- the release experiment runner;
- tests and acceptance criteria;
- specification and release notes;
- frozen evidence/results used to support the release claim;
- information-boundary and scientific-boundary documentation.

For GI v0.26 specifically, the release must be self-contained on `agent/gi`. A summary, patch-only commit, generated result file, or external ZIP alone is not sufficient.

## Scientific boundary

GI is an experimental Grid Intelligence architecture. Completed GI evidence does not establish semantic understanding, general reasoning, autonomous goals, or general intelligence unless a future experiment independently supports such claims.
