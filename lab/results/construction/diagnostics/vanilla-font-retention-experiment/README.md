# Evidence and reproduction

Run in an isolated checkout of the recorded HEAD with measured-sources.tar.gz overlaid,
the captured font/fontconfig bytes installed, and the recorded Rust toolchain. The
archive is a source overlay, not a complete toolchain/dependency distribution. Do not
modify this sealed evidence or rerun the collector in its recorded directory. Preserve
it elsewhere and use a fresh dataset directory with only protocol.md, run.py, analyze.py
and the inputs, at the same repository-relative location in the isolated checkout.
Lab Cargo manifests have absolute paths to the original workspace: update them to
that isolated checkout deliberately. Keep the locked dependencies and recorded fonts.
Capture fresh UTC, HEAD, exact git status --porcelain=v1 --untracked-files=all,
git diff HEAD --stat, rustc -Vv, cargo -V, uname -a and lscpu before a new collection.

```sh
RUSTFLAGS='' CARGO_ENCODED_RUSTFLAGS='' cargo build --release --offline --locked \
  --manifest-path lab/vanilla-font-retention/Cargo.toml --target-dir target -j 1 \
  --message-format=json \
  > 00_nucleo/diagnosticos/vanilla-font-retention-experiment/build-ready.jsonl \
  2> 00_nucleo/diagnosticos/vanilla-font-retention-experiment/build-ready.stderr
PYTHONDONTWRITEBYTECODE=1 python3 \
  00_nucleo/diagnosticos/vanilla-font-retention-experiment/run.py \
  > 00_nucleo/diagnosticos/vanilla-font-retention-experiment/run.stdout \
  2> 00_nucleo/diagnosticos/vanilla-font-retention-experiment/run.stderr
PYTHONDONTWRITEBYTECODE=1 python3 \
  00_nucleo/diagnosticos/vanilla-font-retention-experiment/analyze.py \
  > 00_nucleo/diagnosticos/vanilla-font-retention-experiment/analysis.stdout \
  2> 00_nucleo/diagnosticos/vanilla-font-retention-experiment/analysis.stderr
```

Build before collection; do not run tests/builds/other benchmarks concurrently.
run.py refuses an existing round-receipts.json; rejected/noisy data must be preserved,
not overwritten or retried selectively. Poppler/fontconfig and Pillow are required;
output validation runs only after timing ends. analyze.py also expects a fresh
output-controls directory. Do not execute writers in this preserved dataset.

Primary compiled_session_ms sums measured cycle composition plus retained donor
setup/preload/final drop. Each cycle includes source/World setup, productive pipeline,
instrumentation bookkeeping and explicit input/output/World destruction. Output files,
font/metadata/PDF hashes and validations are outside those intervals. session_wall_ms
and launcher_wall_ms include those controls and are separate observables. The recorded
render_ms is after subset; it is not full export. The outer pipeline interval is complete.

Artifacts: lab source with locked manifest; measured-sources.tar.gz; bin/experiment;
artifact.json/profile/recipe; provenance.json/exact tree and host; font-inventory.json
and fontconfig-list.txt; initial/per-round/final fingerprints; controls/direct and
adapter PDFs; raw/roundN/session stdout/stderr/commands/PDFs; samples.json (270 cycle
rows); process-monitor/round receipts; summary.json (27 groups); paired-session-effects;
phase-summary/retention-controls; output-validation and command list; report; completion
audit; MANIFEST.sha256. The report and lab source are sealed by hashes in completion
audit because they live outside the evidence directory. Verify this directory with
sha256sum -c MANIFEST.sha256. Absolute timings remain limited by the recorded high
background load and 16 groups failing stability.
