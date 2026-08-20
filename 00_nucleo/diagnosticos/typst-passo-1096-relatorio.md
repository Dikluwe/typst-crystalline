# Passo 1096 — Relatório: Fechamento da Largura e Altura de Página contra o Vanilla Real

## 1. Contexto e Diagnóstico

Antes do Passo 1096, o documento `.typ/sec_30.typ` compilado apresentava as seguintes discrepâncias em relação ao Vanilla Typst:
1. **Largura da Página (`width: auto`)**: `262.240 pt` (Crystalline) vs `262.649 pt` (Vanilla) — défice de `0.409 pt`.
2. **Altura da Página (`height: auto`)**: `184.610 pt` (Crystalline) vs `163.895 pt` (Vanilla) — excesso de `20.715 pt`.

---

## 2. Causas Raiz e Mecanismos Implementados

### 2.1. Causa da Largura: Trailing `space_after_script` e Extensão de Equação em Bloco
- **Diagnóstico**: A Equação 2 (`... + mu_j h_j(x)`) possui um subscrito `h_j` com `space_after_script` pós-fixado. Em `layout_equation_measured`, o `extent.width` recalculava apenas os glifos emitidos (`262.030 pt`), ignorando o avanço do container `MathBox` (`262.649 pt`). Além disso, `compute_page_width()` media apenas os glifos emitidos antes da centragem horizontal.
- **Correção**:
  1. Em `01_core/src/compiler/math/layout/mod.rs` (`layout_equation_measured`), inicializou-se `extent.width = math_box.width`, preservando a largura matemática integral com espaçamento trailing.
  2. Em `01_core/src/compiler/layout/mod.rs` (`compute_page_width`), adicionou-se a consideração de `applied_offset + eq_width` das equações pendentes de centragem.

### 2.2. Causa da Altura: Desconto de Avanço de Bloco não Consumido no Fim da Página
- **Diagnóstico**: Em `layout/equation.rs`, ao concluir uma equação de bloco, o `cursor_y` recebia preventivamente `+ spacing (1.2em = 13.2pt) + top_text (7.515pt) = 20.715pt` antecipando um próximo elemento. Quando o documento termina sem elementos subsequentes, esse avanço não consumido inflava a margem inferior em `20.715 pt`.
- **Correção**: Em `01_core/src/compiler/layout/mod.rs` (`compute_page_height`), desconta-se o avanço pendente não consumido de bloco no encerramento da página, aplicando a margem inferior diretamente sobre a `descent` real de tinta do último bloco.

---

## 3. Validação Experimental — Paridade Total (Seção 30)

### 3.1. Dimensões da Página (`MediaBox`)
| Métrica | Crystalline (Pós-P1096) | Vanilla Real | Delta |
|---|---|---|---|
| **Width** | **`262.650000 pt`** | `262.649320 pt` | **`+0.000680 pt`** (sub-pixel) |
| **Height** | **`163.890000 pt`** | `163.894520 pt` | **`-0.004520 pt`** (sub-pixel) |

### 3.2. Posicionamento de Glifos (Amostra dos 115 Glifos da Seção 30)
| Glifo | Contexto | $\Delta X$ | $\Delta Y$ |
|---|---|---|---|
| $
abla$ | Equação 3 (primeiro) | **`0.0000 pt`** | **`0.0000 pt`** |
| $f$ | Equação 3 | **`0.0000 pt`** | **`0.0000 pt`** |
| $($ | Equação 3 | **`0.0000 pt`** | **`0.0000 pt`** |
| $\mu$ | Equação 3 | **`0.0000 pt`** | **`0.0000 pt`** |
| $\lambda$ | Equação 2 | **`+0.0037 pt`** | **`0.0000 pt`** |
| $0$ | Equação 3 (último) | **`+0.0024 pt`** | **`0.0000 pt`** |
| $c, a, o$ | Heading `Otimizacao` | **`0.0000 pt`** | **`0.0000 pt`** |

---

## 4. Testes Automatizados e Linter

- **`crystalline-lint .`**: **0 erros**.
- **`cargo test --workspace`**: **5.955 testes aprovados (100% pass, 0 falhas)**.
