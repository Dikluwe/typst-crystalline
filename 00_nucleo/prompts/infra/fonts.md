# L0 — Font slots, metadata and inventory
Hash do Código: 0f713c1c

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

Metadata extraction leaves `coverage` empty. `extract_coverage` retains its crate-private signature and delegates exact extraction to the private child `03_infra/src/fonts/coverage.rs`, exclusively specified by `00_nucleo/prompts/infra/fonts/coverage.md`; `pair_slots_with_book` inserts metadata and slots together and preserves input order. `00_nucleo/prompts/infra/fontdb.md` owns eager system coverage; `00_nucleo/prompts/infra/world/fonts.md` owns lazy coverage for entries that lack it, cache lifetime and source preloading. Do not state that every loader eagerly extracts coverage or that all system coverage must remain lazy.

`inventory_fonts(font_paths, include_system)` composes existing embedded/system/project loaders into `FontInventoryEntry { family, path, index, style, weight, stretch, embedded }`. Discard invalid pairs, sort by lowercase family, style, weight, stretch, path and index, then remove identical entries. Do not format stdout in L3; `00_nucleo/prompts/wiring/fonts.md` delegates presentation to L2.

## Verification boundary

Existing inline tests cover invalid slots, pairing, synthetic collection reconstruction, Macintosh names, suffixes/style inference, exceptions, exact extraction and mmap loading. Repository fixtures include `03_infra/fixtures/fonts/p839-*.ttf`, `p840-fandolhei-bold.ttf`, `NotoSans-Regular.ttf` and the licensed OTF fixtures. Host CJK/font tests must report environmental absence without treating it as positive coverage evidence.


## Coverage delegation evidence and approval

`lab/results/construction/diagnostics/cmap-intervalos-2026-10-05/measurement.json.gz`
records sources/HEAD/dirty tree/UTC/argv/fonts and isolated 53.19→10.85ms extraction/
canonicalization, not CLI speedup. coverage-ranges-preparacao-2026-10-07/audit.json
verifies this consumer matches the measured HEAD source and the pure font_book
consumer matches the experiment snapshot. ttf-parser0.25.1 format4.rs:103–115,
format12.rs:68–75, format13.rs:41–48 preserve declared bounds/glyph-zero; cmap
iterator stops at first invalid record. Owner `confirmo` on 2026-10-07 ratified
coverage-ranges-2026-10-07, manifesto
`f8f7805c6d2ca08d1d14296ed5c93767c416b536fa9c17532eaa8310ca93a549`.

Declare private coverage child and delegate only existing extract_coverage.
Slots/collection reconstruction/metadata/pairing/discovery/eager-versus-lazy timing/
cache lifetime/API remain unchanged. Parent cannot duplicate range parsing/
normalization/callback fallback; child owns them. Preserve invalid-face/missing-cmap
behavior. Public Coverage constructor belongs only to
`00_nucleo/prompts/core/entities/font_book.md`; recorded ratification satisfies that
package's public-constructor prerequisite, not other API changes. Keep existing
inline tests, child RED→GREEN for syntax/native/fallback/independent membership,
full build/lint/lineage/per-face/fallback/PDF controls and then same-state paired
CLI/cost measurements; no performance promise from isolated historical data.

## Embedded byte storage

### Evidence

`embedded-fonts-memoria2-2026-10-08/before.json.gz` freezes fonts.rs:32,119,158–163:
slot Vec plus first-get byte clone retains two buffers. world_types.rs:40–49,57–61
already offers Font::Vec with Arc<Vec<u8>>/shared clone/slice. This is internal
mechanical duplication, no language obligation or new L1 contract. Lower paired
RSS for a loaded simple embedded face is a hypothesis, refuted without repeatable
process reduction. Initial private Option<Font> (attempt-font-container/candidate.json.gz,
binary SHA `a4d93e4410e329ad7dd34fcef61f9114c863b3a4a96aba98f178b63bcc6e7953`)
showed −1.54% local table RSS in both batteries but repeat.json.gz showed
+0.91/+1.00/+2.01% time for context/simple/dense. Option<Arc<Vec<u8>>> removes
container dispatch; this is a cost hypothesis requiring fresh paired time/RSS,
not proved causality.

`custo-fixo-fontes-item2-2026-10-08/probe-results.json` records three processes per
mode, 17 assets/9, 682, 300 bytes, about 9.24 MiB requested Rust allocation plus
about 9.3 MiB anonymous and 9.3 MiB executable-resident pages. embedded_fonts.rs:92–94
copies static slices; fonts.rs:115–123 retains all Vecs before get. Pinned upstream
kit/fonts.rs:136–141 passes Bytes::new(data), library/foundations/bytes.rs:48–62
specifies direct static backing. Buffer size directly proves duplication; CLI RSS
gain still needs paired experiments. No upstream intent/language parity inferred.

### Current representation and acceptance

Public new_embedded(path, Vec<u8>) retains private Arc<Vec<u8>> made from moved
Vec, no eager validation/loading. source_bytes borrows original bytes. get first
validates requested index in original bytes. Supported reconstructed collection
uses index-zero extracted face/independent owned Font; nonreconstructed path keeps
legacy index-zero validation and clones only the existing Font::Vec Arc. OnceLock
stores first success/failure. Public fields/signatures and embedded requested-index
validation remain unchanged; no global cache/public constructor/Font variant.

The later static-origin contract supersedes only the old static-storage ban for
catalog assets. Internally support &'static [u8], borrowed without copy; catalog
uses it only after metadata validation. get validates requested index/reconstructs
collection where applicable and materializes owned Font::Vec only for the selected
static face, inside OnceLock. Preserve failure/success/post-slot-drop validity,
disk/mmap and L1 Font contracts. No flag/default/phase/public-signature change.

RED→GREEN observes no buffer duplication for a simple public-Vec font, unchanged
bytes/metadata/coverage and post-slot-drop validity; collection retains original
source plus valid independent extracted bytes; invalid index/failure/mmap controls
remain. Physical identity is an internal-cost witness, not language parity. All
load_embedded_fonts slots must reference exact typst_assets slices before first
get; text/selection/PDF/inventory observables remain. Paired exports/full CLI RSS/
time, builds/lint and 1:1/reverse lineage with owner-scoped resealing are required;
never attribute an isolated probe delta to the whole product. Full original
supersession/history remains at
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/fonts.md`.

## Rustdoc maintenance

`infra-imports-docs-ci-2026-10-13/before.json.gz` retains source/owner/spans/original
compiler output, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/
UTC/status/diffstat for fonts.rs:367–370. Inspection distinguishes productive,
descendant-test imports and comments, not new parity. Repair paragraph/list
separation/continuation without technical/test changes. Separate the historical
exception-table correction formerly labeled P840 from the preceding list and
finish the incomplete is_italic sentence without inventing its missing rationale.
Internal ADR-0127: no public/default/phase/formula/acceptance changes or lint
suppression. All-target compile/infra test names/execution/build/workspace Clippy/
Rustdoc/lineage and scoped resealing; no import/comment-format mirror tests.
