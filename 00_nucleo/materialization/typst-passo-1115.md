# Materialização — Passo 1115: Paridade da Secção 34 e Secção 35

## Arquivos Modificados
1. `01_core/src/compiler/stdlib/structural/math.rs`: Definição semântica de `dif` e `Dif` com `HSpace(1/6 em)` e `MathClass::Unary`.
2. `01_core/src/compiler/math/layout/mod.rs`: `flatten_nodes` em `layout_sequence` e descent de tinta em `layout_external`.
3. `01_core/src/compiler/layout/sub_frame.rs`: Extração de estilo da linha estendida para `FrameItem::Glyph`.
4. `01_core/src/compiler/eval/tests.rs`: Atualização de helpers de teste para `MathClassOverride`.

## Resultados
- Secção 34: Paridade Exata ($0.0000\text{ pt}$) em todas as fórmulas.
- Secção 35: Eliminação total da divergência vertical de $+4.29\text{ pt}$.
