# L0 — divider
Hash do Código: 9c3d694d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/divider.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/divider.rs:19`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` records HEAD, working tree, diffstat/UTC and sources/owners before migration. `layout_types.rs:510–511` still transports Paint/Stroke; `shape.rs:151–191` materializes only fill in groups; `tiling.rs:85` limits/clones the grid. `prepare_cell` already returns TilingFrame once per occurrence. Probes `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz` establish that dense-open and stroke-open still diverge in all three targets. The contract was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Preparation/object counts are transport mechanics; render, contours, text, geometry and diagnostics are observable language obligations.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Close horizontally pending text, add the active block-spacing amount above and below the divider, and emit the existing black horizontal stroke of thickness `0.5pt` across `region.width - 2 * margin.left`. Preserve this consumer's present symmetric-margin calculation; it is not a new asymmetric-margin guarantee.

Construct FrameStroke/FramePaint::Solid for the already-resolved black outline, preserving thickness and geometry.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Apply the pinned nucleus. Ratification supersedes older clauses awaiting approval or requiring declarative Paint/Stroke inside Shape only within the approved contract. Preserve all other existing behavior. L0 comes first, followed by RED→GREEN; explicitly migrate fixtures/consumers without a total From<Paint> conversion, PartialEq on carriers or a solid fallback merely to obtain a green build. Do not raise the old limit or move layout into L3. Validate fill/stroke transport, resources, clipping, affine Parent, native targets, Solid/Gradient controls, page semantics and the complete suite/build/doc/fmt/lineage. Prepared transport, native target integration and verification (historical T1c/T2/T3) are complete only together; a broken or partial intermediate state does not certify production. The remaining historical T4 scope and publication remain outside this package.


## Automatic page width and divider — issue 8525 (2026-10-31)

### Measurement before decision

The independent agent-layout and agent-svg-math reports/receipts under
`lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/` reproduce
infinite SVG viewport/divider geometry and PDF Letter fallback/lost second text.
`01_core/src/compiler/layout/divider.rs:35` multiplies the unbounded-region
mechanism into visible geometry; `page_geometry.rs:62-69` then includes that
shape in intrinsic page extent. `finish.rs:44-48` and intermediate page assembly
consume the contaminated result. Exact received HEAD, dirty stat, UTC and
source/owner bytes are frozen in
`lab/results/construction/diagnostics/auto-page-width-8525-2026-10-31/before.json.gz`.
This is observed geometry/content semantics; it does not establish upstream
intent. The pinned vanilla zero-width divider is a separate defect and is not
an acceptance requirement. A later wider sibling, stale index, lost content,
changed fixed-page line or queue leakage on replay refutes the repair.

### Exclusive correction

For an unbounded horizontal region emit a zero-width provisional divider and retain its exact item index and original symmetric margin in a private region-local queue. Once that region has its finite content-derived width, resolve only those certified divider items to max(0, final_width - 2 * original_margin), updating both Shape.width and Line.dx. Never infer divider identity from paint, thickness or a zero-length generic line. Fixed-width dividers keep the existing formula and spacing. A divider alone has no intrinsic horizontal contribution. Test wider later content, multiple dividers, explicit page breaks, isolated subframes, rollback and fixed-width preservation. Resolve before any grouping invalidates retained indices.

Internal correctness correction under ADR-0127 continuous flow: L0 first,
restricted reseal, functional RED then GREEN, build and unfiltered lineage.
Keep public contracts, modes/defaults, compatibility and pipeline phases.
Acceptance requires finite content-derived page/SVG/PDF geometry, a nonzero
divider when body width is positive, and complete text before/after the rule.
Do not copy vanilla zero width or force a paper size. This does not certify
other relative-size shapes, asymmetric margins or general auto-page parity.
