# L0 — Font slots, metadata and inventory
Hash do Código: c71597ba

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fonts.rs`

## Current evidence

- `03_infra/src/fonts.rs:23`, `:101`, `:113`, `:138`, `:153` — `FontSlot` and lazy loading.
- `03_infra/src/fonts.rs:202` — collection-face reconstruction.
- `03_infra/src/fonts.rs:271`, `:339`, `:404`, `:893` — discovery, metadata, coverage and paired insertion.
- `03_infra/src/fonts.rs:52` — `inventory_fonts`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own `FontSlot`, explicit-path discovery, metadata extraction, collection-face reconstruction, the stable coverage facade/delegation and the inventory DTO. Font selection/scoring belongs to FontBook and its infrastructure adapters, not to this loader. External parser types do not cross into L1 contracts.

`FontSlot` retains public `path` and `index` and private embedded bytes, lazy mmap and final-font `OnceLock<Option<Font>>`. `new_embedded` uses index zero and treats the path as display information only. Source bytes prefer embedded bytes to a shared lazy file mapping. Loading/validation is idempotent per slot, including failure. Invalid or unreadable bytes and invalid face indices return `None`; validate the requested index against the original source before reconstructing a collection or parsing the normalized index-zero result, including embedded slots whose public index has been changed; mapping failures are not silently converted to a new eager-read policy. Simple fonts retain opaque mmap storage; reconstructed collection faces use independently owned bytes.

For a supported multi-face collection, rebuild the selected sfnt header/table directory, copy tables from collection-absolute offsets, rewrite offsets with alignment and recompute `head.checkSumAdjustment`. Preserve the existing no-extraction branch for noncollections and collections with at most one face; this document does not broaden collection support.

`discover_fonts` processes supplied files/directories recursively, recognizing lowercase `.ttf`, `.otf`, `.ttc` and `.otc`. Discovery creates one slot per discovered face, using one as the face-count fallback. Discovery does not require final-font validity; metadata pairing discards invalid slots. Preserve directory iteration behavior rather than promise an additional global sort or deduplication.

`font_info_from_bytes(data, index)` validates the requested face and returns neutral metadata or `None`. PostScript nameID 6 exceptions override each available family/style/weight/stretch field. Otherwise family comes from nameID 1 with repeated case-insensitive style-suffix trimming; nameID 16 is deliberately ignored. Decode Macintosh Roman names where applicable. Preserve the residual first-decodable-name fallback for a missing ID1, without claiming upstream equivalence for it. Infer Italic before Oblique using face style bits and full-name spelling; retain weight/stretch extraction, monospace flags and OS/2 PANOSE serif detection `[2, 2..=10, ..]`. Preserve the existing exception entries, including documented New Computer Modern names and weight/style corrections.

Metadata extraction leaves `coverage` empty. `extract_coverage` retains its crate-private signature and delegates exact extraction to the private child `03_infra/src/fonts/coverage.rs`, exclusively specified by `00_nucleo/prompts/infra/fonts/coverage.md`; `pair_slots_with_book` inserts metadata and slots together and preserves input order. `00_nucleo/prompts/infra/fontdb.md` owns eager system coverage; `00_nucleo/prompts/infra/system-world/fonts.md` owns lazy coverage for entries that lack it, cache lifetime and source preloading. Do not state that every loader eagerly extracts coverage or that all system coverage must remain lazy.

`inventory_fonts(font_paths, include_system)` composes existing embedded/system/project loaders into `FontInventoryEntry { family, path, index, style, weight, stretch, embedded }`. Discard invalid pairs, sort by lowercase family, style, weight, stretch, path and index, then remove identical entries. Do not format stdout in L3; `00_nucleo/prompts/wiring/fonts.md` delegates presentation to L2.

## Verification boundary

Existing inline tests cover invalid slots, pairing, synthetic collection reconstruction, Macintosh names, suffixes/style inference, exceptions, exact extraction and mmap loading. Repository fixtures include `03_infra/fixtures/fonts/p839-*.ttf`, `p840-fandolhei-bold.ttf`, `NotoSans-Regular.ttf` and the licensed OTF fixtures. Host CJK/font tests must report environmental absence without treating it as positive coverage evidence.


## Coverage ranges — gate package2026-10-07

Measurement before decision: 00_nucleo/diagnosticos/cmap-intervalos-2026-10-05/measurement.json.gz records sources/HEAD/dirty tree/UTC/argv/fonts and53.19→10.85ms isolated extraction/canonicalization; no CLI speedup measured. 00_nucleo/diagnosticos/coverage-ranges-preparacao-2026-10-07/audit.json verifies this consumer remains identical to the measured HEAD source, and the pure font_book consumer remains byte-identical to the experiment snapshot. ttf-parser0.25.1 format4.rs:103–115,format12.rs:68–75,format13.rs:41–48 retain declared bounds/glyph-zero; its cmap/mod.rs iterator stops at first invalid record.

Decision after package ratification: declare private child coverage and delegate only existing extract_coverage. Font slots, collection reconstruction, metadata, pairing, discovery timing/eager-vs-lazy policy, cache lifetime and API remain unchanged. Parent must not duplicate range parsing, normalization or callback fallback; child owns these algorithms. Invalid face and missing cmap behavior remain. The public Coverage constructor is owned exclusively by 00_nucleo/prompts/entities/font_book.md. No code before human ratification of the public constructor in this package.

Acceptance: existing inline tests remain, child tests exercise syntax/native/fallback and independent membership; full build/lint lineage and exact per-face/fallback/PDF controls, then same-state paired CLI/cost measurement. No performance promise from isolated historical data.
