# L0 — line commit
Hash do Código: 835dbb45

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/horizontal-space-paragraph.toml sha256:bbcb7c79c93e3d2da4580dfbc5586174a9698f02e3d3579ea886a1ff2608ed4b
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/layout/intrinsic-literal-width.toml sha256:49b2cc837799c63414a2f68dfd25375d2238543b9bd1d502adadfc78a8df6ab1
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/optical-overhang.toml sha256:8c4e3b767247e54f966c3a86be1b528b030dc4c8d05ea9b358533351720a3cf2
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:06c0f9a3e9b81ea332d57174938fda46c0865535787945094cea27ee5afbd562
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:68203377765579bc29327c7d8fded6f4f831d11682b6b7c9abbf2dbf65b28bad
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:802ff8f08c2cc2e7a81987f63f980ddc1254d4ff4f8c972804d6bb3e84b4c484
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/line_commit.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

`01_core/src/compiler/layout/line_commit.rs:20,66,189` was inspected. Language text/styles/geometry/pages/errors govern parity; Rust buffers/bytes are mechanics. Historical probes do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3`; full chronology is archived at `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/line_commit.md`.

font-demand-geometry-study/green.stdout identifies a no-material flush still querying text edges: neutral metrics may remove lookup, never drain/reset. Regional-demand/overflow receipts separate real ink fit from hypothetical next-baseline fit and prohibit local-subframe escape. The effective-line-maximum probes/GDB at 476 distinguish actual 10pt from exterior 11pt; columns P1381 requires a separate outer fallback. Box-baseline-initial-2026-10-14/first-candidate.json.gz refutes conflating emitted ascent and historical advance fallback in P1381/P1392. Literal-only line and auto-height receipts (correcao-auto-proximas-2026-09-29/after-legacy33/results.json) require baseline admission before closing carrier-only text.

Logical-before-complete absolute-markup/literal and logical-spacing-after/results.json.gz distinguish trailing literal at normal wrap from mandatory terminal advance, and allocated terminal fractions from bare cursor motion. scope-tail/prefix-nesting receipts isolate geometric multiline mode. Par-box-fronteira-2026-10-17 before/finite/regions measurements certify root finite whole-line migration only with real prior current-region unit; inherited prior-page bottom is not proof. All receipts retain source/state HEAD/dirty-tree/UTC/diffstat/binaries and their historical oracle pins.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz` preserves same HEAD/uncommitted state/status/stat/UTC/source/binary/historical pin. Multiword21 gains page/line leading, par 40/8-fill gains page, OUTER18/INNER8 shifts BODY/upshrinks auto page. fractional_preflight.rs:151-171 refuses multiline; word_layout.rs:49 closes in local style; line_commit.rs:398-480 uses it for floor/leading/predicted top; sequence.rs:239 refuses partial closure and missing prior chain uses spacing 8. Pinned flow/collect.rs:129-145,179-214 resolves shared trunk/spacing/leading, each frame contributing geometry. These are language text/geometry/pages without buffer/algorithm/byte or historical-intent obligation.

mixed-style-metrics-2026-10-18first-after/first-bib-after/first-regions-after/candidate1-sources preserve first candidate changing only line_commit with seven passing tests. audit-candidate1 finds matching SVG heading y in six bibliography cases and italic→roman, but forced still differs 0.156. linebreak.rs:46–58 appends ExplicitLinebreakBoundary with empty exterior-style Text: causal for bidi, not material face. Twenty regional PDF controls preserve same binaries/pin/sources as before-sources: at 39.44pt Tekt sends ITALIC to page 2 versus vanilla page 1;55.14pt forced 2 versus 1 pages;70pt wrap 3+1 versus 2+2 lines. Initial fix corrects wrapped y but not regional decisions. Invalid multipage SVG-without-template attempt stays in regions-before as a failed command, not SVG parity proof; use valid PDF and future regional script requests PDF only.

region_progress.rs:83–88 installs one-face baseline without updating assumed ascent;`:143–152` admits forecast before actual text. paragraph_pagination.rs:43–59 rejects differing edges and disables positive-cost grouping despite known selected ranges. Pinned flow/collect.rs:196–239 protects first/last actual-height frames; distribute.rs:343–364 advances group only if it fits next region. Language geometry/words/pages, not Profile shape/SVG IDs. Premature jump/recount/new callbacks/Group-based metrics/width-selection-leading changes or column/note/float/NoTypography regression refute correction.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and current contract

Own horizontal fraction/repeat expansion, justification and physical line closure/baseline/leading/decor/causal numbering. Hub owns buffers/checkpoint. Delegate ordinary progress through cursor to `00_nucleo/prompts/core/compiler/layout/region_progress.md`, repeat multiplication to `00_nucleo/prompts/core/compiler/layout/repeat.md`, callback matching/final marginal-width adjustment to `00_nucleo/prompts/core/compiler/layout/par_line_callbacks.md`; no page assembly/Sequence preflight ownership.

Finite fraction line sums weights without requiring individual/total positivity. Add provisional Box natural widths back to distributable remaining space. share=max(weight/total*remaining,0) only when ratio/remaining finite, else 0. Replace provisional occupancy by share-natural, h(fr) natural 0. Repeat uses real boundary, shifts suffix once, inserts before it and repairs later indices by actual insertion. Infinite-width/orphan clears defensively without looping. Publish allocated fraction occupancy as prior material extreme+total allocated delta even without following material; use max of extremes, not sum/double count. Preserve zero/negative weights, cursor and repeat formulas.

Justify after fractions over only opportunities before material end, preserving RTL/Link/Semantic bounds; no final-trailing opportunity. TextSeparator contributes no visual metric; update its pos/advance by displacement difference of endpoints and exclude trailing carriers at existing ink limit. Reconstruct TextReplacement from justified children with text intact/no extra envelope delta. Literal-only separator line is nonetheless causal text: call existing text_baseline::admit before metrics, include carrier styles, reset x/publish baseline on close, no ink/duplication. Other empty/direct-marker-only lines drain normally with neutral local edges and no FontMetrics/page/typography.

Consume validated-prefix one-shot on first ordinary flush; explicit certified closure clears it through same close path. Common closure clears terminal_paragraph_line; Paragraph rearms only after its own final close. flush_continued_line justifies only Bool(true) then normal flush; no final stretching or explicit-flag change. Without active justification only, pass private continuation signal from this close to RTL alignment; ordinary/wrapped-prefix closures remain terminal, with no new hub/checkpoint slot or fixed numeric offset.

Preserve ordinary size/leading/next-ascent selections independently of actual emitted edges: mixed sizes or multiple items differing from outer use outer next-ascent/leading with current maxima; certified larger singleton uses outer advance, smaller retains own baseline/ascent/next-ascent and outer leading. Outside columns actual text supplies max_font_size/max_style from first actual item, no exterior-seeded fictitious maximum; no text keeps exterior fallback. Columns preserve exterior max and measured P1381 behavior. force_outer_singleton/fractional-prefix controls remain unchanged except certified geometric mode.

Aggregate actual relevant text edges in one static visitor/local accumulators, no vector per line: top maximum and bottom minimum are independent, not inferred from largest nominal size or first face in tie. Read Text/TextShaped/Literal TextSeparator; Link/Semantic/TextReplacement visit children once with no replacement metric. ExplicitLinebreakBoundary/ParbreakBoundary Semantic markers retain children/positions for bidi/copy but supply no top/bottom; authorial Text-empty outside them still supplies metrics. Other real Semantic bodies remain transparent. Neutral context/position/capture markers do not query fonts. Group keeps transformed box/inline extent, never remeasure local children; Glyph/Shape/Image/Line/weak separators keep prior protocol. Match nominal/exhaustive with visible intentional V19 alternatives.

Installed ascent Some, including 0, is authoritative; only None permits historical fallback. Effective direct-text ascent=max(real text top, inline ascent), nontext only inline. Capture emitted real text before choosing advance fallback. For ordinary real text reconcile predicted baseline with effective ascent using signed difference so smaller face top preserves line top; no text keeps positive-only correction/fallback. Columns/force_outer_singleton keep historical protection unless geometric_text_lines certified. Descent uses greatest actual compatible descent, not largest-size face bottom. Translate line/collectors together, preserve leading/advance and store Some(next_text_top) for next line. Under geometric_text_lines use actual material edges without column font floor; next top/leading come from text_segment_chain and signed reconciliation, preserving ordinary routes/decor/causality/overflow.

After baseline stabilization certify real bottom against effective region before cursor advance: current baseline+max(actual text descent,inline descent), not predicted next baseline. Existing reserved/fractional rejection or whole flow-suffix transaction/ordinary transition remains; never migrate only current_line from a reserved transaction. No regional migration/rejection in is_sub_frame: drain local line/content/positions/extents/decor normally.

In certified ordinary finite root only (no columns/floats/reservations/pending transaction/checkpoint/exterior decoration collector), before numbering/commit measure whole expanded line with still-pending ascent correction and no phantom nontext descent. If too tall for remaining space and a prior causal current-region unit exists, close old region and move the entire uncommitted line with wrappers/internal coordinates together. New baseline uses real ascent at useful top once, retaining horizontal advance/indent relative to newly resolved margins. Collectors/numbers arise after migration, never move already-committed exterior collectors. A first overhigh unit commits once for progress, no invented empty page. Prior-unit test excludes current_line and previous page's last_block_descent_y; use committed ink/current-region causal witness. Fitting ink never creates a page/rejects suffix from next-baseline prediction.

Number only material non-subframe lines after stable baseline/page. Preserve document/page scope, absent/none neutrality, pattern formatting and Func request/realized-Content consumption without executing Func; missing resolution records request, no provisional number. Final physical column uses end margin, prior columns configured margin. Keep greatest-width regional fixups/clearance/alignment/overlay suppression/edge compensation. Segmented pattern context buffers baseline/page/pattern/chain candidates without immediate increment/format; existing callback request path and Columns consolidation/shared-style capture remain.

Before visual alignment/numbering record causal line extent from material visual end and Literal carriers, excluding terminal Markup/Newline/margin numbers. Expanded Box/fractions supply their carriers. A nonjustified continued line excludes terminal Literal advances from extent; final/explicit close retains them; justified line keeps expanded occupancy. Preserve text bytes/order/baseline/style/breaks. The helper also serves subframe after fr/repeat and before RTL without extra vertical flush. Literal-width projector explicitly enumerates neutral variants returning 0, retaining wrapper recursion and requiring future-variant decision.

After geometry/alignment before transfer close active link segments separately in committed buffer and current line; reopen both after transfer so unrelated prefix is excluded, boundaries neutral. Drain in order, clear opportunities, publish causal baseline/advance/descent, reset cursor_x to effective start and request only remaining necessary progress. After causal drain record flow_placement_end=baseline_y+line_descent alongside prepared next cursor; empty flush invents no boundary. Clear only pending paragraph direction, retaining active scope across wraps; justification queries same paragraph direction resolver. Ratified carrier constructions/patterns retain uncertified None/read `..` and pinned origins.

Fractions/repeats/justification use effective paragraph end; numbering retains exterior margin. Notify hanging State of commit only after causal drain, reopening horizontal application for next line. Collector segment uses admitted effective left with unchanged end_x/baseline/order, not a replacement collector or changed pagination.

## Acceptance and limits

Require meaningful RED→GREEN/bilateral text/positions/pages across fractions/repeat suffix indices/zero-negative-infinite weights, terminal h 1fr/h 2fr/h0fr, normal-wrap versus terminal Literal, causal Literal-only spaces, final natural justification, all prefix/singleton orders/column controls, installed 0/nontext Box/smaller text/transparent envelopes, line numbering/callbacks, reserved rollback and ordinary/subframe overflow. Preserve all expected values and actual geometry, not representation snapshots. Finite-root admission includes finite60–70, first tall Box, empty Par→pagebreak→tall Box exactly causal pages, and excluded mechanisms retain their protocols.

Mixed-face tests before code use injected FontMetrics: equal-size faces both orders, smaller size/larger top, distinct top/bottom dominating faces, signed next-line reconciliation, transparent envelopes, opaque Group and neutral weak separators; preserve P1381/P1392/zero/prefix/callback controls. Require 30 PDF/SVG pairs plus regional 20, prior 132+94/decoration 21 corpus and fixed-date PDF snapshots. Separate any new horizontal/container issue, never relax full 0.001pt equality or hide lints. Regional mixed-face expansion remains same size/leading, baseline_offset 0, finite nonnegative edges; variable sizes/leading remain outside certification.

Build/full suites/Clippy/Rustdoc/fmt/reverse lineage/differential warning gates under 12GiB/High8GiB/swap0/jobs1; separation alone does not fix known terminal empty-page limitations, complete Repeat/Box fractions, arbitrary flow/RTL or resource/render parity.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

Mixed-face refinement is private continuous-flow ADR-0127 without public/product-flag/default/phase/callback-order change, preserving earlier refused/scalar paths. Regional certification expands only same-size/leading prepared text, zero baseline_offset, finite nonnegative edges, faces with different tops/descents; variable sizes/leading remain outside it. Require RED→GREEN four added regressions/prior controls,30+20 regional PDFs,132+94+21 corpus, broad suite/Clippy/Rustdoc/lineage under 12GiB/High8GiB/swap0/jobs1, recording failed attempts. No full variable-metrics parity claim.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.

## V16 closed-domain evidence and correction

Measured in the uncommitted working tree at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:02:56.055664+00:00`; exact changed-file diffstat and source/L0 SHA-256 are in `lab/results/v16-corrections-2026-10-19/batch-d/agents/layout/audit-before.json`. `line_commit.rs:660–722` admits only `Some(Value::Str)` and `Some(Value::Func)` after segmented-pattern deferral; the overlay at 693–697 can replace the admitted value with outer `None` after increment. `line_commit.rs:723–843` consumes realized content and positions numbers; it never executes a callback.

This is observed local source behavior, not a new bilateral result or inferred upstream intent. Identity, text, quote choice, geometry and supplement semantics are language obligations; the match representation is mechanics. A caller admitting a supposedly excluded payload, duplicate event, changed text/geometry/request or changed diagnostic refutes preservation.

Keep the admitted numbering domain closed at the existing realization match: Str formats, Func records/resolves the request, outer None means overlay suppression. Name each present non-Str/non-Func Value payload as excluded with the existing no-number result; these payloads cannot reach this match through the current admission, so do not broaden counting. Deferred patterns bypass admission and remain coordinator-owned. Preserve page/document scope, physical baseline, callback request count, unresolved Pending behavior, and counter advancement under overlay suppression. A newly added Value must require review at this exact decision, even though admission is currently narrow.

ADR-0127 classification: continuous-flow internal nominal coverage with no public signature, default, phase or compatibility change. Add behavior-preservation controls before implementation and require baseline/final controls plus disposable future-variant compile-negative evidence at the exact selected decision. Current runtime controls may already pass; the structural RED is failure to reject a future variant at that decision. No new suppression or open catchall is authorized.


## Source-owned note continuation correction

Measurement: `lab/results/set-page-deferred-corrections-2026-10-21/fit-3/receipt.json`, `fit-3/tests.log`, and unchanged pinned-oracle page text under `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage/` precede this amendment. Exact ownership and source-only records are in `agents/layout/` under the correction root.

After an actual note-capture text line has been completely admitted, aligned and drained, report its item-index/physical-end boundary to footnote_fragments. Report no synthetic boundary for an empty flush. Atomic source scopes and nested subframes suppress internal cuts. Existing line geometry, main-flow admission, numbering and diagnostics stay unchanged; this is a private geometry witness, not a source event or a new pipeline phase.


## Private first-flow baseline witness

### Measurement before correction

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/zero-source-after/measurement.json.gz`
binds actual source, candidate/oracle SHA-256, fonts, HEAD, uncommitted diffstat
and UTC. Zero-copy Repeat loses its prepared baseline, moving adjacent BEFORE/TAIL
while its body height remains real. Source inspection at pinned
`lab/typst-original/crates/typst-layout/src/repeat.rs:19–29`,
`inline/box.rs:32–47,79–89`, and `flow/distribute.rs:689–699` shows that
a prepared frame retains baseline even without copies, and the FIRST actual
in-flow frame decides baseline presence. The local Box paint scan cannot see it.
Exact preamendment owners/sources and state are archived in
`lab/results/nested-fractions-repeat-2026-10-21/baseline-witness-before.json`.
These are language geometry obligations; private carrier representation is mechanics.

### Exclusive private obligation

At a nonempty actual line commit, after final ascent/admission/geometry has settled, publish its true baseline before its physical flow end. Empty flush publishes no baseline; no font-dependent synthetic strut or early unadjusted glyph baseline. Keep all current line formulas, numbering and source coverage.

Require genuine RED→GREEN for empty Repeat baseline, positive copies, earlier Text,
earlier empty frame, nested insets/shift, Shape/empty controls, LIFO and checkpoint
restoration, then the same pinned bilateral SVG/PDF observations without relaxing
1e-8 geometry or semantic membership. Unknown legacy producers retain their
previous fallback unless a measured owning frame explicitly supplies a decision.
This is an ADR-0127 continuous private layout correction, with no public contract,
new product mode/default, compatibility break or eval/layout phase change.
L0 precedes implementation and scoped resealing; full tests/Clippy/rustdoc/fmt,
zero Tekt errors/warnings, paths and forward/reverse lineage remain mandatory.


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

Commit the actual deferred line association only after causal fitting, baseline and link-boundary changes have settled, immediately before draining the real local line. Bind collected decoration segments to that line. Past-line decoration paint in a later tail retains its own association; it cannot substitute the later text-line width. Preserve source markers and item order.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.

## R3 math-line text demands

### Evidence before decision

`lab/results/math-geometry-r3-2026-10-23/{candidate-debug/receipt.json,space-reductions/receipt.json}` bind the candidate source, binary, UTC, dirty stat and original/source fixtures. Admitting only the normalized math top corrects isolated alpha, but refutes identity-symbols (0.9211875 pt per paragraph) and selected-multichar (same initial offset). Actual middle Markup/Newline spaces and an explicit linebreak, even without a preceding space, require the outer text strut in pinned vanilla. Trailing weak whitespace at EOF does not; Literal does already. `line_commit.rs:105–129` excludes weak separators and ExplicitLinebreakBoundary, while `linebreak.rs:48–57` already carries the actual break's text style. No new glyph measurement is needed.

### Exclusive obligation

For a pending line containing an actual inline Formula, include the retained interior weak-space text style and the actual ExplicitLinebreakBoundary text style in the existing text-style maximum. Only interior weak spaces with later material qualify; trailing/suppressed weak separators and Parbreak scaffolding do not. Preserve the existing behavior on lines without Formula. TextSeparator remains neutral ink/transport: this is paragraph text-strut resolution from actual authored retained text occurrences, not an invented glyph, export change, new line, new state or independent placement algorithm. Keep all typed edges/leading, region admission and rollback under existing owners. Actual RED→GREEN must require isolated math=math top, interior space and explicit break=outer maximum, terminal weak=math top, mixed text=maximum. Revalidate original R3, all prior line/continuation controls and full gates under continuous ADR-0127.

The final-candidate preflight in `lab/results/math-geometry-r3-2026-10-23/preflight-attempt1/preflight-lint-complete.log:1–8` rejects the compound weak-separator guard at line 143 with V17, after the six focused geometry tests pass. Split only that Boolean conjunction into the existing math-text guard and a nested later-material check. Preserve the false/terminal no-op, style observation and every semantic branch. This is private Rust decision form, not a new geometry rule. Do not suppress V17 or change tests; final full source-frozen suite must reconfirm all six named R3 tests and full lint zero.


## Leading empty footnote line — 2026-10-26

### Measurement before decision

`lab/results/construction/diagnostics/leading-footnote-line-2026-10-26/baseline.json`
and received-source.json.gz preserve HEAD212aed0449150dd296ceea13fc2be665cc5db01a,
UTC, dirty stat/status and2165 inputs received after the terminal-note correction.
The18 paired sources in before/receipt.json bind the received release to oracle
commit bd5ebec285505889e8ff1d4f68804ab147a699f3/tree
cac836c07e4dec2a89219370428292393a3216b5, SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
In the original DejaVu Sans11pt case, per-page words/page count agree but the
entry marker starts1.403865pt below vanilla. leading-study.json preserves Tf
sizes: note9.35pt, entry marker6.54226pt. Half their difference is1.40387pt.
Nimbus Sans native-sized marker is a zero-shift control. Other font/size cases
also expose different region admission; a local y difference is not a global
geometry qualification when marker pages differ. Repeated leading breaks are
retained refuters, not silently certified by the single-leading-line sample.

`line_commit.rs:489-533` resolves uniform-line leading from the last textual
fragment. visit_text_styles excludes ExplicitLinebreakBoundary outside Formula;
a marker-only first note line therefore supplies the reduced script size.
Pinned flow/collect.rs:176-188 resolves paragraph leading before collecting its
line frames. Marker glyph size and note paragraph leading are distinct language
observables. No upstream implementation structure or Rust equality is required.

### Exclusive bounded obligation and gates

Keep actual script glyph geometry/ascent/baseline, next-line ascent, source
Linebreak identity and all captures/positions intact. For the first source-owned
footnote line whose actual textual styles are all superscript, resolve leading
from the active outer note paragraph style, not the reduced marker fragments.
The existing private capture must be enabled, at atomic depth zero, with no
previous committed cuts; require actual text. Observe all retained text styles
through the existing transparent visitor, including multi-run markers. No empty
fake body glyph/strut may be injected to force mixed-text classification.
Normal mixed-text lines already use their existing outer-leading path. Preserve
all other uniform-line/ordinary-subframe/column/scalar-wrap/Formula rules and
later script lines. No changes to certification of actual blank lines, terminal
witness reconciliation, public contracts, modes, defaults or phase ownership.
This is an internal parity correction under ADR-0127 continuous flow, not an
ADR-0127 public addition. Keep the current owning Nuclei and their pins intact.

Add assertional RED tests using actual line commit for a captured marker-only
first line, multiple superscript fragments, and ordinary/no-capture controls;
check emitted marker positions remain unaltered while following-line advance
uses paragraph leading. Require GREEN and source-equivalent bilateral PDF/SVG
at0.001pt on the original leading-note source, three fonts/sizes and terminal/
no-leading controls. Retain repeated-leading-line refuters separately if they
still fail; broader blank-line certification is not authorized by this fix.
Revalidate the terminal48-case corpus, original R8-R13,32 remaining refuters,
A-H/R1-R7 language/render replays, resources/stress, complete workspace/test/
Clippy/fmt/rustdoc/build-all-targets, strict/reverse lineage and zero full lint.
Preserve the received terminal implementation and every unrelated input/history.

Measured candidate refinement: leading-footnote-line-2026-10-26/{red3.json,
candidate-tests.json,candidate-exploration-evaluation.json,cohort-classification.json}
retains the valid RED(three controls pass,one leading assertion fails), four GREEN
tests and all24 paired exploratory sources.21 sources with one initial marker-only
line qualify in PDF/SVG; the3 repeated-leading-break sources still fail and remain
explicit refuters with unchanged source hashes. Final acceptance must rerun both
cohorts separately, preserve all characters/paint and report their different gates.
The earlier cancelled/misconfigured test fixtures remain archived and are not the
assertional RED used to certify this correction.

## Image flow and horizontal paragraph correction — 2026-10-27

Measurement precedes this correction: `lab/results/construction/diagnostics/image-flow-cause-2026-10-27/{baseline.json,results.json,refuters.json,completion.json}` binds HEAD212aed044, uncommitted inputs/diffstat/UTC, pinned64c5e8c0/tree483bd6f1/SHA22897f93 and received Tekt SHA2941a5ba. Direct PDF streams show image1 at y5 and images2–16 at y13.2; vanilla has eight30x20pt images per page at step46.4. Removing h gives vanilla step33.2 but retains Tekt overlap. Margin30 still resets later images to13.2; a60x40pt image does too. Source `image.rs:198,325–335` advances cursor without publishing the physical end consumed by `cursor.rs:128–169`. Vanilla `rules.rs:771`, realization `lib.rs:1061–1087` and flow `collect.rs:162–188,240–285` distinguish image blocks and h-only paragraph gaps. These are language/geometry observations, not a requirement to copy buffers, PDF bytes or upstream execution structure. No upstream intent is inferred.

Classification: private parity repair, continuous flow ADR-0127. No public fields/signatures/trait methods, new product mode, phase change, dependency or compatibility break. Keep all received footnote/R11–R13/JPEG work. New shared obligations are pinned in the horizontal-space-paragraph Nucleus; this owner authorizes only its exclusive consumer.

### Exclusive correction

At real line closure ask the h-space owner to consume or close a pending horizontal-only paragraph before normal line measurement. Actual causal line presence discards only the pending cause and follows the unchanged normal flush; no extra strut, numbering, text or advance. A closed h-only paragraph has zero physical height and no current_line items. Image placement closes it before deriving its own origin; normal empty flush without a producer remains neutral. Preserve all literal/footnote/superscript/line metric protocols.

Require targeted RED→GREEN, bilateral PDF/SVG<=0.001pt, source order/complete image counts, all source-margin/size/spacing refuters, prior R8–R13/notes and A–H/R1–R7, full build/tests/Clippy/fmt/Rustdoc and full lint without warning/error. Preserve known tracking/paint limits; remeasure JPEG latency/RSS against received/legacy/oracle and retain R6 resource bounds. Snapshot provenance and preserve unrelated inputs/historical receipts. Validate V15/V26 before restricted resealing; no commit/push.

### Measured lexical horizontal scope refinement

The candidate2 frozen state and pilot/geometry.json in image-flow-fix-2026-10-27 show mixed-h-scopes and siblings-h-scopes differ by43.2pt at the third image, while sole-h-scope and wrapper-h-scopes match. Equal styled siblings do not establish a shared lexical paragraph wrapper. Resolve h-only paragraph spacing from the nearest actual common StyleChain ancestor of all producers. Preserve the original anchor and monotonically narrow to its ancestor; do not infer lexical scope from equal values. Existing raw_deltas supplies identity without a public contract change. The private pending state stores an Arc-backed source chain and ancestor depth; normal one-producer closure resolves directly, mixed scope may reconstruct only the ancestor suffix for resolution. Checkpoint clones, subframes save/clear/restore, region reset and real-line absorption apply to this complete state. No public API, source reevaluation, paint, dependency or phase change. Require RED→GREEN for mixed outer/sibling scope and sole/wrapped controls, then rerun full validation.

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

Only at actual LTR justified-line emission, add certified terminal optical
advance to the already computed remaining distance. Use actual Font/GID/advance
for TextShaped; unshaped text uses its provider. Keep true terminal opaque
objects, overhang:false, single-glyph-only and unknown cases on the old path.
Do not change break_selection, line fitting, source queues or relative gap
translation; preserve all regions, counters, source phases and width recordings.

Require real RED→GREEN, strict lineage/lint/full suites/build and fresh paired
PDF/SVG, keeping all historical negative and performance/resource obligations.

Keep a private optional final lexical text-overhang decision, recorded by
word_layout::push_text and reset by actual line flush. It certifies only a
terminal text occurrence admitted by the optical policy; missing evidence does
not fabricate an enabled policy. This prevents style restoration from changing
the already emitted terminal occurrence. No public carrier field is added.

## Preserve accepted optical policy through final alignment

figure-optical-second1 preserves the two injected Center/Right failures after
the Cursor formula correction. Source :421 clears line_optical_overhang before
:733 calls alignment, refuting that the formula alone suffices. The existing
false controls pass. Retain the accepted policy in a local variable while
clearing the hub slot at the original closure point. Supply that exact local
policy only around final Cursor alignment and clear afterward. Preserve early
empty returns, justification, selection, regional geometry and next-line state;
no additional field/public/phase or late style-chain inference is authorized.

figure-boundary-red1 confirms final Literal width loss; expose only the
existing literal_line_right projector privately to Cursor, with unchanged
recursion/excluded variants. No new algorithm, cursor advance or metrics API.

## Rustfmt gate repair (2026-10-29)

The frozen `physical13-validation2/fmt.log` under
`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/`
records a genuine formatting RED. Apply Rust 1.92.0 rustfmt to this consumer
without changing executable tokens, fixtures, expectations or assertions.
Preserve all behavioral obligations above; reseal both directions and rerun
the unchanged gates against the final frozen source. Internal ADR-0127 flow.
