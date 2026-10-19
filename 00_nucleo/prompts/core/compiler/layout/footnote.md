# L0 — footnote
Hash do Código: 506cd41f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/footnote.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/footnote.rs:31`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/footnote-composicao-2026-09-30/results.json` fixes historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, crystalline binary, HEAD and probe's uncommitted tree. In defer.typ, vanilla places `Alpha¹ omega.` and the note on the next page; crystalline leaves the call on the prior page and defers only body. `footnote.rs:47-51` emits call/queues body; `footnote_flush.rs:128-169` decides fitting too late to move the source line. Causality is inferred from these points and would be refuted by a path already moving the full line before flush.

The footnote.rs prototype was refuted by source→PDF defer.typ: the prepared paragraph has already committed the line and the local queue cannot move it.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Read the current causal footnote counter from the injected introspector, with the existing fallback of one. Format the supplied pattern or default `1`, falling back to the decimal value, and dispatch a superscript marker without square brackets. Queue the numbered body for deferred emission. Footnote-body drain and region progress remain in `00_nucleo/prompts/core/compiler/layout/cursor.md`; terminal coordination remains in `00_nucleo/prompts/core/compiler/layout/mod.md`.

The call-and-first-body admission proposal remains pending and is not current behavior. It requires future coordination with `00_nucleo/prompts/core/compiler/layout/line_commit.md` and `00_nucleo/prompts/core/compiler/layout/cursor.md`; this text does not authorize changing this consumer as though the correction were complete.

Proposed future behavior: before emitting a call, when the complete note fits a fresh region but not the current region together with the carrier line, move the entire active line to the next region. Preserve items, surrounding text, horizontal advance, styles and causal number; do not move only the superscript. Keep the note pending for ordinary flush in the call's region. Do not migrate without regional progress or if the note exceeds the complete useful height; oversized-note continuation requires separate materialization. This would be a private parity correction under ADR-0127 continuous flow, with no new public field/signature/default/phase/compatibility.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Future proposal acceptance, not certification of completion: defer places call/body on the same page as vanilla; fit remains one page; carrier-line text stays ordered; a note exceeding the entire region does not loop through pages. Prove through text/pages/morphology, never PDF bytes.
