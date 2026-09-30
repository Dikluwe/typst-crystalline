# L0 — block
Hash do Código: cf9f4e77

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/block.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/block.rs:21`, `01_core/src/compiler/layout/block.rs:32`, `01_core/src/compiler/layout/block.rs:43`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve optional relative width/height once against the original outer available width/height and active font, accepting only finite nonnegative results. Height percentages refer to the outer region, not remaining cursor space. Fractional height consumes only a private allocation supplied by `00_nucleo/prompts/compiler/layout/sequence.md`; absence is not zero and does not permit an intrinsic measurement fallback to masquerade as a resolved fraction.

Close prior pending text as currently implemented, resolve above/spacing/below against the effective font and collapse against the existing block/paragraph boundary protocol. Default spacing comes from `00_nucleo/prompts/compiler/layout/vanilla_defaults.md`, not an absolute fixture value. For an unbreakable block, request ordinary progress only when its measured complete size can fit a region but not the remaining one.

Install the body's constrained dimensions/origin and parent bbox temporarily, preserving RTL positioning, inset/outset, radius, clip, fill, stroke and overhang. Dispatch/close body content, restore the outer dimensions/bbox/line start, honor minimum resolved height and emit the existing backing shape in paint order. Preserve the geometric-container versus ordinary-body outgoing-margin paths, including the below-minus-leading adjustment and pending margin state. Block owns its own frame end/gap; Cursor must not compensate it through generic tail translation.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
