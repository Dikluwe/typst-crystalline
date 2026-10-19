# L0 — entities/font_book — exact Unicode coverage and font metadata
Hash do Código: 4582deac

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/font_book.rs`
**Applicable ADRs:** ADR-0022, ADR-0030, ADR-0107, ADR-0108, ADR-0129.

## Evidence before the construction decision

- `03_infra/src/fonts.rs:403` collects codepoints from every Unicode cmap subtable and calls the existing vector-based constructor. L3 owns font bytes, discovery and acquisition.
- Pinned vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, `lab/typst-original/crates/typst-library/src/text/font/info.rs:119-134`, builds compact coverage for each ordered Unicode subtable and unions intervals. Its general vector constructor at `info.rs:296` still sorts and deduplicates individual codepoints.
- `01_core/src/entities/font_book.rs:161` stores alternating outside/inside run lengths. The observable contract is exact Unicode membership and ordered iteration, including gaps and duplicates; the collection algorithm is internal mechanics.

The existing vector API must be retained. Compact contiguous input sequences into intervals, order those intervals and merge overlapping or adjacent intervals before encoding the canonical runs. The earlier refinement kept this API without adding a public range builder or importing font bytes into L1. The range-constructor decision below explicitly updates that restriction; font bytes remain outside L1. Performance claims belong to `lab/results/construction/diagnostics/font-lifecycle-experiment.md` and its reproducible dataset.

## Current fixed-cost evidence and internal refinement

`lab/results/construction/diagnostics/custo-fixo-2026-10-05/audit.json.gz` records the current dirty-tree/binary/input provenance. Its catalog diagnostic composition measures roughly 46 ms in this constructor, after collecting 28,689,553 codepoint occurrences for 1,112 system faces. This is a nested component measurement, not a decomposition of plain CLI medians. `compression.json.gz` compares the current real constructor with an isolated scalar-interval implementation across three processes and nine batches each: about 44–45 ms versus 23–25 ms, with canonical encoding equal for every face. These measurements precede the production refinement; final CLI gains remain to be established.

Keep the vector input and the existing sorting, merging and gap/count encoding. During the initial compaction, retain the active inclusive interval in local scalar start/end variables instead of accessing and updating the last Vec entry for every codepoint. Flush an interval only when a point is disjoint and nonadjacent, then flush the final interval. Empty input has no active interval. Check forward adjacency first for ordered cmap streams, then membership and reverse adjacency; arbitrary input order, overlap, duplicates and both interval edges remain supported. Checked adjacency arithmetic and the Unicode upper boundary remain valid. No API, acquisition policy, pipeline phase, unsafe or mutable global changes.

Classification: internal mechanics, with set membership/ordered iteration as semantic acceptance. Existing independent set-oracle tests are run before materialization and after; they already pass on the baseline. Do not invent a semantic RED for a performance-only change or place wall-clock thresholds in ordinary unit tests. The preimplementation compression measurement is the cost baseline; the isolated candidate, complete CLI comparison and output controls decide acceptance. Preserve original sources and results even if final performance refutes the experiment.

## Evidence and decision for a pure range constructor

`lab/results/construction/diagnostics/cmap-intervalos-2026-10-05/measurement-direct.json.gz` and `measurement.json.gz` preserve exact source/binary/input/dirty-tree provenance for the independent experiments. `03_infra/src/fonts.rs:404–415` currently enumerates and retains individual codepoints. The installed-font control has 1,112 faces and 28,689,553 point occurrences; extraction into native ranges has 1,520,598 input intervals. Simple range sorting costs about 20 ms per catalog in the first experiment. Compacting adjacent/overlapping incoming intervals before sorting costs about 10.8–10.9 ms against about 53 ms for the existing point-extraction/constructor composition in the second. Every supported face had equal canonical encoding; no legacy face fallback occurred in this host control. This is an isolated coverage experiment, not a measured CLI gain.

Measured parser anchors: ttf-parser 0.25.1 `tables/cmap/format4.rs:103–115` enumerates declared inclusive segment bounds and stops at the singleton FFFF sentinel; `format12.rs:68–75` and `format13.rs:41–48` enumerate inclusive group bounds without checking the glyph ID. Raw byte access and eligibility remain L3 concerns. The experimental adapter lives under diagnostics, not in productive L1/L3. The pinned vanilla mechanism at `lab/typst-original/crates/typst-library/src/text/font/info.rs:119–134` streams declared codepoints into a builder and unions subtable coverage; it does not prove direct range extraction equivalent.

Decision/classification: add a public pure constructor for neutral integer ranges, keeping union/canonicalization in L1 and byte parsing in L3. This extends the public contract and requires ADR-0127 ratification before materialization. Existing public signatures, metadata representation, discovery policy and pipeline phases remain unchanged. No foreign parser types, unsafe, I/O, mutable globals or new public entities are introduced.

New constructor:

```rust
pub fn from_ranges(
    ranges: impl IntoIterator<Item = std::ops::Range<u32>>,
) -> Self;
```

Ranges are half-open: start is included and end excluded. Empty and reversed ranges contribute no members. Nonempty ranges may be unordered, overlapping, adjacent or repeated. Build their exact union, preserving the existing canonical gap/count encoding, ordered iteration, membership/equality/hash behavior. Unicode zero and the complete supported upper boundary are representable through `0..0x110000` and `0x10ffff..0x110000`. Range endpoints are ordinary u32 values; do not clamp to Unicode blocks or introduce char/glyph approximation. The exclusive-end representation cannot express a member at u32::MAX; this does not broaden the existing point API's supported Unicode domain.

Compact connected input ranges using local start/end scalars, flushing only at a disjoint/nonadjacent range or end of input. Two ranges connect when their closed endpoint tests permit overlap or adjacency (`next.start <= active.end && active.start <= next.end` for half-open bounds); update with min(start)/max(end). Ignore empty/reversed inputs before compaction. Sort the resulting intervals by start, merge globally and encode the existing gaps/counts. Shared normalization/encoding remains owned by this module; from_codepoints may delegate its already compacted intervals without changing its public vector signature or exact supported point semantics.

Acceptance after ratification: implement the public constructor through a genuine runtime RED with a temporary empty implementation, then GREEN with independent set-oracle tests of unordered/overlapping/reversed/empty intervals, gaps, adjacency, zero and the Unicode maximum. Preserve all existing point-constructor and catalog tests. The adapter owner's separate L0 must precede any productive byte/range extraction; synthetic controls cover formats 4/12/13, fallback formats, Unicode eligibility, glyph-zero declarations, sentinel behavior, invalid-record termination and legacy fallback. Validate complete CLI/PDF/font outputs and paired performance before asserting production gain. Measurements support the experiment, not an unconditional speedup promise.

## Owned coverage contract

Preserve `Coverage(Vec<u32>)` as an immutable exact set after construction, with the existing `Clone`, equality, hash, debug and default behavior. Cloning copies its vector; no constant-time clone is promised.

Preserve these public signatures:

```rust
pub fn new() -> Self;
pub fn from_codepoints(codepoints: impl Into<Vec<u32>>) -> Self;
pub fn from_ranges(ranges: impl IntoIterator<Item = std::ops::Range<u32>>) -> Self;
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

Font storage, mmap-backed bytes and the `Font` API belong exclusively to `00_nucleo/prompts/core/entities/world_types.md`, whose consumer is `01_core/src/entities/world_types.rs`. This prompt does not own that consumer. System catalog lifecycle belongs to `00_nucleo/prompts/infra/world/fonts.md`; byte parsing and extraction are delegated by `00_nucleo/prompts/infra/fonts.md` to its private owner `00_nucleo/prompts/infra/fonts/coverage.md`. Neither L3 acquisition policy nor shaping is decided here.

Verify exact membership and ordered iteration against an independent set oracle for duplicate and overlapping subtables, reversed ranges, arbitrary order, gaps, zero and the Unicode maximum. Preserve catalog selection and fallback tests. Acceptance is set/selection semantics; wall-clock thresholds are not unit-test requirements. The historical bitmap and vector-sort decisions remain evidence in the diagnostic dataset, not current acquisition policy.


Ratificação: dono confirmou o pacote coverage-ranges-2026-10-07 com `confirmo` em2026-10-07; manifesto f8f7805c6d2ca08d1d14296ed5c93767c416b536fa9c17532eaa8310ca93a549.

## Explicit range fixtures for Clippy — 2026-10-13

Measurement: Rust 1.92 Clippy reports the intentional reversed range at
`01_core/src/entities/font_book.rs:913` and the singleton range collections at
`:969` and `:973`. The owned constructor contract above requires reversed
ranges to contribute no members and covers the complete Unicode domain.
Decision: express the invalid interval with explicit Range start/end fields,
and a single valid interval with `std::iter::once`. Preserve all bounds and
assertions. These are test-fixture spellings only; never turn the invalid range
into a descending iterator or expand a range collection into codepoints.
