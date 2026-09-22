# Prompt L0 — `infra/export/tests/p1476_pdfa_attachments_af_tests`
Hash do Código: 943cae9e

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs`
**Criado em**: 2026-09-21 (P1476, então dentro de `tests.rs`)
**Extraído em**: 2026-09-22 (P1479)
**ADRs**: ADR-0037 Regra 5 (extraído "por cluster", ADR-0100 §Positivas), ADR-0129 (1:1)

---

## Contexto

9 testes do contrato P1476 (`/AFRelationship`+`/AF` gated a
A3a/A3b/A3u/A4f/A4e; extensão de `/Info`/`/OutputIntent`/`pdfaid` a
A4f/A4e). Ver `00_nucleo/materialization/typst-passo-1476.md` e
`infra/export/builder.md` §P1476. Constrói `Content::pdf_attach(
PdfAttachElem {...})` diretamente (API pública desde P1286) —
equivalente a compilar `#pdf.attach(...)` para efeitos destes testes.

## P1479 — extração de `tests.rs` (mecânica, sem mudança de comportamento)

Movido verbatim de dentro do antigo `03_infra/src/export/tests.rs`
para ficheiro próprio, per `typst-passo-1479.md` (ver
`infra/export/tests/mod.md` §P1479 para a medição do achado
`[PARSE]`). Conteúdo dos 9 testes bit-exact (ADR-0037 Regra 7).
Declarado em `tests/mod.rs` via `#[cfg(test)] mod p1476_pdfa_
attachments_af_tests;`.

## Restrições estruturais

- `use super::*` traz a API de `tests/mod.rs`.
- Testa via `export_pdf_with_document_id_and_tags_and_serialization_
  and_pages_and_standard` — nunca constrói `PdfBuilder` diretamente.

## Interface

Nenhuma — `#[cfg(test)]` com `#[test]` functions.

## Critérios de verificação

`cargo test -p typst-infra --lib p1476_pdfa_attachments_af_tests` →
9/9.

## Não-objectivos

Ver `infra/export/tests/mod.md`.
