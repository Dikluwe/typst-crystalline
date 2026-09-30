# L0 — table
Hash do Código: a851d61c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/table.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/table.rs:27`, `01_core/src/compiler/layout/table.rs:127`, `01_core/src/compiler/layout/table.rs:155`, `01_core/src/compiler/layout/table.rs:174`, `01_core/src/compiler/layout/table.rs:229`, `01_core/src/compiler/layout/table.rs:243`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render an optional caption above the table, adding a prefix only when table.numbering is a String. Read the causal Table counter from the injected introspector with current fallback one, register its page for LoT, format or use decimal fallback, and retain the existing English `Table number: ` extension prefix. Caption remains outside the table carrier. This direct table-numbering extension is not a Figure and does not acquire figure selectors/show behavior by documentation.

Project visual children/header/footer and delegate all tracks, lines, align/inset, stroke/fill and visual row-group/pagination behavior to `00_nucleo/prompts/compiler/layout/grid.md`. Build the current parallel, visually empty Table semantic carriers after that layout, preserving summary and explicit cell kind/level/scope/spans in their current representation. Header Auto cells receive the current level-one Column classification; explicit Data remains Data. Uniform automatic headers produce grouped head/body carriers, mixed explicit Data retains direct rows. Place header logical rows only on the first affected page; retain the source's current sequential logical-row projection and even partition by affected pages.

The source does not construct logical footer rows, derive arbitrary header levels from TableHeader or align logical partition to measured visual breaks. Full Headers applicability relations and PDF/UA/AT/reflow proof remain unresolved, not promises of this sanitation. The older enclosing-frame proposal must not hide the actual parallel-empty-carrier behavior. Exporters own MCIDs and relation serialization; this owner creates no new layout pass.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.
