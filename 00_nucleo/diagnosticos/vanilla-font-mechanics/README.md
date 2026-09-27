# Structural font investigation

This probe measures structural counts, not elapsed time. It checks the exact four prior font hashes and face indices, each Unicode subtable's strict order, duplicate callbacks, contiguous ranges and sfnt table-directory byte formulas. It does not expose or time private production APIs. Set/range computations are diagnostic independent controls, not the actual private CoverageBuilder.

Read ../vanilla-font-mechanics.md for source anchors and qualified conclusions. results.json contains all counts; provenance.json and source-before/after.json retain exact dirty-tree/input/consumer/pin identity, toolchain, commands and executable hash. Build logs retain the offline resolution and final locked build. bin/ preserves the executable separately from mutable target artifacts. External font bytes require the inputs.json hashes.

For reproduction, use a fresh sibling directory with no previous outputs, then `PYTHONDONTWRITEBYTECODE=1 python3 run.py`. The runner verifies the preceding vanilla-comparison frozen inputs and uses its fingerprint helper read-only. No timing claims, production edits, new APIs, global formatting or hash reseal.
