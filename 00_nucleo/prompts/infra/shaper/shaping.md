# L0 — infra/shaper/shaping
Hash do Código: 2ca36903

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:b234a166cefeba71803da8bcc3cb5e3daf4d7bed8313f5883bcc7bf12bf95f15
- 00_nucleo/prompts/_nuclei/fonts/complete-shaping-products.toml sha256:d612a88c4d14b13ea8e783dc06a984e26cc2c9978983a93dbabc6fac1277c2e3
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:934979e5e3d19d62b8dff899dcc462783b0522bf8bbde4c2bdbd1385e4c7f949
- 00_nucleo/prompts/_nuclei/fonts/primary-selection-proof.toml sha256:5a0fb32f391d73057b0b38d83c211b834b23ce2f8055ffee25ee3be671b5adef
- 00_nucleo/prompts/_nuclei/fonts/raw-shaping-cache.toml sha256:ba89868d23ef2304111b39efef27f781b39b74141f68b4abdc048c0088e0ae61
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml sha256:283876383094ef7a2af90ad0598b9ca3ca5cbe0f7fc7714546cc8758da40878d
- 00_nucleo/prompts/_nuclei/layout/optical-overhang.toml sha256:8c4e3b767247e54f966c3a86be1b528b030dc4c8d05ea9b358533351720a3cf2
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425



**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/shaping.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/shaper/shaping.rs:21` — `ssty_level_for_subrun`.
- `03_infra/src/shaper/shaping.rs:38` — `shaped_width`.
- `03_infra/src/shaper/shaping.rs:106` — `try_shape`.
- `03_infra/src/shaper/shaping.rs:366` — `byte_idx_to_char`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/shaping.rs`.

`lab/results/construction/diagnostics/infra-performance-followup/language-control.json` shows
that explicit ja/zh/ko settings do not affect Tekt glyph selection while a same-face
rustybuzz control does. Both local buffer sites omit language configuration.
Pinned vanilla configures language at
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980`.
Language selection is a style semantic input; glyph IDs are discriminating render
controls, not proof of full language parity. The cache structure is mechanics.
The repository Noto Sans fixture supplies independent ru/sr local substitutions
with different advances; controls must cover measurement and emission separately.

## Owned contract

- Own shaped_width, try_shape, UTF-8 cluster mapping, rustybuzz setup, variations, tracking, ssty and TextShaped materialization. Use sibling bidi/selection/cache helpers rather than duplicate their policies.
- Clone the cached base face before applying axes. Derived variant axes are merged with explicit style variations, explicit values winning by tag; preserve nonvariable/unknown-axis no-op behavior.
- Configure each width-measurement and emission rustybuzz buffer with the explicit TextStyle.lang when present, using its normalized language code. Leave language unset when absent, preserving the existing inferred/default behavior. Pass the same optional explicit language identity to the run-cache owner. Set direction per bidi run; preserve language, offsets, glyph ids/advances and clamped positive units_per_em. Each emitted TextShaped.text is the sub-run text and glyph clusters are UTF-8 byte offsets relative to that text, with replacement character fallback for an invalid mapping. BidiRun.byte_start is a separate original-input coordinate owned by the bidi child. Whitespace-only input takes the unchanged-Text path; traversal owns the rule against reshaping TextShaped.
- Measure multiline width as the maximum paragraph width. Emission resets x for each paragraph and advances y by the first primary's resolved top/bottom edges plus style leading (default 0.65em). This does not repair L1 vertical flow/pagination for an embedded newline.
- Preserve the existing distinct measurement/emission feature and tracking paths rather than harmonize them speculatively. In emission calculate the rounded i32 tracking increment once per candidate from resolved tracking points, positive units_per_em and style.size, using the existing conversion formula. Pass that same increment to the cache key and add it only before a following different cluster, never inside a cluster or after the last glyph of a sub-run; negative tracking remains valid.
- Math mode is decided by style.math. Consume the canonical ssty_eligible_text from font_metrics; request levels 1/2 only for eligible Script/ScriptScript outside math_text_item. TextItem keeps math family/target features and every non-ssty property but no explicit ssty feature (`None`), also represented as absence in its cache key.
- Preserve forced math script behavior independently of requesting ssty and existing default OpenType features. No family/codepoint/fixture exception, speculative feature change, vertical shaping or nonzero face-index support is introduced.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Authored invisible-control shaping

### Evidence

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/first-round/after/`
refutes the initial materialization: trailing/ligature preserve codepoints but
expand ActualText bbox 3.12pt/lose space in ordinary extraction; tracking adds a
placeholder-derived interval. first-round/final-receipt.json retains state/failures.
Rustybuzz0.20.1 src/lib.rs:32–35 documents default Default_Ignorable→zero-advance
space placeholder and REMOVE_DEFAULT_IGNORABLES removal after shaping. Shared
f+SHY+fi cluster can assign empty hex to a space GID, blocking later mapping.
Changed visible glyphs/ligatures under the documented flag or contaminated non-SHY/
non-bidi runs refute the fix.

`controles-bidi-pdf-2026-10-06/before.json` retains HEAD/tree/code/L0/old binary.
rtl_literal_embedding-cmap-before/rtl_forced_mark-cmap-before probes map CID 0001
to U+202C/U+200F, contaminating later spaces/word separation; block-align-2026-10-06/
verification.json.gz retains bilateral extraction/fonts/commands. Historical vanilla
`9f2b6e8715237cb086899a42873660fe744622e8` inline/shaping.rs:992–997 removes
placeholders; export/fonts.rs:307–328 retains controls per GID substring and
builder/fonts.rs:523–535 keeps first map. Observable text/geometry, not bytes/
upstream algorithm, matters. Persisting collision after placeholder removal/visual-
map filtering refutes the cause; no historical intent inferred.

### Contract and limits

Private configure_authored_ignorables_buffer applies REMOVE_DEFAULT_IGNORABLES
before shaping both measurement/emission subruns with author U+00AD or explicitly
pinned bidi control, preserving prior flags/full logical input. Do not remove
characters or filter GIDs/zero advances; runs without SHY/bidi keep behavior.
It removes placeholders, not visible selected-break glyphs; tracking uses remaining
clusters without an interval for removed placeholders. Cache uses full author text,
policy deterministic from text, no public signature/phase/fallback/OpenType-feature/
font change. Entirely invisible SHY and narrow break selection remain outside SHY
certification; later explicit bidi-only empty-result rule is separate.

SHY retains per-occurrence ActualText. Isolated bidi controls gain no replacement
envelope or copied space identity; only the mixed-cluster certificate below may
add one. Keep letters/spaces separated/Unicode clusters/ink controls. Extractors
may invent direction markers, so raw text alone proves no author identity. Internal
ADR-0127 adds no public field/signature/phase/flag/default/unsafe, consumes pinned
bidi-visual-identity and text-replacement without sharing productive ownership.

## Tracking and actual-font materialization

`tracking-residual-2026-09-29/{initial.json,results.json,summary.json,causal-checks.json}`
retains 36 inputs/108 PDFs/vanilla/sources/UTC/HEAD/full dirty tree and quantization/
boundary refuters; correcao-tracking-2026-09-29/initial.json fixes pre-change state.
Keep rounded i32 tracking/cache representation. Next subrun placement uses
tracking owner's width adding only fractional residual at eligible cluster pairs.
No glyph/text/feature/font-selection change; no-tracking measured shaped_width
retains its contract. Continuous ADR-0127 supersedes only prior preservation of
those tracking limits, no public/flag/phase addition. RED→GREEN boundaries/
selection/independent Tc-TJ, bilateral ±/zero/repeated/adjacent/wrap controls,
build/lint zero errors/scoped seals.

Retain actual loaded Font in CachedRun, returned with glyphs: one miss load, hit
uses memoized Font, no World::font at carrier construction after GID production/
lookup. Evidence/contract belongs to `00_nucleo/prompts/infra/shaper/cache.md`;
no new unsafe/API. Approved TextShaped.font field is a cross-owner boundary,
not extra ownership here.

Ratified bidi inventory/manifesto 1ae7513e requires uncertified constructors None/
read `..`, preserving observables with no phase/API change.

## Full-paragraph bidi context

### Evidence

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{initial,probe}.json.gz`
retains bilateral cases/Unicode refutation and sources/state/binaries.
`layout_bidi.rs:73–82,227–240` redetects base/invents separators;
`shaper/bidi.rs:34` redetects a fragment, losing the human-ratified complete input.
This internal parity correction adds no phase or external public signature.

### Boundary contract and acceptance

For certified spans, use the compilation-local registry of auxiliary strings/maps
by input Arc, then normally borrowed BidiInfo views; no unsafe/self-reference.
Virtual LRE/RLE/PDF is inserted only when segment direction differs from base.
Retain real spaces, nonjustified Linebreak LF and author controls; justify=true
is outside certification. Resolve once per input per registry; paged pipeline
shares registry/views between reorder/shape. Existing public wrappers may create
a local equivalent registry.

Reorder queries the actual line interval, exact segment/range intersections and
end-of-line levels, never midpoints or invented spaces. Split mixed Text at original
boundaries preserving style/subspan. Nontext items and unknown sources gain no
certificate. Preserve the L1-accepted physical anchor and actual width/advance;
certified context cannot use suffix/coalescing fallback.

Shaping queries full-input runs for each original span without fragment direction
redetection, preserving sequential subrun offsets and exact TextShaped subspans.
The existing cache key already distinguishes direction; add no context to
TextStyle. Reconciliation uses input base when present, not any RTL run; retain
prior None fallback. Require RED→GREEN for `456 789` refutation, overrides/neutrals,
no auxiliary controls in extracted text and bilateral measurements. Registry
implementation is exclusively owned by `00_nucleo/prompts/infra/bidi_context.md`;
reordering and shaping remain with their own canonical owners.
## Successful empty bidi-only output

first-verification.json.gz in controles-bidi-pdf-2026-10-06 retains residual RLM
CID 000D after placeholder removal. shaping.rs:389–393 returns None for no items,
traversal.rs:98–109 keeps fallback Text. After successful shaping of nonempty
bidi-only text return Some(empty Vec), not None/fabricated glyph/envelope. Earlier
font/context/buffer failures retain None and all other inputs their existing policy.
The early no-paragraph-runs path also returns Some(empty) only with readable primary
and nonempty bidi-only input: first-raster.json refutes final-return-only repair
because LRI fell back to notdef Text. No primary/resolved context/other text keeps
old fallback. Later certified mixed-cluster identity is owned separately and
supersedes only older blanket “bidi gets no envelope” wording for valid certificates.

## Test-only wrapper and nominal arity metadata

ci-helpers-testes-2026-10-14/before.json.gz/references-before.json.gz retains
source/owner/HEAD/tree/UTC/diffstat/calls/imports/modules/references/test list and
hash-checked suite/inventory at :113. Private compilation/usage was audited, not
dead_code alone. Only pub(super) try_shape contextless test facade is cfg(test),
with unchanged signature/delegation to productive try_shape_context; shaped_width/
production paths and SHY/bidi/font/None-versus-valid-empty controls remain.
Selected diagnostic RED→GREEN, unchanged tests/full suite/lib/all-target build/
Rustdoc/fmt/lineage/full inventory and scoped seals; no API/default/phase/dependency/
suppression or cfg/binding-removal mirror tests.

Arity audit before.json.gz/classification.json in too-many-arguments-2026-10-15
retains checked full sources/L0/signatures/hashes/UTC/status/diffstat, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, dirty tree captured
2026-10-15T16:39:18.873477+00:00. try_shape_context :124 has eight arguments,
threshold seven, historical C117. Two caches have independent lifetimes, services/
occurrence data, causal BidiSpan/shared resolved view. Do not redetect fragments
or put context in TextStyle; explicit injection justifies only this expectation:
`#[expect(clippy::too_many_arguments, reason="C117: Owner do shaping recebe dois caches com lifetimes próprios, serviços e dados da ocorrência, mais BidiSpan causal e view resolvida compartilhada")]`.
Keep signature/order/body/calls/cfg/other attributes, no module/impl/crate/global
scope/threshold/other lint. Remove/review if arity drops, never suppress unfulfilled
expectation. The nominal materialization receipt clippy-excecoes-2026-10-16/
before.json.gz plus clippy-locais/clippy-inventory.json (79 warnings) supersedes
its old audit-only pending status only for C117; pending corrections including
C047/C066 stay visible. Rust structure proves no functional/parity/cognitive benefit.

Metadata-only continuous ADR-0127: no executable/public/default/phase/dependency/
architecture change. Removing added attributes/header recovers old rustfmt source.
Real RED then historical batch inventory exactly 46 arities/five test warnings/
49 expectations, no new warnings/unfulfilled expectations; 11 correction arities
visible. Existing Counter/coverage tests, build/fmt/strict-reverse lineage/Tekt
no new warnings under 12 GiB/zero swap/one job, no attribute mirror/performance tests.
Private signature corrections require caller/L0 migration; any new external/public
need requires concrete ADR-0127 approval.

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

In ssty_level_for_subrun keep signature/guard/canonical predicate. Import MathSize
through existing private layout_types use, name all four variants and replace only
wildcard with MathSize::Display | MathSize::Text => None. No shape/measurement/
emission/cache/transport change.

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

R3 fallback-selection pin correction: consume explicit semantic math mode for additional mathematical fallback; a font table alone does not change language mode. This consumer retains its existing operations. Evidence and refuters: `00_nucleo/prompts/infra/shaper/font_selection.md` and source-bound R3 receipts. Catalog entries, declared families, public contracts and phases remain unchanged.


## R8 — approved font-backed superscript integration

Evidence before decision: the source-bound `footnote-layout-r8-2026-10-24/before/receipt.json`,
`font-tables.json` and `red-geometry.json` under `lab/results/construction/diagnostics/`
record wrong 0.65 scaling/positive baseline and font-dependent pinned geometry at
bd5ebec285505889e8ff1d4f68804ab147a699f3. `approval.json` binds the human-approved
FontMetrics::text_superscript_metrics addition. OS/2/whole-run GSUB are text metrics;
MATH constants and fixture/font-name coefficients are not substitutes. This private
integration follows ADR-0127 continuous parity flow, superseding only superscript
scaling/feature assumptions below. Keep subscript behavior and unrelated folds.
For actual superscript text, use the selected varied face and complete original
text to certify native sups. Request the same sups feature in measured widths and
emitted glyphs; isolate cached superscript shaping from ordinary/ssty runs. An
uncovered glyph refuses native use for the entire supplied text even when another
subrun happens to be covered. Preserve all other feature and variation policies.

Require actual RED→GREEN tests, fixed-pin PDF/SVG controls, unchanged geometry tolerance and complete workspace/lineage validation.


R8 mixed-face cache refuter before deciding: native sups eligibility uses the
complete supplied text, while the current emitted-glyph cache key uses only each
subrun text plus a superscript flag. Hypothesis: warming native Nimbus digits can
contaminate the same digit subrun inside a refused mixed Nimbus/DejaVu run. Refute
with portable real-face warm/cold geometry equality and measured/emitted advance
agreement; do not infer a correction solely from source structure. This test
serves the already-approved complete-run feature and cache-coherence obligations.

The initial Cyrillic fixture was invalid (Nimbus covers Ж); red-cache-valid.log
then refutes the assumed two-face split for snowman: the existing longest-cover
policy selects DejaVu for the complete run. Preserve that policy. The corrected
refuter checks actual whole-run fallback and the already-supported multiline
input123\nZ, whose Nimbus digit subrun can reuse a previous native123 entry
while its complete supplied text refuses native coverage. Record the actual
warm/cold outcome before deciding on a cache correction.


R8 confirmed cache decision: red-cache-multiline.json/log bind the unchanged
source and actual portable-font test. Warm123 emits native GIDs150/164/169 with
333du advances inside123\nZ, while cold emits ordinary18/19/20 with556du. The
snowman control preserves the actual whole-face fallback selection and passes.
Key the emitted-glyph cache by the actual native-sups decision for the complete
supplied text, evaluated before lookup from the selected cached face plus existing
axis variations. Request that same decision on a miss. Retrieve the already
selected face through the font-selection owner; do not change selection or
introduce an extra World::font dependency for a cache hit. Ordinary runs retain
the existing fast path. This is internal cache correctness, not a public addition.


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
Some(false) forbids sups in measured width and emitted glyphs; Some(true) requests
it with actual coverage/axes verification; None preserves automatic supplied-run
coverage. Semantic superscript/subscript guards remain. Compute effective choice
before emitted-cache lookup and key that actual decision. Preserve original
text, glyph/source-font identity, offsets, counters and all unrelated features.

Require new field/default/forced-false RED→GREEN, all twelve numbering refuters at0.001pt PDF/SVG, original ten notes, six font controls, warm/cold cache and full preservation/workspace/lineage gates.

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

Add only private/crate-visible terminal-measurement helpers using the same
font candidates, buffer configuration, axes, feature rules and language as current
width measurement. Return actual terminal font/GID/advance and glyph count or
None for unsupported/invalid/multiline/RTL evidence; use caller-supplied ratio data.
Do not alter existing shaped_width/try_shape/emission behavior, caching policy
or source glyphs. Share preparation privately where safe, without speculative
harmonization of the existing measurement/emission feature distinction.

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

## Already-resolved horizontal direction — #8990 residual (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8990-residuals-2026-10-31/`
before-state.json and before receipts bind HEAD 3c6b0ce7, concurrent dirty inputs,
UTC, pinned oracle and RAM binary. Explicit text(dir:ltr)[אבג.] and
text(dir:rtl)[p.v.] inside math.op are accepted yet paint the unqualified
first-strong order. Candidate-1-state.json, green-first.log and after receipts
prove that merely forwarding the property into TextStyle does not correct
visible order. shaping.rs:189,326 calls bidi_runs with no direction even when
TextStyle.dir is already Some. Existing font_metrics/caches.rs width keys
already distinguish dir. Pinned text/mod.rs:517-543 documents explicit base
direction; this is observable bidi semantics/geometry, not a requirement to
copy upstream IR, font IDs or byte output.

### Exclusive correction and boundaries
Forward TextStyle.dir to the optional-direction helper owned exclusively by
00_nucleo/prompts/infra/shaper/bidi.md on the existing local width, terminal
overhang and unregistered emission paths. Keep certified full-paragraph
resolved-context runs authoritative without fragment redetection. Reuse all
existing font selection, language/feature setup, cluster and cache policies;
add no new field or cache and no font-specific workaround. None follows the
identical automatic-base algorithm; do not implement a language-derived base
or impose an unqualified math.op base here. Preserve default #8990 and scope
restoration. Verify consistent actual width and painted glyph placement for
explicit Latin/Hebrew bases, plus existing shaper and math regression tests.

This is internal completion of an already declared direction, not authorization
for the pending automatic-language or unqualified-math default changes.
L0 precedes code, followed by RED→GREEN and bilateral observable checks,
RAM-only builds/tests and paired lineage with a valid V15/V26 graph.


## Structured private run-key integration

### Evidence before decision

The frozen `03_infra/src/shaper/shaping.rs:413–443` computes effective
native_sups before constructing the key, then appends its true/false marker only
when superscript is active, subscript is inactive and superscript_native is not
Some(false). `:444` performs the unchanged cached-run lookup. Those operations
establish existing cache separation, not a new language feature or parity claim.

### Exclusive contract

Consume the owned structured key specified only by
`00_nucleo/prompts/infra/shaper/cache.md`. Replace the string suffix append with
`cache_key.set_native_sups(native_sups)` inside exactly the same condition. Keep
the seven key arguments, their evaluation order, effective native-sups
calculation, lookup/insert order, glyph production and CachedRun cloning intact.
Do not move decisions across phases or change font resolution, language,
tracking, axis merging, feature selection, source font, bidi/SHY identity,
wrappers, fallback, successful empty output or failure behavior. All prior
owned obligations remain in force. No further kernel optimization is authorized.

This private integration follows continuous ADR-0127 with L0 first and scoped
resealing after V15/V26 validation; no public signature, default, dependency,
unsafe or pipeline-phase addition. Verify native-sups None/false/true separation
and repeated/shared versus fresh observable shaping alongside existing controls.
This representation change supplies no new vanilla parity certification.

## C06: immutable glyph storage

Evidence at baseline `0a15ccebf861e3cc0ea16d0397d55dc057664556`: `shaper/shaping.rs:504–551` finishes and clones a local mutable Vec. After all glyph/tracking adjustments freeze once to SharedGlyphs before insertion; cache and occurrence receive shared handles. Preserve full-key lookup, source Font/width, per-occurrence position/style/text/bidi/UPM, None versus successful empty, and pipeline phase. No buffer pooling or walker change. This supersedes earlier storage/clone exclusions only for the glyph payload, preserving the glyph production algorithm.


Freeze the completed local run Vec before cache insertion; share the frozen handle between cache and carrier. Test helpers may explicitly copy to Vec.

## Raw shaping reuse: evidence before decision

The shaping-input-overlap-lab-20261111 census on clean HEAD 48e5a2570159c6fc325babe883bac7d1c1fbd4b2 compared complete inputs and independently replayed 3,788 calls. In layout.typ, 111/111 emission misses had an earlier equal measurement among 113 measurement calls; math admitted only 2/9. Tracking coincidence applied to raw buffers, not final adjusted carriers. Counts do not prove CPU/memory benefit. See the preserved external census and the correction's measurements for exact states/times/ELFs.

Internal correction authorized by the owner; ADR-0127 continuous flow, no public/default/phase/compatibility change. The pinned raw-shaping-cache nucleus supplies shared invariants; this owner authorizes only its following responsibility.

Use the closed raw owner from shaped_width and emission misses only. Resolve full-text native-superscript eligibility and existing ssty/forced-Math policies before lookup. Keep terminal/basic callers, segmentation, width sums, tracking/scale, actual source Font and final CachedRun/occurrence construction unchanged.

Keep regression tests first and preserve source/frame/glyph/position/font/fallback semantics. Directed missing-key and bypass-reuse mutants must fail; compare allocation traffic, peak and CPU in the real pipeline including retention/key costs, without an arbitrary target. Compile only in own /dev/shm and preserve all existing checkout changes. Outline's semantic divergence and prior identity debts remain explicit. The retained eval/font-session prototype is a lab measurement base, not integrated by this correction.


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

Exclusive owning obligation: Own the ordinary complete-text-plus-raw certificate described above, using first_candidate and the unchanged bidi/script split functions/raw owner. Apply it at shaped_width, ordinary trailing-overhang, and try_shape_context only when no BidiSpan is supplied. Full-chain fallback remains unchanged; all single-record metadata lookups use font_info. No new raw store or lifetime, native-feature/script policy, pre-shaped occurrence reuse or public signature.


## Additional selective-chain refuter

The existing R8 fixture proves Nimbus Sans lacks snowman and DejaVu Sans covers it. Add an ordinary (non-superscript) counterpart: warm ASCII in the first font, then require the unchanged longest covering run for ASCII plus snowman and equality with cold shaping. This refuses partial-chain poisoning after certified primary use. Preserve the original native-feature test.


## Laboratory first-face proof for control-only input, 2026-11-12

### Evidence before decision

`lab/results/construction/diagnostics/watch-residual-2026-11-12/fontdemand-v2-cpu-stacks/analysis.json` records catalog validation below advance_shaped/inline_allocation::Certificate::walk. The independent breakpoint in `loremlocal-catalog-trace2/{receipt.json,trace.log}` confirms the first full loader comes from that same path; the earlier breakpoint selected the watch-only loader during compile and did not fire. Each receipt binds UTC/source/ELF. `core/compiler/stdlib/structural/outline.rs:273` inserts a standalone U+2060 before the page; `shaper/font_selection.rs:474-481` sends the existing is_p1404_format_control class directly to candidate zero without cmap/global fallback. `shaper/shaping.rs:47-52` nevertheless requires cmap coverage before accepting the first face. Mixed text is DIFFERENT: best_covering_run compares actual cmap coverage and a later face can win a longer run across a control. This refutes extending the all-character proof by simply ignoring controls inside mixed text. Code evidence motivates a measured RED, not an unmeasured performance claim.

### Exclusive internal correction

Laboratory only, no integration; ADR-0127 continuous flow. Preserve current public signatures, phases, defaults, capture identity, font selection and authored Unicode. No new control category, cache/lifetime, dependency, unsafe, source rewrite, glyph filtering or special outline/fixture/family path.
For nonempty ordinary non-math/non-superscript/non-subscript horizontal text consisting ENTIRELY of the existing is_p1404_format_control set, the existing segmentation itself proves that only candidate zero is consulted. Resolve exactly the first readable primary using the existing helper, then return that one-face primary vector without demanding cmap/raw nonzero evidence that this policy does not use. This distinct control-only lemma also applies to a supplied BidiSpan: actual supplied-context run construction, validation and direction remain untouched, and every subrun still contains only those controls. Retain existing empty-result behavior, authored-ignorable buffer flags, glyph/width/position construction and fallback on first-face resolution failure. Never publish a partial chain as complete.

For every mixed input retain the prior full authored-character nonzero cmap plus raw-output certificate; no skipping controls within mixed words, no broad format/whitespace shortcut, and no ordinary mixed BidiSpan admission. Keep ZWJ, ZWNJ, soft hyphen, zero-width space, variation selectors and arbitrary missing glyphs outside the control-only lemma. Existing longest-covering-run/refusal and all source-font/axes/native-feature regressions remain mandatory. The preceding 'not a resolved BidiSpan' and raw/cmap requirements are superseded ONLY for this exact control-only selection lemma; they remain unchanged for ordinary authored text.


## Current-compilation positive primary proof reuse (laboratory only)

Evidence precedes decision. controlprimary-segmentation-counts/receipt.json
(2026-11-12T21:39:58.227369Z–21:40:16.650035Z) binds frozen ELF/source and
observes 1115 certificate calls plus 2230 bidi analyses per hundred-paragraph
compilation: 1115 inside the proof and 1115 in consumers. There is one compile
entry per cycle; the same work recurs on unchanged-byte rewrites. The separate
CPU profile attributes about12% inclusive nearest-project-frame samples to the
proof (not exclusive CPU or guaranteed savings). The external allocator records
116764 requests/40837726 requested bytes for this workload; bidi is the largest
sampled request-count origin. Sources: evidence/freeze-controlprimary.json,
controlprimary-cpu-stacks/ and familyshare-allocation-before/. These measurements
are mechanisms, not language requirements; nominal benefit remains unproved.

Internal reversible reuse under ADR-0127 continuous flow: update L0 before tests
and code, reseal, no public/default/phase/compatibility change, no integration.
The pinned primary-selection-proof nucleus fixes the shared exact key, guard,
positive-only result, lifetime and immutable partition obligations. Preserve all
existing raw-cache/face/selection/axes/language/source-identity controls, fresh
fallback after success, late font demand/replacement, PDF/SVG/query/raster, and
strict workspace gates. Measure CPU and allocation traffic separately; retain
negative results, RSS and retained-memory costs. No performance claim from counts.

Exclusive responsibility: lookup only after current guards/cmap/current-font retrieval; compute and publish the original proof on miss. Return its immutable bidi partition for shaped_width and ordinary no-BidiSpan try_shape_context, preserving full-chain refusals and control-only/supplied-context routes. Add a test-build-only observation at actual proof computation, before bidi analysis, never on hits.


## Laboratory complete shaping products — 2026-11-13

### Evidence before decision

The frozen fraction-demand release (f34f54de100fc63587f7611d042dc3d836b484f95283a2f5fc029c8afadce65e)
and fraction-demand-work-shift/causal-summary.json record 112 backend executions
per100-paragraph compilation and64 peroutline, unchanged by removing2004/2172
width queries. All three cycle kinds repeat this work. structural-summary.json
attributes18.39% inclusive samples to emitted shaping and5.31% to raw shaping
in100 paragraphs; these groups overlap and are not a speedup prediction.
font_metrics.rs:100-106 shares only raw buffers within a compilation;
shaper/traversal.rs:30-49 reconstructs faces/selection/output caches;
shaping.rs:262-346 rebuilds measured widths and:363-679 rebuilds emitted products.
world/fonts.rs:21,35-39,291 retains one immutable prefix owner across a font
session and revokes before full-catalog exposure. Current PdfCompileSession
at pipeline/pdf.rs:1133 has no shaping products. These are execution mechanics,
not a change to language semantics. The user explicitly authorizes laboratory
implementation/testing without further stops, including experimental APIs;
integration remains prohibited. No performance benefit is presumed.

### Exclusive responsibility

Wrap existing successful shaped_width and try_shape_context products in the exact request/lookup/publication protocol. Preserve the existing cold algorithms and failure behavior. Record original relative subrun ranges with successful emitted carriers so hits use current bidi input. Check token again after original computation; never publish partial failures or expired proofs. Preserve arithmetic order and exact positions; different positions miss.

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
