# Prompt L0 — `infra/export/tests/p1480_pdfua_identification_tests`
Hash do Código: 41caaefe

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests/p1480_pdfua_identification_tests.rs`
**Criado em**: 2026-09-23 (P1480)
**ADRs**: ADR-0107 (paridade de observáveis, não bytes), ADR-0108 (medição antes da decisão), ADR-0129 (owner 1:1)

---

## Contexto

Testes unitários da fatia P1480 do exporter PDF/UA-1. O contrato
produtivo vive em `infra/export/builder.md` §P1480; a medição e a
proveniência bilateral vivem em `00_nucleo/materialization/
typst-passo-1480.md` §§0–1.

Este módulo prova somente os dois observáveis de identificação e
apresentação emitidos por `builder.rs`: XMP `pdfuaid` e catálogo com
`/ViewerPreferences /DisplayDocTitle true`. A validação de título
obrigatório pertence a `infra/pipeline.md` §P1480 e é testada no próprio
consumer `03_infra/src/pipeline.rs`.

## Medição anterior à decisão

Na fixture pinada do passo, `veraPDF 1.30.2 -f ua1` mediu no cristalino
seis regras falhadas; duas são causadas pela ausência de `pdfuaid` (ISO
14289-1 §5 teste 1) e de `DisplayDocTitle` (ISO 14289-1 §7.1 teste 10).
O vanilla ratificado emite `xmlns:pdfuaid="http://www.aiim.org/pdfua/ns/id/"`,
`<pdfuaid:part>1</pdfuaid:part>` e `/DisplayDocTitle true`. As quatro
falhas restantes pertencem a P1481.

Medição combinada pós-primeiro GREEN: A2u+UA-1 cristalino falhou duas
regras PDF/A-2u (144/2) porque `pdfuaid:part` não estava declarado num
schema de extensão XMP; o vanilla passou 146/0. A declaração mínima do
schema PDF/UA é portanto requisito de não-regressão desta fatia, não
expansão cosmética da tabela XMP.

Classificação ADR-0107: presença e valor desses campos são observáveis do
perfil PDF/UA; IDs, whitespace, ordem global de objetos e bytes do PDF são
mecânica e não servem de oracle. A posição de `pdfuaid:part` antes de
`xmpTPg:NPages` é testada apenas como contrato local explícito do builder,
não como equivalência byte-a-byte com o vanilla.

## Contrato dos testes

1. `ua-1` isolado emite o namespace `pdfuaid`, exatamente um
   `<pdfuaid:part>1</pdfuaid:part>` e não inventa `pdfaid`.
2. `ua-1` isolado emite `/ViewerPreferences` com
   `/DisplayDocTitle true` no catálogo.
3. Sem `ua-1` — nenhum standard, `a-2u` e `a-4f` — não emite namespace
   ou elemento `pdfuaid`, nem `/ViewerPreferences` novo.
4. `a-2u+ua-1` conserva os campos `pdfaid` de A2u e acrescenta
   `pdfuaid`/`DisplayDocTitle`; um eixo não substitui o outro. A mesma
   combinação declara o schema de extensão `pdfuaid:part` com categoria,
   descrição, nome e tipo medidos; UA-1 isolado e PDF/A isolado não
   ganham essa extensão.
5. O elemento `pdfuaid:part` precede `xmpTPg:NPages`, conforme o contrato
   local medido de `infra/export/builder.md` §P1480.

Todos os casos atravessam a fronteira pública existente
`export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard`.
Não se cria API produtiva para facilitar testes.

## Aceitação

`cargo test -p typst-infra --lib p1480_pdfua_identification_tests`
termina com 5/5 testes aprovados. O controlo combinado prova coexistência
com PDF/A; os controlos negativos provam que PDFs não-UA permanecem sem os
dois observáveis novos.

## Não-objectivos

- Não declara conformidade PDF/UA-1 completa; P1481 ainda é dependência.
- Não testa `/Direction`.
- Não testa idioma, `Artifact`/conteúdo marcado nem idioma do outline.
- Não compara bytes, IDs de objetos ou whitespace com o vanilla.
- Não testa o erro de título ausente, que pertence a `infra/pipeline.md`.
