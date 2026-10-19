# Font lifecycle experiment dataset

This is a separate intervention dataset. The original `performance-baseline/` remains unchanged. `protocol.md` fixes the controlled comparison; `coverage-design.md` distinguishes the adapted range construction from literal vanilla streaming extraction. There is no persistent disk catalog and no claim of CLI watch reuse.

- `initial.json`, `state/initial.*` and `originals/**/*.snapshot`: exact incoming HEAD, dirty tree, patch and task-owned byte backups. Snapshot suffixes prevent historical Rust headers from masquerading as active consumers in the linter. Initial hashes verify their original bytes.
- `variants.json`, `probes.json`, `bin/`, `logs/build-*`, `receipts/build-*`, `state/*-sources.tar.gz`: immutable executable identities, build recipes, optimized Cargo profiles and source/input snapshots. The before executable is copied byte for byte from the original baseline; `before-build-state.json` preserves that build state. The original baseline contains its complete source/build provenance.
- `host.json`, `vanilla-provenance.json`, `embedded-fonts.json`, font digest cache and `state/*.json`: UTC times, toolchain, CPU/environment settings, pinned vanilla recipe/tree/binary, font identities and consumer/input fingerprints.
- `samples/`, `catalog/`, `receipts/`: raw warmups, admitted timing samples, PDFs/traces/stdout/stderr and contamination/fingerprint controls. Warmups never enter summaries. Rejected blocks, if any, do not enter analysis.
- `memory-results.json`, `control-results.json`, `validation/`, `output-validation.json`: untimed P1410 memory limits and document/font/text controls. Their launcher times are not performance evidence.
- `e2e-summary.json`, `phase-samples.json`, `phase-summary.json`, `catalog-summary.json`: recomputed distributions, paired differences and compatible broad phase boundaries. Catalog discovery is a composed component with a separate destruction timer, not an isolated filesystem or cmap benchmark.

Preparation and measurement commands (recorded in receipts as well):

```sh
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py build --name lazy
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py build-probe --name lazy
# Apply the interval variant only after preserving the lazy binaries/source snapshot.
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py build --name interval
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py build-probe --name interval
cargo bench --manifest-path benches/Cargo.toml --no-run --offline --locked --message-format=json
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py memory
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py controls
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py plain
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py traced
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/experiment.py catalog
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/analyze.py
python3 00_nucleo/diagnosticos/font-lifecycle-experiment/validate.py
```

Builds use release opt-level 3, debug assertions/overflow checks disabled, one Cargo job and cleared Rust flags. Run tests/builds before the timed quiet gate. Sampling requires the exact frozen final consumer/input state and the recorded executable hashes. Changing any measured consumer/input requires rebuilding and invalidates retained blocks. Restore variant source archives in an isolated checkout; HEAD alone is insufficient because the incoming tree contains preexisting edits. Reproduction on another machine requires the recorded external font bytes, fontconfig configuration and toolchain, not merely similar family names.

Do not add nested vanilla spans to their enclosing eval/layout duration. Do not sum catalog probe times with end-to-end fixture times. Broad instrumented phase means reconstruct instrumented wall time only; plain medians are a different population. External launcher RSS may inherit a Python parent floor. Desktop activity, CPU boost/thermal history and processes shorter than the monitor interval remain limitations; fingerprints before/after cannot detect an edit restored between snapshots. No global parity, documentation sanitization or performance guarantee follows from these fixtures.

Final analysis: [font-lifecycle-experiment.md](../font-lifecycle-experiment.md). Next measurements: [microbench-plan.md](microbench-plan.md), a plan only. [completion-audit.json](completion-audit.json) verifies all 33 admitted blocks, unchanged final consumers/executables, task-only scope and the original baseline manifest. `task-files.json` and `state/task-only.patch` compare with incoming bytes rather than HEAD.

`repeatability.json` distinguishes admission from timing stability. Four instrumented fixtures failed the reported round mean/median criterion; their raw results remain descriptive. No-text Tekt PDFs preserve unused standard Helvetica resources absent from vanilla, while required text/font controls pass. Complete PDF byte identity is not claimed.

`MANIFEST.sha256` seals this directory's final files, excluding itself and Python bytecode caches. Verify it read-only with `sha256sum --check MANIFEST.sha256` from this directory. The report hash is recorded separately in `completion-audit.json`. Re-running scripts that write analysis/audit snapshots changes files and requires a new evidence seal; use copies for a new collection. No verification step should modify the original baseline.
