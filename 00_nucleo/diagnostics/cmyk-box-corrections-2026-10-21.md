# CMYK and fractional Box calibration correction

The [pre-batch hardcode audit](hardcode-audit-2026-10-21.md) was followed by
measured L0-first corrections with agents on `codex/tekt-mature-layout`.
HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227` remains an explicitly uncommitted working tree. Exact UTC,
dirty stat/status, inputs and preserved executable are in the
[completion receipt](../../lab/results/cmyk-box-corrections-2026-10-21/qualified-final/completion.json) and
[full report](../../lab/results/cmyk-box-corrections-2026-10-21/README.md).

- CMYK now uses one pure conversion through the ratified immutable ICC asset,
  replacing the fitted cross terms and naive projection. All 961 corpus rows
  and 12 quantization-boundary probes match pinned vanilla
  `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
- Fractional Box replaces its fitted paragraph gap/ascent with actual body
  metrics and causal paragraph spacing, preserves the established horizontal
  queue protocol, and includes committed body content in its line extent.
  Twenty primary sources and eight supported supplemental/transport controls
  pass. Eight causal Box tests and every prior assertion remain.
- Full tests/build/all targets, Clippy, fmt, rustdoc/four indices, all Tekt checks,
  direct/reverse lineage and paths pass. Hash's 366 rows, R7, F/G and running
  content/Context guards pass their unchanged criteria. Five core and two infra
  ignored tests are preexisting; none were added. All lint notes remain visible.

Candidate SHA-256: `94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`. All 2,156 frozen inputs
and the executable remained unchanged through validation. The report separates
this task's changed inputs from preexisting dirty changes and preserves rejected
unified-frame and first narrow candidates.

At this historical snapshot, three nested fractional allocation/Repeat sources
remained oracle RED, with their baseline positions/counts/visible text preserved.
The later [nested allocation/Repeat correction](nested-fractions-repeat-2026-10-21.md)
remeasures and closes those three sources while preserving this CMYK/Box corpus.
Its complete matrix retains nine other RED cases and additional warning/value
presentation debts. Broader backing paint, opaque width-dependent reflow and
native SVG page templates are not generally closed. The
[broader parity plan](vanilla-parity-plan-2026-10-21.md) remains active; neither
snapshot establishes general Box or overall language parity.
