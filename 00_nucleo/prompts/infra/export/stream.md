# L0 — stream
Hash do Código: c378164d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

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

The façade reexports the private marked Glyph emission variants to the draw
owner for Large-operator occurrence identity. This is internal composition of
existing PDF stages: it does not choose text from GIDs, create a new mode or
alter the ordinary Glyph emission API.

## Delegated owners

- `00_nucleo/prompts/infra/export/stream/draw.md` owns `03_infra/src/export/stream/draw.rs`.
- `00_nucleo/prompts/infra/export/stream/page.md` owns `03_infra/src/export/stream/page.rs`.
- `00_nucleo/prompts/infra/export/stream/shape.md` owns `03_infra/src/export/stream/shape.rs`.
- `00_nucleo/prompts/infra/export/stream/text.md` owns `03_infra/src/export/stream/text.rs`.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/stream.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Declarar o módulo descendente separator; manter despacho estático e recursos do stream existentes.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

Em UA1, TextSeparator recebe o mesmo tratamento de folha textual Span e MCID coordenado, preservando enclausuramento e semântica do parágrafo.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Declarar módulo privado text_replacement. No mapa de tagging, descer nos filhos, sem novo MCID próprio do envelope.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
