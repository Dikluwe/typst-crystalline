# L0 — infra/export/p1405_pdf_label_tests
Hash do Código: 0df506a8

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/tests/p1405_pdf_label.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/tests/p1405_pdf_label.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Focused tagged-label integration harness

Own the existing integration test consumer only. A minimal SemanticKind::Label
document must resolve a structural Lbl node and "/Lbl << /MCID 0 >> BDC" in
the Verbose stream. The fixture starts page-local MCIDs at zero.
Do not fix complete PDF bytes, offsets, global object IDs or general list structure.

The stream context/prepass is owned by
`00_nucleo/prompts/infra/export/stream.md`, the marked envelope by
`00_nucleo/prompts/infra/export/stream/draw.md`, and the structure node by
`00_nucleo/prompts/infra/export/builder/metadata.md`.
This harness authorizes no additional productive behavior.

## Verification

Keep the focal role/MCID assertion through the public export path; relationships
and nonvisual meaning are observable, not an upstream serialization algorithm.
