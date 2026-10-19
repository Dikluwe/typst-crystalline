# Final operational path review (read-only)

Checkpoint used: `tekt-construction-2026-10-18` / `8ec827fd975bf14cd849bebb9ede245e49033337` (current HEAD). No repository files were changed and no Rust compilation/build was run.

## Method

- Read `.gitignore`, `.dockerignore`, `.cargo/config.toml`, root `Cargo.toml`, `lab/benches/Cargo.toml`, `Cross.toml`, `Dockerfile`, CI/release/Docker workflows, and release/oracle/bench helpers.
- Searched operational scripts, CI configs, bench docs, and selected fixture instructions for old root `target/`, `tools/`, `benches/`, `00_nucleo/diagnosticos`, and `materialization` paths. Distinguished executable/config references from prose/history.
- Ran `python3 lab/tools/ci/check_mature_layout.py`: passed.
- Compared `git diff --no-ext-diff --unified=0 HEAD` under `01_core/`, `02_shell/`, `03_infra/`, and `04_wiring/`. There are 746 modified Rust files and 1,538 nonblank added/deleted Rust lines; all changed lines are line/block/doc comments or headers, with zero non-comment Rust lines. No Rust file mode changes were found.
- Checked path existence for the moved benchmark corpus and target binary, and `git check-ignore -v` for moved result destinations.

## Findings

- Cargo artifact routing is internally consistent: `.cargo/config.toml` sets `lab/target/product`; release packaging reads the target-specific binary under that directory and writes packages under `lab/target/release-packages`; release workflow uploads that exact package directory. The benchmark helper’s explicit `lab/target/benchmarks` override is also consistent with its README. Oracle builder deliberately uses its separate checkout’s `source/target`, and CI consumes the same path.
- Docker retains `lab/benches` through explicit `.dockerignore` exceptions, while omitting other `lab/*` content. Its `xx-cargo` build and final-stage copy both use `lab/target/product`; Cross passes through `TYPST_COMMIT_SHA`. These are static path checks only; Docker/Cross were not executed.
- **Two actionable stale operational references remain.** `lab/tools/perf/bench.sh` reads the corpus from `00_nucleo/diagnosticos/medicao-pre-f-passo-318-corpus.typ`, which no longer exists; the file is now at `lab/results/construction/diagnostics/medicao-pre-f-passo-318-corpus.typ`. The runner will fail before measuring. `03_infra/fixtures/p307b/MANIFEST.md:60,61,65` gives fenced commands using `./target/debug/typst`; root `target/` is absent and the configured binary is `lab/target/product/debug/typst`.
- Remaining `target/release/typst` occurrences found in oracle helpers/docs refer to the deliberately separate upstream oracle checkout (`lab/typst-original` or `lab/ci-oracle`), or to explanatory historical prose. They are not stale product output paths. Old diagnostic path strings in the fixture manifest are historical evidence references, not commands.
- `.gitignore` intentionally ignores all of `lab/results/`. `git check-ignore` confirmed both a moved diagnostics file and the new verification receipt are ignored. The construction manifest contains 161,130 diagnostics rows, of which 34,580 were previously tracked; those tracked destinations need explicit force-add handling to preserve their tracking state. The newly created validation receipts under `lab/results/mature-layout-2026-10-18/` are ignored as well and will not enter a commit without explicit inclusion. No staging was performed.
- The remaining `.gitignore` rules for old `.typ/*` and `00_nucleo/diagnosticos/p916-work/` are stale but harmless: they do not match the relocated history/results paths.

## Limits

This review checked paths and repository diffs statically; it did not run Docker, Cross, Cargo, Rust tests, or release packaging. The two stale references above are concrete failure risks for the perf runner and the documented fixture procedure.
