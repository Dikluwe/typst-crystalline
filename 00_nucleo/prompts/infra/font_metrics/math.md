# L0 — infra/font_metrics/math
Hash do Código: c89e17f5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml sha256:283876383094ef7a2af90ad0598b9ca3ca5cbe0f7fc7714546cc8758da40878d

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/math.rs`

## Current evidence

- `03_infra/src/font_metrics/math.rs:17` — `extract_variants`.
- `03_infra/src/font_metrics/math.rs:52` — `extract_assembly`.
- `03_infra/src/font_metrics/math.rs:101` — `extract_variants_horizontal`.
- `03_infra/src/font_metrics/math.rs:159` — `extract_assembly_horizontal`.
- `03_infra/src/font_metrics/math.rs:207` — `build_math_glyph_reverse_map`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/math.rs`.

## Owned contract

- Own MATH constants/variants/assemblies/kern extraction, reverse cmap and canonical ssty eligibility/substitution helpers. Translate parser data into existing neutral L1 types only.
- Keep every existing MATH field in design units and percentages divided by 100. Missing tables fall back; a present zero value is not missing.
- Variant stretch advance and hmtx horizontal advance remain distinct. Attachment uses the table value or (hmtx+italic-correction)/2. Assemblies preserve connectors/extenders, horizontal advances and minimum overlap.
- Kern tables preserve four quadrants with n heights and n+1 values, including the final unbounded None. Reverse cmap retains first base for a shared glyph, including large operators and existing base/underscore fallback.
- ssty eligibility is one character or an ASCII numeric sequence with at least one digit and at most one decimal point. Only math Script/ScriptScript outside TextItem requests level 1/2; missing GSUB leaves the base unchanged. No family-name special case or duplicated shaper predicate.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Avoid discarded reverse cmap without MATH variants

### Evidence

`lab/results/construction/diagnostics/cpu-rebaseline-2026-10-10/{profiles-normalized,font-selection-probe,font-resources}.json.gz`
retains source/HEAD/tree/UTC/binaries/selection: 800-weight corpus uses three
non-MATH faces but 800 resources. Profile contains 6, 070 inclusive reverse-map
samples, not exclusive time. math.rs:239–254 builds cmap_reverse before querying
variants/assemblies; all four extractors :17–202 return empty without MATH/variants,
so intermediate work is discarded and result empty.
math-reverse-map-skip-2026-10-10/{before, allocation-equal-red}.json.gz retains post-
Cell HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/release
`71e968c20d5ba32c652b2b66c4fd444c672158ba9bb27b0272f59cfaa5530c78`.
variants-1600 baseline requests 1, 117, 479 glibc allocations. This supports removing
private work, not vanilla corpus equivalence: resource geometry/identity still differ.

### Contract and acceptance

Return empty HashMap before traversing cmap only if parsed face lacks MATH variants.
No MATH or no variants leaves all extractors empty; present variants keeps the
whole algorithm/subtable order/first-wins/real Unicode/fallback/vertical-horizontal
assemblies/large operators. No family/glyph-count/weight/resource-byte heuristic,
FontKey/binding/axes/subset/fallback/font-selection change. Private continuous
ADR-0127 early return, no global cache/new API.

Before code semantic controls (already passing baseline) cover embedded non-MATH,
MATH-without-variants and variants with existing assembly/variants/real ToUnicode.
Resource RED requires variants-1600 ≤1, 116, 479 allocations (≥1,000 below baseline)
and no families/no-fill increase. GREEN must qualify old/candidate text/pages/
bboxes/raster/warnings including math/variants corpus. Paired native time/RSS;
remove if output changes or gain fails. font_metrics tests/build/lint/scoped seals.

## Explicit MathSize ssty decisions

### Evidence

`lab/results/construction/diagnostics/tekt-v16-ssty-2026-10-17/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
status/diffstat/sources/L0 and 2, 128 hashes verified against previous closure.
import-syntax-2026-10-17/validation-architecture.log has 117 V16 including
font_metrics/math.rs:337 and shaping.rs:28. Actual MathSize at layout_types.rs:147
is Display/Text/Script/ScriptScript. Existing functions return Some(1/2) for script
sizes, wildcard None for other two, silently admitting future variants. Existing
L0 already defines math/TextItem/eligibility, no new language decision.
before-probe-result.json retains sources/hashes/rustc/argv/UTC and 224 controls;
probe extracts actual functions/enum, models only three read TextStyle fields.
Disposable future TektSstyFuture compiles both matches (zero E0004): that missing
growth protection is structural RED while current functional controls pass.

Rust enum maintenance is mechanics, no new parity or measured AI-reading speed.
Display/Text intentionally request no ssty under pinned nucleus; Script/ScriptScript
retain distinct levels. Any control/text replay/regression divergence refutes
preservation. No cosmetic enumeration replacing semantic judgment.

### Contract and acceptance

In ssty_level_of preserve signature/initial guard; replace only wildcard with
MathSize::Display | MathSize::Text => None. No eligibility/GSUB/reverse-cmap/other
function change.

Pinned fonts/ssty-math-size supplies shared policy. Continuous ADR-0127 changes no
public/default/phase; public fields/enum remain byte-identical. Record structural
RED/functional controls before code. Same future-variant probe afterward must
produce exactly two E0004 at audited matches and preserve 224/224 observations.
Partial TextStyle model does not prove full shaping: existing real font_metrics/
ssty/TextItem integration/workspace/final gates remain required. No match-mirror
tests; growth guard is the verified new property. Replay allows only prescribed
nominal repair/header hash. Differential SARIF removes precisely two V16, adds two
V19 Display/Text notes with rationale/coverage for both sizes; every other source/
rule/severity/message unchanged. No V20/suppression/config change. V15/V26 graph
before two-pair scoped seals; build/fmt/strict all-target Clippy (-D warnings,
-D unfulfilled_lint_expectations)/strict Rustdoc/V1/V5/V15/V26/reverse lineage/
full suite/diff --check/integrity. Heavy commands sequential MemoryMax12GiB/
High10GiB/swap0/jobs1; no commit/push/merge/release. General objective remains open.


## Fraction slash reverse mapping — current measured correction

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/
fraction-slash-map-findings-before-decision.json pins immutable build1/release
699b6e883f96c33bce88d4cea79fd4be9ec18d20928b5409a497ec84d6f8d480,
full source archive/HEAD/dirty stat/UTC and math-private-after1/receipt.json.
PDF extraction loses U+2044 while isolated skewed PDF/SVG paint is identical.
The pinned NewCMMath-Book fixture has eight fraction slash variants (font SHA
and actual glyph names/GIDs/advances/cmap coverage retained). math.rs:213 omits
fraction slash from STRETCHY_BASES. builder/fonts.rs:474,969 consumes that map
for ToUnicode/subsetting, stream/text.rs:802 also uses it for Glyph face lookup.
Extraction/mapped character are language observables; no font-name heuristic,
mechanical byte equality or global extraction-order claim is inferred.

### Exclusive amendment and acceptance

Add U+2044 to the existing vertical stretchy-base mapping table after the old
bases, preserving every old entry/order, first-wins, real cmap preference,
variant/assembly fallbacks and absent-MATH early return. Change no algorithm,
font selection/axes/cache/public signature or frame/paged pipeline. This
supersedes only the old fixed known-character slice; prior ssty-only limitations
remain applicable to their historical change, not to this measured mapping.

Before code add real embedded-fixture RED requiring each actual fraction slash
variant to map to U+2044, with nonempty variants mandatory and no skipped font.
Then GREEN plus all font_metrics regressions and actual isolated/original frac
PDF extraction/boxes/PDF/SVG. Old assembly real Unicode and missing-MATH controls
remain unchanged. Internal table parity correction follows continuous ADR-0127,
L0 first; no new API/default/phase. All final twelve gates/history/unchanged
JPEG criteria remain binding. Preserve any residual render/extraction negatives.


## Measured missing horizontal accent reverse entries — readiness 2026-10-29

### Measurement before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/accent-font-map-before7.json`
SHA256 7e76619278901096a8f9b3749c5bbbbed1fb4a6f3380222a212c806790cac787 retains UTC, HEAD
212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted exact dirty stat, actual release7 source/archive,
font SHA 60346e6fc27773d96c3cc9d3ac8a550a85e3ca624437179bd7a88b4ca18f0a18, MATH variants/assemblies/advances/GIDs/cmap coverage,
actual PDF hashes and mutool/pdftotext argv. The source fixture is
03_infra/fixtures/fonts/NewCMMath-Book.otf. Product's arrow-variant ToUnicode
stream is empty and Poppler falls back from CID1 to U+0001, making bbox XML
invalid; native extracts U+20D7. Original failed replay files are retained in
accent-angle-post7-extraction-failure.json; extraction-aware7b keeps invalid XML
negative and independently measures paint, without filtering source characters.

math.rs:239-240 lists braces/hat/tilde only for horizontal constructions.
The measured fixture has nonempty horizontal variants for the missing U+0305,
U+0306, U+030C, U+20D6, U+20D7, U+20E1, U+20D0, U+20D1. Builder fonts consumes
this map for ToUnicode/subset/font lookup. Missing map entries are a bounded
textual-identity defect. Do not infer all adapters' geometry from this: basic
metrics exposes glyph_to_char while Fallback does not override it. Native
geometry/paint residues remain independent and measured.

### Exclusive correction and acceptance

Append exactly these eight codepoints after all existing horizontal bases,
preserving every prior entry/order and first-wins. Keep real cmap preference,
vertical/horizontal assembly fallbacks (including the deliberate P946 policy),
no-MATH early return, extraction algorithms, glyph selection/shortfall/placement,
font binding, axes and public APIs unchanged. Do not replace source text by a
CID or fix XML by deleting characters. This extends the existing table only;
no generic registry, dynamic corpus scan, new cache or phase is introduced.

Before code add a real embedded-fixture RED requiring all eight marks to have
multiple horizontal variants and every selected variant to map to its actual
canonical Unicode identity. No skipped font or manufactured mapping. Preserve
existing fraction slash, real-cmap assembly and missing-MATH tests. Then GREEN,
full font_metrics/core/workspace and twelve gates/build/lineage; repeat actual
PDF text/XML and independent PDF/SVG paint, retaining residues without tolerance
changes. Internal mapping-table parity follows ADR-0127 continuous flow, no
public/default/compatibility/phase gate. Do not certify full accent rendering or
JPEG/resource qualification from the mapping test.
