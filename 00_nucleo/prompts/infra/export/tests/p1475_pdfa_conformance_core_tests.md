# Prompt L0 — `infra/export/tests/p1475_pdfa_conformance_core_tests`
Hash do Código: 9145354e

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests/p1475_pdfa_conformance_core_tests.rs`
**Criado em**: 2026-09-21 (P1475, então dentro de `tests.rs`)
**Extraído em**: 2026-09-22 (P1479)
**ADRs**: ADR-0037 Regra 5 (extraído "por cluster", ADR-0100 §Positivas), ADR-0129 (1:1)

---

## Contexto

11 testes do contrato P1475 (Núcleo PDF/A-2/3/4: trailer `/ID`
universal, `pdf:PDFVersion` dinâmico, XMP `pdfaid`, `/OutputIntent` e
supressão de `/Info` em A4). Ver `00_nucleo/materialization/
typst-passo-1475.md` e `infra/export/builder.md` §P1475 (contrato
completo). Testa exclusivamente através da fronteira pública já
existente do builder — nenhum símbolo interno é referenciado
diretamente.

## P1479 — extração de `tests.rs` (mecânica, sem mudança de comportamento)

Movido verbatim de dentro do antigo `03_infra/src/export/tests.rs`
(módulo `mod p1475_pdfa_conformance_core_tests { ... }`) para ficheiro
próprio, per `typst-passo-1479.md` (achado `[PARSE]` do
`crystalline-lint`, ver `infra/export/tests/mod.md` §P1479 para a
medição). Conteúdo dos 11 testes **bit-exact** (ADR-0037 Regra 7) —
só o wrapper `mod { }` foi removido (convenção ficheiro-como-módulo) e
a indentação de um nível ajustada. Declarado em `tests/mod.rs` via
`#[cfg(test)] mod p1475_pdfa_conformance_core_tests;`.

## Restrições estruturais

- `use super::*` traz a API de `tests/mod.rs` (que por sua vez traz a
  de `export/mod.rs`).
- Testa via `export_pdf_with_document_id_and_tags_and_serialization_
  and_pages_and_standard` — nunca constrói `PdfBuilder` diretamente.

## Interface

Nenhuma — `#[cfg(test)]` com `#[test]` functions.

## Critérios de verificação

`cargo test -p typst-infra --lib p1475_pdfa_conformance_core_tests` →
11/11. Regressão de P1476/P1477 (extensões intencionais do escopo de
A1a/A1b/A4f/A4e, documentadas em `infra/export/builder.md` §P1476/
§P1477) já refletida nos três testes ajustados (`p1475_pdfaid_absent_
for_scope_out_profiles_confirmed_negatively`, `p1475_output_intent_
absent_for_none_a1_ua1_alone_regression_guard`, `p1475_info_present_
for_all_other_cases_regression_guard`).

## Não-objectivos

Ver `infra/export/tests/mod.md`.
