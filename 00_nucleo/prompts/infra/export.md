# Prompt L0 — export_pdf

## Módulo
`03_infra/src/export.rs`

## Propósito
Serializa um `PagedDocument` para bytes PDF-1.7 válidos. Sem crates
externas de PDF — geração manual de objectos, xref table e trailer.

## Diagnóstico do original
O `typst-pdf` original usa `krilla` + `krilla-svg` (~7559 linhas).
O cristalino diverge: stub manual suficiente para validar o pipeline.
Passo 21 pode adicionar embedding de fontes reais.

## Geometria: inversão do eixo Y
PDF usa y=0 no canto inferior esquerdo (y cresce para cima).
O cristalino usa y=0 no topo (y cresce para baixo).
Transformação: `y_pdf = page_height - y_cristalino`

## Escaping de texto PDF
- `(` → `\(`
- `)` → `\)`
- `\` → `\\`
- Caracteres não-ASCII (> 127) → `?` (placeholder; UTF-16BE no Passo 21)

## Estrutura do PDF gerado
```
%PDF-1.7
%<bytes binários>
1 0 obj  << /Type /Catalog /Pages 2 0 R >>
2 0 obj  << /Type /Pages /Kids [...] /Count n >>
3..3+n   page dictionaries
3+n..    content streams (BT...ET por FrameItem::Text)
font_id  /Helvetica Type1
xref table
trailer
%%EOF
```

## Interface pública
```rust
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8>;
```

## Critérios de verificação
- Começa com `%PDF-1.7`
- Termina com `%%EOF`
- Contém `xref`, `trailer`, `startxref`, `/Catalog`, `/Pages`, `Helvetica`
- Texto ASCII aparece no PDF (sem escaping que o esconda)
- `(`, `)`, `\` escapados; caracteres não-ASCII → `?`
- Inversão Y: y_cristalino=84 → y_pdf=758 (para página A4 842pt)
- Documento vazio → PDF válido com zero páginas
- MediaBox contém 595 e 842
