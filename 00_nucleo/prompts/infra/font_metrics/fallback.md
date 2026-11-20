# L0 — infra/font_metrics/fallback
Hash do Código: 12eefa2f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:934979e5e3d19d62b8dff899dcc462783b0522bf8bbde4c2bdbd1385e4c7f949
- 00_nucleo/prompts/_nuclei/fonts/immutable-metric-epoch.toml sha256:2ae956d6a0693c848353f4edfc70aea0888231adbb30b23765917b2bbb57d60e
- 00_nucleo/prompts/_nuclei/layout/optical-overhang.toml sha256:8c4e3b767247e54f966c3a86be1b528b030dc4c8d05ea9b358533351720a3cf2
- 00_nucleo/prompts/_nuclei/paragraph-product-reuse.toml sha256:69dd4f797e17336227f56e038f9d833a0c6eff81bfc8e6beef4cf268fec27430


**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/fallback.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/font_metrics/fallback.rs:23` — `face_kerning`.
- `03_infra/src/font_metrics/fallback.rs:46` — `advance`.
- `03_infra/src/font_metrics/fallback.rs:168` — `advance_shaped`.
- `03_infra/src/font_metrics/fallback.rs:184` — `vertical_metrics`.
- `03_infra/src/font_metrics/fallback.rs:226` — `cap_height`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/fallback.rs`.

## Owned contract

- Own FontMetrics for FallbackFontMetrics and face_kerning, using the resolution/cache children. Resolve per character and apply style variations to a cloned face, never mutate the cached base.
- The explicit `FontMetrics` size argument governs advance and shaped advance, including scaling and cache identity. For shaping, use a local style with that size when it differs from `style.size`; preserve all other style fields and the caller's style.
- Preserve legacy kern then kerx within one face, not cross-font kerning or a new GPOS path. advance_shaped delegates to the shaper with the shared face cache.
- Preserve FixedMetrics/0.6 defaults for unresolved vertical/ink/bbox metrics. Apply canonical math/ssty and TextItem predicates; never remap ink codepoints.
- Math constants come from the first applicable primary with MATH, while variants/assemblies/kern/italic-correction/attachments use the active face. Keep fallback behavior and neutral L1 return types.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Rustdoc structure and explicit fixture initialization

### Evidence

`lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`
retains sources/owners/spans/original compiler output, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat.
fallback.rs:638–639 has selected warnings. Reference inspection distinguishes
productive references, descendant-test-only imports and comments; this is no new
parity measurement.

`field-reassign-2026-10-15/before.json.gz` retains sources/spans/prior owners,
HEAD/tree/UTC/diffstat, and initial-diagnostics.json records fallback.rs:759,896,
1041, 1160, 1225, 1263, 1322 constructing default test styles then assigning fields.
This is Rust representation, no new parity or product-default change.

### Contracts and acceptance

Repair Rustdoc paragraph/list separation/continuation while preserving technical
content and every test assertion/case. The textual “mais” between the two helpers
becomes conjunction “e”, without starting a Markdown list. No public signature,
default, phase, formula, acceptance or lint-suppression change. Run all-target
compile, infra test inventory/execution, build/workspace Clippy/Rustdoc/lineage,
owner-scoped resealing; no import-removal/comment-format mirror tests.

Initialize contiguous preparation fields directly in literals for the seven styles:
math; font/size; font/size/lang/dir; font/size/weight. Preserve each expression/order,
style_800 clone and later weight mutation, MATH/fonts/kerning/RTL and every shaping
control. Use ..Type::default() for remaining fields; remove mut only where no mutations
remain. No new types/builders/APIs/lint suppression. L0 first and scoped resealing,
continuous ADR-0127. Selected gate must fail before and pass after; preserve test
names/inputs/expectations/tolerances. Revalidate all-target Clippy/workspace suite/
build/fmt/Rustdoc/lineage. No tests merely mirroring initializer syntax.


## Approved font-backed underline metrics — 2026-10-22

### Evidence and public-contract gate

The fresh paired sources and exact source/font/binary/UTC/dirty-state provenance
are in `lab/results/remaining-parity-2026-10-22/underline-before-font-scoped/measurement.json.gz`
(SHA-256 `db81849f43eaa92212629aeaddce587ba5fff8465fd878a22890cd05e894a838`).
DejaVu at 11pt requires downward offset 0.21484375pt and thickness
0.4833984375pt; Nimbus at 11pt requires 1.386pt and 0.55pt. Both 17pt
witnesses preserve the actual table scaling. These measured differences
refute a universal fixed-font coefficient. Noto's numeric equality does not
waive the separately measured SVG open-line fill failure. Exact font-table
hashes and reads are retained in the sibling
`agents/inline/underline-font-table-measurement.json`; rendered pairs and
source language geometry govern, rather than a required Rust representation.

The human approved the exact additive contract and bounded integration in
`lab/results/remaining-parity-2026-10-22/agents/inline/underline-contract-proposal.md`,
SHA-256 `e7f3db142865646f13f9041f36f572582802cd302f1aebb7f8b42bb0610f2c6f`.
Approval is recorded in `lab/results/remaining-parity-2026-10-22/underline-approval.json`.
This satisfies ADR-0127 section 2.1 for exactly the method below; it supersedes
only this addition's earlier no-new-public-method restriction. No new entity,
FrameItem field, phase, dependency, mode or unrelated API is authorized.

### Exclusive resolved-face provider contract

Implement the exact approved underline_metrics method for FallbackFontMetrics
using existing style/variant/family/covers resolution, cached real faces and
the private basic face helper. Resolve the actual non-control text run by
its covering face; empty text uses the declared primary face. Preserve the
existing fallback order without arbitrary book-first preference over a
resolved family. Unusable/missing face falls back explicitly to the approved
0.2em/0.06em tableless policy. Do not execute source callbacks or mutate
cached base faces. The explicit size argument governs conversion.

Require a portable injected World holding both distinct actual faces: direct
family changes must produce corresponding table-derived metrics at both
sizes, empty-text primary selection must match, and borrowed/dynamic calls
must retain the actual override. No system-font assumptions, early-return
passing test or relaxed tolerance. All current font fallback/math/variation
contracts and their tests remain unchanged.

R3 fallback-selection pin correction: consume explicit semantic math mode for additional mathematical fallback; a font table alone does not change language mode. This consumer retains its existing operations. Evidence and refuters: `00_nucleo/prompts/infra/shaper/font_selection.md` and source-bound R3 receipts. Catalog entries, declared families, public contracts and phases remain unchanged.

## R3 variation selectors have no independent advance

Evidence: `lab/results/math-geometry-r3-2026-10-23/candidate-debug/receipt.json` preserves source/binary/UTC/dirty state; the supplementary-selector glyph cluster has a spurious 7.2 pt advance between x and y. Pinned `math/ir/resolve.rs:326–358` retains the selector in its base grapheme, and `math/shaping.rs` measures the shaped cluster. Current advance loops visit selector codepoints independently; when cmap lacks a supplementary selector they add the missing-glyph 0.6em fallback. The existing single-base italic-correction guard already excludes both selector ranges. Geometry and extractable source identity require parity; loop structure is mechanics. Refute by altered base advance, dropped selector identity or changed ordinary missing-glyph behavior.

Exclude FE00–FE0F and E0100–E01EF only from the scalar advance summation, preserving complete source/shaping payload, base italic correction, ssty and every other missing-glyph policy. Selector-only input has zero advance. This does not invent a scalar cmap glyph for a variation sequence or certify every unsupported variation glyph. Internal ADR-0127 flow: L0 first, real font-backed advance RED→GREEN, original and supplemental source→PDF/SVG controls, valid pins/seals and full validation.


## R8 — font-backed note marker geometry (public contract approved)

Status: **APPROVED FOR MATERIALIZATION**. The human approved exactly
FontMetrics::text_superscript_metrics on 2026-10-24. The original drafts and
explicit approval are bound by `lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/approval.json`.
This satisfies ADR-0127 for this addition; all RED→GREEN and lineage gates apply.

### Measurement before proposal

`lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/baseline.json`
and `received-source.json.gz` bind the exact dirty HEAD and all inputs. Fresh
`before/receipt.json` reproduces ten current-pin note controls; all ten fail the
actual-output 0.001 pt physical page/word-box gate in `red-geometry.json` and
`red-geometry.log` (ten assertion failures, no harness errors). The original
extracted word order remains separately available; physical matching preserves
page membership and every word occurrence without replacing these observations.

`font-tables.json` binds actual table/font hashes. DejaVu Sans at 11pt has
OS/2 superscript ySize1433/upem2048 =>7.69677734375pt and upward offset
983/upem2048 =>5.27978515625pt. Liberation Serif has ySize1331/upem2048
=>7.14892578125pt and upward offset928/upem2048 =>4.984375pt, plus a `sups`
GSUB feature requiring actual whole-run coverage verification. The current
`compiler/layout/text/style.rs:288–300` instead uses 0.65 and a positive
0.3em baseline resolved after scaling; `cursor.rs:72–75` adds that offset.
The pinned oracle reads OS/2 script metrics in
`typst-library/src/text/font/metrics.rs:67–80`, default superscript metrics
in `text/shift.rs:213–218`, and actual GSUB-or-synthesis selection in
`typst-layout/src/inline/shaping.rs:1160–1215`. Its rendering rules apply 85%
entry text size, 0.5em leading, 1em indent and 0.05em number gap in
`model/footnote.rs:334–341` and `typst-layout/src/rules.rs:401–411`.
These are font-dependent render/language observations; Rust representation and
vanilla iteration mechanics are not acceptance criteria (ADR-0107/0108).

The current public FontMetrics API exposes advances, vertical/ink edges,
underline and mathematical metrics, but no text superscript metrics or native
script-feature coverage. Mathematical MATH constants select a different
semantic source and cannot be repurposed as OS/2 text metrics. L1 must not read
font files or select arbitrary measured coefficients to bypass this boundary.

### Approved exclusive provider obligation

Resolve the injected method through existing text-family/variant/variation selection and actual complete-run coverage. Do not select a mathematical face just to obtain metrics; retain existing fallback behavior and explicit mixed-face boundaries. No new global cache or source side effect.

The only approved public addition is the exact method and tuple contract in
`00_nucleo/prompts/core/compiler/layout/metrics.md` §R8. This provider obligation
is authorized by that recorded human approval. Tests first must compare real font tables,
scaled synthetic metrics and complete native-feature coverage at multiple sizes;
advance and emitted geometry must agree. Preserve all unrelated methods, fonts,
source effects and current error behavior. Full workspace and lineage gates apply.

R8 native measurement obligation: an active superscript run certified by complete sups coverage uses native glyph advances at unchanged outer size, including direct advance calls. The single-face provider may use rustybuzz on its varied face for this existing operation; do not apply native shaping to ordinary text or synthesize from MATH.


## R8 approved resolved native-script choice

Measurement before decision: R8 numbering-candidate5/receipt.json records
Nimbus11 numbering1 Z with1.4718pt error; the full marker chose synthesis but its
shortened digit was shaped with sups again. The root is
lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/.
The exact public TextStyle::superscript_native: Option<bool>, default None, was
approved from owner draft SHA4b14829d1eb38475d1985a9707a6e6ce40166196dfe524afe512d99b9cabf4d0
and is recorded in native-carrier-approval.json. Its data/default contract belongs
only to core/entities/layout_types.md; this owner retains its existing consumer.
No MATH flag, baseline sign, font-name constant, private string or source identity
may substitute for the explicit choice. No extra public field/signature/phase.
Direct/native and shaped-width calls preserve the explicit render decision;
Some(false) cannot request native glyphs in a shorter fragment. Preserve the
existing resolved text-face/fallback policy and borrowed providers. No MATH reuse.

Require new field/default/forced-false RED→GREEN, all twelve numbering refuters at0.001pt PDF/SVG, original ten notes, six font controls, warm/cold cache and full preservation/workspace/lineage gates.


## R10 — source superscript runs and mandatory breaks

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,baseline-audit.json,approved-metrics.md,approval.json}` binds received R9 source/UTC/dirty stat and pinned bd5ebec2 binaries. Six of fifteen controls fail the unchanged complete physical words/pages and0.001pt gate: three literal-newline cases, Nimbus hard break, and Nimbus/Liberation mixed fallback. Pinned inline/collect.rs:191-196 includes mandatory breaks in source script text; inline/shaping.rs:970-1004,1090-1107,1167-1217 retains outer size, actual face/run and full causal coverage. This is language geometry and source preservation, not a byte/algorithm obligation. Font-specific synthetic metrics cannot be guessed from the primary face after fallback. The exact additive FontMetrics::text_superscript_runs contract was approved at SHA65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9; every other public method/field/default/phase stays unchanged.

Classification: approved public method plus private parity integration under ADR-0127. No font I/O, callbacks, dynamic evaluation or font-name coefficients in L1. Require real module RED→GREEN before runtime code, default/override/borrowed contract tests, source partition and restoration, native/synthetic/mixed-face/warm-cold/explicit-size refuters; all original controls, R8/R9 and A–H/R1–R7 replays, PDF/SVG0.001pt and full workspace/lineage/resources validation. Preserve all historical observations and unrelated received input bytes.

### Exclusive owning obligation

Override the approved runs method using the actual selected varied faces and byte ranges, preserving the input outer size for every face. Within superscript runs, the first declared available face keeps the prefix it covers; fallback begins only for uncovered characters, so a later broad emoji face does not steal the numeric prefix. Scope this correction to the new script resolver; ordinary longest-run selection stays intact. Determine native eligibility on the complete causal text offered to that face before fallback, never an arbitrary covered prefix; an explicit Some(false) stays synthetic. Return fully resolved single-face TextStyle, synthetic explicit-size override, absolute baseline and paint-only X compensation. Preserve existing singular capability-query semantics and shared caches. Use current covering/scoring/variation helpers, no extra public signature.

R10 measurement refinement before code: `font-tables.json` in the R10 evidence
root records actual font hashes and Nimbus OS/2 yOffset350/1000, correcting the
400/1000 numeric typo in the preserved approved draft without changing its exact
public signature or scope. Pinned inline/shaping.rs:473-510 measures line edges
at outer styles.resolve(TextElem::size), while build:350-363 paints at each
reduced glyph size/offset. Resolve run top/bottom edges from the actual face at
that outer size and carry them as existing absolute TextEdge lengths; do not
use the reduced paint size to shrink the line's typographic envelope. No new
field or public-method change follows. Require actual-face line-origin and
explicit-edge controls; a regression in R8/R9 refutes an unrestricted change.

R10 causal refinement before correction: candidate1-bitmap-red.json/log/source archive records four L1 tests GREEN, but the two-face provider still returns one range. resolution.rs:167-193 prioritizes any MATH-capable face before declared order even for ordinary text, so the explicit DejaVu fallback steals Nimbus digits. Inside the new script resolver, ordinary semantic mode first selects the earliest declared/normal fallback face that actually covers the character; only uncovered characters use existing global scoring. Semantic math mode retains its existing priority. Do not change the shared covering helper or ordinary non-script consumers. The same portable two-face test is the RED witness; no fixture-specific branch or MATH-table-as-language inference.


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

The new resolver delegates ordinary face selection to the resolution owner's private text-order helper; semantic math keeps existing priority. Ordinary direct advance uses that same first-declared covering policy, aligning separator width with its actual rendered face, rather than allowing an incidental MATH table to steal it. The ordinary longest-run shaper and old capability queries are unchanged. Require real primary/fallback-space RED and all prior math/font gates.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.

## Approved actual-glyph optical margin ports

Measure first: physical5-candidate/receipt.json and comparison provenance under
readiness-fixes-2026-10-28 bind bed9a095… source/build/UTC/dirty state and exact
article. Accepted justified lines are1.935998pt short at terminal period. Pinned
inline/line.rs:493-526 adds optical remainder only after line selection; :676-716
uses actual GID/face and advance. Do not modify candidate choices or costs.
The direct human approval in optical-overhang-metrics-approval.json binds review
SHA-25629d7f6fcd280ecfe5abd27b54c50b8326822504ea397ab978e62771de9c10ecb.
It supersedes no-new-API clauses ONLY for the two exact methods in that review.
All earlier APIs, phases, source/effect and validation boundaries remain.

Implement both ports on FallbackFontMetrics. Text measurement reuses the
existing candidate/fallback/axes/language/shaping path and current face cache;
actual shaped glyph metadata, not isolated-character width, determines the
terminal amount. Preserve requested size, unsupported/empty/RTL None, and actual
GID alias behavior. Glyph input uses the supplied Font through the shared private
face helper, without choosing another World slot. No callbacks/content/effects.

Require real RED→GREEN, strict lineage/lint/full suites/build and fresh paired
PDF/SVG, keeping all historical negative and performance/resource obligations.

## Default-face and refusal revalidation

physical9-build.json/source.json.gz fixes release2fc7b30db7477edaae837669bd4f4b347495afcf65661a550d99c5e3ef850a99,
dirty HEAD212aed0449, UTC/diffstat/inputs. physical9-optical-visual records explicit
families matching PDF/boxes while physical9-article-route still ends completo.
at394.52756pt instead of396.463558pt. Source article sets size/lang with no family.
Fallback text port delegates to shaped_trailing_overhang, whose required
style.font.as_ref()? rejects that existing default-face route. Existing
resolution.rs:31-108 already selects the default actual face and fallback chain.
This is an indication for a real no-family RED, not permission for a new font
policy. Query must use that actual existing selected face, retain input style,
provided size, glyph/count and existing fallback/effects/order. No width or
selection-cost change. Also test approved LTR refusal for RTL/TTB/BTT; absence
may be inferred only from actual text shaping, never turn an explicit vertical
direction into horizontal certification. Existing API/public/phase remain.

optical-default-red.json/source.json.gz now records actual failure with unchanged
inputs: the no-family real-font query returns None and injected vertical policy
returns3pt rather than0pt. On the missing-family route resolve the existing
provider primary chain, use its actual selected font metadata in a local style
copy, then query the existing shaping candidates. Preserve the input font=None,
size, variant/axes and all widths/costs. Decline explicit non-LTR directions
before loading a default face. No new public/pipeline/default font policy.


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

Exclusive owning obligation: Use first_primary only at existing first-face-only vertical/cap/edge and default-overhang lookup sites. Direct advance may use the certified ordinary per-character candidate, otherwise exactly the existing full math/text route. Metadata-only lookups use font_info. All other feature/MATH/fallback logic stays unchanged.


## Laboratory paragraph retention — 2026-11-12

Evidence first: subtree-layout-admission/receipt.json records102 preparations
and two refused complete plans per100-paragraph compilation at frozen subtree.
paragraph-validation records actual route geometry GREEN and cold-cache runtime
RED (22 vs11 calls). Prepared contains word/gap measurements and selectors,
without World, positions or callbacks. This experiment preserves all current
phase order and creates a bounded explicit product session. User waives gates
for laboratory work; no checkout integration.

Implement the optional paragraph_session and paragraph_identity ports. Identity
requires a current first-readable face, real immutable owned Font and positive
cmap coverage for every supplied character including space/dash. Reject math,
sub/superscript and unsupported direction. Requery current World font; parse
these actual bytes and refuse if unreadable. Return named deterministic provider
identity owning that Font, not a slot, family name, digest or borrowed address.

Dependency proof: resolution.rs:first_primary and shaper/font_selection.rs:
first_candidate use the same declared-order first-readable policy. Full cmap
coverage wins direct covering_text and longest-run selection (strictly greater
length replaces winner, so a full first run cannot lose). All substrings and
hyphen candidates remain covered; admitted preparation excludes other scripts
and transformed characters. shaped_width uses that face's raw advances even
when its preliminary raw proof declines, without replacing zero-glyph outputs
by another font. This certifies stable horizontal dependency, not a new raw
shaping shortcut. Size, axes, language, direction and all style fields belong
to exact L1 key. Current final emission/shaping remains unchanged and fresh.

Require real-font RED/GREEN through fresh metrics and PDF compilation. Current
font replacement at same slot, newly available earlier family, axis/style/width
changes must match cold output and invalidate; missing proof stays cold.

Actual-byte immutability boundary: only Font::Vec owners certify retention in
this first provider. A read-only Mmap may still observe external file writes,
so its identity alone is not proof across compilations; Mmap remains cold.
Font Vec ownership is immutable while retained (Arc mutation must replace owner).


## Laboratory current metric owner proof — 2026-11-13

Evidence before decision: frozen paragraph source, fallback.rs:72-78 obtains
first_primary (which caches a parsed face in resolution.rs:62), then reacquires
World.font and parses its bytes independently. caches.rs:260 keeps the earlier
face by slot. Inference: a World returning a replacement owner can certify
measurements made with the previous owner; a changing-World runtime test must
refute or confirm this before correction. This is local cache mechanics, not
new language semantics or an upstream requirement. No timing gain is established.
User permits laboratory changes without a separate gate; no integration.

Supersede the repeated standalone parse in paragraph_identity with validation
against the actual compilation-local cached face. Requery current World.font,
obtain existing cached_face, require its retained owner to match exactly, and
check cmap coverage on that parsed face. A mismatch declines certification;
never refresh existing metrics selectively or label old measurements with a
new owner. Only the immutable Font identity and existing pure product may
cross compilations, never a face/metrics handle. All other admission rules
and provider name remain.

Tests first: prime metrics with a real font, replace bytes at the same slot,
then require absent identity despite positive coverage in both fonts. Also
replace by distinct Arc with identical bytes: conservatively decline. Stable
same-owner providers and fresh providers for the replacement certify normally.
Use real metric differences to discriminate the changed-font test. Revalidate
font/PDF/warm-cold controls and full infra/lineage/build gates.

Runtime evidence: paragraph-proof-validation/owner-red.log and red-source.json
record both new tests failing at the final identity assertion, after stable
and fresh-provider positive controls and real changed-metric discrimination.
This confirms an identity proof gap; it does not claim a general World snapshot
or arbitrary concurrent-mutation consistency. Correct only this admission.


## Laboratory immutable metric epoch — 2026-11-13

Evidence before decision: physical-retain/conclusion.json and its exact release
freeze record six-to-three physical execution reduction but only about5% CPU gain
on the outline. physical-retain-deep-stacks/structural-summary.json measures
11.5% inclusive samples in Journal::validates, with additional metric request
recording/style copying. These are execution mechanics, not language semantics.
Source world/fonts.rs:168-216 routes every query beyond the immutable prefix
through font_catalog(); world.rs stores the full catalog in OnceLock. FontSlot
get at fonts.rs:179-216 keeps embedded bytes in owned Vec and disk fonts may be
Mmap. Fallback metrics and transitive shaper queries use only the font-facing
World methods. Therefore a still-uninitialized full catalog plus the owned
embedded-prefix identity can certify this restricted domain; full-catalog or
arbitrary provider identity alone cannot. Runtime tests must refute expiry,
provider replacement and a falsely complete journal. No performance gain is
assumed. The user explicitly waives approval stops for laboratory work; this
additive experiment supersedes prior no-new-API clauses only for named methods.
Integration remains excluded. Follow the pinned immutable-metric-epoch nucleus.

Exclusive owner obligation: Override physical_epoch by combining the instance owner with current world.font_query_epoch; return None when World lacks proof. Do not change metric algorithms or current-owner checks. Test clone sharing, independent provider identities, immutable actual metrics and immediate revocation after full catalog access.


### Test-domain correction from observed refusal

The initial metric-epoch infra run (initial-default-family-refusal receipt,
2026-11-13) revokes the certificate when TextStyle::regular has no explicit
family. resolution.rs:38-103 and fallback_fonts.rs:32-33 show that this query
starts with a system sans family, not the embedded prefix. Preserve that
revocation as a negative control. The positive real-metric case must explicitly
request Libertinus Serif, then verify book acquisition revokes it. This corrects
the test domain; do not change font selection or widen certification.

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

## Reúso de bordas tipográficas no fechamento

O censo `lab/results/construction/diagnostics/references-500-reused-lines-2026-11-19/probe/observed.json` registra a repetição de text_edges.
O fechamento chama text_edges ao visitar fragmentos e novamente para o
estilo de avanço. A paginação usa a mesma consulta. A implementação atual
seleciona a face primária e resolve cap-height/baseline quando ambos os
bordos são padrão.

Reutilize o resultado real de bordas padrão por domínio de fonte imutável
atual, família/lista, variante e bits de tamanho finito. Nunca publique
se a consulta revogar o domínio. Exija a mesma autoridade ao ler; sem ela,
para bordos explícitos ou quando não há face primária utilizável, execute
o caminho anterior. Preserve o resultado bit a bit, inclua troca de
família/variante/tamanho e revogação nos controles. O fechamento continua
resolvendo largura, escopo, posições, efeitos e transferência ao vivo.

## Reúso da largura shaped

O censo de references_500 em lab/results/construction/diagnostics/references-500-item-width-2026-11-19/probe-summary.json
distingue a entrada de advance_shaped e a chamada à produção da largura
pelo shaper. O caminho anterior adquire o lock de faces antes da chave
legacy. Os hits da tabela certificada devem devolver a largura shaped já
observada antes desse lock e dessa chave. Ausência/refusa mantém a
medição anterior, inclusive tracking, fallback, estilo efetivo e tamanho
explícito do argumento. O caminho genérico do fechamento continua vigente.
