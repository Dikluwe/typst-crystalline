# L0 — System font discovery
Hash do Código: 667faea7

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb.rs`

## Current evidence

- `03_infra/src/fontdb.rs` — `font_info_with_coverage`, `load_system_fonts`, `load_fonts_from_dir`.
- `03_infra/src/world/fonts.rs` — composition builders.

These are current local source anchors, not a new upstream parity measurement.

## Measurement before the acquisition decision

Historical `00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05.md` records38.10→29.00ms metadata acquisition and94.33→88.04ms with old coverage; these are not current CLI gains. Audit `00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07/audit.json` records HEAD/dirty/UTC/assets,1,140 faces/max20 records/max259,927 projected bytes, unchanged historical assets and unchanged metadata extractor body; priority2 coverage differs and stays current. `03_infra/src/fontdb.rs:42–113` previously mandates with_face_data. fontdb0.23.0 lib.rs:721–727,900–920 distinguishes File reopening from SharedFile retained bytes. Metadata/source order and coverage observable in language must stay equivalent; I/O/buffer mechanics may differ. Infer a possible benefit; wrong metadata/coverage/selection/order/render or lack of repeatable benefit refutes it. Never infer upstream intent or add historical percentage improvements.

## Decision after ratification

Changing the default acquisition path needs the concrete catalogo-seletivo-2026-10-07 package ratified under ADR0127§2.2/§5 before code. No new public API or product flag. Parent owns path selection and pairing, child owns bounded File acquisition; rollback/refusal retains original path. Do not duplicate parsing or convert FontSlot mmap failure into eager reads.

## Contract

Own operating-system font discovery and directory-limited discovery through `fontdb`. Both loaders return `(Vec<FontSlot>, FontBook)` with identical slot/book indices. Accept File and SharedFile sources with their collection index; skip nonpersistent Binary sources. Invalid metadata or an invalid face is omitted from both collections.

For Source::File, first try the private selective child specified by `00_nucleo/prompts/infra/fontdb/selective.md`. If it declines, use `Database::with_face_data` and the original metadata/coverage callback at the supplied face index. Source::SharedFile always uses `Database::with_face_data` over retained bytes; do not reopen its path as a substitute. A readable Unicode cmap is reflected in `FontInfo.coverage` before the Database is dropped. Reuse `font_info_from_bytes` and `extract_coverage` from `00_nucleo/prompts/infra/fonts.md`; do not duplicate their algorithms. The resulting slots own lazy file mappings independently of the Database.

Keep discovery order and existing duplicate path/index entries. A system without fonts and an empty or irrelevant directory return empty collections without panic. `load_fonts_from_dir` supports deterministic test discovery without assuming installed host families.

This prompt owns neither `SystemWorld` builders nor CLI defaults. Their owners are `00_nucleo/prompts/infra/system-world/fonts.md` and the wiring command prompts. L1 receives its existing neutral metadata/opaque font types, never a fontdb Database or parser face.

## Verification boundary

Existing tests check empty/nonfont directories, paired indices and exact coverage using `03_infra/fixtures/fonts/NotoSans-Regular.ttf`. Availability of host fonts is environmental, not a fixed expected catalog size.


## Selective acquisition integration

Declare a private child selective and delegate only Source::File metadata acquisition to its private facade. Successful Some(FontInfo) includes exact coverage; None means declined attempt, so run the original callback rather than omit the face. Source::SharedFile must preserve retained bytes even if its path has changed or disappeared. Source::Binary remains excluded. Do not restructure discovery into sorting/dedup/cache phases, alter public loaders or slots, reparse the projected bytes at a collection index, or use them as final font source. Existing FontSlot and coverage owners remain unchanged.

The child is exclusively `03_infra/src/fontdb/selective.rs` ↔ `00_nucleo/prompts/infra/fontdb/selective.md`. No shared materializable prompt. Keep original metadata callback for fallback. Unsupported/nonUnix/unstable/overbudget/invalid selective input must conservatively use it; existing invalid-face metadata filter and slot/book insertion stay identical.

Acceptance: inline parent tests demonstrate File fallback, SharedFile retained data, empty/nonfont directories, complete metadata/coverage and exact slot/book indices/order. Child tests RED→GREEN first; per-face comparisons and equivalent-language/PDF/CLI corpus against frozen previous and pinned vanilla before any speed conclusion. Final build/tests/scoped lineage and three-round combined catalog/CLI/RSS measurement per ratified package; no public instrumentation or unrelated owners changed. If refuted, preserve/restore prior productive path and record negative evidence.
