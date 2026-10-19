# Prompt L0 — `infra/export/builder/metadata`
Hash do Código: 7a93975f

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/builder/metadata.rs`
**Criado em**: 2026-09-26 (P1506)
**ADRs**: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medicao antes da decisao

Na working tree de origem sobre `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`,
`03_infra/src/export/builder.rs:467-578`, `:3115-4141`, `:4186`,
`:4267-4349`, `:4393` possuiam geracao de metadados e estrutura.

## Responsabilidade

Este modulo emite `/Info`, output intent PDF/A, XMP, structure tree/tagging e
viewer preferences PDF/UA. Tambem possui os helpers de IDs XMP, timestamp e
escape XML usados exclusivamente por essa familia.

Preserva standards, acessibilidade, archival, attachments associados e IDs
document/instance definidos em `00_nucleo/prompts/infra/export/builder.md`;
nao decide o header PDF nem o
xref.

## Criterios de verificacao

- Suites PDF/A e PDF/UA permanecem verdes.
- XMP e trailer reutilizam o mesmo par de IDs.
- A extracao e content-preserving e nao amplia visibilidade publica.
