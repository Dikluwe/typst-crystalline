# L0 — infra/system-world/tests
Hash do Código: 3da27756

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb


**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/tests.rs`

## Current evidence

- `03_infra/src/world/tests.rs:18` — `MockWorld`.
- `03_infra/src/world/tests.rs:26` — `new`.
- `03_infra/src/world/tests.rs:38` — `library`.
- `03_infra/src/world/tests.rs:41` — `book`.
- `03_infra/src/world/tests.rs:44` — `main`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/tests.rs`.

## Owned contract

- Own World mocks, filesystem/font/path/bibliography/package fixtures and unit cases under world::tests, not production implementations.
- Characterize synthetic main without I/O, memoized source/error, path/id shared successful bytes, retry after byte error, canonical identity/dependencies (including concurrent registration of the same path), lexical-escape rejection and book coverage without source access.
- Characterize a valid project font added after configuring the world and before its first catalog request, retained catalog identity under later/concurrent requests, and old coverage cleared when replacing a configuration. Keep the precomputed-coverage-without-font-bytes regression.
- Use local deterministic temporary fixtures and no network. Execute serially because a legacy test mutates XDG_DATA_HOME. Arc-identity assertions characterize cache mechanics, not language parity.

## Verification and boundary

Run `cargo test -p typst-infra --lib world::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Source-identity integration controls

### Evidence and approval

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json`
retains commands/sources/UTC/HEAD/changed tree and Tekt/vanilla hashes at historical
pin `9f2b6e8715237cb086899a42873660fe744622e8`. Document source was lost while
expression received its name; eval/mod.rs:609–615 uses world.main() for Source/
base and wiring/eval.rs:66–95 repeats it for primary. cache.rs:58–78 registers/
recycles ids; upstream eval.rs:159–179/world.rs:176–190 distinguish expression,
stdin and absent input. Integer equality/global upstream interning is mechanics;
shown source/origin/position/relative resolution are language observables.
Human `confirmo` on 2026-10-06 approved the gate, receipt
`lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json`.

### Tests and boundaries

Check distinct document/expression ids, resolvable code Source, exact UTF-8 world.file
bytes, equal expressions with independent ids, stdin without a file and relative
read/import. Real physical `<input-expression>`/`<stdin>` filenames cannot alias
transients; dependencies omit markers but include these files when actually read.
Check concurrent physical/transient registration, last available identity and
exhaustion without overwrite: old sources/caches/dependencies remain resolvable,
including after captured physical failure. Internal counter fixture reaches the
boundary without thousands of files. Exclusive ownership/code seal follow the
materialization; this test does not own Source allocation.

## Private fixture maintenance

`/tmp/watch-green.log`, copied in the retained execution report, records E0596 at
tests.rs:493–498 after ratified SystemWorld Arc storage. The precomputed-coverage
fixture may mutate only its exclusive Arc through Arc::get_mut. Preserve coverage,
missing slot and all assertions; internal ADR-0127 test adaptation adds no public
contract/behavior and retains exclusive ownership.

`clippy-expressions-2026-10-16/before.json.gz` retains sources/L0/HEAD/uncommitted
tree/status/diffstat/UTC; initial-diagnostics.json retains uses/types/suggestions/
spans at tests.rs:206 (clippy::len_zero). Use
!world.font_catalog().0.is_empty() rather than len >= 1 in the project-font test,
preserving catalog access/valid fonts/nonempty requirement. Internal representation
claims no parity/performance improvement. Continuous ADR-0127: no public/default/
phase/dependency/suppression change, L0 first/scoped resealing, unchanged control
inputs/results/tolerances, selected-lint RED→GREEN then workspace suite/build/
all-target Clippy/fmt/Rustdoc/lineage; no syntax-mirror tests.
