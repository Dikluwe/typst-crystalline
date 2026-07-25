# Prompt L0 — `math/layout/stretchy` — operadores extensíveis
Hash do Código: 3042b89c

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/stretchy.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

Operadores extensíveis. Consome `GlyphVariants` via
`self.metrics.vertical_glyph_variants(c)` com `.select(min_advance)`
(P255 §2 item 2; `stretchy.rs:22`).

**Critério**: `stretchy.rs` selecciona variant via `select(min_advance)`.

## P906 — `layout_stretchy_glyph_horizontal` (esticamento no eixo X)

Ver `engine/layout.md` §P906 (contexto, dados da fonte, decisão de design).

Segundo método em `stretchy.rs`, espelhando `layout_stretchy_delimiter` — mesmo algoritmo
(variante única via `select_with_advance`; se insuficiente, `horizontal_glyph_assembly` via
`layout_assembly_horizontal`, ver `assembly.md` §P906; sem variantes nem assembly, fallback ao
glifo base via `layout_text_node`), só troca `vertical_glyph_variants`/`vertical_glyph_assembly`
por `horizontal_glyph_variants`/`horizontal_glyph_assembly` e `min_height_du` por `min_width_du`.
Adicionado ao mesmo ficheiro (não um ficheiro novo) — é a mesma unidade conceptual
("esticar um glifo extensível"), só o eixo muda; ADR-0109 não exige separação por eixo.

```rust
pub(super) fn layout_stretchy_glyph_horizontal(
    &self,
    c: char,
    min_width_du: f64,
    style: &TextStyle,
) -> MathBox
```

**Chamador**: `layout_underover`/`layout_accent` (`math/layout/mod.rs`) — ver `_comum.md` §P906.
`min_width_du` = largura da base convertida para design units (`base_box.width * upem /
style.size.val()`), mesma conversão já usada em `layout_root` para `min_height_du`.

**Critério**: para um char com dados horizontais na fonte (`⏟`/`⏞`/`⎵`/`⎴`/`hat`/`tilde`
combinantes), `layout_stretchy_glyph_horizontal(c, min_width_du, style).width >= min_width_du`
convertido para pt (dentro da tolerância normal de granularidade discreta de variantes/assembly);
para um char sem dados horizontais, comportamento idêntico ao `layout_node` anterior (fallback
`layout_text_node`, sem regressão).
