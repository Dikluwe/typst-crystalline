# Prompt L0 — `infra/export/builder/navigation`
Hash do Código: 14109b43

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/builder/navigation.rs`
**Criado em**: 2026-09-26 (P1506)
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medicao antes da decisao

Na working tree de origem sobre `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`,
`03_infra/src/export/builder.rs:165-210`, `:2597-3113`, `:4143`,
`:4351-4391`, `:4411`, `:4480-4534`, `:4558` possuiam selecao,
annotations, destinos, anexos e rotulos.

## Responsabilidade

Este modulo possui a conversao de indices de pagina selecionada e a emissao
de links, named destinations, attachments, outline e page labels. Helpers de
URI, destination names, labels e coleta de links pertencem a esta unidade.

A iteracao fisica crescente continua sendo a unica origem de ordenacao e
deduplicacao da selecao. Destinos para paginas excluidas continuam omitidos
ou remapeados conforme o contrato P1468 vigente.

## Criterios de verificacao

- Selecao de paginas, links, outline e labels preservam os resultados atuais.
- A extracao nao muda indices, IDs, ordem, API publica ou defaults.
- Attachments preservam relacionamento e associacao PDF/A existentes.
