# Place font acquisition dataset

Read ../place-font-acquisition-study.md, protocol.md and default-policy/protocol-addendum.md before using the results. This is a new diagnostic dataset; prior sealed studies are unchanged.

- provenance.json: exact incoming HEAD, dirty/untracked state, diff-stat, clock, machine/toolchain.
- before-sources.json, measured-sources.tar.gz, dirty-tree.patch: source identities and reproducible non-vanilla working-tree source snapshot, including untracked consumers. Do not reconstruct the measured tree from HEAD alone.
- probe/, build*.jsonl/stderr, probe-artifact.json, cli-artifact.json: diagnostic source, dependency lock and optimized artifact receipts. The first offline build failed dependency resolution; build-locked is the successful diagnostic build. The root CLI build succeeded separately.
- vanilla-provenance-reference.json: pinned upstream tree/recipe receipt and byte identity. bin/: actual measured executable copies.
- inputs/, controls/: equivalent input bytes, untimed stack/trace/warmup controls and PDFs/raster controls. FONT path uses the preceding study's preserved project fixtures; system/fontconfig identities are retained in fingerprints.
- raw/, samples.json, round-receipts.json, *fingerprints.json, process-monitor.json: all samples, commands, UTC, raw output, PDFs and admission controls.
- default-policy/: independent cohort differing only in removal of the explicit font declaration. Do not pool cohorts; its timing distributions failed stability.
- summary.json, attribution-summary.json, output-controls.json: distributions, compatible nested interval attribution and output differences. Ignored bytecode caches are excluded from evidence.

Reproduce in a separate copied directory, with the source archive restored into an isolated checkout, the recorded toolchain and exact font bytes/configuration. The archive excludes upstream sources: use the ratified vanilla pin/tree and prior recipe receipt. Diagnostic manifests use absolute paths to this workspace; update them deliberately in the copy when reproducing elsewhere.

Commands used:

```sh
cargo build --release --offline --locked --manifest-path 00_nucleo/diagnosticos/place-font-acquisition-study/probe/Cargo.toml --target-dir target -j 1 --message-format=json
RUSTFLAGS='' CARGO_ENCODED_RUSTFLAGS='' cargo build --release --offline --locked -p typst-wiring --bin typst -j 1 --message-format=json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/place-font-acquisition-study/run.py
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/place-font-acquisition-study/run-default-policy.py
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/place-font-acquisition-study/analyze.py
```

The measured build had no ambient Rust flags for the probe; both artifacts were verified opt-level 3 with debug assertions disabled. Clear Rust flags explicitly on reproduction. Analyze requires Pillow and Poppler tools. Analyses and rasterization run after timing collection, not during it. Launcher elapsed times from stack/trace/warmup receipts are not admitted benchmark samples.

For read-only seal verification, run sha256sum --check MANIFEST.sha256 from this directory; the separate report digest is in completion-audit.json. Do not rerun writers in this sealed directory.
