# L0 — html frame
Hash do Código: e118dd21

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/html_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/html_frame.rs:22`, `01_core/src/compiler/layout/html_frame.rs:25`, `01_core/src/compiler/layout/html_frame.rs:30`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

In ordinary paginated dispatch, delegate once to the body without a visual wrapper or default page margin. Intrinsic entrypoints create an isolated Layouter with supplied chain/metrics/sizer, unbounded region and transparent Page cropped to supported visual dimensions, logical_number one, without joining the global paginated sequence. Nesting remains transparent inside the same island.

The detailed variant captures ContextBlock ids using private zero-size carriers in the existing FrameItem tree, not a public variant or HtmlElem sentinel. Transport carriers through box/place/grid/link/group/transform geometry; after all composition, extract deterministic id/HtmlPosition pairs in the island's causal reference frame before private vertical visual cropping. Then normalize only the visual Page, remove all carriers, and derive final visual dimensions without them. Missing id/Location correspondence does not fabricate a position.

Own the shared marker identity/material predicates: only a direct private marker is non-material. Existing Semantic, Link and Group wrappers retain ordinary legacy material classification even if they contain only a carrier; do not flatten them. Extent helpers ignore a direct marker but transformations still reach its point. No SVG encoder, World, PageStore or logical PDF-label behavior is introduced. The hub owns public reexports, this unit owns intrinsic implementation.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/html_frame.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator é neutro para items_have_material e visible_top; nunca o confundir com marcador de contexto removível.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

O refutador P1493 de Page visível compara a projeção sem folhas TextSeparator, preservando dimensões, matriz/pivô e posição do marker; verifica separadamente a retenção do carrier textual. Estrutura do Vec não é a geometria visível.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Conservar TextReplacement na remoção de marcadores contextuais, descendo nos filhos; materialidade/posição consultam somente os filhos visuais.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Linhas causais vazias em página auto — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/before/results.json` mede fontes→PDF com binários pinados e entradas sem newline artificial; `initial.json` guarda HEAD, working tree, hashes e snapshots. `#text("")` e a variante 18pt têm Tekt 3×3pt versus vanilla 3×8.568/12.852pt. `abc\n\n` termina em Tekt 19.872×31.536pt versus vanilla 19.872×8.568pt, enquanto `abc\n\nxyz` coincide a 19.872×31.536pt. `#text("  ")\n\nabc` fica numa linha Tekt (26.112×8.568pt), mas em duas linhas vanilla (19.872×31.536pt). Fonte: `text.rs:37-86` não emite item para Text vazio; `html_frame.rs:72` exclui todo TextSeparator de materialidade; `line_commit.rs:280-286`, `parbreak.rs:43-48` e `mod.rs:1965` usam essa materialidade para linha; `parbreak.rs:40-85` avança cursor após fechar linha e `page_geometry.rs:180-201` usa cursor se não há fundo confirmado. Vanilla `inline/linebreak.rs:695-699` oferece breakpoint obrigatório até para texto vazio. Isto mede o observável; não infere a intenção histórica nem exige a mecânica do vanilla.

### Contrato interno segregado

Manter `items_have_material` como predicado de tinta/material visual, com TextSeparator neutro. Acrescentar predicado privado distinto para existência de linha causal: material visual ou TextSeparator Literal; Markup/Newline inicial e marcadores contextuais não criam linha.

Aceitação: testes RED→GREEN e matriz bilateral, regressões anteriores, build/lint sem erros; resselo somente dos owners deste manifesto.
