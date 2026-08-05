# testing/math_oracle — oráculo de fórmulas de posição do vanilla (P969)
Hash do Código: 29b622b6

**Data:** 2026-08-05 · **Passo:** 969 · **Camada:** L1 (`#[cfg(test)]`)

## Propósito

Módulo de verificação, **não** caminho de execução. Conjunto de funções
puras, uma por fórmula de posição do vanilla já lida e confirmada na frente
P901–P968, transcritas literalmente com `file:line` da fonte
(`lab/typst-original/...`) no doc-comment. Usado pelos testes de geometria
para comparar o motor de produção contra a fórmula de referência, em vez de
valores hardcoded soltos.

## Regras do módulo

1. `01_core/src/testing/math_oracle.rs`, declarado
   `#[cfg(test)] pub(crate) mod testing;` — nunca compilado em produção.
2. Funções puras sobre primitivos (`f64`, pt): constantes MATH e dimensões
   entram como parâmetros; **proibido** importar `MathLayouter`,
   `FontMetrics`, `Content` ou qualquer código de produção — o oráculo não
   pode partilhar falhas com o motor que verifica.
3. Cada função documenta: fórmula literal, `file:line` do vanilla, passo
   que a confirmou.
4. Uma fórmula sem `file:line` confirmado **não entra** — re-ler a fonte
   primeiro (disciplina ADR-0108: medir antes de decidir).
5. Fórmulas lidas mas deliberadamente não portadas no cristalino (ex.:
   `binom` frac-like, P946) não entram — o oráculo espelha o que o motor
   pretende implementar, não o catálogo inteiro do vanilla.

## Funções do lote inicial

| Função | Fórmula vanilla | Fonte | Passo |
|---|---|---|---|
| `large_operator_upper_shift(base_ascent, t_descent, gap_min, rise_min)` | `base.ascent + max(rise_min, gap_min + t.descent)` | `typst-layout/src/math/scripts.rs:290-313` | P959 |
| `large_operator_lower_shift(base_descent, b_ascent, gap_min, drop_min)` | `base.descent + max(drop_min, gap_min + b.ascent)` | `scripts.rs:290-313` | P959 |
| `grid_total_descent(rows, gap)` | `d_1 + Σ_{r≥2}(a_r + d_r) + gap × (n−1)` | `typst-layout/src/math/table.rs:103-106` | P945 |
| `grid_axis_baseline(height, axis)` | `height/2 + axis` | `table.rs:188` | P919 |

**Nota de ajuste ao rascunho da Fase A** (confirmado pelo dono com o
desenho): o rascunho incluía `grid_cell_dy(baseline_offset, row_ascent,
cell_ascent)` na forma `baseline_offset + row_ascent − cell_ascent`
(tradução de `run.rs:137`). P952b (revisão cética retroativa) revogou essa
forma — para items baseline-relativos o dy correcto é simplesmente
`baseline_offset` (`_comum.md` §P952b, `layout_grid_boxes`). Um oráculo
dela codificaria uma fórmula já revogada — **excluída** do lote; se um dia
houver fórmula não-trivial confirmada para a ancoragem de célula, entra
então.

## Uso

Testes de geometria chamam a função com os mesmos inputs do código de
produção e comparam (tolerância típica 0.01pt). Valores esperados soltos
em testes existentes migram para chamadas ao oráculo à medida que cada
fórmula entra — primeira migração neste passo: os 4 asserts de shifts de
limites de `p959_tests` e o assert de `p945_grid_total_descent_sem_dupla_
contagem`.
