# L0 — Exact cmap coverage extraction by ranges
Hash do Código: 4e66a279

Layer L3. Exclusive productive consumer: `03_infra/src/fonts/coverage.rs`. Package ratified by owner `confirmo` on 2026-10-07; current canonical owner.

## Evidence before decision

`lab/results/construction/diagnostics/cmap-intervalos-2026-10-05/{measurement-direct,measurement,execution}.json.gz`
retains program/recipe/sources/binaries/HEAD/dirty state/UTC/argv/samples. Isolated
extraction/union changed 53.19→10.85ms; seven synthetic tests passed. That state's
1, 112 installed faces had 1, 520, 598 intervals/3, 582 native subtables/no fallback.
These are historical counts, not current counts/CLI speedup.
coverage-ranges-preparacao-2026-10-07/audit.json confirms pure consumer byte-identical
to measurement and old L3 extractor identical to clean HEAD; other tree/current
release differ and require post-implementation measurement.

Primary ttf-parser0.25.1 format4.rs:103–115/format12.rs:68–75/format13.rs:41–48
callbacks enumerate declared bounds without glyph0 filtering. cmap/mod.rs:196–219,
251–260 uses typed get/cmap-relative offsets and stops at first unparseable record.
Format4 parse uses segCountX2/2/full arrays; length field does not justify new
incompatible validation. Historical vanilla
`9f2b6e8715237cb086899a42873660fe744622e8` font/info.rs:119–134 enumerates/joins
points without this direct mechanism. No intent inferred or current-pin certificate.

## Decision and private adapter contract

Use native segments only for typed-parser-accepted 4/12/13. Preserve Unicode
classification/subtable order/termination/all prior declared members. This L3
owner alone selects/validates byte paths; union/encoding belongs only to
`00_nucleo/prompts/core/entities/font_book.md` through Coverage::from_ranges.
No parser types in L1 or approximate Unicode-block semantics. Parent
`00_nucleo/prompts/infra/fonts.md` retains extract_coverage(&Face)->Coverage facade;
this private child reads only supplied Face bytes, no catalog/discovery/shaping/
FontInfo ownership, public API/counters/flags/lab imports/unsafe/globals/new cache.

1. Missing cmap yields empty Coverage. Visit cmap.subtables.get(i) in order; failure
   stops there, never skip invalid record. Ignore typed non-Unicode subtables.
2. Choose native format by typed enum, read this same Face's raw cmap/matching
   record offset, validate raw format/access with checked_add/checked_mul/slices.get.
   Never use another subtable's bytes as origin.
3. Format4 converts declared inclusive start/end to half-open ranges. start>end
   is empty. Stop at singleton FFFF even if later segments exist. glyph0/idDelta/
   idRangeOffset do not filter membership: old callback observes codepoints,
   not mapping validity.
4. Formats12/13 visit declared groups in order, retaining overlaps/duplicates/holes;
   reversed groups empty. Inclusive end→checked_add(1), no saturating/clamping/wrapping.
5. Per-subtable checkpoint: any native read/validation failure rolls back the whole
   attempt and uses its typed callback. Other Unicode formats use callback, no
   additional native support without new evidence.
6. Fallback points compact into connected intervals with local accumulator;
   arbitrary order/duplicates leave union unchanged. Unrepresentable exclusive
   endpoint requires the whole old Face extractor: discard partial attempt,
   enumerate at original boundary, no lost members/fabricated empty interval.
   Existing point-constructor Unicode domain stays; invalid out-of-domain inputs
   receive no new promise.
7. On valid completion call Coverage::from_ranges once. Preserve membership/iter/
   fallback order/canonical equality/hash/slot indexing. L1 owns data, no Face borrow.
8. Return owned Coverage only; no shaping/export/file reread/World/catalog forcing.

## Native-payload reuse

### Evidence and classification

coverage.rs:76–105 reextracts aliased same-cmap-offset/native-format payloads.
`cmap-payload-reuse-2026-10-10/prototype-probe.json.gz` retains full source copies/
hashes/HEAD/dirty tree/UTC/assets/argv/recipe/samples: 1, 140 faces, 1, 111 with
native duplicates, 1, 819 repeated records, 1, 533, 618→767, 936 intervals. Every face
matches received path/legacy callback. Isolated corpus median 17.7808196→11.81227295ms
(three rounds/eight pairs/ten corpora per sample), no CLI gain/RSS promise. Fonts
unchanged across audits. Coverage set/iteration/fallback is language; repeating
same immutable payload union is mechanics. Combined CLI/RSS benefit is refuted by
membership/termination/fallback changes or persistent regression without benefit.
Private reversible continuous ADR-0127, no catalog cache/public/mode/flag/phase/L1
representation change.

### Reuse contract

Within one collect call retain at most eight successful native keys on stack,
no heap: (same-Face cmap-relative offset, native 4/12/13 format). Continue typed
get(i)/order/is_unicode before key lookup and validate matching raw offset/format/
access. Only then an already successful key skips native append. Store only after
complete successful extraction; full list means process further records normally
without remembering, never skip/terminate early.

No content/format-only dedup, ignored distinct offsets, partial/failed/callback-only
memoization, cross-Face/file persistence/borrows or altered L1 union. Keep all eight
adapter clauses, checkpoints/rollback/callback/overflow→legacy. Valid same payload
has same intervals, so repeated union removal is platform/encoding independent.

## Tests and acceptance

Before code use an independent membership/BTreeSet oracle plus legacy callback as
additional control. Cover format4 glyph0/sentinel/multiple/reversed segments;
12/13 holes/overlap/duplicates/zero/max Unicode; non-Unicode/format6/another callback-
only; missing cmap; invalid-record-before-valid; truncated/count/offset/format
mismatch with rollback; unrepresentable private exclusive endpoint. No unit speed
threshold/new malformed-font policy.

Reuse tests: consecutive/separated Unicode aliases, identical bytes at distinct
offsets, invalid marker between aliases, non-Unicode alias, >8 unique payloads and
later duplicates; baseline runtime RED observes duplicate collection. Counts are
mechanical controls, not language definition. Preserve independent membership/
legacy oracle and all productive extraction behavior.

Outside timers compare every installed/embedded/fixture face to old path, recording
file/index/exact hashes/fallback. Preserve selection/fallback/style/text/glyph/
positions/CLI-PDF errors including missing coverage/multiscript. Missing equivalent
font asset is an explicit gap, never parity. Revalidate whole real corpus/fallback/
documents/glyphs/positions/raster/query. Build/core-infra-shell-query suites/lint
exit0, owner no V5/V6/V15/V26 (reuse requires V5/V15/V26); retain received warnings.
Reseal only owner. Freeze current/previous/ratified vanilla binaries for extraction/
catalog/CLI cost/RSS, three monitored/gated rounds with complete state/fonts/argv/
UTC. Repeatable equivalent-work/output benefit, not extrapolated historical 42ms,
defines performance. Preserve negative results/received tree, no commit/reset or
selective-acquisition changes in this package.

## Internal imports, padding and nominal test expectations

`infra-imports-docs-ci-2026-10-13/before.json.gz` retains sources/owners/spans/
original compiler output, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty
state/UTC/status/diffstat at :155. Inspect productive/test-only/comment references;
remove only proven unused private imports, retaining public reexports/required
imports. No public/default/phase/formula/acceptance/suppression changes. All-target
compile/infra test inventory/execution/build/workspace Clippy/Rustdoc/lineage,
scoped seals, no import/comment mirror tests.

`clippy-expressions-2026-10-16/before.json.gz` retains sources/L0/HEAD/tree/status/
diffstat/UTC; initial-diagnostics.json suggestions/spans/types/uses at :216
(clippy::manual_is_multiple_of). Only synthetic test-font preparation replaces
len % 4 != 0 with !len.is_multiple_of(4), preserving zero padding/offset/length/cmap.
Do not change productive extraction/union or single-Range arrays in that slice.
Internal representation claims no parity/performance; continuous ADR-0127 adds no
public/default/phase/dependency/suppression, L0 first/scoped seals, unchanged
controls/tolerances, selected RED→GREEN then workspace suite/build/all-target
Clippy/fmt/Rustdoc/lineage, no syntax-mirror tests.

The separately materialized single_range_in_vec_init exceptions preserve collections
of Range<u32>, not integer collections: literally [2..6], vec![40..42], [40..42],
bounds/checkpoints/partial append/rollback. Only these exact test attributes:

- alias_memoization_respects_invalid_and_nonunicode_records:
  `#[expect(clippy::single_range_in_vec_init, reason = "Esperados são coleções de Range semiabertos de cobertura, preservando o intervalo único 2..6")]`.
- checked_native_reader_rejects_truncation_after_a_partial_group:
  `#[expect(clippy::single_range_in_vec_init, reason = "Fixture e esperado guardam um Range 40..42 para verificar append parcial e rollback de intervalos")]`.

First covers two [2..6] expectations, second fixture/expected [40..42]. No type
change/point enumeration/lint-evasion helper/extraction change/module suppression.
If no longer single-Range remove/review expect; unfulfilled expectations stay error.
This metadata-only continuous ADR-0127 batch narrowly supersedes generic bans for
named items/lints, no executable/cfg/public/default/phase/dependency/architecture
change or pending-correction suppression. L0 first/scoped seals; removing added
attributes/header recovers old rustfmt source. The historical batch requires real
RED then all-target inventory exactly 46 arities/five test warnings satisfied by
49 expectations, no new warnings/unfulfilled expectations, 11 correction arities
still visible; build/fmt/strict-reverse lineage/Tekt no new errors/warnings/existing
Counter+coverage tests under 12 GiB/zero swap/one job. Prior full suite remains
same-executable evidence, no new run/performance claim or attribute-mirror tests.

## Exhaustive native-format selection with callback fallback

### Source evidence before decision

At HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
`03_infra/src/fonts/coverage.rs:71–150` contains `collect`; the V16-090
`match subtable.format` occupies `:86–91`. Only typed formats 4/12/13 select
a native reader; None continues through the existing checkpoint/callback path,
not empty Coverage. `Cargo.lock:2344–2345` pins ttf-parser 0.25.1; its
`src/tables/cmap/mod.rs:35–45` defines all nine Format variants, read from the
installed source at
`/home/dikluwe/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ttf-parser-0.25.1/src/tables/cmap/mod.rs`.
The current source/L0 digests are in
`lab/results/v16-cause-analysis-2026-10-19/entities-boundaries-inventory.json`.
This is static adapter evidence, not a new extraction benchmark or corpus result.

### Decision and preserved complement

Replace the wildcard None with the explicit callback-path variants
ByteEncodingTable, HighByteMappingThroughTable, TrimmedTableMapping,
MixedCoverage, TrimmedArray and UnicodeVariationSequences, preserving payload
shapes and the single None body. Keep native Some(4), Some(12) and Some(13)
unchanged. A future dependency Format variant must require an explicit decision.
Do not implement new native formats or interpret callback selection as a dropped
subtable. A typed callback that emits no points retains that existing behavior;
this does not certify new variation-sequence coverage.

Preserve typed Unicode selection, first-invalid-record termination, same-Face raw
offset/format validation, successful native-payload memoization, per-subtable
rollback before callback and whole-Face legacy fallback on unrepresentable
exclusive endpoint. Preserve declared codepoints, including existing glyph-zero
behavior, order/union membership, no approximate Unicode-block inference and no
new malformed-font policy. No parser type or I/O crosses into L1.

### Acceptance and exact-site evolution proof

Keep the existing coverage tests unchanged, especially
`nonunicode_is_ignored_and_format6_retains_callback_fallback`,
`formats0_and10_keep_callback_membership`,
`format4_retains_declared_ranges_zero_glyph_and_sentinel_stop`,
`formats12_and13_preserve_holes_overlap_and_unicode_max`,
`first_invalid_record_keeps_the_pinned_iterator_termination`,
`unrepresentable_exclusive_endpoint_requires_legacy_face_fallback`,
`checked_native_reader_rejects_truncation_after_a_partial_group`, and all four
native-payload reuse controls. Propose additional independent membership versus
legacy-callback controls for any other callback format with a valid available
fixture. A missing fixture is an explicit gap; do not fabricate new semantics,
weaken the existing expected set or claim complete corpus coverage.

The mutation proof copies the exact locked ttf-parser source into
a disposable workspace dependency and adds a Format sentinel there only. Record
copied dependency identity and how that target is resolved; do not alter the
productive registry, Cargo.lock or public dependency API. Build the real infra
library consumer and require E0004 at collect's format-selection match after
repair, with no E0004 there in the wildcard baseline. Errors confined to the copied
dependency or other matches do not prove this site and must be reported as an
inconclusive probe until the consumer is type-checked. Ordinary no-sentinel
compilation and existing functional controls remain separate required evidence.

This is an internal classification correction under ADR-0127 continuous flow:
no public field, trait, signature, product default, compatibility rule, dependency
version or pipeline phase changes. Preserve the owning layer and static dispatch.
No suppression, severity change, `if let`/`matches!` replacement, or helper that
relocates the unchecked complement is an acceptable repair. Validate V15/V26
before restricted forward/reverse resealing. Freeze source/dependency identity during structural and functional validation;
record exact targets, compiler diagnostics, resource settings and outcomes before
claiming acceptance. A structural proof is not a functional test result.

### Attributable actual-domain omission proof

For V16-090, an equally strict structural alternative avoids a new enum variant
preventing core or external-dependency metadata emission before the real infra
consumer is type-checked. Use two disposable copies with the same unchanged real
dependencies: one with the original wildcard decision and one with its repaired
exhaustive classification. At the collect native-format match at coverage.rs:86–91, remove only
the complete unguarded SegmentMappingToDeltaValues(_) => Some(4) arm in each copy. This arm is unique and unguarded; no other arm
in that match handles SegmentMappingToDeltaValues. With the omission, the wildcard baseline
still covers SegmentMappingToDeltaValues and must lack E0004 at this site; the repaired copy must
emit E0004 at this exact match. Retain function/target identity, mutation diff,
source/dependency hashes, compiler JSON and no-omission controls. An unrelated
error or dependency failure remains inconclusive. No replacement stub, new public
API, linter or target-scope change is allowed.

This paired omission experiment proves that the actual-domain classification no
longer has an implicit complement. The normal repaired match must remain fully
nominal and exhaustive, so future enum growth still forces an explicit owner
decision. This alternative changes only how the structural obligation is measured;
it weakens neither the future-variant obligation nor the independent functional
controls, and no probe result is claimed by the specification.
