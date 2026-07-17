# Prompt L0 — `math/layout/frac` — `MathFrac`
Hash do Código: 311e044a

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/frac.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathFrac` — fracções. Consome `fraction_rule_thickness` + `fraction_num_gap` +
`fraction_denom_gap` de `MathConstants`. Baseline x-height via
`MathLayouter::apply_axis_offset` (ver `_comum.md`); test regressão
`frac_com_axis_height_nao_regride` (`tests.rs:520+`).

**Critério**: `MathFrac { num: a, den: b }` → sem `[` nos items.
