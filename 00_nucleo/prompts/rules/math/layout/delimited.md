# Prompt L0 — `math/layout/delimited` — `MathDelimited`
Hash do Código: 22d815c8

**Camada**: L1 · **Alvo**: `01_core/src/rules/math/layout/delimited.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathDelimited` — par de delimitadores fixos. Baseline x-height aplicado via
`MathLayouter::apply_axis_offset` (ver `_comum.md`); test regressão
`delimitado_com_axis_height_nao_regride` (`tests.rs:520+`).
