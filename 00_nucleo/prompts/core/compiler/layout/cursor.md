# L0 — cursor
Hash do Código: 8e04cf2a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/intrinsic-literal-width.toml sha256:49b2cc837799c63414a2f68dfd25375d2238543b9bd1d502adadfc78a8df6ab1
- 00_nucleo/prompts/_nuclei/layout/optical-overhang.toml sha256:8c4e3b767247e54f966c3a86be1b528b030dc4c8d05ea9b358533351720a3cf2
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:a3772778f9e542de87823509c8eac57904fc73fea9526cebe6052833a69ef806
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:06c0f9a3e9b81ea332d57174938fda46c0865535787945094cea27ee5afbd562
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/cursor.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/cursor.rs:27`, `01_core/src/compiler/layout/cursor.rs:33`, `01_core/src/compiler/layout/cursor.rs:41`, `01_core/src/compiler/layout/cursor.rs:94`, `01_core/src/compiler/layout/cursor.rs:195`, `01_core/src/compiler/layout/cursor.rs:314`, `01_core/src/compiler/layout/cursor.rs:429`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Historical pins do not certify current ratified `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new run.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Thirty bilateral controls in `lab/results/construction/diagnostics/estudo-correcao-falhas-texto-2026-09-28/` measure Tekt/vanilla terminal 2/1, explicit terminal break 3/2, continuation 3/2 pages. Isolated preexport emits Four/additional on page 2 baseline 101.84 with page height 100. line_commit.rs:657-696 predicts next baseline; region_progress.rs:113-128 transitions postline; cursor.rs:80-135/parbreak.rs:41-80 set baseline/spacing. `lab/results/construction/diagnostics/correcao-demanda-regional-2026-09-28/{initial.json,before/}` preserves original source/L0 hashes/state. First implementation's 36 real-font controls in first-after-pdf/first-pdf-green.log predate later rebuild; terminal-columns loses Three, terminal-footnote changes 2→1 pages with main/note overlap. Column startup must clear old block state, and terminal note fitting must keep conservative next-line clearance despite trimmed final geometry. No new footnote format/reservation/public contract is inferred.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

`lab/results/construction/diagnostics/correcao-tracking-escopos-2026-09-29/{before-pdf/results.json,summary.json,initial.json,before-pdf/provenance.json}` preserves 30-source/90-PDF state/commands/sources and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Repeated/nested equal-looking scopes and Hide/Box expose missing lexical producer identity, not a requirement to copy PartialEq/chain representation. `text_tracking.rs:50`, pinned inline/collect.rs:287-295/shaping.rs:1269-1295 and hide.rs:21-24/boxed.rs:157-208 locate boundaries.

`lab/results/construction/diagnostics/correcao-tracking-rtl-2026-09-29/{before-pdf/results.json,summary.json,provenance.json,initial.json}` preserves 66 inputs/198 PDFs, full source/commands/binaries/HEAD/tree against that historical pin. Explicit Latin/Arabic RTL show signed missing interunit tracking while 0pt controls discriminate independent Hebrew/initial-Literal-space defects. `text_tracking.rs:65`, `word_layout.rs:34,72`, `space.rs:25`, `cursor.rs:150-186` and pinned shaping.rs:1269-1295 locate rejection, signed application and real-right-edge anchoring. Ligatures/mixed direction/global pagination remain unmeasured; these observations do not establish intent/general RTL parity.

correcao-tracking-rtl-2026-09-29/after-pdf/results.json/summary.json/provenance.json and wrap-initial.json preserve 66 sources/198 PDFs and state: continued Latin RTL line differs one signed tracking interval while final line/text/breaks match. line_commit.rs:241-248 knows continuation but calls cursor.rs:150 without it; text_tracking helper eligibility, not global cursor nominal trailing width, supplies interval. Other wrap/hyphen offsets refute this inference; it is not universal mixed-bidi/initial-space rule.

`lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/final-rtl/{results.json,provenance.json}` preserves 66 sources/198 PDFs/source/binaries/tree/pin: initial Literal pair before abc shifts entire Tekt RTL ink by pair causal width 6.24pt/6.44pt/5.04pt at 0/+0.2/-1.2 tracking, while remaining glyph order/text match. cursor.rs:154-195 aligns ink right and helpers.rs:95-107 excludes separators. Prefix's physical right reservation, not fixed number or copied bytes, is refutable hypothesis.

`lab/results/construction/diagnostics/direcao-alinhamento-2026-10-05/initial.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/tree/UTC/source/binaries/thirteen probes where opposite-direction-only lines lose shared paragraph anchor (cursor.rs:156-162,line_commit.rs:210-216). Pinned flow/collect.rs:129-136/inline/mod.rs:189,232 resolves shared direction/alignment; whole-style/parbreak/wrap tests refute missing paragraph direction. `block-align-2026-10-06/baseline-verification.json.gz` preserves 14 source/command/box/binary controls and state before.json for automatic/fixed Block and lexical Align; positions/form, not copied buffer mechanics.

Raw margin failed candidates after/after-additional/refuters-red/refuters-second/edges-red/suites-third in raw-block-spacing-2026-10-07 preserve state/binaries/UTC and show wrong default/exterior size contribution, Strong regional ascent, explicit-zero-negative/Auto priorities and unwanted image/shape font queries. `00_nucleo/prompts/core/compiler/layout/raw.md` retains full measured contract; no broader paragraph-size fix is inferred.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

containers-clip-2026-10-13/before-renders.json shows oversized Block Shape committing two blank pages versus one clipped region. shape.rs:119/cursor.rs:125/line_commit.rs:438/region_progress.rs:115,131 permit external overflow inside is_sub_frame, while sub_frame.rs:109-121,317-340 cannot restore a committed page. `lab/results/construction/diagnostics/subframe-isolamento-2026-10-13/before.json.gz` preserves source/L0/HEAD/dirty state/status/diffstat/UTC; test conditions before changes, without raising heights/dropping items. Helper repair alone does not complete Block/Box collection/dimensions/clip.

box-baseline-inicial-2026-10-14/before/results.json.gz/before.json.gz preserves 12 sources/pinned binaries/dirty state/HEAD/UTC and phantom standalone Box ascent versus real adjacent-text strut; preserve absent versus installed 0. `block-par-spacing-before`/`block-par-extra-before`/before-edit.json.gz at transformacoes-2026-10-14 preserve Auto-margin/zero-gap/scope19 failures with same full provenance; run-size consensus is not shared lexical scope.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/scope-tail-before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/UTC/status/diffstat/source/binary hashes against historical pin 9f2b6e871; block-par-spacing-after/block-par-extra-after refute run-size consensus (one 18pt wrapper differs from two adjacent 18pt wrappers). sequence.rs:196-245 closes in exterior chain while paragraph.rs:12-90 already finds lexical common ancestor; pinned flow/collect.rs:129-145 resolves shared style before spacing. Tail after 1fr spills despite fitting, Styled18 height is wrong, explicit Linebreak overflows; cursor.rs:127-135 rejects exact bound, fractional_preflight.rs:170-211 separately measures nodes/direct Text correction. Source/L0 before-edit.json.gz and wrap/font 8-18/Strong/sibling/Align/column probes support whole-segment measurement, not copied Rust/bytes or universal too-tall-tail parity.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

mixed-style-metrics-2026-10-18first-after/first-bib-after/first-regions-after/candidate1-sources preserve first candidate changing only line_commit with seven passing tests. audit-candidate1 finds matching SVG heading y in six bibliography cases and italic→roman, but forced still differs 0.156. linebreak.rs:46–58 appends ExplicitLinebreakBoundary with empty exterior-style Text: causal for bidi, not material face. Twenty regional PDF controls preserve same binaries/pin/sources as before-sources: at 39.44pt Tekt sends ITALIC to page 2 versus vanilla page 1;55.14pt forced 2 versus 1 pages;70pt wrap 3+1 versus 2+2 lines. Initial fix corrects wrapped y but not regional decisions. Invalid multipage SVG-without-template attempt stays in regions-before as a failed command, not SVG parity proof; use valid PDF and future regional script requests PDF only.

region_progress.rs:83–88 installs one-face baseline without updating assumed ascent;`:143–152` admits forecast before actual text. paragraph_pagination.rs:43–59 rejects differing edges and disables positive-cost grouping despite known selected ranges. Pinned flow/collect.rs:196–239 protects first/last actual-height frames; distribute.rs:343–364 advances group only if it fits next region. Language geometry/words/pages, not Profile shape/SVG IDs. Premature jump/recount/new callbacks/Group-based metrics/width-selection-leading changes or column/note/float/NoTypography regression refute correction.

Candidate links ensure_line_baseline in paragraph_pagination to cursor's initial/block top computations: KnownTop 6 could still test exterior7 and evict prepared text prematurely. Pinned inline/line.rs:559–568 uses actual maximum frame. Prechange RED after real material boundary with previous baseline 7/below6.5/page 20/italic top6 requires baseline 19.5 on current page, zero widow/orphan isolating admission. This later refinement supersedes ordinary-continuation-only discrimination only for certified KnownTop in initial/block branches.

## Ownership and existing coordination contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the thin word/chunk and regional transition adapters, initial baseline, RTL line placement, marker-prefix coordination, pending-float drain and footnote-body emission. Word emission/selection and regional transition implementation belong to the separate owners specified below. The hub retains all state/checkpoints. Line closure and horizontal allocation belong to `00_nucleo/prompts/core/compiler/layout/line_commit.md`; transactional suffix decisions to `00_nucleo/prompts/core/compiler/layout/flow_suffix.md`; float fitting/reservation/emission to `00_nucleo/prompts/core/compiler/layout/place.md`; cell-tail emission to `00_nucleo/prompts/core/compiler/layout/cell_tail_commit.md`; auto dimensions to `00_nucleo/prompts/core/compiler/layout/page_geometry.md`; intermediate Page construction to `00_nucleo/prompts/core/compiler/layout/page_commit.md`; column close/merge/start hooks to `00_nucleo/prompts/core/compiler/layout/columns.md`.

Use FontMetrics.text_width as the word-width authority, including shaping and tracking; use the active space advance and baseline offset for emission. Establish the first baseline from the active top edge. At a finite full region with committed content, progress before emitting the next first line; do not drain new text beyond the old region then move it. Empty/unconstrained regions do not create no-progress loops. For an already-certified fractional tail only, preserve the existing bottom snap bounded by the outer below-minus-leading boundary and fitting tolerance; ordinary overflow is unchanged.

Maintain inline ascent/descent maxima and translate all current-line items, link/semantic children and matching metadata consistently when a baseline changes. Do not replace child geometry by recognizing Block, Place or future Content.

Delegated word handling retains ordinary measured wrapping, hyphenation fallback and overlong-word progress; `00_nucleo/prompts/core/compiler/layout/word_segment.md` owns script segmentation and quote tailoring, `00_nucleo/prompts/core/compiler/layout/hyphenation.md` owns break-point generation. Ordinary singleton closing must not acquire Sequence's validated-prefix exception.

The float marker drains only its captured prefix through the active regional distributor, preserving order and excluding later occurrences. Ask Place for admissible fitting and reservation; finish/advance only by the existing transition rules and require progress. After the prefix stabilizes, mark the hub's suffix checkpoint boundary without flushing pending text or unconditionally breaking. Accept a valid suffix before ordinary transition through `00_nucleo/prompts/core/compiler/layout/flow_suffix.md`; rejection is handled there, not by migrating only the line.

Regional transition preserves existing float/footnote order, auto-dimension timing, column delegation, page commit and next-region reset. Invoke cell-tail emission after the next top is prepared. Footnote bodies retain the existing separator, usable bottom, split/overflow and bounded first-body fallback; no I/O or callback execution is introduced. Use shared direct-private-marker neutrality via `00_nucleo/prompts/core/compiler/layout/html_frame.md`; transport them without inventing line height or an occupied region.

The implementation boundary is thin: layout_word/layout_chunk delegate to `00_nucleo/prompts/core/compiler/layout/word_layout.md`, space_width to `00_nucleo/prompts/core/compiler/layout/text_measure.md`, new_page to `00_nucleo/prompts/core/compiler/layout/region_progress.md`. Remove duplicate local word-width/emission/break/transition algorithms; keep every descendant signature/call order, cursor origin/baseline/inline bookkeeping, RTL positioning and marker/float/note adapters. Hub state/checkpoints remain `00_nucleo/prompts/core/compiler/layout/mod.md`.

## Current baseline and geometric rules

Establish initial text baseline from active top edge without fictitious typography for empty/nontext-only origin. Generic facade ensure_initial_baseline retains existing protocol; private ensure_text_baseline/ensure_line_baseline(top) use same algorithm with Text/KnownTop admission under region_progress. No duplicate geometry or new Layouter field. Store Some(top_edge) actually installed for text/defined transition, preserving zero; pending initial keeps None. Do not decide Box natural baseline here.

Initial/block baseline admission never requests exterior region during is_sub_frame; keep local baseline progress/formula and explicit new_page/float drains unchanged. After resolved initial/block paths, ordinary next-line demand delegates privately to region_progress, not during a preceding fitting-line flush with old-page paragraph baseline. The later KnownTop correction applies provided top also inside initial and block-chain branches; Generic/Text use current face there. Preserve font/size/leading/margins, no body measurement or invented paint/height.

For text after active block chain with zero margin, valid causal-flow witness gives physical end+gap+top. Zero margin is not proof baseline installed. Preserve existing positive/weak route, fractional snap/regional progress/fallback on invalid witness; pending geometric initial remains first branch. No Content-kind or paint inference.

Certified fractional tail equality at admissible bottom is included in existing snap gate, with existing upper fitting tolerance and below-minus-leading bound. This never permits ordinary/uncertified text beyond region. Revalidate top/bottom/next-region controls rather than relax generic overflow.

## Horizontal, textual identity and margins

TextSeparator transport preserves position/consumed nominal advance/style/origin/child order without new paint/baseline/advance. Internal baseline_y is pub(super) only for descendants to place carriers with same baseline_offset as Text. TextReplacement remains transparent visual children with preserved text/order/coordinates/formulas, no new algorithm/advance.

align_current_line_rtl() remains ordinary/subframe facade. Continued nonjustified RTL receives private continuation signal and adjusts right anchor by signed tracking from last eligible material text via `00_nucleo/prompts/core/compiler/layout/text_tracking.md`; finals/LTR/inside-glyph positions unchanged, no eligible unit→no adjustment. Do not use nominal trailing-space cursor as ink reference.

For direct leading Literal TextSeparator prefix before first text, derive causal extent from carrier positions/advances including already-applied intervals. Reserve right-side prefix width in RTL ink anchor and reposition prefix into its corresponding right band, preserving logical Vec/PDF text order and no duplicated tracking. No prefix retains old shift; LTR/final spaces/context markers/justification/continuation retain contracts. No hardcoded offset.

Shared paragraph alignment query from `00_nucleo/prompts/core/compiler/layout/paragraph.md` supersedes first-Text direction anchor, preserving run.dir/other formulas/Literal prefix/terminal tracking/Auto regions. Private explicit_line_alignment overrides automatic direction: Left retains origin, Center uses half free width, Right existing regional edge with real-RTL tracking correction. Absence is exact legacy path; no rewritten bidi text/direction.

Raw weak margin follows current par.spacing via existing prev_margin_is_parbreak protocol, including Raw-inherited below; explicit below remains strong. Earlier exterior paragraph and effective Raw-size margins remain separate. Explicit priority wins weak both directions; same strength max. Auto edge breaks shorthand and returns weak default/par.spacing. Explicit below<=0 keeps actual end+below with geometric baseline pending; weak below including 0 adds next text top through weak Cursor branch. Preserve regions/fractions/checkpoints and no extra nontext font query. `00_nucleo/prompts/core/compiler/layout/raw.md` owns Raw begin/end/style classification.

Hanging alignment uses effective final paragraph edge while retaining shared direction/Literal prefix/tracking. Do not rewrite regions/line_start_x to simulate indent or alter vertical admission.

## Acceptance and explicit limits

Preserve current language content/morphology/causal positions-pages/styles/diagnostics/static interfaces and all source-qualified limits. Internal ADR-0109/0127 parity changes need L0 first/RED→GREEN/restricted reseal, no new public/default/phase. Pure adapter split is not completion of audit findings; selectors' Unicode/overlong/measurement-override controls and corpus stay intact.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

Require occurrence/UTF8/restoration/parbreak tests and bilateral wraps for logical transport. Tracking corrections require RED→GREEN, bilateral text/position/line controls for both directions/signed values and preserved LTR/uniform/terminal controls, error-free build/lint and restricted resealing; all other recorded limits stay open.

Continued-RTL correction needs bilateral same-text/line controls, terminal/LTR invariance and tracked eligibility. Literal-prefix repair requires RED→GREEN/bilateral replay and 401 regressions without numeric offsets. Shared paragraph direction covers structural text only; unknown input falls back, no promise of mixed bidi/explicit-alignment/HTML/every paragraph. Test RTL paragraph/LTR line and converse, whole style/parbreak/checkpoints/subframes, no L3 metadata. Align certificate is bounded by nested/fixed Block/post-Align scope/auto/ordinary-text/prior corpus controls, unchanged negatives/no time claim.

Subframe overflow tests retain local oversized items/no exterior pages and ordinary external pagination; no test-height workaround. Installed-ascent tests preserve None/Some(0), initial Box geometry and all regression expectations. Auto-margin/fractional-tail fixes retain explicit overrides/style/region/terminal/Raw-Place-Align controls; unmeasured lexical/negative/mixed-priority scopes stay with owning protocols. Keep conservative terminal-footnote clearance despite terminal page extent trimming.

Necessary private causal state is authorized only for measured contracts, with LIFO save/restore. Certified line/shape/Block/Align/Grid/Raw/VSpace anchors do not complete Block/inset/fragmentation/late inline Box-Transform or other Heading/List/Stack/Equation/Columns. Invalid witness keeps legacy fallback/pending status; no broad Place or tiling parity. Revalidate full suite/HTML-context/PDF-SVG/build-doc-fmt-check/lineage/warning inventory.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

Mixed-face refinement is private continuous-flow ADR-0127 without public/product-flag/default/phase/callback-order change, preserving earlier refused/scalar paths. Regional certification expands only same-size/leading prepared text, zero baseline_offset, finite nonnegative edges, faces with different tops/descents; variable sizes/leading remain outside it. Require RED→GREEN four added regressions/prior controls,30+20 regional PDFs,132+94+21 corpus, broad suite/Clippy/Rustdoc/lineage under 12GiB/High8GiB/swap0/jobs1, recording failed attempts. No full variable-metrics parity claim.


## Measured nonexpanding Repeat alignment — 2026-10-22

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/repeat-refuters-before/measurement.json.gz`
retains exact source/font/binary/build inputs, UTC2026-10-22T12:02:34.973885+00:00
through12:02:51.345977+00:00, uncommitted HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227` and its dirty-file stat. Product
SHA-256 `9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`
and ratified oracle commit`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
are pinned. The eight-source manifest and full matrix are in the adjacent
`agents/repeat/` evidence. Nested fixed-child center/right and unequal-width
multiline center/right fail; decorated multiline also loses line paint.
Direct Shape, natural text and an explicit inner-left override are GREEN.
Repeat clears inherited alignment before composing its piece; pinned upstream
`typst-layout/src/repeat.rs:19,55–59` preserves lexical styles and separately
places copies. `shape.rs:251–283` emits flow geometry without line alignment,
refuting global translation. Nested-only inheritance does not fix the measured
multiline defect. Language geometry/paint are required; data structures and
iteration mechanics are free under ADR-0107. Classification and pre-edit hashes
are preserved in `agents/repeat/classification-before-L0.json`.

### Private contract

When an actual Repeat outer-piece scope is active, record current-line alignment using the same resolved direction, origin and continuation reservation as the existing alignment path, and defer its translation until that piece width is known. Exclude only causally associated decoration paint from logical text-line width. Inner independent frames keep their ordinary alignment. All other alignment/direction paths remain unchanged.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.


### Direct-flow source carrier regression control

Read-only review after the five directed alignment tests passed is recorded in
`lab/results/remaining-parity-2026-10-22/agents/repeat/marker-control-before.json`
with exact UTC, HEAD, dirty stat and source/L0 hashes. `shape.rs:195–201` flushes
its preceding marker-only line; `line_commit.rs:369–370,655` distinguishes no
causal line from actual line fitting but still invokes Cursor.
`html_frame.rs:83` already defines source/context markers as causally neutral.
The new piece scope must record only an actual causal line after excluding its
causally associated decoration paint; a source marker alone cannot acquire an
independent text-line alignment. A genuine Locator-generated Located Shape
control must preserve its actual source anchor at the same Repeat residual
origin as the shape, without a hidden half-frame displacement. This is a
regression hypothesis until its new test runs before the guard, not a measured
closure claim. Preserve the five earlier alignment tests and all source/physical
coverage checks. The private visibility warning is repaired by keeping the
new hub state private to its actual layout descendants, never widening it or
adding a lint suppression.

## Measured optical Center/Right accepted-line remainder

Current64c5e8c0 comparison alignment-overhang-before (physical12-build source
and executable identities, UTC/diffstat) isolates0.968pt Center/1.936pt Right
for Texto.; overhang:false, one period and plain Texto match. cursor.rs:271
subtracts content_right without the optical remainder that pinned
inline/line.rs:493–526 includes after selection. This is language geometry.

Before implementation add an injected actual3pt optical query RED with natural
aa. advance15pt in an80pt region starting10pt: Center starts44pt enabled and
42.5pt disabled; Right78pt enabled and75pt disabled. Preserve all source/width/
vertical/line-selection facts and default provider refusal. Use the shared
optical owner only for certified LTR with the final accepted policy enabled;
add its amount to free width before the existing Center/Right multiplier,
also when the translation is retained in a private intrinsic scope. Unknown,
RTL, disabled, opaque and existing literal/tracking/hyphen routes retain their
contracts. No second justification, selection cost or public/phase change.

figure-optical-red1 is functional: the48 existing readiness controls pass and
both injected enabled optical cases fail after their disabled controls pass.
Apply the measured accepted-line remainder now, preserving natural advances.

figure-boundary-before isolates retained terminal Literal space: enabled
Center/Right differ2.343/4.686pt; weak counterparts agree. The actual cursor
received a Literal advance but only aligns painted text, then compensates
preceding punctuation. Require supplied-metric RED before using the existing
line_commit literal extent projector as a separate final LTR alignment bound.
Do not change FontMetrics.line_content_right. For retained final/explicit lines
combine actual text and Literal extents; normal continued lines retain existing
trimming. Use the optical owner's retained-tail proof only for the former,
preserving current soft-wrap justification, RTL literal-prefix and every policy.
