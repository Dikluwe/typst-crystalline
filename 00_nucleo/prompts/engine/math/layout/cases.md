# Prompt L0 — `math/layout/cases` — `MathCases`
Hash do Código: e4129806

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/cases.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathCases` — chaves grandes (`cases`). Usa o despacho e composição do
`MathLayouter` (ver `_comum.md`).

## P912 — Margem de 10% na Altura da Chave de Cases

Em `layout_cases`, a altura da grelha usada para calcular `min_height_du` da chave esquerda
aplica a margem de 10% do vanilla: `grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1`.
