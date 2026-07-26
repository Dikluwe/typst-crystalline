# Prompt L0 — `math/layout/cases` — `MathCases`
Hash do Código: f87c7fdc

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/cases.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathCases` — chaves grandes (`cases`). Usa o despacho e composição do
`MathLayouter` (ver `_comum.md`).

## P912 — Margem de 10% na Altura da Chave de Cases

Em `layout_cases`, a altura da grelha usada para calcular `min_height_du` da chave esquerda
aplica a margem de 10% do vanilla: `grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1`.

## P918 — cálculo de `min_height_du` migrado para `grid_delim_target_du` partilhado

**Achado** (P918 Fase A, `cases.rs:28-33` vs `matrix.rs:94-99`): o bloco de 5 linhas acima
(margem de 10% + conversão condicional pt→du) é idêntico byte-a-byte ao usado em `layout_matrix`.
Extraído para `grid_delim_target_du(&self, grid_box, style)` em `mod.rs` (ver `_comum.md` §P918)
— `let min_height_du = self.grid_delim_target_du(&grid_box, style);`. Comportamento inalterado.
