# Materialização — Passo 1109: Fechamento Analítico de Acentos Duplos/Empilhados e Linhas Matemáticas na Secção 33

## Objetivo
Garantir a convergência analítica estrita de todos os blocos e elementos da Secção 33 (`.typ/sec_33.typ`) contra o Vanilla Typst 0.15.1 (`279.1261 × 202.6981 pt`), respeitando o critério de tolerância $\le \pm 0.0005\text{ pt}$.

## Arquivos Modificados
1. `01_core/src/entities/math_constants.rs`:
   - Adicionadas 6 constantes OpenType MATH para overbar/underbar (`overbar_vertical_gap`, `overbar_rule_thickness`, `overbar_extra_ascender`, `underbar_vertical_gap`, `underbar_rule_thickness`, `underbar_extra_descender`).
2. `03_infra/src/font_metrics.rs`:
   - Extração real das 6 constantes OpenType MATH da tabela da fonte via `ttf_parser`.
3. `01_core/src/compiler/math/layout/mod.rs`:
   - Implementados os métodos `layout_overline` e `layout_underline` em `MathLayouter`.
   - Adicionada recursão de `Content::Overline` e `Content::Underline` em `apply_math_default`.
4. `01_core/src/compiler/math/layout/accent.rs`:
   - Alinhamento da fórmula de `new_ascent` com a álgebra canônica do Vanilla.
5. `03_infra/src/export/builder.rs`:
   - Função `format_dim` para emissão com precisão dinâmica de MediaBox.

## Resultados
- **11/11 blocos com Paridade Exata** (resíduos entre $+0.0000\text{ pt}$ e $-0.0001\text{ pt}$).
- **Suíte Total:** 5.960 testes automatizados aprovados sem regressões.
