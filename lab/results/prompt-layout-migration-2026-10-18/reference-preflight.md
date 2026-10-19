# Pre-migration reference review

Read-only investigation; no tracked files were edited. Recorded at UTC 2026-10-19T00:21:26.941705+00:00, HEAD `d330c717eb06c86a560c4cf3d8d0ba40224609b9`. `git diff HEAD --stat` was:

```text
 lab/tools/ci/check_mature_layout.py | 46 ++++++++++++++++++++++++++++++++-----
 1 file changed, 40 insertions(+), 6 deletions(-)

```

## Required rewrites

- Resolve old file identities before moving. ADR-0130 permits nested owner paths but requires the full `00_nucleo/prompts/` prefix in headers and visible prose identities. Document-relative Markdown targets must be resolved against their **old** parent and rendered relative to the **new** parent; a global canonical-path replacement alone does not do this.
- Rust external test modules require scope-specific identifier and filename edits. Repeated basenames (`p1425_tests`, `p1430_tests`, `p1360_ab_tests`) identify different suites in different directories. Do not globally map a basename to one semantic name.
- The pre-migration Rust scan found 52 numbered external `mod ...;` declarations and 142 numbered inline `mod ... {` declarations. External declarations correspond to files and must migrate with them. Inline identifiers are an independent cleanup scope: leaving them unchanged does not break file organization; report that limitation if retained.
- `#[path]` attributes are present in eval/mod.rs, stdlib/html.rs, stdlib/structural/mod.rs and wiring wrappers. `html.rs` specifically uses `html/p1423_tests.rs` and `html/p1424_tests.rs`; default resolution alone would be wrong. Eleven wiring tests include L3 support modules using `../../03_infra/tests/support/wiring/...`; ten target numbered filenames and need both ends updated. `entities/syntax_node/representation.rs` uses `linked.rs` and should stay unchanged.
- Auto-discovered integration test target names change with filenames. `cargo metadata --no-deps` before/after can prove target counts and types are unchanged while mapping names. Search found no explicit numbered `--test` invocations in .cargo/.github/lab/tools/ci, no explicit `[[test]]` targets in the layer Cargo manifests, and no relevant numbered source references in lab/benches. Existing full-workspace validation does not depend on old test basenames.
- `crystalline.toml:43` explicitly excludes `04_wiring/tests/p1286_contract.rs`. Update its path (and preferably descriptive TOML key) with the rename; preserve its exclusion rather than accidentally treating it as a new orphan owner. Other numbered exclusion entries describe obsolete/historical temporary paths and are a separate configuration cleanup, not owners.
- Guidance and ADR examples are active references too: CLAUDE.md:98 has `prompts/entities/version.md`; ADR-0130 contains old canonical example paths/source-owner examples and says all examples must resolve. Update examples, root L0 index, CI/workflow templates, and current diagnostics as appropriate. Do not rewrite historical lab/results or lab/history receipts to pretend the new paths existed at measurement time; the migration map supplies continuity.

## Governance compatibility

The rule is layer + actual consumer path, removing only the immediate leading `src/`, then `.rs` → `.md`. Keep `tests`, `mod`, `lib`, `main`, and `build` when these are actual source identities. ADR-0129 requires a 1:1 owner mapping; no merging/splitting or duplicate navigation owners. An index belongs outside materializable L0. ADR-0130's prior non-renaming statement describes that decision's scope, not a prohibition on this explicitly user-authorized migration.

Do not mutate nucleus content or paths merely because prompt directories move. The 109 nucleus files' pre-migration SHA-256 values are saved in `/tmp/tekt-migration-nuclei-before.json`; compare exact bytes afterward. Each pin's value and logical path should remain unchanged, then V26 validates the graph. Correct ownership (V15/V26) before resealing, and run forward strict and reverse checks after header changes.

## Validation implications

Validate all-target compilation (resolves relative modules and Cargo discovery), unit/integration suites, doc tests, Clippy, formatting, Markdown link/path checking, strict lineage and reverse seals. Compare full lint against the preexisting 105 V16 baseline using mapped file identities, not raw filenames or line numbers. Tests being renamed makes historical test names change, so preserve an explicit old/new mapping rather than interpreting the rename as lost/new coverage.
