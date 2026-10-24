# L0 — Private captured-position projection
Hash do Código: 31311917

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/position_marker.rs`
**Architecture:** ADR-0073, ADR-0074, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

box-contexto-2026-10-14/probe.json.gz measures invariant local(0,0) under inset/prefix despite changed geometry, refuting baseline-only explanation. `mod.rs:1131-1148,1180-1200` and `realized.rs:66-75` capture during layout; `finish.rs:295-302` seals maps; html_frame already transports private points without public FrameItem variant. Decision snapshot: posicoes-compostas-2026-10-14/before.json.gz. Identity/geometry/query are language; index/carrier coding mechanics.

First-candidate review at position_marker.rs:173-192 finds that skipping a last capture without surviving frame lets an older projected capture overwrite latest runtime fallback. Latest write remains authoritative even if its frame is discarded; first-candidate-validation.json.gz preserves this refuter for Location and Label with older surviving capture.

links-captura-2026-10-14/followup-probes/results.json.gz preserves HEAD/tree/UTC/binaries for page-ref/logical-ref: heading physical page 1 but ref page 2/R8, while heading query page 1/counter 7. position_marker.rs:129-131 queues every carrier; references.rs:39-41 registers Label after body; heading.rs:111-130 already closed its line, leaving isolated carrier for next text's admission. labels-fronteira-2026-10-14/before.json.gz preserves prechange state. RED must retain heading/Label on first page under automatic overflow to test the inference.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [242]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/position_marker.rs`. Own private write/capture helper and final projection. Private enum distinguishes Location/Label; hub stores append-only list. Write same runtime-map values during layout with existing overwrite/only-if-absent policy; only an actual write creates capture. Allocate no Location or closure execution.

Emit direct carrier with capture index/point using only existing FrameItem: Semantic Artifact(Other)/Inline with own NUL prefix and zero Rect Shape, no fill/stroke. Recognize exact format, never legitimate artifact/product text; index creates no public ID. Detailed HTML retains its separate protocol without duplicate paged carrier, while runtime writes remain.

Final projection walks pages/layers/tree once, composing Group.pos/matrix; transparent wrappers add no origin. Record first geometric occurrence of each capture index and remove carriers. Resolve only latest logical capture per destination via reverse traversal/private seen-destination sets independently of paint order; visual copies are not new observations. If latest capture has no surviving marker, preserve existing runtime write and never reapply older projection, invent Location/page or require discarded occurrence.

Update only corresponding positions/pages in existing sub-stores before finish sealing. No per-Box global-map scan, layout replay/logical-index rebuild/I/O/API/phase change. Work is proportional to iteration tree/captures. Preserve producer's captured anchor rather than converting every anchor to vanilla baseline. Removal also serves intrinsic HTML visual return.

A Label registered after a body whose line is closed accompanies committed items when they contain content and pending line has no causal content. Emit carrier into committed buffer with same point/latest-write policy; never move it to next text's page. If pending line still has causal content, retain carrier there through admission/wrap/composition; with no committed content retain old pending path. Prebody Location captures do not adopt Label's postbody rule.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. Group reconstruction captures/reinserts paint_root unchanged; projection/marker removal/rebase never reset true→false.

## Acceptance and limits

RED→GREEN: Box inset/prefix/nesting and Labels project coordinates; first-point Shape retains origin; identity/order/or_insert/overwrite survive; visual clones do not change event; respect matrices/wrappers/pages/layers/rollback. Carriers create no paint/baseline/extent or public-document residue. Existing HTML positions/visual remain. Validate build/lint/suites.

Test Heading then text requiring another page, Label in pending line admitted on next page, inline same-line Label and no-label content/pagination neutrality. Revalidate Box/rollback/latest-write/context/explicit Pagebreak. Preserve prebody point capture/counter formatting; unmeasured multipage labeled bodies remain open. These are internal continuous-flow ADR-0127 corrections.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Extend only the private capture target domain with an already-admitted source-packet event index and Root/PostChildren placement role. Reuse the existing zero-ink exact marker format and the same geometric projection/removal pass to associate an ordinary source event with its accepted page. A deferred event retains its separately reserved semantic anchor; projecting its geometry never moves that semantic page or ordering token. This is geometry transport, never reconstruction of semantic membership, parents, query fields or counters from paint. Source admission occurs before capture and requires immutable source-plan authority. Post-child captures accompany committed material under the existing Label discipline; prebody captures retain the existing pending-line rule. Preserve all Location/Label overwrite and first-capture behavior, visual alias identity and per-store sealing. No fake Location, public FrameItem variant, additional global frame walk, callback execution or morphology change is introduced. Rollback truncates capture indices with the same source-event transaction; markers cannot survive in final output or create visual height.


### Deferred anchor evidence

The pinned oracle cross-page receipt `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage/receipt.json` records NOTE at physical page2 with counter3 before page2 HEADER counter4; FIRST is page1 counter2. `oracle-extended/receipt.json` there separately records a deferred note before SECOND on the same page despite later painting. These language observations refute sorting semantic events by final paint page or flush time. A deferred child reserves its semantic page and hierarchical source-order token at its actual enqueue frontier; it executes and admits facts only when the accepted deferred body is processed. Descendants and remainder retain that token. Final geometric positions remain separately sealed and may be later pages. Rollback restores reservations and admissions atomically. No public API, default or additional pipeline phase changes.


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


## Terminal nonmaterial source positions

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/positions-final/receipt.json` binds actual CLI query observations to the unpainted candidate (build/source snapshot retained in `unpainted-candidate.json` and `unpainted-candidate-source.json.gz`). `terminal-metadata-query` emits TAILPAGE 2 on Tekt's only page versus TAILPAGE 1 in vanilla; a following real page correctly retains page2 in both. `finish.rs:297-310` projects only committed pages before sealing; `position_marker.rs:182-200` never sees pending carriers from an omitted successor, leaving provisional runtime page2. Pinned `lab/typst-original/crates/typst-layout/src/pages/collect.rs:11-14` explicitly describes terminal tags and `pages/mod.rs:236-243` places them at (0,last frame height). Page/position queries are language observations; carrier storage is mechanics. This refutes closing the correction by page counts alone.

### Exclusive correction

When terminal composition has omitted an optional weak/PageRun successor, project only the actual pending private position carriers onto the end of the final effective page, at x=0 and y=page height. Use their existing capture indices and normal latest-logical-write/first-visual-occurrence policy. No runtime-map scan/clamp, fake identity, frame paint, semantic replay or new public API. Preserve Packet geometric projection without changing deferred semantic anchors. Pending direct carriers are guaranteed nonmaterial by the terminal decision; do not reinterpret arbitrary discarded frames as terminal carriers.

Require RED→GREEN for terminal Location/Label positions and actual source-derived metadata query, repeated weak breaks with following material, parity/strong/PageRun controls, marker-free output, latest writes, transformed positions, rollback and complete revalidation. This is continuous-flow ADR-0127; no default, public contract or phase change.
