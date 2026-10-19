# L0 — Private captured-position projection
Hash do Código: 5304343f

Núcleos Tekt:
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
