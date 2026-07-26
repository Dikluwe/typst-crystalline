# Prompt L0 — `math/layout/delimited` — `MathDelimited`
Hash do Código: 77354839

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/delimited.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathDelimited` — par de delimitadores fixos. Baseline x-height aplicado via
`MathLayouter::apply_axis_offset` (ver `_comum.md`); test regressão
`delimitado_com_axis_height_nao_regride` (`tests.rs:520+`).

## P912 — Cálculo de Altura de Delimitadores Balanceados

Em `layout_delimited`, a altura-alvo do delimitador balanceado (quando `balanced = true`)
é calculada simetricamente em relação ao eixo matemático como `2.0 * (ascent - axis).max(descent + axis)`,
onde `axis` é a altura do eixo (`axis_height`) em pontos tipográficos, em vez de soma simples.
