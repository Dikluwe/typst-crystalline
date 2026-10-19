# L0 — infra/export/subset
Hash do Código: b083e686

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/subset.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/subset.rs:19` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Subset adapter

Own only FontSubset and subset_font_with_mapping and remap_glyph_id
in the exclusive consumer. Preserve public signatures and FontSubset fields:
data contains subset font bytes; mapping is old GID to new GID.
L1 continues carrying original glyph IDs; it does not know this adapter.

Use subsetter::GlyphRemapper to include .notdef explicitly (0 → 0), every GID
from char_to_old_gid, and additional_gids such as ligatures. Call subsetter::subset
at face index zero on the already extracted individual-face bytes. Build mapping
from that remapper, not from a subset cmap; removal of cmap is intentional.
Return None on subsetter failure, retaining the existing test-only failure message
"subsetter::subset falhou: {:?}". Return valid TrueType or CID-keyed CFF subsets
when supported by the library. The mapping adapter retains default-coordinate behavior. A private sibling adapter accepts variation coordinates from the font builder and delegates instancing plus subsetting to the installed subsetter 0.2.6 API, without changing public signatures.

The unused internal legacy subset_font wrapper is retired by the 2026-10-16 maintenance below. remap_glyph_id returns the mapped ID
or zero for an absent key. Whether an empty mapping means full embedding is a
caller decision, not a different meaning of that helper.

## Ownership boundaries

- `00_nucleo/prompts/infra/export/builder/fonts.md` owns fallback to full
  embedding, variable instancing, font names, descriptors, widths/ToUnicode
  orchestration, CIDSet and the font compression threshold.
- `00_nucleo/prompts/infra/export/fonts.md` owns Unicode/cluster helpers.
- `00_nucleo/prompts/infra/export/stream/text.md` owns mapped text operators.
- `00_nucleo/prompts/infra/export/bitmap_glyphs.md` owns raster collection.

Do not duplicate those contracts or claim that bitmap-only fonts must be embedded.
Unsupported CBDT/CBLC subsetting retains None/fallback rather than invented support.
Shaping positions already supplied by L1 do not depend on preserving shaping tables
in the embedded subset.

## Verification

Check .notdef for empty input, char and additional-GID mappings, absent remap,
parseable TrueType/CFF fixtures and text extraction through the caller integration.
Exact subset size, object IDs or equality to an upstream subset algorithm are not
language-parity criteria.

## Variation-aware private adapter

### Evidence

subset.rs:56 originally called subset without coordinates. Installed subsetter
0.2.6 src/lib.rs:108–126 distinguishes default subset from subset_with_variations,
which instances/subsets together and converts CFF2→TrueType.
`lab/results/construction/diagnostics/eixos-variaveis-export-2026-10-05/native.txt`
measures H=689/734 at wght=250/900 versus old frozen H=697, with probe/provenance
in before.json/native.rs. variable-cff2-after.pdf still contained a no-explicit-
axis space resource with CFF2/fvar/HVAR, while axis-bearing resources were static
TrueType; Poppler emitted a residual warning. This requires separating legacy
public no-coordinate subset from private explicit-coordinate preparation, even
when the coordinate request is empty.

### Contract and acceptance

Private pub(super) subset_font_with_variations accepts mapping inputs plus
coordinates ttf_parser Tag/f32, reuses remapper/mapping/error path and converts tags
without rounding axes. Public APIs keep default subset/legacy CFF2 behavior; the
private adapter uses subset_with_variations even for empty coordinates to resolve
the default instance/CFF2→TrueType. Axis/fallback/descriptor decisions belong only
to `00_nucleo/prompts/infra/export/builder/fonts.md`. No new public API/dependency/
unsafe/phase. Require PDF-owner tests/parseability/font widths, RED→GREEN and
revalidation; test default Cantarell through the private adapter and a readable
resource in the complete PDF control.

## Internal binding and retired wrapper

### Evidence

`avisos-clippy-tekt-2026-10-15/before.json.gz` retains sources/owners/HEAD/uncommitted
tree/UTC/diffstat; selected-red.json records compiler diagnostics. subset.rs:85–87
binds an error used only in cfg(test) diagnostic. `clippy-retencao-2026-10-16/before.json.gz`
retains source/L0/global hashes/HEAD/tree/UTC/diffstat: export/mod.rs:42 privately
declares subset without reexporting subset_font; all-layer/bench search finds
only its definition at subset.rs:113 and two local test calls. Productive adapters
are subset_font_with_mapping/subset_font_with_variations. pub inside an unreexported
private module is not externally reachable API. This source/use evidence refutes
the prior compatibility-wrapper requirement, without a new parity claim.

### Contract and acceptance

Rename only the test-diagnostic binding to _error, retaining the same message,
None result/fallback. Internal continuous ADR-0127 preserves public/default/phase,
adds no suppression/syntax-mirror test; selected warnings GREEN then existing
suite/build/all-target Clippy/fmt/doc/lineage with scoped resealing.

Remove only subset_font and legacy-wrapper docs. Migrate its two fixtures to
subset_font_with_mapping(&data, &BTreeMap::new(), &used), taking FontSubset.data.
For GIDs 65/66 or empty set preserve remapper order and explicit .notdef. Keep
test names/assertions and every other adapter/signature/None/mapping/variation/
message/fallback. Do not duplicate GID→character conversion in tests, add an unused
replacement helper or expose a symbol to silence lint. This ADR-0127 §2 item 4
internal change breaks no consumer outside the module and supersedes only old
wrapper-preservation clauses. L0 first, subset controls/dead_code RED→GREEN,
owner-only resealing, all-targets/workspace/build/doc/fmt/lineage under 12 GiB,
zero swap, one job. No allow/expect or expected/tolerance changes.
