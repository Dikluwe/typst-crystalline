# Prompt: GlyphVariants — Variantes de Tamanho de Glifos Matemáticos

## Módulo

`01_core/src/entities/glyph_variants.rs`

## Contexto

Fontes OpenType com tabela MATH definem variantes de tamanho para glifos
extensíveis como delimitadores (`(`, `)`, `[`, `]`, `{`, `}`) e o símbolo
radical (`√`). O `MathLayouter` usa estas variantes para seleccionar o glifo
com a altura mínima necessária para cobrir o conteúdo que envolve.

Este módulo define os tipos de domínio `GlyphVariant` e `GlyphVariants` em L1.
L3 preenche as variantes a partir de `ttf-parser::tables::math::Variants`.

## Tipos exportados

```rust
pub struct GlyphVariant {
    pub glyph_id: u16,   // ID do glifo alternativo na fonte
    pub advance: f64,    // medida de avanço em design units
}

pub struct GlyphVariants {
    pub variants: Vec<GlyphVariant>,  // ordenadas por tamanho crescente
}
```

## Comportamento

- `select(min_advance)`: retorna o glyph_id da primeira variante com
  advance >= min_advance; None se nenhuma for suficiente
- `is_empty()`: true se não há variantes (fonte sem tabela MATH)
- `Default`: variantes vazias (fallback para glifo base)
- Zero I/O de sistema — tipo de domínio puro

## Critérios de verificação

- `select(600.0)` com variantes [500, 800, 1200] → Some(glyph_id com advance=800)
- `select(500.0)` com variante exacta de 500 → Some(glyph_id)
- `select(1000.0)` com variante máxima de 500 → None
- `select(...)` em GlyphVariants vazio → None
- `is_empty()` em GlyphVariants::default() → true
