# Prompt L0 — `infra/export/mod` — API pública do exporter PDF
Hash do Código: 3fd31db5

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/mod.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0027 (CIDFont), ADR-0033 (paridade observable), ADR-0055 (multifont), ADR-0098 (SSoT), ADR-0100 (coesão L3)

---

## Contexto

Sucessor de `00_nucleo/prompts/infra/export.md` (umbrella pré-P307).
P307b decompôs `export.rs` em `export/` subdirectório com 6 submódulos.
Este L0 cobre apenas o `mod.rs` — entry point com API pública +
dispatch para `PdfBuilder`.

## Restrições estruturais

- Camada L3. Não L1: pode usar I/O em sub-helpers (via flate2, ttf-parser).
- API pública mantém 3 entry points: `export_pdf`, `export_pdf_with_font`, `export_pdf_multifont`.
- Dispatch para `PdfBuilder` em `builder.rs`. mod.rs **não** contém lógica de emit.
- Reexporta `PdfImagePayload` e `process_png_for_pdf` (consumidos por outras crates do workspace via `crate::export::*`).

## Interface

```rust
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8>;
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8>;
pub fn export_pdf_multifont(doc: &PagedDocument, fonts: &[(FontList, Vec<u8>)]) -> Vec<u8>;
pub use self::images::{PdfImagePayload, process_png_for_pdf};
```

## Submódulos declarados

```text
mod builder;     // PdfBuilder + build_helvetica/cidfont/multifont
mod fonts;       // CIDFont helpers + escape_pdf_string
mod gradients;   // gradient cluster (sub-decomposto P307b.2)
mod images;      // JPEG/PNG/XObject
mod stream;      // PageContext + emit
```

## Não-objectivos

- Não conhece formatadores user-facing (vive em L2).
- Não conhece world/filesystem (vive em pipeline.rs).
- Não decide dispatch baseado em conteúdo do doc — caller decide via fonts.

## Critérios de verificação

`compile_to_pdf_bytes(world, source)` em `pipeline.rs` despacha para:
- `export_pdf(doc)` se `fonts.len() == 0`
- `export_pdf_with_font(doc, bytes)` se `fonts.len() == 1`
- `export_pdf_multifont(doc, &fonts)` se `fonts.len() >= 2`
