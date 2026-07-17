# Prompt L0 — `math/layout/root` — `MathRoot`
Hash do Código: 79a5a625

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/root.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathRoot` — sqrt + n-th roots. Consome `radical_vertical_gap` +
`radical_rule_thickness` de `MathConstants`. Baseline x-height aplicado via
`MathLayouter::apply_axis_offset` (ver `_comum.md`); test regressão
`sqrt_com_axis_height_nao_regride` (`tests.rs:520+`).
