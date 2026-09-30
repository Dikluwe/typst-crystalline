# Prompt L0 — `infra/export/builder/serialization`
Hash do Código: a14b646b

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/builder/serialization.rs`
**Criado em**: 2026-09-26 (P1506)
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medicao antes da decisao

Na working tree de origem sobre `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`,
`03_infra/src/export/builder.rs:1044-1050`, `:4198-4265`,
`:4416-4478`, `:4536-4556` armazenavam objetos e serializavam PDF.

## Responsabilidade

Este modulo armazena objetos no builder e serializa header, objetos, xref e
trailer. Possui apenas escapes e insercoes de dicionario ligados a essa
representacao fisica. A versao do header vem do standard ja resolvido; este
modulo nao escolhe standards nem defaults.

## Criterios de verificacao

- Offsets xref apontam para os objetos emitidos e `/Size` permanece correto.
- `/Root`, `/Info`, `/ID` e metadata preservam as referencias existentes.
- A extracao nao promete igualdade com uma mecanica externa; deve preservar
  os observaveis PDF e, nesta refatoracao, os bytes atuais.
