# Font microbenchmarks — first battery

A diagnostic-only crate. Production/L0/ADRs, existing benches and prior baseline/experiment datasets remain unchanged. Implements the first coverage/acquisition battery from the [plan](../font-lifecycle-experiment/microbench-plan.md). Catalog setup-context, full shaping and World reuse experiments are separate later batteries.

Existing public `Coverage`, font metadata and FontSlot APIs are measured. `src/historical.rs` is an exact slice of preserved historical Coverage code, identified by `historical-provenance.json`: an algorithm reproduction, not the historical crate or vanilla. The diagnostic cmap traversal matches production subtable collection but does not call the private extractor. No new production API.

Four fonts, same bytes/face index/ordered cmap input for both constructors. `fixture-controls.json` records SHA-256s, Unicode subtable formats, collected/distinct counts, sequence hash, canonical coverage hash and independent set controls. Preflight and font I/O/fixtures remain outside timers. Filesystem and memory are warm. First slot is uninitialized but storage is not cold.

Read [protocol.md](protocol.md) for timed boundaries, caches, setup/input/output destruction, profile and admission criteria. Criterion PerIteration setup excludes Vec clone for constructor-only; that API consumes/frees the owned Vec inside. clone_and_build includes clone. First FontSlot uses batched references, keeping slot and returned Font destruction outside. Returned results of metadata, collection and cached Font get are also destroyed outside. Query loops include black-box/iteration overhead; the bool control indicates the floor and must not be blindly subtracted.

Build/preflight, before timing:

```sh
CARGO_BUILD_JOBS=1 CARGO_ENCODED_RUSTFLAGS='' RUSTFLAGS='' cargo build --manifest-path 00_nucleo/diagnosticos/font-microbenchmarks/Cargo.toml --release --offline --locked --target-dir target --message-format=json
./target/release/font-microbench-diagnostic --check
cargo bench --manifest-path benches/Cargo.toml --no-run --offline --locked --message-format=json
```

The diagnostic lockfile was seeded from the incoming root lockfile to retain offline-cached dependency versions. Initial unseeded resolution failed on an unavailable zerocopy-derive version; it produced no samples. No network download or root lockfile change was needed. Actual profiles, source/executable hashes and exact dirty lists/patches must be recorded for the final build. Ambient compiler/profile overrides are not supported; use the cleared environment and verify artifact JSON.

`measure.py` requires a final recorded build log, verifies its actual optimized profile and executable, does preflight, then runs three fresh processes with 30-second quiet gates and contamination monitoring. It writes only this directory and uses prior process-inspection helpers read-only. Freeze its source and the analysis/protocol before starting. Refuses to overwrite a round. Collect a new complete matched round set in a new evidence directory when necessary.

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/font-microbenchmarks/measure.py
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/font-microbenchmarks/analyze.py
```

Each raw Criterion `new/sample.json` retains iteration counts and elapsed ns; `new/estimates.json` includes Criterion's estimates/confidence intervals. `summary.json` adds equal-round summaries and predefined stability classifications. Cases failing criteria remain descriptive. Constructors are measured sequentially within each round, so their ratio is not an interleaved paired statistic. Do not sum these timings into the earlier CLI totals or equate the historical algorithm comparator with vanilla.

First collection: [analysis](../font-microbenchmarks.md). Three rounds admitted by integrity, but only 1 of 56 cases passes the predefined absolute-cost stability criteria. These are exploratory measurements, not a stable baseline. Original and prior experimental datasets remain intact. TTC first get includes face reconstruction/table copying/checksums, not only mmap or parser work.

`consumers-inputs.tar.gz` preserves frozen repository source/input files; external font bytes remain identified by path/hash and must be supplied for reproduction. `bin/font-microbench-diagnostic` preserves the final executable. `completion-audit.json` records final scope/integrity and report hash. Run `PYTHONDONTWRITEBYTECODE=1 python3 verification/verify.py` for read-only verification from this directory; it does not rewrite old evidence. The source archive and raw Criterion data are independent of Cargo's mutable target directory.
