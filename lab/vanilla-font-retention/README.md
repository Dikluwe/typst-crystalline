# Quarantined vanilla-style font retention experiment

Nonproductive public-API World adapter. No L1–L4 module imports this lab code.
Specification: reproduce the pinned vanilla CLI's separation of reset file state
from session-retained lazy font store (`typst-cli/src/world.rs` SystemWorld::reset).
Use unchanged Tekt font discovery, slot loading, metrics, shaping, layout and PDF
pipeline. Only font-store lifetime changes. Fresh file Worlds on every edited input
model invalidation conservatively; this is not the vanilla FileStore algorithm or
production watch implementation. No persistence in disk, ShapePlan port, SoA rewrite,
font suppression, metric cache retention, or new production API is implemented.

Three policies: fresh font store per compilation, retained lazy font store per
session, and retained eagerly scanned font store with its preload cost recorded.
Each process compiles six distinct prewritten source revisions; setup, pipeline and
per-cycle destruction are timed, PDF writes/hash/validation are outside intervals.
Final retained-store destruction is timed and included in the session cost. First
compile is reported separately from subsequent revisions; never hide acquisition.

Run with: executable INPUT_DIRECTORY OUTPUT_DIRECTORY POLICY, where POLICY is
fresh / retained / prewarmed. Release required. Evidence/protocol/results are under
00_nucleo/diagnosticos/vanilla-font-retention-experiment/.
