# L0 — infra/shaper/shaping
Hash do Código: 37f8280b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:b234a166cefeba71803da8bcc3cb5e3daf4d7bed8313f5883bcc7bf12bf95f15
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:934979e5e3d19d62b8dff899dcc462783b0522bf8bbde4c2bdbd1385e4c7f949
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml sha256:283876383094ef7a2af90ad0598b9ca3ca5cbe0f7fc7714546cc8758da40878d
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
