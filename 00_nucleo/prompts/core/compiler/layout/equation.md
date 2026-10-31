# L0 — equation
Hash do Código: 5ffa64a7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/nonexpanding-math-frame.toml sha256:8e6bd0ef718cc6d7e11292996777437b7f0fdb88c3c39d9d441c721459758563
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/equation.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/equation.rs:158`, `01_core/src/compiler/layout/equation.rs:479`, `01_core/src/compiler/layout/equation.rs:871`, `01_core/src/compiler/layout/equation.rs:38`, `01_core/src/compiler/layout/equation.rs:658`, `01_core/src/compiler/layout/equation.rs:673`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the ratified reference `64c5e8c02fe0394d913ead36e42ef8c1334d2107` without a new measurement; fresh source-bound measurements for the current corrections are recorded below.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distinguishes regular GIDs from Oblique selection. The human confirmed public TextShaped.font: Option<Font> in `00_nucleo/prompts/core/entities/layout_types.md` (SHA `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`). This round's snapshot/inventory are in `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

`lab/results/construction/diagnostics/math-bare-2026-10-06/{before,additional,refuters}.json.gz` and `lab/results/construction/diagnostics/math-bare-medicao-2026-10-06.md` preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diff/status/sources/binaries and historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`. Bare frac/mat/sqrt/lr/op have one vanilla Equation versus zero Tekt; math.bold([x])/ordinary text zero both; nested/nested-equation one both; reuse two vanilla. block-style/local-block/number-style probe insertion scope, show-generated/context-generated repeat gap. Explicit Equation retains fraction render; vanilla rejects it.body with `maximum show rule depth exceeded` while it is valid; later show affects only second insertion. Raw frac/original label versus explicit Equation are refuters.

Pinned `typst-realize/src/lib.rs:245–265,302–334` normalizes Mathy before show outside math; `equation.rs:680–689` projects bare math to text; `realize/mod.rs:155–199` prepares show/identity without math normalization; `content/traversal.rs:14–108` is bottom-up into math body; `rules/apply.rs:564–607` handles only existing locatables. Semantics/morphology/render are language, without vtable/buffer/Rust-equality/PDF-byte requirements. Human confirmation 2026-10-06 and completed bounded materialization are recorded at `lab/history/construction/records/00_nucleo/materialization/math-bare-2026-10-06.md:3`, receipt `lab/history/construction/records/00_nucleo/materialization/math-bare-aprovacao-2026-10-06.json`, manifest `149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6`. This supersedes the stale proposal-only gate only for approved measured scope.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [440, 450, 484, 935]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

FrameItem F005 at equation.rs:814 (formula_group), F006 at:841 (text_x).

AST spans, patterns/guards/bodies and complements were checked before code. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, sources/L0/configuration, hashes, status and git diff HEAD --stat. classification.json records 70 cases/complements across 26 owners using pinned CI-parser tree. Relevant L0 clauses were read with source; this proposes neither new architecture nor inferred upstream intent. Rust patterns/exhaustiveness are mechanics; product semantics/morphology/geometry/effects/diagnostics remain unchanged.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own block/inline Equation dispatch, the current math fallback, math-to-FrameItem composition, Formula carrier, equation number placement and deferred equation fixups. Mathematical node layout and callback occurrence matching belong to their existing math owners, not to this prompt.

Install math=true, New Computer Modern Math family, weight 450 and Display/Text math-size according to blockness, inheriting the existing outer style fields. Keep the current lexical chain and supplied stable equation Location, effective region height and attempt-wide callback pass when constructing MathLayouter. A directly constructed Layouter retains its callback-free fallback pass. Do not load fonts, call Func, allocate a second equation Location or reconstruct math callback indexes here.

Lay out and measure the math body once. Use that same raw extent and returned items for composition. Block numbering may extend ascent to the active top edge; preserve collapse state across mathematical measurement. Inline equations stay on the text baseline, not displaced by the math axis. Resolve their paragraph contribution once as the current bound/text-edge ascent/descent versus raw extent minus 0.7 times effective paragraph leading, with nonnegative descent, then submit it to Cursor's canonical inline-extents aggregator. Do not rescan glyphs or lay out again to estimate line bounds.

Block equations close pending text, center within the available finite width (including a negative offset for an overwide equation), and preserve the current region-top suppression of above spacing and the ordinary block boundary/collapse protocol elsewhere. Default above/below is resolved from `00_nucleo/prompts/core/compiler/layout/vanilla_defaults.md`; track equation descent for subsequent blocks. Auto width defers centering/number placement to this owner's pending fixups rather than using infinite coordinates.

Translate supported math FrameItems into the actual equation baseline/origin, then wrap them in an identity Group whose inner width/height are the logical measured extent, inside a Formula Semantic envelope with Inline/Block placement and the supplied String alt or no alt. Keep prescripts/negative child coordinates and trailing logical extent: cursor advance is based on the full extent, not a sum of Rust glyph advances or a first-child bound. The envelope does not create layout twice or change visual order.

For an enabled block number, consume pre-realized equation-numbering Content from the introspector, with the current causal pattern/counter fallback and empty fallback; this is not layout-time function execution. Preserve the current plain-text width projection and marginal Text emission for numbering; rich function-produced numbering is not newly guaranteed by this cleanup. Resolve number-align's physical start/end with direction and top/bottom/horizon against the actual measured multi-line equation baselines/extent. Keep finite alignment and the existing number/gutter reservation interface with `00_nucleo/prompts/core/compiler/layout/page_geometry.md`. Deferred fixups apply after final page width is known, preserving indexes/reference frame and clearing their buffers once.

Bare supported Math variants use this math bridge without inventing a new native dispatcher or interpretation for Lr/Mid/Primes/Stretch/Underover. MathAlignPoint outside that structured context remains the explicit central no-op. Direct private HTML markers do not make a pending material line. Unmeasured layout/callback/rich-numbering combinations remain limitations, not success inferred from a non-empty frame.

Transport TextSeparator position, consumed advance, style and origin without new paint/baseline/advance; preserve child order. In foreign equation content, translate carrier without adding advance to cursor.

Treat TextReplacement as a transparent visual envelope in this owner's traversals; descend into children preserving text, order, coordinates and existing calculations, without a new algorithm or advance. The pinned text-replacement nucleus shares invariants only; this prompt owns only its declared consumer.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Migrate only this owner's shaped-face references: legacy constructors use None, transformations preserve font, and shaped comparison/join discriminates face/axes. The shaper owner's emission provides effective-font clone also on cache hit, without changing semantics/style to induce selection. Collectors/subset/CMap/PDF bind the same original face with style coordinates; PNG/SVG prepare local bindings and instantiate once per face/request before render, keeping original resource index distinct from instantiated bytes. None retains legacy resolution; existing public API/FontKey remain. No artificial fonts, unsafe, globals or new phase. Private functions may receive explicit font/bindings.

Shared shaped-face-identity invariants govern these cross-owner boundaries: associate original face bytes plus axis request rather than name/slot. Export-local tables preserve supplied resource order and append missing faces; multifont distributes items once by binding before per-font collection, never sharing global GIDs/CMap across faces. This owner remains 1:1 and materializes only its own references; the shared nucleus owns common claims.

Consume automatic Equation through the exact explicit Equation bridge/MathLayouter, same font/math-size/Formula/style/Location/region/callbacks/counters. Layout creates no wrapper or Location. Realization owns normalization; for normalized insertions only, this supersedes productive bare-math textual fallback. Private legacy-caller fallback proves no parity and must not silence productive preparation failure. Preserve bridge signature/formula/fonts/numbering/geometry rather than compensating normalization. The approved gate supersedes prior phase restrictions only at named insertion boundaries, never universal parity/D07/math-label/other debt.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. Capture/reinsert paint_root unchanged in Group reconstruction, including projection/marker removal/rebase; never reset true to false.

F005 formula_group and F006 text_x replace only wildcard with nominal FrameItem TextSeparator/Text/TextShaped/Line/Glyph/Image/Shape groups. Preserve complete existing fallback body, prior arms, guards, transforms/calls/returns/order and cases failing preceding guarded/restricted fields. F005's SemanticKind::Formula restriction and F006 guard do not cover the whole variant; retain failed cases in fallback.

Preserve callbacks/effects/metadata/clipping/fonts/positions/paint/width under their current owners. This work does not expand traversals, turn wrappers into leaves or substitute text/ActualText for geometry. Test helpers preserve fixtures/expected outputs/tolerances/counts/Unknown failure; cleanup cannot accept missing observation. Nominal alternatives retain the prior shared body and visible informative V19 notes; one arm's execution does not prove every alternative.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection. Shaped-face controls require RED→GREEN/bilateral miss/hit/VF without promised gain. Automatic insertion requires RED→GREEN and bilateral approved-scope revalidation plus D05/query regressions, no broader public/phase changes without another concrete gate. Existing math/callback/rich-number limitations remain.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.

Internal continuous-flow ADR-0127 maintenance introduces no enum/field, trait method/public signature, default, dependency, compatibility or phase. Do not materialize historical pending proposals. L0 precedes restricted bilateral resealing. Add no suppression, threshold change or match→if-let rewrite to hide V16.

Before code, run new FrameVisitor control and workspace suite with unchanged expected outputs, and establish structural RED for 70 family warnings through AST reading. Afterwards require zero covered-family V16 and no new warning, retaining external V16 as pending. For productive core matches reached by cargo check -p typst-core, use a disposable copy with a sentinel FrameItem variant to measure absent E0004 barriers before/present after. The probe covers only compiled sites, not cfg(test) helpers or L3 blocked by dependency. Across the whole package replay patterns/guards/bodies, verify unchanged enum/signatures, real build/tests and no stub bodies.

Revalidate full workspace suite, cargo build, Clippy workspace/all-targets -D warnings and -D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/configuration during commands. Heavy commands run sequentially with MemoryMax 12GiB/MemorySwapMax 0/CARGO_BUILD_JOBS1. Structural cleanup does not establish new global parity, memory gain or measured AI benefit.


## Planned direct Equation source bridge

### Measurement before correction

`lab/results/set-page-header-implementation-2026-10-21/route-probes11/receipt.json`
records the source-bound diagnostic11 product and the pinned vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Actual public syntax
`math.equation(metadata("equation-child"))` and
`math.equation(figure([EQUATION CHILD], caption: none))` both succeed in the
oracle and fail source-plan verification in the product. Ordinary `$a + b$`
succeeds with equal typed observations in both. Exact before source/L0 bytes,
hashes, UTC and working-tree provenance are archived in
`agents/layout/equation-bridge-before.json.gz` under that implementation root.

The source plan already declares EquationBody and recursively plans the direct
body according to the existing semantic partition. The equation painter calls
MathLayouter without that cursor; its external bridge creates an isolated
unplanned Layouter. Therefore required direct-body facts cannot be admitted by
actual painting. This is a missing internal handoff within the human-approved
physical-page phase, not authorization to open terminal MathSequence subtrees.

### Exclusive correction

For an installed source plan, inspect exactly the declared EquationBody subtree
for required operations. Without such operations retain the existing math path.
Otherwise paint the transformed direct body once through the parent Layouter's
existing planned subframe before constructing the borrowing MathLayouter. This
preserves the actual tracked introspector, image sizer, callback transcripts,
queues, diagnostics, source pass and append-only position capture namespace in
one existing transaction. Save and restore lexical chain/style and margin-collapse
state. Project the same math TextStyle onto the actual lexical chain; apply the
math-default transformation once. Compose that actual frame through the existing
math width/anchor/extent algorithm without a second painter. Final outer marker
projection supplies physical positions. Preserve public signatures, Formula,
stable Equation Location and callback budget; no synthetic semantic admission.

### Acceptance

Require actual metadata and Figure direct-body source admissions exactly once,
complete trace, actual second-page positions and a once-only visible Figure body.
Retain terminal MathSequence behavior with no newly admitted opaque descendants
and unchanged rendered text. Replay the two actual public-syntax RED programs,
callback/geometry controls, full lineage and strict validation. This finite repair
does not establish general embedded math/callback/footnote parity.

The direct nonlocatable zero-paint CounterUpdate control compares the actual
Formula frame extent with the unchanged unplanned equation painter, while
requiring its actual source admission. Authenticated packet markers must remain
available for final projection without becoming visible ink or math-axis ascent.
This control does not reinterpret inherited Metadata Location-marker geometry as
a newly measured regression or promise general hidden-math parity.

The planned external path transports its returned Image and Link FrameItems and
translates Semantic children into the same equation origin as other items. The
legacy unplanned Image/Link branches remain unchanged. This preserves the actual
painter output and authenticated position markers rather than substituting text
or retaining external local coordinates as final physical positions.


## Opaque physical trace storage

The source-bound Clippy RED in
`lab/results/set-page-deferred-corrections-2026-10-21/candidate-2/validation-remainder/clippy.log:83–113`
identifies oversized private pass variants and the public result's opaque trace
storage. Exact before texts, SHA-256, UTC and working-tree state are in
`lab/results/set-page-report-blockers-2026-10-21/agents/layout/trace-callers-before.json.gz`
and `agents/layout/before.json.gz` under that evidence root.

The trace owner retains every existing source, admission, request, sealed geometry
and completeness value behind one compiler-private owning payload Box. Access
those same fields through `trace.data`; preserve exact mutation/read order,
cloning, authority, geometry, verification and acceptance. This is private storage
indirection only: the public opaque PageRunningTrace and all public fields and
function signatures stay unchanged. No new semantic fact, source traversal, phase,
callback, diagnostic suppression or relaxed gate is authorized.

## Measured display-equation continuation — 2026-10-23

### Evidence before decision

`lab/results/math-continuation-2026-10-23/{baseline.json,baseline-source.json.gz,before/receipt.json}`
retain UTC, uncommitted HEAD, exact dirty stat, sources/configuration, commands,
inputs and binaries. Pre-D product is frozen C, SHA-256
`c63a1efdab3870f883c7a8511afccabc4fcb92a1e354151dabf7253571af4fe6`.
Oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
The original 35 expressions distribute 6/7/6/6/6/4 versus 6/6/6/6/6/5.
The first formula on page two is 6.72358pt too high by the PDF observer.
Thirteen expressions retain the transition. On a 100pt page, four simple
fractions distribute 3+1 versus 2+2, with the resumed formula too high.

`equation.rs:197–223` submits normalized inline extent only for !block;
block placement uses raw extent, but the Formula's Group is intentionally
opaque to `line_commit.rs:105–138`. Its ordinary root admission at 580–615
therefore sees only an exterior strut, missing the full descent and installing
that strut on migration. `equation.rs:591–608` also discards block-collapse
bookkeeping when flush changes the page and retains the old captured baseline
for later numbering. Pinned `math/mod.rs:115–190` measures display frames and
`flow/distribute.rs:369–399` admits the complete block height. Language geometry,
line/page membership and spacing govern acceptance, not upstream machinery.
A changed fitting first page, inline slack, duplicate body/callback, misplaced
number, lost source position, broken rollback or no-progress overflow refutes
this correction.

### Exclusive internal correction

For block equations only, pass the already measured raw ascent/descent to the
existing canonical line-extent aggregator after the block baseline is installed;
record that actually installed ascent, including zero. Preserve the inline
0.7-leading normalization exactly. Do not inspect Formula children to recover
metrics or add a second page-admission algorithm, new state, glyph measurement,
body evaluation or public interface. Ordinary/reserved-region and rollback
policies remain with their owning line/region protocols.

After closing a block equation, use its settled baseline in the actual region
for block-below/descent state and equation-number placement even when closure
moved it. Preserve explicit top suppression, first-page geometry, centering,
auto-width fixups, source-owned occurrences, once-only callbacks and numbering.
Do not compensate with page-specific offsets, alter the oracle or discard pages.

Require genuine geometry/continuation and following-text-spacing RED→GREEN;
bilateral original/reduced/small-page equations, inline/block, fractions/roots,
leading/gaps, numbers, overflow and existing rollback/source controls. Retain
all PDF-box failures at the original threshold and independently check SVG
geometry; this does not close separate export precision or multiline-math debt.
ADR-0127 continuous flow applies: no public contract/default/phase/compatibility
change. L0 precedes code; reseal only this owner with valid ownership/Nucleus
graph, then complete the full validation gates under one job/12GiB/high8GiB/no
swap while preserving prior changes.

The same before matrix's explicit-break case preserves words/pages but starts
its second fraction at y31.505 instead of y5.996: root region transition resets
collapse state but the equation reuses the previous region's baseline. At a
truly empty root region only, with no causal flow placement or block chain,
use the geometric region top plus raw ascent. Consult existing physical-empty
witnesses (including unpainted material and deferred insertions), never infer
emptiness from lack of visible glyphs alone. Columns/subframes and deliberate
vertical flow remain under their existing protocols. Revalidate explicit/weak
breaks and VSpace/hidden-material controls; no new page-intent state is needed.

## Measured R3 geometry correction — 2026-10-23

### Measurement before decision

`lab/results/math-geometry-r3-2026-10-23/{baseline.json,baseline-source.json.gz,before/receipt.json,reductions/receipt.json}` retain the received R2 state, UTC, dirty HEAD/stat, full source map, exact inputs, binary hashes and commands. Oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Seven of 19 render probes fail 0.001 pt: five at 0.92119 pt vertical, unicode at 2.136005 pt, and selected-multichar at 0.492 pt. Isolated alpha without show has the same vertical error; adjacent ordinary text refutes a show-specific cause. Markup base and variation form both select NewCMMath rather than oracle FreeMono. Symbol arrow has omitted per-letter italic correction; sum happens to match.

Pinned `math/mod.rs:49–102` admits the normalized inline frame; `math/ir/resolve.rs:326–358` resolves each Symbol grapheme to a glyph; `inline/shaping.rs:929–936` uses ordinary fallback without inferring math from a font table. Current `equation.rs:49–52` installs the outer text strut before measurement, `math/layout/mod.rs:1354–1373` treats an entire Symbol as one glyph run, and `shaper/font_selection.rs:66–70` injects math fallback for ordinary DejaVu Sans because it has MATH. Coordinates, selected visual glyphs and source identity are observable output; storage/algorithm are mechanics. These are internal parity corrections under continuous ADR-0127, with no public contract/default/phase/compatibility change. Refutation requires bilateral PDF and SVG coordinates, adjacent-text, ordinary base/variation and grapheme controls; a passing glyph count alone is insufficient.

### Exclusive obligation
For inline equations admit the already measured normalized ascent through existing ensure_line_baseline, before placement, instead of first installing the outer text top. Keep the existing canonical maximum for mixed text, 0.7-leading slack, bounds edges, block continuation, actual callback/source bridge and Formula envelope. Do not add a coordinate compensation, reread glyph geometry, new state or phase. Test initial math-only baseline and mixed before/after text using distinct injected font edges, then replay all prior continuation controls.

Require actual RED→GREEN, scoped resealing with valid V15/V26, full workspace validation, unchanged 0.001 pt bounds and preservation of received changes.

## Natural block mathematics in horizontal Stack — 2026-10-27

### Measurement before classification

`lab/results/construction/diagnostics/math-stack-width-fix-2026-10-27/before.json`
freezes HEAD212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diff/stat/status,
2167 inputs and received release SHA d141e2ee123c262b37d0bf7e7c83fd00429504782b9f8f3c030072cd834cbdf1.
The source inputs exactly match the previous Stack completion. The ratified oracle
is commit64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1 and executable
SHA22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
Fresh bilateral `before3/{receipt.json,geometry.json}` retains sources, hashes,
UTC, argv and physical PDF/text observations. At400pt the following image starts
at210.146 rather than18.292pt; at200pt it starts110.146 rather than18.292pt.
A fixed100pt Block matches, but an Auto Block differs187.354pt, Auto Box194.354pt,
compound math/text173.0715pt and finite wrapped prose10.266pt. Fixed Box differs
2.5pt and a fixed inset Block3pt in equation position. Root/vertical/inline
controls match. Numbered LTR instead requires a full390pt child: image402 rather
than received210.146; blindly shrinking every equation is refuted.

`stack.rs:128-146` supplies the finite available region to each child;
`equation.rs:309-313` centers against that region using page margins, then
`:586-590` advances cursor through the displaced extent. `line_commit.rs:184-205`
records that right edge. `sub_frame.rs:289,357,569,614` isolates finite regions,
auto-width intent and the causal width witness. `block.rs:310,358-373` already
settles generic deferred X paths against completed allocation; `boxed.rs:496-535`
composes once and settles local paths after deriving Auto size. Pinned upstream
`stack.rs:173-179` disables child expansion on the stacking axis while retaining
finite region size; `math/mod.rs:204-209` returns raw unnumbered frames and
`:230-244` allocates numbered frames using region width. These are observed
language geometry/accepted content; Rust algorithms/bytes need not match. No
upstream intent is inferred. A following image shifted by region size, changed
finite wrapping, duplicate effects, moved fixed-width math, or changed root/
vertical controls refutes the candidate.

### Classification and preservation

ADR-0127 continuous internal parity flow. Keep public signatures/entity fields,
traits, dependencies, features, default modes, pipeline phases and compatibility.
Only existing layout-time frame policy changes. Private purpose/adapter arguments
may carry natural-width intent; no new Layouter field or independent dispatcher.
Keep received JPEG validation/streaming/dedup and the target alignment recipe byte
identical. Preserve all received notes/image/h/Stack/counter work. Invalid initial
multi-letter abc fixtures were rejected by both compilers; corrected a b c is the
usable control. `phantom` is rejected by the oracle and remains an explicit
out-of-scope binding refuter; the preexisting Repeat/math mismatch is retained,
not qualified or used to claim width parity. Probe completion, not an assumed
hang, is recorded; no process was stopped.

### Exclusive responsibility

Honor existing automatic-width intent for block equations. For unnumbered formulas publish the same measured raw logical width at the local line origin, compose there and enqueue one generic center fixup for its exact Formula envelope before line drain. Preserve finite wrapping; defer visual slack until the owning frame knows its width. The equation's own flush must not independently apply paragraph RTL/explicit alignment to an already owned deferred block position; save/restore that lexical setting around this flush. Numbered block formulas retain finite available-region logical allocation and their current number/callback protocol. Finite immediate centering uses actual line origin and regional available width, not global page margins; root origin remains equivalent. Existing infinite-root page fixups, inline math, raw extent, height and numbering semantics otherwise remain. Do not change font selection, mathematical layout, callback occurrence matching, counters or source identity.

### Acceptance

Add module-level functional RED before implementation for natural width independent
of200/400pt constraint, LTR/RTL placement, different-width display lines, compound
prose and finite wrapping, Auto/fixed Box and Block allocation, numbered reservation,
exterior/root/vertical/inline preservation and state/source effects once. Compare
complete source PDF and SVG geometry/pages/text at the unchanged0.001pt gate and
independent raster; keep refuters explicit. Revalidate all previous Stack/image/h,
Here/counter/footer, math/notes/relative-line and A-H/R1-R13 controls. Require all12
technical gates, unfiltered lint zero warnings/errors, V15/V26 before restricted
resealing and frozen source/L0/configuration during execution. Repeat JPEG17922,
frozen timing/CI/Luma and nine-case RSS bounds plus direct received comparison,
R6 normalized size/RSS/repeatability/math-time limits. Use one job,12GiB,zero swap;
no competing build during resource timings. Preserve raw previous receipts and
unrelated received inputs. No commit/push or new universal container parity claim.

## Regional origin and exterior RTL refuters

Measurement before refinement: `lab/results/construction/diagnostics/math-stack-width-fix-2026-10-27/attempt1/`
retains the complete frozen first candidate, release SHA66edef6720050e9e955a3912ff29c1b00473617337a4287b95fd3c35453cd1ea,
source archive, UTC/argv and all42 PDF sources. It qualifies36/42 and fixes natural
advance, but Auto/fixed inset Block math is still3pt too high: same preexisting
Y as before3, discarding active inset. `equation.rs:215-232` resets an empty
subframe baseline to raw ascent even when block_place_region supplies inner Y3;
`block.rs:289-309` already owns that actual local region. The first finite
text-dir RTL row instead needs the whole43.292pt Stack placed at351.708pt:
vanilla math/image x351.708/365 versus candidate5/18.292. This is exterior frame
alignment, distinct from formula centering. No intended upstream policy follows
from those observations; these are finite layout/source parity corrections.

Classification remains ADR-0127 continuous. Preserve prior positive geometry,
public contracts, phases and JPEG recipe. Add functional RED for active Block
inset origin and exterior RTL/nested-frame composition before changing code.

Exclusive refinement: the existing empty-subframe block baseline uses the active
block_place_region's Y plus raw equation ascent; if no such Block region exists,
retain the existing zero-origin policy. Do not add a font baseline or inset twice,
change ordinary vertical gaps, or create another region/source pass. The existing
Block owner continues to resolve its allocation and inset transport.


## Geometric continuation after closed block mathematics (2026-10-27)

### Measurement before classification

`lab/results/construction/diagnostics/block-math-vertical-advance-fix-2026-10-27/`
contains before.json, exact dirty file list/diff HEAD --stat, before-source.json.gz,
UTC and frozen2168 inputs, source-bound received release5c6e11a8f386a4d6e6d087188f72721fe9e729d74b07661f7861b7c920907226,
and oracle64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17,
SHA22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The36 bilateral PDF/SVG probes and36 public-position queries complete before this
specification. Strict SVG currently qualifies21/36. Stack/Auto/fixedBox lines after
display math differ by1.188pt at11pt; size8 differs0.864pt, size20 differs2.160pt.
Par leading0/20 yields5.962/14.038pt absolute differences. The public second-equation
position repeats those errors while its painted formula is repositioned correctly.
Root shape placement already uses the block-chain physical end and qualifies.
Three root-inset3pt controls and the inline-equation/line control are explicit
preexisting refuters, not evidence for a blanket container/inline rewrite.

At equation.rs:623-651 the own flush closes the formula through line_commit.rs:975-996,
which prepares next text top + text descent + par leading. The equation restores
raw descent/below fields and physical terminal bottom but leaves that text cursor.
shape.rs:259-268 takes cursor_y directly inside a subframe; realized.rs:311-316
records the same cursor as the next object's public origin. cursor.rs:107-174 already
admits subsequent prose from raw block end + collapsed gap + its own text ascent.
Pinned math/mod.rs:104-209 and flow/collect.rs:253-285 compose a measured block frame
with block above/below; this source observation does not infer upstream intent.

### Classification and scope

Positions, geometry and finite-page continuation are observable language behavior
(ADR-0107/0108). A transient cursor is mechanics, not an API requirement. This is
an internal parity/formula correction under ADR-0127 continuous flow; no public
signature/entity/trait, default mode, phase or compatibility change. It supersedes
the earlier scope-out of ordinary post-math gaps only for this measured causal
continuation. Keep inline admission and unrelated root-inset refuters explicit.

### Exclusive responsibility

After the display equation's own flush has settled/migrated its baseline, publish
the pending next geometric origin as that settled baseline plus raw equation
descent plus the existing pending block below gap. Keep last_flush_advance coherent
with this prepared origin rather than the discarded text-line advance. Preserve
the raw terminal physical bottom separately: pending spacing is not painted height
and must not enlarge a terminal Auto page or force a blank next page. Let the next
prose/inline object use its existing admission for its own ascent; never add text
edges or par leading to this geometric origin. Do not hardcode1.188pt, mutate the
math extent/font or shift already-painted formula glyphs/numbers. Preserve equation
numbering, horizontal natural-width/deferred centering, inset policy, context/source
identity, callback occurrence, spacing collapse and page/column admission owners.

### Acceptance

Functional module RED before implementation for line after block math across font
sizes/leading, independence from body text edges, empty paragraph preservation,
and numbered continuation; retain positive prose and terminal Auto-page controls.
GREEN then complete12 technical gates, build, full lint zero warnings/errors,
V15/V26 before scoped resealing, frozen source/L0/config throughout. Repeat all36
source PDF/SVG/queries and the previous42 width corpus/eight public-position probes:
close the two relative-line Y refuters and the second public equation origin at
unchanged0.001pt, preserving prior positives and explicitly classifying unrelated
refuters from actual before/after observations. Independently render/inspect.
Repeat prior Stack77SVG/32PDF/math6, effects/Here/footer, images/h/boundaries,
A-H/R1-R13 replay, language matrices, R6 size/RSS/repeatability/math-time and full
JPEG17922/936-paired/CI/Luma/nine-RSS/direct-received bounds. Keep JPEG validator,
Cargo recipe and every unrelated received input unchanged. One job,12GiB,zero swap;
no competing builds during timings. Preserve historical receipts; no commit/push.


## Active root Block origin exposed by geometric continuation

Measurement precedes this refinement: preflight-build/receipt.json and its complete
source archive bind the first corrected candidate to36 PDF/SVG/position sources
and5 edge sources in this task's raw diagnostics directory. SVG improves21/36 to32/36;
all11 targeted post-display line variants and ordinary second-equation origins close.
Three root Block(inset:3pt) controls retain a3pt painted omission. Their second
public equation origin goes from1.812pt error to3pt because the old1.188pt text
advance accidentally compensated part of the missing inset. The fresh-root arm
at equation.rs:234-245 still uses global page.margin.top even though Block has
already installed block_place_region.y (block.rs:289-309); the earlier subframe arm
already honors that region. This is an actual active-origin omission, not a reason
to reintroduce text advance. Root heights29/30/40/45 preserve complete SVG/page
geometry and now close all public origins. The columns-boundary fixture is an
explicit preexisting column-assignment refuter (max202.8pt both candidates), with
its post-first-equation Y corrected but no new column/pagination policy inferred.

This extends the earlier root-inset scope-out only for that measured first-block
origin. ADR-0127 remains continuous/private layout-formula correction. Before code,
add a functional RED using a real root Block with3pt inset. The fresh-root block
baseline takes the active Block's geometric Y when present, falling back to the
existing page margin only when there is no active Block region. Do not apply the
inset twice, alter the Block's own sizing/transport, or change inline math/column
assignment. Preserve every prior positive. Final ordinary complete SVG and public
origins must qualify35/36, leaving only the identical inline control negative;
four physical-page edge cases qualify and the columns refuter remains explicit.


## Rebind the certified physical end after changing its prepared cursor

Measurement before refinement: release-attempt1/relocation.json preserves the
complete failed release, frozen source/L0, build receipt, binary and all outputs.
The36 new sources close display math (35 SVG/positions, inline refuter retained),
but the42-source width preservation gate rejects fixed-block-control: the image
after Block(width:100pt,[$ x $]) is at18.2pt rather than23.18298pt, whereas the
received5c6e11a8 release qualified. The missing height is the4.98298pt raw formula.
At place.rs:112-120, the existing private end witness binds physical end to the
current prepared cursor and is valid only while that cursor is unchanged.
line_commit.rs:995 binds it before Equation replaces the text cursor. The new
origin therefore invalidates the witness; block.rs:349-353 correctly sees no
body end and allocates zero causal height. The consumer Block is not the owner
of this producer omission and must not grow a special-case repair.

Keep this as an internal ADR-0127 correction under the same exclusive owner.
Before code add a functional RED for a root fixed-width Block ending in a display
formula followed by a geometric flow object, deriving expected gap from the
actual painted formula frame. After publishing the new cursor, rebind the raw
physical equation bottom through the existing record_flow_placement_end helper.
Do not bind the pending gap as physical height or change the witness representation,
Block's sizing logic, phase, public contracts or any unrelated consumer. Preserve
the entire failed release and rerun module GREEN, exact old width/image controls,
all new observations, JPEG performance/compatibility and the complete validation
on a freshly frozen and rebuilt final release. There are now8 new tests in this
owner, including this distinct producer-witness regression.


The frozen geometry candidate passes, but its JPEG resource RED and qualified
same-source compiler controls are recorded in this task's timing-attempt1 and
decoder-opt3-qualification receipts. The earlier unchanged-Cargo restriction is
superseded solely by the measured package recipe specified in the exclusive
`00_nucleo/prompts/wiring/main.md` owner. Keep decoder productive bytes and all
other configuration unchanged; final gates must use the actual rebuilt default
recipe, not the experimental artifact. Equation geometry/contract is unchanged.


Integral lint acceptance also covers the new cfg(test) frame collectors. The
first full twelve-gate validation and JPEG qualification passed, but the extra
unfiltered lint rejected two `_ => {}` collector arms (V16, equation.rs:1511/1661),
recorded in lint-attempt1. Keep their semantic search unchanged: recurse through
containers; explicitly enumerate non-container FrameItem variants and continue.
Do not hide a wildcard empty arm or suppress V16. Reseal after V15/V26 and rerun
module/full validation and observations on the new frozen test/source state.

The collector terminal list refers strictly to FrameItem: TextSeparator, Text,
TextShaped, Line, Glyph, Image and Shape. Url/Destination belong to LinkTarget
and are not frame alternatives; the rejected compile is retained in
collector-compile-attempt1. No test expectation or productive algorithm changes.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Migrate only this consumer's ShapeElem literal/destructure: legacy rect_radius
and rect_stroke are None (or explicitly bound/observed for observation). Preserve
all existing values, expectations and other APIs; public addition is approved.
Do not materialize unrelated square/container features.
