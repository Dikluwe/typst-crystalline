# L0 — entities/font_book — exact Unicode coverage and font metadata
Hash do Código: d0e3fa73

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/font_book.rs`
**Applicable ADRs:** ADR-0022, ADR-0030, ADR-0107, ADR-0108, ADR-0129.

## Evidence before the construction decision

- `03_infra/src/fonts.rs:403` collects codepoints from every Unicode cmap subtable and calls the existing vector-based constructor. L3 owns font bytes, discovery and acquisition.
- Pinned vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, `lab/typst-original/crates/typst-library/src/text/font/info.rs:119-134`, builds compact coverage for each ordered Unicode subtable and unions intervals. Its general vector constructor at `info.rs:296` still sorts and deduplicates individual codepoints.
- `01_core/src/entities/font_book.rs:161` stores alternating outside/inside run lengths. The observable contract is exact Unicode membership and ordered iteration, including gaps and duplicates; the collection algorithm is internal mechanics.

The existing vector API must be retained. Compact contiguous input sequences into intervals, order those intervals and merge overlapping or adjacent intervals before encoding the canonical runs. This adapts vanilla's compact interval mechanism without adding a public range builder or importing font bytes into L1. It does not eliminate L3's intermediate vector. Performance claims belong to `00_nucleo/diagnosticos/font-lifecycle-experiment.md` and its reproducible dataset.

## Owned coverage contract

Preserve `Coverage(Vec<u32>)` as an immutable exact set after construction, with the existing `Clone`, equality, hash, debug and default behavior. Cloning copies its vector; no constant-time clone is promised.

Preserve these public signatures:

```rust
pub fn new() -> Self;
pub fn from_codepoints(codepoints: impl Into<Vec<u32>>) -> Self;
pub fn contains(&self, codepoint: u32) -> bool;
pub fn is_empty(&self) -> bool;
pub fn iter(&self) -> impl Iterator<Item = u32> + '_;
```

The input may be unordered and may contain duplicates. Preserve exact set membership throughout Unicode `0..=0x10FFFF`, including zero and the upper boundary. Empty input yields empty coverage. Iteration yields each member once in ascending order. Never replace exact coverage with a block approximation.

Encode alternating lengths outside and inside the set, starting outside at zero. The set `{2,3,4,9,10,11,15,18,19}` encodes as `[2,3,4,3,3,1,2,2]`. Canonical encoding must be independent of input order, subtable overlap and duplicate occurrences.

During construction, compact adjacent input points into inclusive intervals. An interval may extend at either edge; a repeated point already inside it changes nothing. Sort intervals by their start, merge overlap/adjacency and emit the existing gap/count pairs. Arithmetic at the Unicode upper boundary must remain valid. Contains scans runs linearly, stopping once its cursor exceeds the requested codepoint. Iteration expands inside runs. No I/O, mutable globals or external font parser types are allowed.

## Font metadata and catalog contracts

Preserve `FontInfo { family: String, variant: FontVariant, flags: FontFlags, coverage: Coverage }`, existing font style/weight/stretch types, clamps, symbolic names, defaults, distance functions and trait behavior. `FontFlags` retains its existing monospace and serif fields. L3 populates metadata and owns whether coverage is extracted during discovery or on demand. A populated coverage is exact and immutable.

`FontBook` remains a pure ordered collection of `FontInfo`. Push appends; indices preserve acquisition order. Existing new/default, infos, len and is_empty behavior and public signatures remain unchanged.

Family selection is case insensitive and chooses by weight distance, then style distance, preserving the existing tie behavior. Pattern selection uses the existing literal/regex matching rules and the same variant ranking. Family enumeration preserves insertion order. `candidates_for_char` returns precisely the indices whose populated coverage contains that character, in catalog order, without opening font faces. Its cost is linear in catalog size with a linear run scan per candidate.

Fallback selection accepts already covering indices, skips invalid indices and preserves the first candidate on a complete score tie. Preserve existing ranking: likeness by monospace, serif, shared family-name prefix words and shorter family name, then variant distance by style, stretch and weight. Do not introduce vanilla's variable-font preference or axis fields here.

## Boundaries and verification

Font storage, mmap-backed bytes and the `Font` API belong exclusively to `00_nucleo/prompts/world-types.md`, whose consumer is `01_core/src/entities/world_types.rs`. This prompt does not own that consumer. System catalog lifecycle belongs to `00_nucleo/prompts/infra/system-world/fonts.md`; byte parsing and extraction belong to `00_nucleo/prompts/infra/fonts.md`. Neither L3 acquisition policy nor shaping is decided here.

Verify exact membership and ordered iteration against an independent set oracle for duplicate and overlapping subtables, reversed ranges, arbitrary order, gaps, zero and the Unicode maximum. Preserve catalog selection and fallback tests. Acceptance is set/selection semantics; wall-clock thresholds are not unit-test requirements. The historical bitmap and vector-sort decisions remain evidence in the diagnostic dataset, not current acquisition policy.
