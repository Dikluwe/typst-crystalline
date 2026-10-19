# L0 — stream
Hash do Código: f2c4ec93

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream.rs:54` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own FontScenario, PageContext, borrowed resource/font maps, constructors and internal
compatibility reexports. Preserve Type1/CIDFont/Multifont static variants, existing
visibility and supplied graphic mode. Raw context constructors initialize tags to
Disabled and UA1/oracle to false; product defaults belong to the export façade,
not these constructors. Preserve independent tagging/oracle settings. Descendants
receive this context; font/image/gradient resources are prepared outside this owner.

Own the deterministic page-local MCID prepass in with_pdf_tags. Restart at zero per
page and traverse `page.items` in the current content order. This prepass does not
traverse the page background or foreground layers. Formula, Label and TH/TD own
their MCIDs outside Artifact. Artifact suppresses descendants; UA1 boundary carriers
consume nothing, semantic owners stop further UA1 leaf collection, and real Text,
TextShaped, Glyph and Image leaves receive UA1 MCIDs outside those owners. Group/Link
recurse. Disabled tagging does not populate marked-content assignments. Expose the
same assignments to drawing and builder structure construction, not parallel rules.

Keep the existing inline regression suite owned by this consumer. Page assembly,
recursive painting, text operators and shape primitives belong only to the children
below; this façade does not own their algorithms or PDF stream compression.

## Delegated owners

- `00_nucleo/prompts/infra/export/stream/draw.md` owns `03_infra/src/export/stream/draw.rs`.
- `00_nucleo/prompts/infra/export/stream/page.md` owns `03_infra/src/export/stream/page.rs`.
- `00_nucleo/prompts/infra/export/stream/shape.md` owns `03_infra/src/export/stream/shape.rs`.
- `00_nucleo/prompts/infra/export/stream/text.md` owns `03_infra/src/export/stream/text.rs`.
