# L0 — page running
Hash do Código: 7f2bbac6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-running.toml sha256:88345b9a32ff8678a2ff236d21cbd7c18913820ed151fc704c116ab2542288ed

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_running.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_running.rs:15`, `01_core/src/compiler/layout/page_running.rs:19`, `01_core/src/compiler/layout/page_running.rs:40`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own marginal eligibility, positions, offsets and isolated layout of explicit/realized running Content. Automatic numbering is enabled only for PageMarginal::Auto; None and explicit Content suppress automatic text in that marginal. Resolve offsets as relative margin plus absolute/em length and preserve the current typed top/bottom and left/center/right placement.

Lay out explicit header/footer Content in the corresponding finite marginal rectangle, recompose at the left margin and its proper y, and retain rich frame children. For a realized numbering Content, empty means no layer; the current helper estimates width from plain_text using active metrics, then lays out the rich Content in an isolated subframe and places the resulting group at the calculated position. Do not claim rich intrinsic-width parity or replace the width projection here. Shared defaults/types belong to the existing entity/nucleus owners; no callback is executed and no main-flow cursor policy is added.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
