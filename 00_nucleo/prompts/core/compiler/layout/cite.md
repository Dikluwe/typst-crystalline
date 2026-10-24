# L0 — cite
Hash do Código: 452fd034

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/cite.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/cite.rs:25`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Use cached CSL Content for the requested citation form/key when available, then render any supplement. Otherwise retain the local style/form fallbacks: numeric citation-order number, bibliography-order fallback or key; prose/author/year use the stored entry fields where available. Preserve consecutive numeric Normal `ibid.` and previously cited non-consecutive `op. cit.` handling and the causal last/previously-cited-key updates. Render supplements on every applicable branch. This unit does not own bibliography ordering or CSL cache construction.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Source-planned citation supplement

### Evidence before decision

At the approved physical-page implementation baseline, cite.rs:43, :75, :101
and :159 render the actual rich supplement with ordinary layout_content; the
CSL cache and local fallback text use that same generic call but are generated
content, not source child packets. The exact source and L0 bytes before this
amendment are retained in
lab/results/set-page-header-implementation-2026-10-21/agents/realization/citation-routing/before.json.
The approved five-facade package and physical-page Nucleus require the actual
selected source child to retain its typed plan edge without assigning source
identity to generated citation text. This is source-routing mechanics, not a new
citation syntax, numbering rule, callback or target.

### Exclusive routing contract

In every existing cached, ibid., op. cit. and local fallback branch, render the
actual supplement through layout_planned_child(CitationSupplement, supplement).
Keep the current style chain and citation-state update order. Cached CSL Content
and generated fallback/prefix strings remain unplanned. An ordinary Cite root
uses its own source edge; an actual bibliography-selected raw Ref uses the
conditional CitationSupplement edge under that Ref packet. A merely prospective
bibliography candidate does not authorize traversal or query membership.

Legacy rendering with no planned cursor preserves ordinary behavior through the
private adapter. Preserve all forms, style and empty-supplement precedence,
public signatures, errors, geometry and language morphology. Require actual
bibliography/non-bibliography controls and causal source supplement membership
with no generic Ref-field traversal or layout callback.
