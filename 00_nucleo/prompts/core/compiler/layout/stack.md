# L0 — stack
Hash do Código: d7697968

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:029c2929c9e294efceb796a5358a6f425f02ae91c519314080f413374f4f4403
- 00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml sha256:67e2de824eb08850828b20d39da52b8d33c59ced2138ddb4e8a2a9113aaa3cf1
- 00_nucleo/prompts/_nuclei/layout/block-spacing-resolution.toml sha256:0177da7d5af56bbce26198ac775e603cec4529be91fb3567373c243543fa85e3
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/horizontal-space-paragraph.toml sha256:bbcb7c79c93e3d2da4580dfbc5586174a9698f02e3d3579ea886a1ff2608ed4b
- 00_nucleo/prompts/_nuclei/layout/nonexpanding-math-frame.toml sha256:8e6bd0ef718cc6d7e11292996777437b7f0fdb88c3c39d9d441c721459758563
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/single-region-stack.toml sha256:cf78ebaac0d5f3c4adbdc6ca2e6853ccd6b9ca565ec814e9f81b75a546089024
- 00_nucleo/prompts/_nuclei/layout/stack-frame-composition.toml sha256:f0c684c7c0dcaf29f7aaa29b8dbc260f15baba094fa4f6a605fc8cf16e81d8da
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/stack.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/stack.rs:15`, `01_core/src/compiler/layout/stack.rs:25`, `01_core/src/compiler/layout/stack.rs:115`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

FrameItem sites: F009 at `01_core/src/compiler/layout/stack.rs:88` (walk), F010 at `:252` (first_baseline).

AST spans, patterns/guards/bodies and complements were checked before code. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, sources/L0/configuration, hashes, status and git diff HEAD --stat. classification.json records 70 cases/complements across 26 owners using pinned CI-parser tree. Relevant L0 clauses were read with source; this proposes neither new architecture nor inferred upstream intent. Rust patterns/exhaustiveness are mechanics; product semantics/morphology/geometry/effects/diagnostics remain unchanged.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Close preceding text, establish initial baseline and preserve the incoming block boundary/bbox. Empty children retain the current no-emission path. Measure through `00_nucleo/prompts/core/compiler/layout/measurement.md`, lay out each child once in an isolated subframe and recompose according to the source's vertical/horizontal direction branches and resolved inter-child spacing.

Vertical composition uses measured baseline/ascent/descent and prior-child descent; horizontal composition uses actual child width and the shared top/baseline offset. Preserve current parent-block entry adjustment and outgoing last-bottom/pending-below state. Direct HTML markers do not influence child width or ink metrics but are translated and emitted. Existing non-text approximations and direction-branch limitations are not replaced by a new reverse-direction/balancing promise. Restore outer bbox; no independent page dispatcher is created.

In horizontal Stack, transport TextSeparator but exclude it from child terminal width/descent; shift only its position by the same delta as other items.

Treat TextReplacement as a transparent visual envelope in this owner's traversals; descend into children preserving text, order, coordinates and existing calculations, without a new algorithm or advance. The pinned text-replacement nucleus shares invariants only; this prompt owns only its declared consumer.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Publish composed-subframe causal width: maximum for vertical composition, sum with spacing for horizontal. Do not count Place as child width. Preserve existing vertical policy in this correction.

F009 walk: replace only wildcard pattern with nominal TextSeparator/Line/Image/Shape/Link group under existing FrameItem prefix. F010 first_baseline: replace only wildcard with TextSeparator/Line/Image/Shape/Group/Link. Retain full existing fallback bodies, preceding arms, guards, transformations, calls, returns and order.

Preserve callbacks/effects/metadata/clipping/fonts/positions/paint/width under their current owners. This work does not expand traversals, turn wrappers into leaves or substitute text/ActualText for geometry. Test helpers preserve fixtures/expected outputs/tolerances/counts/Unknown failure; cleanup cannot accept missing observation. Nominal alternatives retain the prior shared body and visible informative V19 notes; one arm's execution does not prove every alternative.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

This internal parity correction under ADR-0127 adds no public contract/default/phase. L0 precedes RED→GREEN/restricted resealing. Causal width is only part of the ongoing block-region/width/transport correction: Auto height/backing, late alignment, fractions, fragmentation, outset and PDF Parent retain their own criteria. Testing a carrier alone does not approve render. Preserve full bilateral controls/refuters, suite/build/doc/fmt/lineage/inventories.

Internal continuous-flow ADR-0127 maintenance introduces no enum/field, trait method/public signature, default, dependency, compatibility or phase. Do not materialize historical pending proposals. L0 precedes restricted bilateral resealing. Add no suppression, threshold change or match→if-let rewrite to hide V16.

Before code, run new FrameVisitor control and workspace suite with unchanged expected outputs, and establish structural RED for 70 family warnings through AST reading. Afterwards require zero covered-family V16 and no new warning, retaining external V16 as pending. For productive core matches reached by cargo check -p typst-core, use a disposable copy with a sentinel FrameItem variant to measure absent E0004 barriers before/present after. The probe covers only compiled sites, not cfg(test) helpers or L3 blocked by dependency. Across the whole package replay patterns/guards/bodies, verify unchanged enum/signatures, real build/tests and no stub bodies.

Revalidate full workspace suite, cargo build, Clippy workspace/all-targets -D warnings and -D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/configuration during commands. Heavy commands run sequentially with MemoryMax 12GiB/MemorySwapMax 0/CARGO_BUILD_JOBS1. Structural cleanup does not establish new global parity, memory gain or measured AI benefit.

## Source-owned physical page child transport

### Measurement and approved scope

At the implementation baseline, `stack.rs:168–180 and :234–246` selects the
actual body/subframe or deferred-body operation. Exact source/L0 texts, full
SHA-256, UTC, HEAD and working-tree state before this amendment are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/child-adapters/before.json`.
The approved review SHA-256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
and human `approval.json` under the implementation evidence ratify this finite
physical-page source-routing phase. Existing geometry is preservation, not a new
claim of upstream intent, global page parity or completed historical proposals.

### Exclusive routing contract

Both direction branches identify each actual child by StackChild(source_index), carry that cursor into the existing finite subframe, then retain current baseline, width, spacing, item/decor translation and composition. The preliminary stack measurement remains unplanned and contributes no semantic event. Never use the child value or final coordinates as its source address.

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

## Stack image frame correction — 2026-10-27

### Measurement before decision

`lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/before.json` records HEAD212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diff/stat/status and every received productive/L0 input, UTC and executable identities. before/{receipt.json,geometry.json}, observe-before.log and effects-before/{receipt.json,query-absolute.json} are fresh bilateral measurements against ratified64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1/SHA22897f93; received product17f39776 includes the completed image/h and JPEG work. Stack3 image tops14.834/22.072/29.310 versus vanilla5/25/45; Stack16 drift181.596pt; Stack20 is one page versus19+1 images on two pages. RTL/BTT source-order placement is wrong; mixed text/image, hidden and clipped children also fail. Pure horizontal text remains qualified. Negative spacing is rejected only by Tekt's P156I gate. Context metadata puts child19 on page1 versus2. Source stack.rs:115–119 returns zero descent for every child without text, discarding a computed bottom; :144 adds an outer text baseline, :181 uses the legacy-origin route, :219 advances by that zero descent plus text top. sub_frame.rs:315–344 distinguishes geometric from baseline origins, and :497–521 returns a cursor-based fallback except for previously owned cases. Pinned upstream stack.rs:274–300 accumulates completed frame sizes and :303–366 composes their geometry. These are observed language geometry/pages/accepted arguments; no upstream intent is inferred and matching Rust/PDF bytes is not acceptance.

### Classification and scope

Private parity correction, continuous ADR-0127: no public API/entity field, trait method, dependency, new product mode, phase or compatibility break. Existing signatures and source identities remain. The old ascent/descent fallback and reverse-direction limitations are superseded only for this measured frame-composition correction. Existing Ordinary/Footnote/Repeat subframe routes and all received JPEG/image/h/notes/line work remain preservation controls. No new page dispatcher, source replay, callback cache or semantic reconstruction from FrameItem is authorized. Use the existing region transition and static source-child protocol. Individual child-internal multipage text, fractional stack spacings and general alignment are not certified by these finite image controls.

### Exclusive Stack responsibility

Resolve margins by explicit above/below then spacing, Auto override, par.spacing, default1.2em, preserving presence strength including zero. Close a preceding real horizontal-only paragraph through its owner; flush real text and derive a geometric block top, trim the first above margin, restore parent bbox. Preliminary measure_stack may supply the existing bounded parent paint reference but never events. Layout each actual child once with its StackChild source edge through the Stack-specific geometric route, retaining returned logical height and producer causal width before the next child overwrites private results. Do not use image drawing width or text ink bounds as child frame extent. Preserve item/group/link/clip/style translation and existing decoration ownership.

Compose all four directions: vertical sums child heights and signed inter-child spacing; horizontal sums logical widths and uses the maximum logical height, top aligning children. Reverse direction reverses coordinates within the composed frame or physical fragment, without reversing source realization/order. Use the existing main-flow next-region route between atomic children when their physical end would exceed page_bottom_limit; preserve exact fits, trim inter-child spacing at the new region top, avoid empty leading pages and never create a page solely for prepared lower margin. Do not paginate isolated subframes. Source context/label/page metadata and counters must be checked after transport; failure refutes the candidate. No re-layout of a child to calculate dimensions or repair effects.

Publish the actual composed physical end, prev_line_baseline/end with zero extra equation descent, trim-able pending lower margin and causal frame width/placement; publish no inherited child text baseline as the Stack's own baseline. Existing frame-only and no-paint children keep real allocated height. Restore bbox on completion. Empty Stack retains no emission. Formula refinements here are independently grounded in frame measurements, not inferred from margin priority.

### Acceptance

Require functional RED→GREEN before implementation, source-current bilateral PDF/SVG<=0.001pt, all four directions, spacing/default/negative, target clipping, mixed/no-paint children, nesting and exact-fit/19+1/14+6 pagination, text adjacency and h boundaries. Preserve real Context/page/label/counter source semantics and all current positive image/h, A–H/R1–R13/notes/relative-line controls. Repeat all12 established technical gates, unfiltered lint with zero warning/error, frozen provenance and restricted lineage reseal. Recheck JPEG acceptance/complete decode/dedup/time/RSS and R6 size/memory gates with no competing builds. Bound the known invalid oversized oracle case explicitly; never hide a timeout/signal as a qualified rejection. Historical self-sealed stdout limitations remain explicit; do not rewrite old manifests. No commit/push.

### Diagnostic text control correction before implementation

The first RED has27 passes/13 failures. The horizontal text preservation control failed an incorrect expected AB: layout_types.rs:931–935 deliberately joins per-run diagnostic text with a space. Preserve this behavior and expect A B; likewise the mixed two-A projection is A A. Neither diagnostic separator is a source Content/glyph correction, and this control failure is not counted as a product defect. Real geometry, page and typed-negative failures remain strict. Repeat RED with corrected diagnostic projections before implementation. Region transitions must restore the outer parent bbox while committing a page so page-owned adornments never inherit the temporary Stack paint reference.

### Positive frame certificate refinement

The existing subframe tri-state flow_baseline (sub_frame.rs:472,527) distinguishes an explicit non-text producer Some(None) from absent certification and real text baseline. Restrict the new Stack-purpose causal-end return override to that positive non-text certificate with no pending real line. Retain text/uncertified/manual-flush height policy, including terminal literal text, and all existing purpose routes. This adds no witness field or new public API.

## Stack child origin transport (2026-10-27)

Measurement before decision: `lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/effects-candidate/context-binding.json` records the twentieth child with metadata location page 2, but both captured `here().page()` and the retained Here Location page 1; vanilla reports page 2 for all three. `page_running_requests.rs:990-1003` and `realized.rs:285-307` register descendant original identities only when absent. Ancestor registration can therefore prevent a local child carrier from being captured. This is observable location semantics, not a requirement to imitate vanilla mechanics.

Decision (ADR-0127 continuous flow): while entering the existing private Stack subframe, recapture the child's already-authorized original identities at the local origin, before dispatch. Reuse the selected source-plan traversal, operational/pruned/ref predicates and existing marker protocol; this anchoring mode must not admit source events a second time or execute callbacks. Without a plan, reuse the existing original-location collection, without introducing a new allocator/identity rule. Ordinary original registration remains only-if-absent. Later explicit placements remain authoritative and nested Stack frames repeat the same local transport. Geometry and semantic event ordering remain separate. No public contract, phase, global dispatcher or Context solver changes.

Acceptance: an ancestor-pre-registered realized child origin must follow that child's transported frame onto page 2; source-level stored Here Location, captured page integer and metadata Location must all match vanilla. Counter/footer source ordering and all current context stability gates remain mandatory.

Preserve a Stack child with image, descender text and literal terminal break under an explicit bottom-edge descender through the existing Styles/StyleDelta: the following sibling frame must clear the declared baseline+2.2pt edge in FixedMetrics at11pt. Default bottom-edge is baseline (metrics.rs:434-441), so assuming that extent without declaring the edge is invalid. The original tail fixture failure is retained in full-attempt2/tail-red.log as invalid setup, never product evidence. The corrected fixture passed the pre-guard implementation (tail-descender-red.log, exit0), refuting the proposed terminal-chain guard: existing line_commit already updates the closed bottom to the declared edge. Keep this as a positive preservation control; no new guard/default is authorized. Mathematical closed-block sizing has its separate unchanged p1132b RED→GREEN.

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

For actual horizontal children request the private nonexpanding Stack subframe policy, retaining the finite available width. Keep vertical children on their current policy. Use the completed causal child width; settle its returned generic deferred X paths once against that width using the existing resolver and a temporary queue that restores all parent entries. Keep children in source order and layout each once. Preserve geometric height, direction placement, spacing, page boundaries, original identities and pending Y behavior; never extract a glyph bbox or rerender the child to shrink it.

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

Exclusive refinement: for horizontal Stack with lexical text.dir RTL and no
explicit line-alignment override, align the complete composed frame to its finite
exterior right edge. Preserve source child order and each child's own stacking
direction. Under an existing automatic-width parent, defer that single whole-frame
Right operation through generic X paths until the parent's width is known;
otherwise apply the finite available-minus-frame-width translation during item
placement. Record causal width before this visual translation. Nested frames must
not each add the exterior regional slack. Other alignments/infinite constraints
retain their existing policy; this is not a universal alignment certification.


## Issue 8790 — single-region Block/Stack composition (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8790-fix-2026-10-31/before.json`
freezes dirty HEAD, UTC, exact diffstat/status and source hashes. The freshly
built combined-tree binary and each PDF/SVG source/result/argv/hash/UTC are bound
in `before/receipt.json`; the independent pinned64c5e8c0 oracle is in
`vanilla/receipt.json`. Literal, unclipped and TTB each produce two Tekt pages;
adding TAIL produces three. Underfill remains one page. Vanilla literal is one
page but overlaps children after exhausting the region; do not reproduce that bug.
`01_core/src/compiler/layout/block.rs:171-191` performs only an initial
unbreakable admission. Its existing private InlineRegion is saved/restored through
Block, checkpoint and subframe as an opaque Copy context. `stack.rs:173-201`
checks only is_sub_frame to select fragmentation; :220-226 anchors reverse
placement to total logical main extent. A Block body retains main-flow semantics,
so classifying it as a generic subframe would also change Shape margin policy.
These are source-measured mechanisms; geometry/page/content/effect preservation
are language obligations, not a requirement to copy vanilla mechanics or defects.
The issue's proposed algorithm and expected positions are hypotheses. Refute them
through co-located tests: five logical22pt frames must remain present, with local
BTT y48,26,4,-18,-40 under fixed70pt; unclipped/TTB must keep22pt separation.

### Exclusive internal correction

Use the Block owner's private allows_fragmentation query at existing vertical
and horizontal page-fragment admission decisions. Preserve actual StackChild
source order and one child layout, full logical child widths/heights, spacing,
paint/items and semantic transport. A vertical reverse Stack that cannot
fragment inside an actual finite Block region uses min(total logical main extent,
max(0, local region bottom minus Stack top)) only as the placement anchor. Never
cap child extent, advance or committed logical height to remaining space. This
permits negative overflow before clipping without accumulating children at zero.
Unbounded/Auto, ordinary paginated, horizontal and underfilled controls keep
existing allocation/placement. Restore bbox and retain pending margin behavior.
No new Layouter field, public API, phase, source replay or paint-bbox sizing.

### Gates and boundaries

ADR-0127 continuous internal layout/parity correction: L0 and seals before code,
genuine RED→GREEN, all-item/page/clip/TTB/BTT/underfill/prefix/nested/state
controls, actual source PDF/SVG/raster, full build/tests and crystalline-lint.
Preserve previous approvals and every unrelated working-tree change. Add no
ignored test, tolerance relaxation, normalization, hidden callback or wholesale
fallback. Exact provenance and effects-once controls must distinguish this Tekt
fragmentation defect from vanilla overlap. This does not certify arbitrary
multipage Stack children, general table/footnote pagination or explicit breaks
inside unbreakable containers. No universal parity/performance claim follows.


### Auto-nested finite-anchor refuter (before refinement)

`lab/results/construction/diagnostics/issue-8790-fix-2026-10-31/candidate1-rejection.json`
and `candidate1/receipt.json` retain the rejected candidate, exact source/owner
bytes, binary/inputs/hash/UTC/dirty state. The source nested control uses an Auto
inner Block under fixed70 outer Block. It keeps one page/five22pt children but
positions them at global98,76,54,32,10 instead of58,36,14,-8,-30. The inner
Auto block_place_region has infinite height; relying only on that immediate
region loses the nearest actual declared ancestor. This refutes full closure.
Keep an optional Stack placement bottom in the same opaque private Copy
InlineRegion: own declared inner bottom wins, otherwise inherit the outer
context through Auto Blocks. A feature-owned query exposes only that optional
bottom to Stack. Checkpoints copy and subframes suspend/restore the existing
opaque value unchanged. Do not change Block Auto allocation, actual Place
region, percentages, clipping or child logical height to carry this reference.
A nearer explicit120pt inner Block must override outer70; an outer Auto with
no declared ancestor retains natural placement. Add actual Auto-nested RED and
explicit-inner precedence control before refinement; no new public/phase gate.

The optional placement bottom is qualified by the producing physical page and
column. The feature-owned query refuses it after either changes; never transport
a finished region's vertical origin into its successor. Checkpoints still copy
and subframes still suspend/restore the opaque existing InlineRegion. This
qualification adds no page dispatcher or change to explicit-break semantics.


## Issue 8790 — local horizontal budget with Block inset (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8790-inset-2026-10-31/before/receipt.json`
and `vanilla/receipt.json` preserve exact UTC, HEAD, dirty-file list/diffstat,
source and binary hashes, commands and PDF/SVG results. At this combined
uncommitted state the five 22pt bars measure 138pt for a 140pt Block with
2pt left/right inset; the ratified vanilla commit
64c5e8c02fe0394d913ead36e42ef8c1334d2107 measures 136pt. With left7/right11,
Tekt measures129pt against122pt. Right-only2 and zero inset controls agree at
138pt and140pt. TTB and nested Auto retain the symmetric discrepancy.

`01_core/src/compiler/layout/stack.rs:114-118` takes child origin from the
installed line start but budget from available_width.
`01_core/src/compiler/layout/mod.rs:1179` subtracts physical page margins in
main flow; `01_core/src/compiler/layout/block.rs` installs the actual inner
origin and right boundary. For origin12, encoded right boundary158 and right
margin10, the local span is136, whereas subtracting both page margins yields
138. Asymmetric left7 produces the observed excess7. This is measured geometry
(language parity); boundary encoding is internal mechanics, not upstream intent.

### Classification and exclusive correction

ADR-0127 continuous internal formula/parity correction; no public contract,
product default, pipeline phase or compatibility change. Stack must derive its
horizontal child budget from its installed local origin and right edge:
max(0, current region width minus right margin minus local line start).
Use that same budget for Stack measurement and child subframes. Do not subtract
inset a second time or change the general available_width helper, Block
allocation/restoration, child logical heights, fragmentation or BTT anchoring.
The existing block-inline-region Nucleus supplies the shared local-boundary
obligation. No new field, dispatcher, mode, source replay or paint-bbox sizing.

### Specific acceptance

Add co-located RED tests for actual percent-width children inside real Blocks:
symmetric/asymmetric inset in four directions, nested Auto, translated local
origin, restoration after Block exit, and zero/right-only inset controls.
Then implement the local formula and require GREEN for these and existing
issue8790/Stack tests. Recompile and repeat the bilateral source PDF/SVG controls
using the exclusive fresh RAM target. Preserve page count, five child occurrences
and vertical placement/clip structure. Record exact code-state provenance.
This scoped closure does not assert global repository qualification.
