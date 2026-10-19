# L0 — page canvas
Hash do Código: cdcd83ec

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/page-canvas.toml sha256:2aa7464200337a5915eec7850b8cf4488b9a471877220342546661f9823f19f1
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_canvas.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_canvas.rs:17`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`page_canvas.rs:27` lays out background/foreground through layout_sub_frame. Link inheritance in that path gives the canvas the active main-flow link when a page closes, although canvas content is independent. Snapshot: `links-captura-2026-10-14/before.json.gz`. The control requires an unlinked background and foreground with its own destination on pages closed during a multipage link.

The human approved manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [39]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

For optional background/foreground Content, return no layer when absent. Otherwise lay out once in an isolated canvas sized trim plus resolved bleed, and place the resulting group at negative left/top bleed with identity transform and no clip. Preserve finite canvas-relative dimension resolution and rich body content. Do not treat this region as the main-flow content rectangle, rasterize it, execute page functions or alter trim/PageConfig. Typed bleed resolution belongs to the existing page-canvas entity owner.

Suspend ambient lexical link scopes during canvas composition, restoring them after the subframe. Continue capturing explicit links within the canvas. Preserve bleed, dimensions, translation and layer order. This is internal, without a new API/phase/default.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.
