# L0 — System font discovery
Hash do Código: f82a32be

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0f2985807cc3c7b4aeed9a5a26b77ad66dbbdb67ccfb43ba8336dbc8e8eea1a5

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb.rs`

## Current evidence

- `03_infra/src/fontdb.rs` — `font_info_with_coverage`, `load_system_fonts`, `load_fonts_from_dir`.
- `03_infra/src/world/fonts.rs` — composition builders.

These are current local source anchors, not a new upstream parity measurement.

## Measurement before the acquisition decision

Historical `lab/results/construction/diagnostics/catalogo-seletivo-2026-10-05.md` records 38.10→29.00ms metadata acquisition and94.33→88.04ms with old coverage; these are not current CLI gains. Audit `lab/results/construction/diagnostics/catalogo-seletivo-preparacao-2026-10-07/audit.json` records HEAD/dirty/UTC/assets, 1, 140 faces/max20 records/max259, 927 projected bytes, unchanged historical assets and unchanged metadata extractor body; priority2 coverage differs and stays current. `03_infra/src/fontdb.rs:42–113` previously mandates with_face_data. fontdb0.23.0 lib.rs:721–727,900–920 distinguishes File reopening from SharedFile retained bytes. Metadata/source order and coverage observable in language must stay equivalent; I/O/buffer mechanics may differ. Infer a possible benefit; wrong metadata/coverage/selection/order/render or lack of repeatable benefit refutes it. Never infer upstream intent or add historical percentage improvements.

## Decision after ratification

Changing the default acquisition path required concrete package ratification under ADR-0127 §2.2/§5; the approval below satisfies that prerequisite only for catalogo-seletivo-2026-10-07. No new public API or product flag. Parent owns path selection and pairing, child owns bounded File acquisition; rollback/refusal retains original path. Do not duplicate parsing or convert FontSlot mmap failure into eager reads.

## Contract

Own operating-system font discovery and directory-limited discovery through `fontdb`. Both loaders return `(Vec<FontSlot>, FontBook)` with identical slot/book indices. Accept File and SharedFile sources with their collection index; skip nonpersistent Binary sources. Invalid metadata or an invalid face is omitted from both collections.

For Source::File, first try the private selective child specified by `00_nucleo/prompts/infra/fontdb/selective.md`. If it declines, use `Database::with_face_data` and the original metadata/coverage callback at the supplied face index. Source::SharedFile always uses `Database::with_face_data` over retained bytes; do not reopen its path as a substitute. A readable Unicode cmap is reflected in `FontInfo.coverage` before the Database is dropped. Reuse `font_info_from_bytes` and `extract_coverage` from `00_nucleo/prompts/infra/fonts.md`; do not duplicate their algorithms. The resulting slots own lazy file mappings independently of the Database.

Keep discovery order and existing duplicate path/index entries. A system without fonts and an empty or irrelevant directory return empty collections without panic. `load_fonts_from_dir` supports deterministic test discovery without assuming installed host families.

This prompt owns neither `SystemWorld` builders nor CLI defaults. Their owners are `00_nucleo/prompts/infra/world/fonts.md` and the wiring command prompts. L1 receives its existing neutral metadata/opaque font types, never a fontdb Database or parser face.

## Verification boundary

Existing tests check empty/nonfont directories, paired indices and exact coverage using `03_infra/fixtures/fonts/NotoSans-Regular.ttf`. Availability of host fonts is environmental, not a fixed expected catalog size.


## Selective acquisition integration

Declare a private child selective and delegate only Source::File metadata acquisition to its private facade. Successful Some(FontInfo) includes exact coverage; None means declined attempt, so run the original callback rather than omit the face. Source::SharedFile must preserve retained bytes even if its path has changed or disappeared. Source::Binary remains excluded. Do not restructure discovery into sorting/dedup/cache phases, alter public loaders or slots, reparse the projected bytes at a collection index, or use them as final font source. Existing FontSlot and coverage owners remain unchanged.

The child is exclusively `03_infra/src/fontdb/selective.rs` ↔ `00_nucleo/prompts/infra/fontdb/selective.md`. No shared materializable prompt. Keep original metadata callback for fallback. Unsupported/nonUnix/unstable/overbudget/invalid selective input must conservatively use it; existing invalid-face metadata filter and slot/book insertion stay identical.

Acceptance: inline parent tests demonstrate File fallback, SharedFile retained data, empty/nonfont directories, complete metadata/coverage and exact slot/book indices/order. Child tests RED→GREEN first; per-face comparisons and equivalent-language/PDF/CLI corpus against frozen previous and pinned vanilla before any speed conclusion. Final build/tests/scoped lineage and three-round combined catalog/CLI/RSS measurement per ratified package; no public instrumentation or unrelated owners changed. If refuted, preserve/restore prior productive path and record negative evidence.

Approval: owner `confirmo` on 2026-10-07 ratified catalogo-seletivo-2026-10-07,
manifesto SHA-256 `045ef18d7a16143e640669cf5923db1a213eb677b6c278d4ee4a0bf542973c2e`.

## Font retention evidence and approval

`lab/results/construction/diagnostics/watch-retencao-2026-10-07.md` retains
HEAD/tree/binaries/UTC, actual edits/recovery and dynamic per-cycle acquisition
counts. `watch-validacao-2026-10-07/measurement.json` compares discovery validation
and complete acquisition at the same state; controls.json covers additions/removals/
replacement/restored mtime/links/configuration. These are isolated costs, no CLI
gain or upstream-intent proof. FontBook/slots/coverage must preserve language
selection/content/diagnostics/render; Arc/duration are mechanics. Cost, stale
resources, regression or unbounded retention refute possible benefit. Fixtures/
provenance/limits remain in the reports.

Owner `Confirmo` ratified
`lab/history/construction/records/00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json`
under ADR-0127 §2.1/§2.2; receipt
`lab/results/construction/diagnostics/watch-catalogo-execucao-2026-10-07/ratification.json`.
Each owner remains 1:1.

## Prepared discovery without retention here

Declare pub(crate) mod generation, owned exclusively by
`00_nucleo/prompts/infra/fontdb/generation.md`. Expose catalog_from_database(&fontdb::Database)
only inside L3, containing the existing metadata/pairing loop. Public loaders keep
signatures/discovery/order and may delegate to it; session materializes the prepared
Database without a second initial discovery. Preserve selective File, retained
SharedFile bytes, excluded Binary, invalid-metadata filtering, indices and the
selective-acquisition fallback. No Database caching, sorting/deduplication or
ordinary compile-default change here. generation prepares/validates without
metadata/coverage/selective duplication; fonts/coverage owners remain unchanged.

## Private import maintenance

### Evidence

`lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`
retains sources/owners/spans/original compiler output, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat.
Selected warnings occur at fontdb.rs:13. Inspection distinguishes productive
references, descendant-test imports and comments; no new parity measurement.

### Contract and acceptance

Remove only private imports proven unused here; keep public reexports and imports
required by calls/contracts. Exact pre-edit spans remain in the diagnostic.
Internal continuous ADR-0127 maintenance changes no public signature/default/
phase/formula/acceptance. No lint suppressions. Revalidate all-target compile,
infra test names/execution, build/workspace Clippy/Rustdoc/lineage with scoped
resealing. No import-removal/comment-format mirror tests.
