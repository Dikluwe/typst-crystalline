# L0 — infra/font_metrics/resolution
Hash do Código: 268111a2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:934979e5e3d19d62b8dff899dcc462783b0522bf8bbde4c2bdbd1385e4c7f949
- 00_nucleo/prompts/_nuclei/fonts/primary-resolution-cache.toml sha256:34e78c2ed34beeefbbd0e6a025b94322f27dadce8641ae58fe6a074068a1f30a


**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/resolution.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/font_metrics/resolution.rs:17` — `FontCandidate`.
- `03_infra/src/font_metrics/resolution.rs:27` — `resolve_primary`.
- `03_infra/src/font_metrics/resolution.rs:86` — `resolve_primary_with_math_fallback`.
- `03_infra/src/font_metrics/resolution.rs:118` — `covering`.
- `03_infra/src/font_metrics/resolution.rs:193` — `resolve_font_combo`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/resolution.rs`.

## Owned contract

- Own FontCandidate and primary/MATH/covering/FontKey resolution. Preserve declared-family order, class fallback when primary lookup fails and first-resolved-family similarity reference.
- Append explicit math fallback faces when style.math is true, with slot deduplication; preserve the existing readable-face check without adding a MATH-table filter at insertion. Covering lookup first tries MATH-covering candidates, then primary coverage; preserve None and ordering.
- Global fallback uses World::candidates_for_char and FontBook::select_fallback. Coverage is authoritative for global filtering; load the winning eligible face only, not intervening slots or a second full catalog scan.
- Resolved identity includes family, variant and variations. Do not infer math semantics from family spelling; consume the pinned shared fallback-selection nucleus.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Common fallback and bounded primary reuse

### Evidence

resolution.rs:27–65 previously used only a resolved primary, while historical
vanilla text/mod.rs:1083–1101 appends the common chain before global search.
The Hebrew correcao-rtl-restante-2026-09-29/after-leading probe shows a different
emitted face. Inference: independent metrics may diverge without the common chain,
refuted by an already identical measured face without alteration.

`content-cost-profile-2026-10-10/{resolution-calls,cpu-style-current,checked-phases}.json.gz`
retains HEAD/tree/UTC/scripts/sources/commands/release
`9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`.
1, 600-cell tables call select_pattern 48, 019, resolve_primary 6, 401,
text_edges/vertical_metrics 3, 200/3, 201; selection occupies 25–29% inclusive CPU
samples. resolution.rs:27–85 uses FontList/effective variant/face readability, not
size/variations; :105–124 adds MATH separately; facade :49–86 shares clone-local
caches. Reusing the last resolution may amortize scans without growth; altered
identity/chain or lock/retention costs eliminating gain refute the hypothesis.

### Contracts and acceptance

Append the ordered/deduplicated common chain after readable declared families in
text mode; math retains math fallback before common. Coverage measurement and
emission cannot choose different families merely from catalog order.

Define private entry/key and prove exact identity before base-chain lookup. Hit
clones existing candidates; miss runs the unchanged algorithm, publishing only
nonempty results with every selected face readable. Replace the prior entry;
release lock before resolving/loading. Slots/units use original face; variations/
final FontKey resolve on the existing path and MATH additions stay outside this
cache. Consume the pinned nucleus without signature changes.

Continuous ADR-0127: repeated-query RED across clones; family/regex/order/variant,
independent instances/transient failures/math/variation controls against fresh
instances, quantitative allocations/paired-time gate, preceding-release-equivalent
documents/many-identity refuters/core/infra/build/lint. Observe language selection/
metrics/origin/effects/geometry, never copied vanilla mechanics or hash-inferred
hits. Remove on divergence or robust regression.

## Internal casts and fixture initialization

### Evidence

`ci-simplificacoes-2026-10-14/before.json.gz` retains source/owner/spans/HEAD/
uncommitted tree/UTC/diffstat at resolution.rs:66,85,107,154,220; red.log records
selected-lint errors before editing. Uses/types were checked under pinned 1.92.0.
`field-reassign-2026-10-15/before.json.gz` retains source/spans/prior L0/HEAD/tree/
UTC/diffstat; initial-diagnostics.json records default-then-assignment test fields
at :302, 426, 555, 598. Both are Rust-representation maintenance, no new parity or
product-default change.

### Contracts and acceptance

units_per_em already returns u16: retain max(1), remove only u16→u16 casts in
primary/common/MATH/covering chains. Keep order/deduplication/face identity/cache
eligibility. No public/default/phase/unsafe/dependency/suppression changes. L0
first, owner-only resealing; existing functional controls with exact fixtures/
expectations/tolerances, selected-lint RED→GREEN as errors, full suite/build/
all-targets/Rustdoc/fmt/lineage/full inventory; no simplification-mirror tests.

Initialize font, plus math in the two math controls, directly in four TextStyle
literals. Keep families/mocks/query counters/MATH-face selection/fallback checks
and productive algorithm. Use ..Type::default() for other fields, remove mut only
when no mutations remain; no new types/builders/APIs/suppression. Preserve names/
inputs/expectations/tolerances; selected RED→GREEN/all-target Clippy/workspace
suite/build/fmt/Rustdoc/lineage/scoped resealing, no initializer-syntax tests.

R3 fallback-selection pin correction: consume explicit semantic math mode for additional mathematical fallback; a font table alone does not change language mode. This consumer retains its existing operations. Evidence and refuters: `00_nucleo/prompts/infra/shaper/font_selection.md` and source-bound R3 receipts. Catalog entries, declared families, public contracts and phases remain unchanged.


## R10 — expanded signed-tracking, fallback-space and literal-tail refuters

Measurement before decision: candidate2-build/receipt.json/source.json.gz and
candidate2-matrix/receipt.json bind exact dirty inputs, UTC, sources and binary
f2a61237c15688e655ff7c62a117cb50a4d3a154e426bd31bed61fa999385e68
against the fixed bd5ebec2 oracle. Original15 script PDF/SVG controls and all
R8/R9 controls pass; the54 additional controls expose22 failures. At Nimbus11,
trailing literal LF loses a3.058pt following markup space; explicit DejaVu
fallback overmeasures that space by0.43858pt. At DejaVu11 tracking0.2pt, the
script's three intervals overadvance by0.18018pt. Nimbus synthetic digits need
0.12pt intervals and the following fallback glyph needs the same last-prefix
interval rather than zero. Pinned inline/shaping.rs:1269-1295 divides tracking
by outer ctx.size before each glyph's actual reduced size is applied; :473-510
retains outer-size line edges. Source sequence/space causes and actual selected
text face explain the remaining cases, without a font-name branch or looser
physical-word/page/0.001pt gate. This is private parity integration of the
approved method, not another public contract/default/phase change.

### Exclusive owning obligation

Own a private covering_text helper: choose the first actual primary face covering a character, then use existing authoritative global coverage/scoring when none covers it. Reuse existing cache/variant/face data. Preserve covering's MATH priority and every existing caller; new script resolution and ordinary direct text advance opt into explicit text semantics. This supersedes no mathematical selection rule or ordinary longest-run shaper.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.


## Ratified selective-font consumers — 2026-11-12T19:19:44.102371+00:00

Measurement before decision: structural/repeat2/receipt.json isolates about 13ms
of catalog freshness work, not a safe optimization. selection-proof/receipt.json
checks actual prefix metadata/bytes/indices and minimum-score winners; impact.json
identifies eager chain and late PDF requests. Source font_metrics/resolution.rs:95
and shaper/font_selection.rs:73 construct unused later families; shaping.rs:302,542
and pipeline/resources.rs:214 demand full book for individual selection/metadata.
The approved World query contract now has default-method GREEN; foundation RED
proved ordinary prefix selection unnecessarily invoked the full loader, and GREEN
removes that demand with stable paired immutable data. No new performance is yet
claimed. User explicitly authorizes this candidate only in RAM, no integration.

Selective queries preserve complete current selection/metadata/bytes. A private
first-primary lookup follows the unchanged ordered declared-family/readability
policy; if none resolves, use the existing class fallback then common list. It
is NOT a complete-chain result and never enters a PrimaryResolution cache.
Existing complete functions/caches remain the fallback and regression oracle.
No family-name/phrase/fixture/paragraph-count special case, negative memoization,
cache lifetime extension, new public method, dependency, unsafe, phase or mode.
This narrowly supersedes previous mandatory eager full-chain query order only
where the result is proven unchanged, not source font/fallback/script semantics.

For ordinary metrics that use only the first readable face (vertical/edge/cap
properties), query that face directly. For direct ordinary text advance, a first
face with nonzero cmap coverage of every authored character already wins the
existing per-character covering_text rule. Math/native scripts/uncertain coverage
keep the full route. This proof concerns the unchanged direct-metric operation;
it is not certification of a raw shaping result.

Shaping may use a one-face primary vector only when non-math, non-superscript,
non-subscript and not a resolved BidiSpan occurrence. Prove full authored-text
nonzero cmap coverage in the first readable face; then run the EXISTING bidi and
script segmentation with that sole candidate and the existing raw owner, actual
ordered axes, language, direction, script=None/features=[] policies. Require
nonempty raw results and no zero glyph. Actual emission/measurement uses the same
unchanged segmentation and raw inputs and can hit the exact current-compilation
raw store. Any failure/uncertainty uses the unchanged full primary chain before
producing output. No partial primary result is cached as complete. No adjusted
glyph/position/width/cache-key changes. This certificate covers each original
sub-run, not a whole-text shape used to bypass script segmentation. Terminal
measurement keeps its current output algorithm; using the raw store for the
additional ordinary certificate does not replace its existing terminal policy.

RED first: complete ordinary PDF compilation must not invoke a full-catalog
loader, including metrics, shaping, metadata and export-resource resolution.
Verify full path on uncertain/default/regex/mutable/missing/longest-run cases;
keep existing snowman, scripts, math, axes, raw identity, missing-face and cache
controls. A successful prefix followed by a fallback request must match a fresh
full result and not poison complete caches. Full source/font-update/PDF/SVG/raster
controls and paired release CPU/allocation measurements remain required.

Exclusive owning obligation: Own a private first_primary lookup and ordinary full-text direct-metric candidate certificate. Existing resolve_primary and math functions remain complete and publish only full chains; selection-only queries call World::select_font, single-record queries font_info. Preserve global fallback book/scoring and MATH priority. Do not return a partial result through the existing complete resolve_primary API.


## Laboratory first-query reuse correction — 2026-11-12

Evidence precedes decision: fontdemand-fallback-comparison/freeze.json and summary.csv record unchanged release d7c9024946a3ffde6f5ca0825829eeff29c7b17f60ce26b6ef69670a0e39c7a8 regressing in system and nonminimum-variant workloads. resolution.rs:first_primary and font_selection.rs:first_candidate query World again whenever no complete chain was populated; the selective successful path never populates that chain. phase-counts/receipt.json records 1115 certifications for 113 actual shapes on layout, not a timing attribution.

Keep a DISTINCT latest successful first-readable-query result, scoped exactly like the existing per-compilation face/metrics cache. Key by exact full ordered FontList (optional for metrics) and effective FontVariant. It answers only first-face queries and can never substitute for a complete primary/fallback chain, glyph coverage, bidi splitting or raw-shape certification. Store only key, slot and units-per-em; no additional bytes, global state, cross-World/compilation reuse, negative selection cache or public API. Query/read without holding the metrics selection lock. Do not publish if any earlier selected face failed loading/parsing; shaper conservatively refuses while any existing face-cache failure is present, preserving its old negative-face policy. Existing full-chain hits remain usable and take precedence. New starts empty; metrics Clone shares first-query state only with the same World. Size/color/language/axes retain existing downstream application.

Continuous ADR-0127 internal repair. Require counter RED before implementation; repeat hits do not query catalog, keys and Worlds remain isolated, failed faces keep prior retry policy, and subsequent full-chain resolution equals fresh output. Preserve whole-PDF no-acquisition control, all prior semantic tests and font freshness. Rebuild/lint/paired nominal and forced-full regressions before acceptance; laboratory only.

Exclusive obligation: Own the private first-query record and first_primary hit/publication rules with exact key and complete-readable-prefix proof, plus independent counter/refusal/key/chain tests.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.
