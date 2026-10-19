# Prompt L0 — `infra/export/builder/fonts`
Hash do Código: 74702211

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/builder/fonts.rs`
**Criado em**: 2026-09-26 (P1506)
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medicao antes da decisao

Na working tree de origem sobre `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`,
`03_infra/src/export/builder.rs:419`, `:580-792`, `:834-894`,
`:1031`, `:1262-2237` continham esta familia. Extracao interna,
sem mudanca de contrato ou comportamento (ADR-0127).

`builder.rs` concentrava os caminhos CIDFont e multifont, subsetting,
instanciacao variavel, descritores, streams de fonte, CIDSet, ToUnicode e
larguras. Essa familia domina o arquivo e tem fronteira propria: transforma
fontes e uso tipografico em objetos PDF, sem possuir xref/trailer, metadata ou
navegacao.

## Responsabilidade

Este modulo possui a preparacao e emissao de fontes PDF e os caminhos de
build CIDFont/multifont. Deve preservar mapeamentos GID, `/W`, ToUnicode,
subsetting, instanciacao variavel, bitmap-only, descritores e acumulacao de
`subset_ms` exatamente como especificados em
`00_nucleo/prompts/infra/export/builder.md`.

Nao altera a API publica, nao escolhe defaults do produto e nao serializa o
arquivo final. A comunicacao com o orquestrador ocorre por `PdfBuilder` e
helpers internos estaticos.

## Criterios de verificacao

- PDFs single-font e multifont permanecem semanticamente equivalentes.
- Fontes TrueType, CFF1, CFF2, variaveis e bitmap preservam os seus caminhos.
- O modulo e owner unico deste ficheiro e nao e apontado por outro consumer.
