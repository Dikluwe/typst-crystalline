# Achado: Inflação de Padding em Delimitadores de Math Cases, Matrix e Vec (`cases.rs` e `matrix.rs`)

**Data**: 2026-08-14
**Origem**: Decomposição e isolamento diferencial do Passo 1041 (DENY 3 / `spacing.rs:72`)
**Módulos responsáveis**: 
- `01_core/src/compiler/math/layout/cases.rs` (`layout_cases`)
- `01_core/src/compiler/math/layout/matrix.rs` (`layout_matrix`, que também processa `vec(...)`)
**Estado**: `CATALOGADO (Pendente de Passo Dedicado)`
**Requer Gate**: Sim (`ADR-0127` — alteração de posicionamento visual/layout)

---

## 1. Mapa Completo de Delimitadores e Gaps no Math Layout

Foi realizado um varrimento exaustivo em todos os construtos matemáticos comparando a largura total gerada contra o **Typst Vanilla 0.15.1**:

| Construto | Exemplo Testado | Largura Vanilla | Largura Crystalline | Delta Total ($\Delta w$) | Diagnóstico |
| :--- | :--- | :---: | :---: | :---: | :--- |
| **`lr(..)`** | `$lr((1 + 2))$` | 33.00 pt | 33.00 pt | **`0.00 pt`** | Paridade exata |
| **`lr[..]`** | `$lr([1 + 2])$` | 30.63 pt | 30.56 pt | **`-0.07 pt`** | Paridade sub-pixel |
| **`delimited`** | `$(1 + 2)$` | 33.00 pt | 33.00 pt | **`0.00 pt`** | Paridade exata |
| **`frac`** | `$frac(1, 2)$` | 5.50 pt | 5.50 pt | **`0.00 pt`** | Paridade exata |
| **`root`** | `$sqrt(x + y)$` | 34.29 pt | 34.29 pt | **`0.00 pt`** | Paridade exata |
| **`underover`** | `$overbrace(x, "top")$` | 27.48 pt | 27.48 pt | **`0.00 pt`** | Paridade exata |
| **`accent`** | `$hat(x)$` | 6.37 pt | 6.37 pt | **`0.00 pt`** | Paridade exata |
| **`cancel`** | `$cancel(x)$` | 6.29 pt | 6.29 pt | **`0.00 pt`** | Paridade exata |
| **`cases`** | `$cases(1, 2)$` | 13.75 pt | 14.85 pt | **`+1.10 pt`** | **Inflação de 1× padding (+0.1em)** |
| **`mat`** | `$mat(1, 2; 3, 4)$` | 32.69 pt | 34.89 pt | **`+2.20 pt`** | **Inflação de 2× padding (+0.2em)** |
| **`vec`** | `$vec(1, 2)$` | 21.69 pt | 23.89 pt | **`+2.20 pt`** | **Inflação de 2× padding (+0.2em via matrix.rs)** |

**Conclusão de Âmbito**: O padrão `padding = style.size * 0.1` está **estritamente circunscrito a `cases.rs` e `matrix.rs`** (e por herança `vec`). Nenhum outro módulo de layout matemático (`lr.rs`, `delimited.rs`, `frac.rs`, `root.rs`, etc.) replica essa injeção manual.

---

## 2. Decomposição de `cases`: Vírgula vs. Ponto de Alinhamento `&`

A investigação isolou os dois mecanismos que operam em `cases`:

### A. Separação por Vírgula (Ramos/Linhas do `cases`):
Em `$cases(1 "if" x > 0, 0 "otherwise")$`:
- O avanço horizontal entre as colunas (`col_gap`) possui paridade exata de pitch.
- O delta é **estritamente fixo em +1.10 pt** independentemente do número de linhas, correspondendo ao `let padding = style.size * 0.1` somado após o delimitador `{`.

### B. Ponto de Alinhamento `&` em `cases`:
Em `$cases(a & b & c)$`:
- No Vanilla: `&` é um `MathAlignPoint` onde os elementos são alinhados entre si com espaçamento de símbolo natural (avanço dinâmico da fonte `advance(" ")`, ~3.65pt em 11pt).
- No Crystalline: `cases.rs` não processa `align_boundaries` e delega para `layout_grid_rows`, que converte cada `&` em uma **nova coluna inteira de grelha**, adicionando `col_gap = style.size * 0.5` (**5.50 pt**) a cada `&` extra.
- Por isso, no teste com múltiplos `&`, a largura agregada inflava proporcionalmente ao número de `&` (+5.50 pt por gap extra).

---

## 3. Plano de Correção Unificada para o Passo Dedicado

1. **Remover padding manual**:
   - `cases.rs`: remover `padding = style.size * 0.1` após `left_box.width`.
   - `matrix.rs`: remover `padding = style.size * 0.1` à esquerda e à direita dos delimitadores.
2. **Tratar `&` em `cases.rs`**:
   - Fazer `cases.rs` utilizar o mecanismo de `align_boundaries` / `split_cell_on_align_point` já adotado em `matrix.rs` para não inflar `col_gap` integral em pontos de alinhamento intra-linha.
3. **Tratar `mat(delim: #none)`**:
   - Suprimir delimitadores e paddings quando `delim: #none`.
