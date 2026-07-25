# Prompt L0 — `math/layout/assembly` — assembly de delimitadores grandes
Hash do Código: c36cc2d7

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/assembly.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

Assembly por partes para delimitadores grandes. Consome `GlyphAssembly` via
`self.metrics.vertical_glyph_assembly(c)` (P255 §2 item 2; `assembly.rs:14,20`).

**Critério**: `assembly.rs` constrói assembly de partes para delimitadores.

## P906 — `layout_assembly_horizontal` (empilhamento no eixo X)

Ver `engine/layout.md` §P906 (contexto, dados da fonte, decisão de design).

`layout_assembly` (vertical) acumula `y_cursor` por peça (`part.full_advance` menos sobreposição de
conector com a peça seguinte) e emite `FrameItem::Glyph` com `x=0, y=y_in_box` — **não é genérico
por eixo**, está hardcoded a empilhamento vertical (confirmado por leitura antes de decidir: `y_
cursor`/`total_height`/inversão topo↔fundo são todos específicos de Y). Espelho novo, não reuso:
`layout_assembly_horizontal` acumula `x_cursor` (mesma fórmula de sobreposição de conector, eixo
trocado) e emite `FrameItem::Glyph` com `x=x_cursor, y=0` — sem a inversão bottom→top (partes
horizontais assumem-se esquerda→direita na fonte, mesma convenção OpenType usada pelo vanilla, não
verificada por reordenação explícita — se uma fonte listar partes na ordem inversa, é uma limitação
partilhada com o caminho vertical, que também assume a ordem da fonte sem reordenar).

```rust
pub(super) fn layout_assembly_horizontal(
    &self,
    c: char,
    assembly: GlyphAssembly,
    _target_advance: f64,
    style: &TextStyle,
) -> MathBox
```

`ascent`/`descent` da `MathBox` resultante: `ascent = vertical_metrics(style.size, style).0`,
`descent = 0.0` — mesma convenção já usada por `layout_stretchy_delimiter` no ramo de variante única
sem mapeamento de char (`stretchy.rs`), não um valor novo inventado.

**Critério**: para `⏟`/`⏞`/`⎵`/`⎴` (assembly de 5 partes confirmado na fonte via `fontTools`),
`layout_assembly_horizontal` produz uma `MathBox` com `width` próxima do `target_advance` pedido
(dentro da granularidade de conectores) e pelo menos 2 `FrameItem::Glyph` (não 1 — testar que
realmente compôs peças, não caiu no fallback de glifo único).
