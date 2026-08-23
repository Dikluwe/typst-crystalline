# Prompt: GlyphVariants — Variantes de Tamanho de Glifos Matemáticos
Hash do Código: b68083b2

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
    pub glyph_id: u16,     // ID do glifo alternativo na fonte
    pub advance: f64,      // medida de avanço no eixo de esticamento (design units) —
                            // SÓ para comparar com o alvo (`select_with_advance`), nunca
                            // para posicionar o glifo (ver `hor_advance` abaixo)
    pub hor_advance: f64,  // avanço horizontal NATIVO do glifo (hmtx, design units) —
                            // usado para `FrameItem::Glyph.x_advance`/largura de caixa
}

pub struct GlyphVariants {
    pub variants: Vec<GlyphVariant>,  // ordenadas por tamanho crescente
}
```

## P917 — `advance` vs `hor_advance`: dois eixos, dois campos

`advance` é a medida ao longo do eixo de esticamento (`AdvanceMeasurement` da tabela MATH:
altura para construções verticais, largura para horizontais) — usada exclusivamente para
decidir SE uma variante é grande o suficiente (`select`/`select_with_advance`). `hor_advance`
é o avanço horizontal nativo do glifo (`hmtx`, via `face.glyph_hor_advance(glyph_id)`,
independente do eixo de esticamento) — é o que o vanilla usa (`font.x_advance(glyph_id)`,
`glyph.rs:293`) para posicionar o glifo e para a largura da caixa que o contém. Conflar os
dois (usar `advance` também como avanço horizontal) foi o achado de P917: para variantes
verticais, `advance` é uma medida de ALTURA, não de largura — usá-la como `x_advance`
reserva espaço horizontal do tamanho da altura do glifo, não da sua largura real.
Consumidores (`stretchy.rs`, `assembly.rs`) devem ler `advance` só para `select`/limiar de
tamanho, e `hor_advance` para tudo que posiciona ou mede largura.

`GlyphPart` (montagem por partes, mesma ficheiro) ganha o mesmo campo pelo mesmo motivo —
`full_advance`/`start_connector`/`end_connector` são medidas ao longo do eixo de empilhamento,
nunca avanço horizontal:

```rust
pub struct GlyphPart {
    pub glyph_id: u16,
    pub start_connector: u16,  // sobreposição mínima com a peça anterior (eixo de empilhamento)
    pub end_connector: u16,    // sobreposição mínima com a peça seguinte (eixo de empilhamento)
    pub full_advance: u16,     // avanço da peça sem sobreposição, eixo de empilhamento
    pub is_extender: bool,     // se true, repetível para preencher altura/largura
    pub hor_advance: f64,      // avanço horizontal nativo do glifo (hmtx) — ver acima
}
```

## Comportamento

- `select(min_advance)`/`select_with_advance(min_advance)`: comparam **só** `advance` (eixo
  de esticamento) com o alvo; retornam o glyph_id (+ `advance` no caso de
  `select_with_advance`) da primeira variante suficiente, None se nenhuma for
- `is_empty()`: true se não há variantes (fonte sem tabela MATH)
- `Default`: variantes vazias (fallback para glifo base)
- Zero I/O de sistema — tipo de domínio puro

## Critérios de verificação

- `select(600.0)` com variantes [500, 800, 1200] → Some(glyph_id com advance=800)
- `select(500.0)` com variante exacta de 500 → Some(glyph_id)
- `select(1000.0)` com variante máxima de 500 → None
- `select(...)` em GlyphVariants vazio → None
- `is_empty()` em GlyphVariants::default() → true
- `hor_advance` não participa em `select`/`select_with_advance` — só `advance` decide a
  seleção; `hor_advance` é lido separadamente pelo caller depois de seleccionar

## P945 — campo `GlyphAssembly::min_overlap: u16`

**Medição**: `assembly.md` §P945 — o algoritmo do vanilla consome
`minConnectorOverlap` da tabela MATH (NewCMMath: 20du) e o cristalino ignorava.

Novo campo `pub min_overlap: u16` em `GlyphAssembly` (design units; `Default =
0`, que reproduz exactamente o comportamento anterior — retrocompatível com os
testes sintéticos de P913 existentes). Preenchido em L3
(`infra/font_metrics.md` §P945); consumido em L1 por
`resolve_assembly_repeat` e pelo posicionamento das peças
(`compiler/math/layout/assembly.md` §P945).
## P1132n — attachment da variante horizontal selecionada

`GlyphVariant` transporta opcionalmente `top_accent_attach` em design units.
O valor pertence à variante concreta, não ao caractere Unicode base. L3 lê
`MathTopAccentAttachment`; sem entrada, aplica o fallback do vanilla
`(hor_advance + italics_correction) / 2`. O layout de spreaders usa esta
métrica para propagar dinamicamente a largura exacta em composições aninhadas.
