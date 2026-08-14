# Relatório de Execução — Passo 1042

**Data**: 2026-08-14
**Status**: `APROVADO (100% dos 20 casos com Delta 0.00pt)`
**Gate**: `ADR-0127 (Categoria 2 — Refinamento de Posicionamento e Largura de Math Cases, Matrix e Vec)`
**Base**: `00_nucleo/diagnosticos/typst-achado-math-cases-gutter-width.md`

---

## 1. Contexto e Diagnóstico

Durante a verificação diferencial do Passo 1041, identificou-se que blocos `$cases(...)$`, `$mat(...)$` e `$vec(...)$` apresentavam largura superior ao compilador oficial Typst Vanilla 0.15.1 (+1.10pt em `cases` e +2.20pt em `mat`/`vec`).

A investigação isolou duas causas fundamentais:
1. **Mecanismo A (Padding Redundante)**: Os delimitadores esticados (`layout_stretchy_delimiter`) já incorporam nas suas próprias métricas de avanço OpenType MATH a margem horizontal. A injeção manual de `padding = style.size * 0.1` duplicava a folga nos dois lados (+1.10pt por delimitador).
2. **Mecanismo B (`&` como Ponto de Alinhamento em `cases`)**: `cases.rs` não processava `align_boundaries`, convertendo cada `&` em uma coluna separada com `col_gap = 5.5pt`, em vez de alinhar com espaçamento natural de símbolo (`metrics.advance(" ")` / `1/3 em`).

---

## 2. Alterações Realizadas

1. **Prompts L0 com Proveniência Real**:
   - `00_nucleo/prompts/compiler/math/layout/matrix.md`: atualizado para remover a regra incorreta `padding = 0.1em`, citando `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:1164-1200` e `typst-layout/src/math/table.rs`.
   - `00_nucleo/prompts/compiler/math/layout/cases.md`: atualizado com a eliminação do padding manual e documentação do suporte nativo a `&`.
2. **Implementação de Layout**:
   - `01_core/src/compiler/math/layout/cases.rs`:
     - Removido o padding manual `+ padding` após `left_box.width`;
     - Integrado suporte a múltiplos pontos de alinhamento `&` via `split_cell_on_align_point` com `col_gap = self.metrics.advance(" ", style.size, style)`.
   - `01_core/src/compiler/math/layout/matrix.rs`:
     - Removido o padding manual `+ padding` nos lados esquerdo e direito dos delimitadores;
     - Expostos `split_cell_on_align_point` e `edge_node` como `pub(super)` para reaproveitamento limpo.
   - `01_core/src/compiler/eval/math.rs`:
     - Adicionado suporte a `Expr::None(_)` em `eval_math_arg_value`, permitindo que `mat(delim: #none)` desative corretamente os delimitadores.
   - `01_core/src/entities/layout_types.rs` e `01_core/src/entities/mod.rs`:
     - Integrado `FrameVisitor` em `layout_types.rs` em total conformidade com a linhagem causal L1 (0 erros no linter).

---

## 3. Bateria Diferencial Completa (Vanilla 0.15.1 vs. Crystalline)

| Caso de Teste | Construto / Sintaxe (.typ) | Largura Vanilla | Largura Crystalline | Delta ($\Delta w$) | RMSE (150 DPI) | Status |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Cases 1L** | `$ cases(1) $` | 11.00 pt | 11.00 pt | **`0.00 pt`** | 0.8616 | **APROVADO** |
| **Cases 2L** | `$ cases(1, 2) $` | 13.75 pt | 13.75 pt | **`0.00 pt`** | 2.2545 | **APROVADO** |
| **Cases 3L** | `$ cases(1, 2, 3) $` | 15.42 pt | 15.42 pt | **`0.00 pt`** | 0.4653 | **APROVADO** |
| **Cases 4L** | `$ cases(1, 2, 3, 4) $` | 15.42 pt | 15.42 pt | **`0.00 pt`** | 0.8760 | **APROVADO** |
| **Cases Ramos Texto** | `$ cases(1 "if" x > 0, 0 "otherwise") $` | 62.71 pt | 62.71 pt | **`0.00 pt`** | 4.1204 | **APROVADO** |
| **Cases com `&`** | `$ cases(1 & "if" & x > 0, 0 & "otherwise" & x <= 0) $` | 92.82 pt | 92.83 pt | **`0.00 pt`** | 4.5579 | **APROVADO** |
| **Mat 1×2** | `$ mat(1, 2) $` | 25.06 pt | 25.06 pt | **`0.00 pt`** | 1.2526 | **APROVADO** |
| **Mat 2×2** | `$ mat(1, 2; 3, 4) $` | 32.69 pt | 32.69 pt | **`0.00 pt`** | 3.0555 | **APROVADO** |
| **Mat 2×3** | `$ mat(1, 2, 3; 4, 5, 6) $` | 43.69 pt | 43.69 pt | **`0.00 pt`** | 3.4956 | **APROVADO** |
| **Mat Delim None** | `$ mat(delim: #none, 1, 2; 3, 4) $` | 16.50 pt | 16.50 pt | **`0.00 pt`** | 2.0057 | **APROVADO** |
| **Vec 2** | `$ vec(1, 2) $` | 21.69 pt | 21.69 pt | **`0.00 pt`** | 2.6093 | **APROVADO** |
| **Vec 3** | `$ vec(1, 2, 3) $` | 24.75 pt | 24.75 pt | **`0.00 pt`** | 3.3403 | **APROVADO** |
| **P1041 Composta** | `$ frac(a + b, c) + sqrt(x) + mat(1, 2; 3, 4) + cases(1 "if" x > 0, ...) $` | 176.44 pt | 176.44 pt | **`0.00 pt`** | 5.2351 | **APROVADO** |
| **Ctrl LR ()** | `$ lr((1 + 2)) $` | 33.00 pt | 33.00 pt | **`0.00 pt`** | 1.4106 | **APROVADO** |
| **Ctrl Delimited** | `$ (1 + 2) $` | 33.00 pt | 33.00 pt | **`0.00 pt`** | 1.4106 | **APROVADO** |
| **Ctrl Frac** | `$ frac(1, 2) $` | 5.50 pt | 5.50 pt | **`0.00 pt`** | 1.1995 | **APROVADO** |
| **Ctrl Root** | `$ sqrt(x + y) $` | 34.29 pt | 34.29 pt | **`0.00 pt`** | 3.1197 | **APROVADO** |
| **Ctrl Underover** | `$ overbrace(x + y, "topo") $` | 27.48 pt | 27.48 pt | **`0.00 pt`** | 1.1822 | **APROVADO** |
| **Ctrl Accent** | `$ hat(x) $` | 6.37 pt | 6.37 pt | **`0.00 pt`** | 0.0000 | **APROVADO** |
| **Ctrl Cancel** | `$ cancel(x) $` | 6.29 pt | 6.29 pt | **`0.00 pt`** | 0.8053 | **APROVADO** |

---

## 4. Validação Geral do Workspace

- **Crystalline Linter**: `0 erros`
- **Cargo Test Workspace**: `5.865 aprovados, 0 falhas, 3 doc-tests ignorados`
- **Release Build**: `100% verde (26.17s)`
