# L0 — placement
Hash do Código: 0be91b7e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:ad1c3b199f21c8a84384ff679c5c79a9b9e7cbbcc8d49e2d3f30a288fa210c47
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:4a93d5c55bf975b506b6f02ae484a91fdf7449716f9248dea2fcaa6b7b0738b6
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/placement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The consumer's operations were inspected at `01_core/src/compiler/layout/placement.rs:21,277,539`. Source inspection establishes current behavior and ownership, not upstream intent or new bilateral parity. Language-level text, morphology, geometry and diagnostics govern acceptance; traversal, storage and buffer structure are mechanics. Historical snapshots below retain their original oracle pins; none certifies the current reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Full chronological measurements and superseded wording remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/placement.md`.

The five `place_regression_*` refutators and the inspected ascender/clip/empty-page paths are recorded in `lab/results/construction/diagnostics/correcao-falhas-2026-09-28/`. The pending-line p1292 witness refuted the initial unrestricted geometric-origin attempt. Later `lab/results/construction/diagnostics/place-origem-local-2026-10-14/` before/extra-before, green-attempt1/attempt2 and baseline-expanded/candidate-expanded distinguish body origin from anchor: root/Box acquired a font-dependent Y strut, Align(Box(Place(shape))) exposed accidental cancellation, cell recomposition exposed absolute versus local Y, and simple/double Align exposed lost X. These later measurements supersede the early root-only/no-pending-line restriction for body composition, while preserving separate adapter flow correction and anchor obligations.

`lab/results/construction/diagnostics/block-align-2026-10-06/baseline-verification.json.gz` preserves 14 bilateral controls and full before-state provenance. It separates physical container alignment from run direction and baseline origin; after-inline/newline/spaced/leading and linebreak-baseline controls isolate par.spacing from leading. Two old P898 Bottom/Horizon Auto-height controls initially restricted the horizontal text certificate to absent/Top VAlign. `lab/results/construction/diagnostics/bordas-cantos-2026-10-06/verification.json.gz` preserves 12 controls separating Grid/Table geometric origin from baseline. Pinned source `flow/collect.rs:193`, `flow/block.rs:58-70,145-166,285` and local tracks calculations support these distinctions, not copying Rust mechanics.

The selected helper spans 722/766, AST/source, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, status and diffstat are in `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`; this is mechanical API evidence, not a performance claim.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` preserves 24 root/Box pairs after 12 prefixes; block-regions-before/results.json.gz preserves 24 local-region/clipping pairs. Both record HEAD, dirty tree, diffstat/UTC, sources, argv and binaries against historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`. The former refutes pending-line-only anchoring after committed producers; the latter separates Block inset/region/clip from zero Place advance. Body origin, flow boundary, Block region, fragmentation and late inline baseline remain distinct obligations.

`lab/results/construction/diagnostics/block-auto-width-2026-10-14/before/results.json.gz` and blue-geometries-corrected.json distinguish causal width from available width; the latter re-extracts actual #0074d9 rather than #0000ff from the same SVGs, without a rerun. `lab/results/construction/diagnostics/block-auto-alignment-2026-10-14/before/results.json.gz` measures lost geometric right/center/link clipping before causal width is known, including a wider sibling. All preserve the exact dirty state, UTC and binaries. `paint-root-2026-10-14/layout-first.log` and before-extra-test-owners isolate the affected test observer; the public paint-root manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved, as recorded in `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/align-flow-before/results.json.gz` records 40 documents, full dirty HEAD/UTC/diffstat/hashes/sources/binaries and historical pin. Positive Align frames lost Auto-page height and finite Top/Horizon entry; the old P898 no-margin assumption was refuted. align-flow-after/results.json.gz and tail21/25/29 distinguish own-frame consumption from later siblings; edge40/44 refute a paragraph gap after initial strong VSpace, and edge41/45 refute retaining a previous page's bottom. The horizontal Source-certificate gap and Align containing only Place remain separate refutations, not certified by this correction.

## Exclusive ownership and current contract

Own `layout_align`, non-float `layout_place`, `resolve_alignment`, and deferred horizontal/vertical fixups. State storage belongs to `00_nucleo/prompts/core/compiler/layout/mod.md`; element adapters belong to `00_nucleo/prompts/core/compiler/layout/align.md` and `00_nucleo/prompts/core/compiler/layout/place.md`. Preserve the static descendant interface and all pinned nuclei. This owner adds incremental alignment deltas to existing sublayout origins once; never add the full origin twice. Determine width from laid-out items and FontMetrics. Resolve vertical regions in cell, unconstrained-sublayout fallback, remaining-page priority, subject to the explicit Block and positive-Align rules below.

Preserve Place Page/Parent/Column scope, inherited per-axis cell alignment, dx/dy and out-of-flow emission. The adapter supplies pending-flow correction; no global post-layout scan is allowed. Compose every non-floating Place body through `layout_sub_frame_at_geometric_origin`, without subtracting an ascender or cell_origin_y from its local zero. No text-presence heuristic selects body origin, and non-text origin must not acquire a font catalog. Parent subframes anchor Y at sub_frame_origin_y; cell subframes also use that local origin rather than absolute body_y, while outside subframes retain cy. Preserve existing root/cell dimensions and absolute X conventions. In the parent-subframe Column/Parent branch, use `regions.current.line_start_x.0` as origin_x, preserving the cell path; Box/Transform already start their reference at zero. Text installs its own measured top edge. Scope, sizing, deferred metadata and outer cursor restoration remain intact.

Compose Align bodies geometrically too, without subtracting ascender. For the structural text certificate—Empty/Text/separators/Linebreak/Sequence/Styled/Strong/Emph/Par/Align and simple Block without paint/clip/outset—install a lexical horizontal override with Start/End resolved by container direction; a vertical-only Align inherits the parent's horizontal override. Layout once and rebase Y to zero; align each line through the cursor without a second horizontal envelope translation. Restore the override before parent recomposition. This horizontal flow-aligned strategy remains restricted to absent/Top VAlign; uncertified bodies and other horizontal strategies retain legacy recomposition. Auto dimensions must never introduce infinite coordinates.

For finite width, absent/Top VAlign and exactly one Grid/Table, optionally wrapped by Styled/Located/Realized or single-child Sequence, use geometric origin and the lexical override. The envelope fills available width to avoid a second X translation; local Y does not subtract ascender. Column starts belong to `00_nucleo/prompts/core/compiler/layout/grid/tracks.md`. Mixed bodies, infinite width and Bottom/Horizon retain the previous horizontal strategy. Gaps/nesting are refutators, not presumed parity.

Vertical Align consumption is independent of that horizontal certificate and Top/Horizon/Bottom. After laying out the body once, a positive causal height enters at the actual previous line end plus resolved par.spacing, or consumes the previous Block's pending margin through its existing protocol, respecting region changes. A newly opened outer region with a prepared baseline recovers its physical origin once; preserve local subframe bases and fit on a new page from its physical top. Use a previous-line witness only after the first baseline has actually been acquired: initial strong VSpace while initial_baseline_pending is true does not invent a paragraph. Do not use an already-prepared next baseline as the previous line end.

Publish the current positive frame end directly in last_block_descent_y and the existing block protocol, with below separate; never max it with a preceding page's end. The protocol retains prev_line_baseline=frame_end, below=resolved par.spacing and initial_baseline_pending=false, consumed only by following material. Do not add leading or terminal weak margin to final physical page height, duplicate a margin at completion, or invent positive consumption/margin for zero height. Finite Horizon/Bottom preserve consumption of the remaining known region. In Auto height, use the positive Align frame's own consumption for Y fallback; do not register its recomposition against the entire final page including later siblings. This supersedes the earlier positive-Align Auto-height deferral and P898 expectation. Place fixups and transferred subframe fixups retain their scopes; finite page/cell regions retain their known dimensions. At Align completion record target_y+sub_h and the prepared cursor as physical flow end, never Place ink or page bottom.

For an active Block, resolve its local region before composing the Place body, and isolate that region from the body's sublayout. Apply flow_y only when v is absent; outside Block preserve cell/subframe/page region selection. Auto-height Block defers Y resolution to its owner. Emit one matching translation for items, decorations and alignment metadata. Keep private decoration ranges associated with auto-Place Y fixup indices: hub stores them, checkpoint clones/restores, Block/subframe isolate/restore, transitions clear, this owner registers only segments emitted by the occurrence, and Block applies their same geometry delta before scope closure. Private state needed by this contract supersedes old extraction-only bans on new state; save/restore is LIFO.

Publish the Align subframe's causal width at its previsual logical origin. Place contributes no flow extent; only restoring its parent subframe retains the parent's existing extent. For geometric Align outside flow_aligned inside unresolved Auto-width Block, position initially against content_w and register the existing X fixup; preserve subframe transfer, metadata and causal width. Textual/Grid line alignment keeps its established route.

When auto dimensions remain infinite, use finite-content fallback and existing path-based fixups. Preserve Group/Link/Semantic paths, orphan rebase, physical/logical conventions, count/path resolution, entry order, mem::take of queues, incremental delta, axes and formulas. Horizontal/vertical pub(super) helpers accept `&mut [FrameItem]`, do not change length/capacity, and allow existing Vec coercion; no storage/public-entrypoint change. Recompose item/decor/metadata deltas in their own conventions, with baseline compensation only where required, apply once at final dimensions, then clear. Do not create page policy, checkpoint ownership or entity-to-render imports.

The paint-root change in this owner affects only its test observer: use the tree-coordinate helper owned by `00_nucleo/prompts/core/compiler/layout/tests.md`, projecting Group.pos/matrix to find Text leaves. Preserve expected content, values and tolerances; this observer change authorizes no productive algorithm change.

## Acceptance and explicit remaining scope

Require L0 first, meaningful RED→GREEN and unchanged negative controls, bilateral SVG/PDF positions and morphology, full core/workspace suite, all targets, build, rustdoc, fmt, Clippy, lineage and warning inventories with restricted resealing when separately authorized. No suppression, unsafe, new public field/trait/signature/default/mode/compatibility/pipeline phase or performance claim is authorized. Preserve text, styles, pages, errors, clips, images, fixup paths, markers, selection differential controls and rollback. Check simple/double Align with Place, text/non-text/mixed bodies, cells, Box, nested Align, explicit Block width, finite/Auto pages, Bottom/Horizon, gaps, scoping after Align, pending/committed lines, initial strong VSpace and page transitions. Check actual X/POST positions and inner coordinates, not only final page height; regressions demand measurement, not rewritten expectations.

Committed-flow anchoring covers certified line/Shape/Block/Align/Grid/Raw/VSpace producers only. Invalid witnesses use the legacy route and remain pending. Block local region/clipping is bounded to the surviving region; fragmentation, Auto backing/height, outset, deferred external decoration, fractional sizing, textual/mathematical Auto alignment, Place horizontal behavior and full metadata/decorator integration retain separate owner criteria. Late pending-line settlement with inline Box/Transform and audit of Heading/List/Stack/Equation/Columns are still required before universal Place certification. Align containing only Place and the horizontal Source-certificate gap remain open; no RTL expansion is implied. Tiling transport approval exists, but production rendering is certified only by integrated owner gates, never carrier-only tests. Do not declare the whole Place/Block/Tiling objective complete from these bounded corrections.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Transient Grid placement reconstruction copies source_order exactly. Preserve cells/header/footer/line placement, geometry, source provenance, and existing layout behavior. No order inference from separated storage.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.

## Source-owned physical page child transport

### Measurement and approved scope

At the implementation baseline, `placement.rs:96–160 and :428–476` selects the
actual body/subframe or deferred-body operation. Exact source/L0 texts, full
SHA-256, UTC, HEAD and working-tree state before this amendment are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/child-adapters/before.json`.
The approved review SHA-256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
and human `approval.json` under the implementation evidence ratify this finite
physical-page source-routing phase. Existing geometry is preservation, not a new
claim of upstream intent, global page parity or completed historical proposals.

### Exclusive routing contract

Actual Align and nonfloating Place bodies choose AlignBody and PlaceBody respectively and use the existing geometric planned-subframe adapter. Preserve all finite/Auto region choice, certified horizontal flow alignment, affine offsets, translations, fixup paths, style and cursor restoration. Neither a generated wrapper nor a measurement may borrow that source edge.

Use private parent-descendant protocol helpers with explicit typed source edges;
no new public API, entity field/variant, generic mapper, callback execution,
source re-evaluation, result cache or layout-from-Frame semantic reconstruction.
Retain legacy behavior when the planned cursor is absent. Actual source packets,
source parentage and physical placement remain distinct; measurement cannot
publish actions, consume transcripts or leak tentative diagnostics. Preserve
existing current styles, original/visual identity, per-store position sealing and
all stated geometry/effect/error limits. In particular, deferred footnote/float
causality versus later physical placement requires source-bound language controls;
this routing amendment does not declare that universal paint order is semantics.

Require actual nested source Context/Figure/label/counter cases, repeated pages,
no-paint children, wrong-edge/foreign cursor rejection, measurement inertness,
retained/deferred/rollback and existing geometry/baseline/clip/float/note tests.
No old assertion, fixture, diagnostic or explicit pending limitation is weakened.
L0-first source-current RED→GREEN, scoped lineage/graph/seals and full established
gates remain mandatory before completion.


## Full-region Repeat horizontal alignment (2026-10-21)

Measurement before decision: `lab/results/nested-fractions-repeat-2026-10-21/repeat-before/measurement.json.gz`
preserves exact source, font, oracle and product bytes, HEAD/uncommitted stat and
UTC for `fixed-align-right`. The existing scalar Repeat path matches that
control's residual horizontal offset. Pinned `typst-layout/src/repeat.rs:43–58`
applies inherited alignment to the repeat remainder. `placement.rs:75–92,120–124`
currently grants a lexical override only to a single Grid/Table frame; the new
full-region Repeat needs the same private flow alignment handoff to preserve
the measured language result. Envelope-only shifting of a full-width frame
cannot position its internal remainder. This is geometry, not a new phase/API.

The private full-flow certificate admits exactly one Repeat as well as the
existing Grid/Table, transparently through Source/Styled/Located/Realized and
single-child Sequence. Keep finite width and absent/Top vertical restrictions
and every other refusal. Install/restore the existing lexical horizontal override
so Repeat owns its residual and the outer full-width envelope receives no second
X translation. Preserve source AlignBody routing, actual height, styles, all
other alignment/Place formulas and prior tests/tolerances. Require meaningful
RED→GREEN for actual repeated glyph positions under external right alignment,
plus the immutable bilateral right/center/justify controls and full validation.
This is ADR-0127 continuous internal parity correction without public contracts,
new modes, compatibility or eval/realize/layout phase changes.


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

Preserve the optional resolved decoration association in existing coordinate translations; independently completed subframes return resolved tokens, so outer scopes cannot reuse child line identities. Alignment formulas and public contracts stay unchanged.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.

## Source-owned textual Align override (2026-10-22)

Before this correction, `lab/results/remaining-parity-2026-10-22/agents/repeat/source-align-regression-before.json` preserves sources/owners, UTC, exact dirty tree and immutable measurement hashes. The real candidate-2 build `feb025296531c92350e29bff032f655cc2fa47542cf8ab4aca0e239108bc4e8f` and its `qualification/repeat-8-matrix.json` observe explicit-child-left X at65.307373047pt versus oracle56.575195313pt; the same control was GREEN in `repeat-refuters-before/measurement.json.gz`. The excess8.732177734pt equals centering the actual7.535644532pt glyph in its25pt child. `placement.rs:22–70` rejects a Source wrapper in the textual classifier; `:121–159` consequently leaves the inherited center installed during the explicitly left-aligned actual body. The independently retained Repeat alignment exposes this previously masked Source-certificate gap. This is language placement, not a requirement for upstream private frame structure. A Source-versus-bare declared-metric contrast must refute or confirm the cause before implementation.

For this owner's actual Align admission only, treat an existing Source wrapper as transparent when its entire body passes the existing closed textual classifier, recursively through already admitted textual envelopes. Preserve the original actual body, span, source cursor and dispatch; do not unwrap/reconstruct emitted Content or execute anything during classification. Unknown/effectful/geometric bodies still fail the positive textual proof. Keep finite width and absent/Top vertical restrictions, explicit per-axis resolution/restoration, and all original geometry calculations. The existing `text_alignment_body` caller in Auto RTL Block retains its prior Source refusal: a private classifier choice avoids widening that separate certificate. No public API, new producer/mode/default, or phase is added; this supersedes only the textual Align Source gap, leaving the Place-only gap open.

Require actual layout controls for inherited center around Repeat of a25pt fixed Box: explicit child Left with bare10pt text stays GREEN, while the same Source-wrapped text must first demonstrate the RED and then reach the same expected residual-only x37.5pt in an80pt region at origin10pt. Retain the immutable eight-source bilateral matrix, earlier six piece/source-anchor regressions, source metadata/effects, Block caller controls, all tests and forward/reverse lineage. No fixture/tolerance/alert relaxation.

## Intrinsic textual Align inside a captured automatic piece

figure-frame-build1/figure-geometry-after1 caption-right loses actual caption
words; figure-alignment-red1 shows declared10pt CAP x724.181 outside the page
instead of its independently expected centered intrinsic frame. Current
placement.rs:168 uses an ordinary child subframe; :216–218 gives textual Align
the entire provisional available width. These measurements bind exact dirty
inputs, UTC, oracle64c5e8c0 and unchanged test fixtures/bounds.

For the existing finite absent/Top textual certificate only, when the parent
already has both active PieceAlignment capture and automatic intrinsic width,
compose the same actual AlignBody once through the intrinsic-piece adapter.
Settle its own captured lines against its actual causal width, preserving the
lexical explicit alignment and optical-margin policy. Its envelope has that
intrinsic width and no second page-width horizontal displacement. Keep the
original typed source cursor, own height/baseline, decorations, parent state
restoration and all old ordinary/Repeat/Block controls. Other textual/full
flow/geometry/vertical branches keep the previous strategy. This supersedes
only the old entire-width envelope for this proven captured intrinsic branch;
no new certificate, phase, source callback, mode or public contract is added.

release2 fixed-block-left actual BODY differs116.074pt although its wider
caption matches. The current exact source/native report precedes this proposal.
Add painted Block/root-Left functional RED before extending the captured
intrinsic branch. Only an actual independent Block (through existing transparent
source/style/location/visual or one-child Sequence wrappers) qualifies beyond
textual content, under the same finite absent/Top and active captured Auto
conditions. Its lexical horizontal override must reach the real body, while
its intrinsic envelope is settled once rather than displaced by provisional
page width. All other geometry/certificates/default callers stay unchanged.

figure-boundary-red1 confirms actual painted Block/root Left failure, retaining
all52 earlier controls. The intrinsic certificate includes a proven independent
Block as stated above; apply/restore its lexical horizontal override even when
it follows this bounded geometric intrinsic route. No other geometric branch
changes its current inherited-line strategy.

## Rustfmt gate repair (2026-10-29)

The frozen `physical13-validation2/fmt.log` under
`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/`
records a genuine formatting RED. Apply Rust 1.92.0 rustfmt to this consumer
without changing executable tokens, fixtures, expectations or assertions.
Preserve all behavioral obligations above; reseal both directions and rerun
the unchanged gates against the final frozen source. Internal ADR-0127 flow.


## Template paragraph geometry — measured repair 2026-10-29

### Measurement before decision

The immutable table-alignment-build1/source archive and
readiness-fixes-2026-10-28/template-isolation1, template-scope-before and
 template-explicit-before receipts preserve exact inputs/HEAD/dirty stat,
UTC, source/asset/binary hashes and pinned64c5e8c0 native coordinates.
Whole show and direct function call have identical defects: title is centered,
but author stays at the title's left edge. Plain one19pt wrapper before a
paragraph break places AUTHOR9.6pt too high; two sibling19pt wrappers or mixed
19/8 keep the correct11pt common paragraph spacing. A shared two-word19pt
wrapper still needs9.6pt more; custom2em needs16pt, absolute25pt stays unchanged.
An explicit Par around the19pt run intentionally keeps11pt outer paragraph
spacing in native; it is a control, not an equivalent replacement. These
observations support lexical common scope, not maximum/run-value consensus.
Current sequence.rs:347 closes shared segments only for Block/Align;
placement.rs:152 refuses Parbreak in an otherwise textual Align body.

### Exclusive amendment

Admit Parbreak as a neutral textual boundary only in the existing positive
text-alignment certificate, under the same finite-width/absent-or-Top vertical
restrictions and recursive proven envelopes. Each paragraph then receives the
existing per-line explicit horizontal alignment, rather than centering the
largest completed frame and left-aligning smaller paragraphs inside it. Keep
the actual source body/cursors, direction, heights, decorations/fixups and
lexical restoration. Opaque/geometric bodies retain their old route. Add a
pure physical RED verifying both differently sized paragraphs independently
center at their actual region, without changing baselines or expected widths.

### Acceptance and limits

ADR-0127 continuous internal language-parity correction: no public field,
method/signature, dependency, flag, mode or eval/layout phase move. Require
co-located genuine RED→GREEN, unchanged prior scope/align/Repeat/Figure/Table
and source/planning/rollback controls, full core and twelve stable-batch gates.
Rebuild original local-template and all retained controls, compare native
PDF/SVG/words/pages/0.001pt/paint without normalization. Explicit Linebreak's
separate mixed-size vertical mismatch remains open; admitting Parbreak does
not certify it. Keep opaque text-scope refusals, no guessed coordinates, new
frame scanning or linter suppressions. JPEG/historical gates remain required.

The shared legacy text_alignment_body caller used by Auto RTL Block keeps its
prior Parbreak refusal as well as Source refusal. Only actual layout_align
uses the source/paragraph-transparent classifier choice; a private parameter
may be renamed to identify this scope without any public signature change.
