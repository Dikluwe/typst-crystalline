# L0 — equation
Hash do Código: cef420b1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
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

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

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
