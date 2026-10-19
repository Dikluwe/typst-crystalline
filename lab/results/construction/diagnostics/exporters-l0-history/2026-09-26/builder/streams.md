# Prompt L0 — `infra/export/builder/streams`
Hash do Código: 05687117

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/builder/streams.rs`
**Criado em**: 2026-09-26 (P1506)
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medicao antes da decisao

Na working tree de origem sobre `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`,
`03_infra/src/export/builder.rs:794-832`, `:990`, `:1015`
envolviam os operadores ja produzidos pelo modulo de streams.

## Responsabilidade

Este modulo possui somente o envelope fisico dos content streams de pagina:
compressao rentavel em modo Compact, bytes legiveis em Pretty e aplicacao
opt-in da transformacao oracle. Nao gera operadores graficos; isso permanece
em `export/stream.rs`.

## Criterios de verificacao

- Compact preserva a decisao FlateDecode rentavel de P884.
- Pretty preserva stream nao comprimido de P1360.
- Oracle e identidade quando desligado.
- A extracao nao altera bytes ou assinaturas publicas.
