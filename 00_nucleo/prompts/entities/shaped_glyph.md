---
prompt: entities/shaped_glyph
layer: L1
created: 2026-06-27
passo: P482
adr: ADR-0120
---

# Prompt L0 — `ShapedGlyph` (P482)

## Propósito

Struct que representa um glifo com shaping real produzido por rustybuzz.
Definido em `01_core/src/entities/layout_types.rs`.

## Estrutura

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    pub glyph_id:  u16,   // ID do glifo na fonte (índice na tabela de glifos)
    pub x_advance: i32,   // Advance horizontal em unidades de fonte
    pub x_offset:  i32,   // Offset horizontal (kerning, marks)
    pub y_offset:  i32,   // Offset vertical (diacríticos)
    pub cluster:   u32,   // Índice byte no string original (ToUnicode CMap)
    pub char_code: char,  // Codepoint Unicode derivado de cluster
}
```

## Divergências vs ADR-0120

- ADR-0120 propõe `glyph_id: u32` (rustybuzz nativo).
- Implementação usa `u16` (truncado com `as u16`) para compatibilidade
  com `FrameItem::Glyph.glyph_id` e o path CIDFont do export.
- `x_advance`, `x_offset`, `y_offset` são `i32` (unidades de fonte,
  não pontos tipográficos). Conversão: `pt = units / upm * font_size`.
- `cluster` é o índice byte UTF-8 no string original — derivado de
  `GlyphInfo.cluster` do rustybuzz.
- `char_code` é derivado de `cluster` via `byte_idx_to_char`.

## Uso

Armazenado em `FrameItem::TextShaped.glyphs: Vec<ShapedGlyph>`.
Consumido por:
- `export/stream.rs::emit_shaped_pdf` — para emitir hex string de glyph IDs.
- `export/fonts.rs::collect_codepoints` — para extrair chars usados.
- `export/fonts.rs::collect_glyph_ids` — para extrair IDs usados.
