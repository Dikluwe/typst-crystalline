# Mature repository layout — 2026-10-18

## Scope and provenance

The user authorized execution on a new branch, preservation of history, English
active prose, and validation of paths, lineage, and tests. The branch is
`codex/tekt-mature-layout`. Its construction checkpoint is
`8ec827fd975bf14cd849bebb9ede245e49033337`, tagged
`tekt-construction-2026-10-18`. Migration measurements refer to the uncommitted
working tree after that checkpoint; each validation receipt records its command
and time. This is an organizational migration, not a new language-parity claim.

## Resulting organization

`00_nucleo` contains ADRs, L0 owners/nuclei, and current diagnostics. The four
productive layers retain their directory names and import topology. Benchmarks,
operational scripts, generated results, scratch work, and Cargo build output are
under `lab`. The root Cargo workspace includes `lab/benches`; `.cargo/config.toml`
sets product output to `lab/target/product`. The upstream oracle retains its
separate build and provenance in `lab/typst-original`.

Active `materialization` and `diagnosticos` directories have been retired.
Construction records are preserved byte for byte, with a relocation manifest,
under `lab/history/construction` and `lab/results/construction/diagnostics`.
Historical paths inside those originals remain historical and can be restored
through the manifest. ADR identifiers are unchanged; filenames are English and
the ordered index is canonical. Process templates have moved out of L0 into
`lab/tools/workflows`. Non-owner specifications are classified in the
[retirement record](retired-nonowners-2026-10-18.md).

The remaining executable documentation corpus moved to `lab/corpus/docs`.
Legacy `.typ` inputs required by integration tests are byte-identical active
copies in `lab/corpus/legacy-sections`; their historical originals remain in the
construction records. Twelve `include_str!` paths were updated after the first
test compilation exposed those dependencies. Test assertions and input bytes
were preserved. The compilation helper writes outputs under `lab/results`.

## Preservation and review evidence

The [history verification](../../lab/results/mature-layout-2026-10-18/history-verification.json)
checked all 163,863 manifest entries and 14,224,786,882 bytes with zero hash/size
errors. Three independent restored samples matched. The
[Git object comparison](../../lab/results/mature-layout-2026-10-18/tracked-history-objects.json)
also found exact blob identity for 37,313 historical tracked files and 820
benchmark result files at their staged destinations. Previously ignored local
evidence remains local; it was not silently added wholesale to Git.

The [ADR review](../../lab/results/mature-layout-2026-10-18/adr-review.md)
verified the 116 mapped records, IDs, statuses, and ordered index. The
[final pilot review](../../lab/results/mature-layout-2026-10-18/final-pilot-review.md)
records the realization clause comparison and approval receipts. Its reference
findings were corrected as recorded in
[review fixes](../../lab/results/mature-layout-2026-10-18/review-fixes.json).
The [bounded reading comparison](context-reading-comparison-2026-10-18.md)
recovered the same occurrence/error-restoration obligations before and after;
it does not demonstrate reduced cognitive or implementation cost.

## Validation

Execution receipts are collected in
[lab/results/mature-layout-2026-10-18](../../lab/results/mature-layout-2026-10-18/).
Commands, timestamps, source hashes, and initial failed attempts are retained there.
The linter is built from CI pin `12797073972e375d9ab078cf6750c8b30b71e2b9`.
A separate copy of the checkpoint provides the before-migration baseline using
that same executable. Full lint findings are compared with the baseline; the
strict lineage gate and reverse seals are checked independently.

The [matched-source lint comparison](../../lab/results/mature-layout-2026-10-18/lint-comparison-matched-oracle.json)
found identical findings before and after: the same 105 V16 errors remain.
The initial isolated checkpoint lacked the ignored vanilla source, producing
five V21 missing-source warnings. Repeating the comparison with byte-identical
copies of those five cited source files eliminated that environmental difference;
the initial and inconclusive symlink attempts are retained. Informational V19/V20
counts are unchanged. This repository is not globally lint-clean; the existing
V16 failures still block the full architecture CI gate.

The [clean snapshot check](../../lab/results/mature-layout-2026-10-18/fresh-checkout.json)
passed layout, Cargo metadata, strict lineage, and reverse lineage after extracting
only the staged Git tree. Ignored local files were not copied into that snapshot.
The [Rust change-scope check](../../lab/results/mature-layout-2026-10-18/rust-change-scope.json)
found only comment/header edits and the twelve literal test-fixture path changes;
no productive algorithm was changed.

Initial validation attempts are retained, including the missing fixture paths
and the three SVG observer failures caused by `rsvg-convert` being absent from
the service's PATH. The final runner validates observer availability before
starting tests and records the resolved executables and hashes.

The [functional test summary](../../lab/results/mature-layout-2026-10-18/test-summary.json)
records 9,128 unit/integration tests and five doctests passing, with seven ignored
tests and no failures. The workspace build passed. The final Python helper suite
passed all 28 tests. Strict V1/V5/V15/V26 checks and reverse lineage passed for
991 consumers. The oracle binary retained its pinned SHA-256.

The all-target build and Clippy continuation cover the relocated benchmark crate,
including its corrected `include_bytes!` fixture path. The subsequent formatting
continuation changes only the wrapping of that expression. Product sources did
not change after the functional tests. Source-freeze receipts and the
[validation summary](../../lab/results/mature-layout-2026-10-18/validation-summary.json)
join these runs without treating earlier failed attempts as successes.
The final `cargo fmt --check --all` and workspace Rustdoc build with warnings
denied also passed.

These are local Linux checks. Remote CI, Windows/i686 cross-builds, and Docker
execution were not run. This migration does not certify parity with vanilla or
claim a benchmark speedup.

## Editorial boundary

The root guidance, layout indexes, workflow templates, selected governing ADRs,
and the two realization pilot prompts use English prose. This migration does
not claim that every existing ADR, L0 prompt, source comment, or tool guide has
been translated. The remaining corpus needs semantic review by owner; changing
the language must preserve acceptance criteria, public API literals, nucleus
pins, and observable messages. The original documents remain recoverable from
the checkpoint. The pilot and ADR review receipts record the scope actually
reviewed.
