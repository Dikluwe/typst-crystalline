# L0 — System font discovery
Hash do Código: 667faea7

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb.rs`

## Current evidence

- `03_infra/src/fontdb.rs` — `font_info_with_coverage`, `load_system_fonts`, `load_fonts_from_dir`.
- `03_infra/src/world/fonts.rs` — composition builders.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own operating-system font discovery and directory-limited discovery through `fontdb`. Both loaders return `(Vec<FontSlot>, FontBook)` with identical slot/book indices. Accept File and SharedFile sources with their collection index; skip nonpersistent Binary sources. Invalid metadata or an invalid face is omitted from both collections.

Use `Database::with_face_data` to extract metadata and exact Unicode coverage from the temporary bytes at the supplied face index. A readable Unicode cmap is reflected in `FontInfo.coverage` before the Database is dropped. Reuse `font_info_from_bytes` and `extract_coverage` from `00_nucleo/prompts/infra/fonts.md`; do not duplicate their algorithms. The resulting slots own lazy file mappings independently of the Database.

Keep discovery order and existing duplicate path/index entries. A system without fonts and an empty or irrelevant directory return empty collections without panic. `load_fonts_from_dir` supports deterministic test discovery without assuming installed host families.

This prompt owns neither `SystemWorld` builders nor CLI defaults. Their owners are `00_nucleo/prompts/infra/system-world/fonts.md` and the wiring command prompts. L1 receives its existing neutral metadata/opaque font types, never a fontdb Database or parser face.

## Verification boundary

Existing tests check empty/nonfont directories, paired indices and exact coverage using `03_infra/fixtures/fonts/NotoSans-Regular.ttf`. Availability of host fonts is environmental, not a fixed expected catalog size.
