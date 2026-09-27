# Performance baseline data index

Analysis: [performance-baseline.md](../performance-baseline.md).
This dataset describes a dirty working tree, not HEAD alone. All accepted timings
share the recorded consumer/input fingerprint and executable identities. Timing
and inference limits, including noisy cases and one unused project font, remain
explicit in the analysis. No global parity or sanitization conclusion is made.

| Artifact | Contents |
|---|---|
| `protocol.md`, `benchmark-audit.md` | Admission rules, exact options, timed boundaries, cache/destruction/setup audit |
| `host.json`, `ambient-paths.json`, `machine-control-state.json` | Machine, toolchain, CPU/governor/boost, relevant environment |
| `vanilla-provenance.json`, `vanilla-rustc-cache.json` | Verified pin/tree/binary, source blob comparison, ratified recipe |
| `artifacts.json`, `build-state.json`, `consumers-inputs.tar.gz` | Optimized artifact profiles/SHA-256, exact source/input identities and archive |
| `fixtures/`, `fonts/`, `embedded-fonts.json`, `font-selection-controls.json` | Qualified identical inputs, font bytes/identities, licenses and candidate controls |
| `state/`, `receipts/`, `logs/` | Before/after HEAD/fingerprints, expanded dirty/untracked list, diff-stat, content-addressed patches, commands/times and workload telemetry |
| `criterion/` | Accepted/raw Criterion benchmark descriptors, batch iteration counts/times, estimates/CIs; rejected whole-target records may also exist and are excluded by receipts |
| `micro-summary.json`, `micro-results.md` | Every accepted case's three estimates, distributions and repeatability flags |
| `pipeline-filter-plan.json`, `content-filter-plan.json` | Complete existing Criterion inventories and group-filter plans |
| `e2e/plain/`, `e2e/instrumented/` | Per-invocation commands, CPU/wall/RSS/context switches, stdout/stderr, PDFs and vanilla traces/Tekt counters; warmup outputs also retained |
| `e2e-summary.json`, `phase-samples.json`, `phase-summary.json` | Paired distributions and compatible broad phase arithmetic |
| `font-trace-summary.json`, `measurement-window.json` | Nested font-discovery spans and admitted observation window |
| `validation/`, `output-validation.json` | Untimed Poppler text/font/page/image controls for each plain fixture/round |
| `qualification/`, `rejected/` | Unsuccessful math/font/flow-shape controls and rejected/incomplete preparation/measurement records; never pooled |
| `driver-revisions/`, `micro-ordering-v2.json` | Exact additional scheduling-driver versions, also hashed before/after in each used receipt |
| `completion-audit.json` | Complete coverage, unchanged identities, phase conservation and admission checks |
| `controlled-experiments.md` | Proposed confirmation/refutation controls; not performed causal interventions |
| `phase-envelopes.png`, `phase-envelopes.svg`, `plot.py` | Standalone scientific figure and source |
| `packed-records.json`, `MANIFEST.sha256` | Lossless gzip storage index and complete dataset/report SHA-256 manifest |

Large `state/*.json` records are stored as `.json.gz`. `gzip -dc` recovers the
exact captured bytes; `packed-records.json` provides both original and stored
hashes. Built/initial/final snapshots, dirty lists and diff-stat remain directly
readable. Compression occurs only after all measurements.

Reproduce on the same source/font/machine environment using the protocol.
`run.py build` must precede sampling. `finish_micro.py` schedules the six whole
targets and all existing pipeline/Content groups in three rounds; it does not
change benchmark code. `collect.py e2e` and `collect.py e2e --instrumented` resume
only isolated, matching blocks. `analyze.py`, `validate.py`, `verify_results.py`
and `plot.py` consume stored outputs. Existing accepted directories are retained
on resume; use a separate copy of this diagnostic directory for a new dataset.
External system-font files are fingerprinted but not bundled; matching their
identities is required for exact environmental reproduction.

Run `sha256sum --check MANIFEST.sha256` from this directory to verify storage
integrity. The manifest also includes the sibling analysis document.
