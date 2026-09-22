# Prompt L0 — `infra/export/tests/p1478_pdfa4f_empty_embedded_files_tests`
Hash do Código: fdfc364b

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests/p1478_pdfa4f_empty_embedded_files_tests.rs`
**Criado em**: 2026-09-22 (P1478, então dentro de `builder.rs`)
**Movido em**: 2026-09-22 (P1479)
**ADRs**: ADR-0037 Regra 5 (extraído "por cluster", ADR-0100 §Positivas), ADR-0129 (1:1)

---

## Contexto

6 testes do contrato P1478 (A4f sem anexo requer `/EmbeddedFiles`
vazio — retificação de P1477). Ver `00_nucleo/materialization/
typst-passo-1478.md` e `infra/export/builder.md` §P1478.

## P1479 — mudança de localização (fecha a exceção temporária de P1478)

P1478 colocou este módulo dentro de `builder.rs` (não em
`export/tests.rs`) **só** para não agravar o achado `[PARSE]` do
`crystalline-lint`, que P1479 resolve (ver `infra/export/tests/mod.md`
§P1479 e `infra/export/tests/p1477_pdfa_icc_version_and_cidset_tests.md`
§P1479 para a causa-raiz real, não relacionada a este módulo). Com a
causa-raiz corrigida, a exceção deixa de ter motivo — o módulo volta
ao lugar arquitetural natural, ao lado dos irmãos `p1475`/`p1476`/
`p1477`.

### Ajuste de imports (mecânico)

`super::super::export_pdf_with_document_id_and_tags_and_serialization_
and_pages_and_standard`/`super::super::PdfTags` mantidos sem alteração
— mesma profundidade de aninhamento antes e depois (`export/mod.rs` →
`builder.rs` → `mod{}` tinha 2 níveis; `export/mod.rs` → `tests/mod.rs`
→ `tests/p1478_....rs` também tem 2 níveis). Três imports explícitos
acrescentados (`PdfStandards`, `AttachedFileRelationship`,
`PagedDocument`) — antes vinham de bare-imports do topo de `builder.rs`
(`use typst_core::entities::document_format_options::{PageRanges,
PdfArchival, PdfStandards}` etc.), não disponíveis a partir de
`tests/mod.rs`, que não os importa bare. Conteúdo dos 6 testes
bit-exact (ADR-0037 Regra 7) — só os imports e a indentação mudaram.

## Restrições estruturais

- `use super::*` traz a API de `tests/mod.rs`.
- `super::super::` alcança `export/mod.rs` (dois níveis de
  aninhamento).

## Interface

Nenhuma — `#[cfg(test)]` com `#[test]` functions.

## Critérios de verificação

`cargo test -p typst-infra --lib p1478_pdfa4f_empty_embedded_files_tests`
→ 6/6.

## Não-objectivos

Ver `infra/export/tests/mod.md`.
