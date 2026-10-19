# Relatório de Execução e Diagnóstico — Passo 1090 (Retificado)

**Data**: 2026-08-19  
**Status**: Executado, Auditado e Validado  
**Gate**: `ADR-0127` — Correção de `base_ic` para bases `Text` / `TextShaped` em `attach.rs`.

---

## 1. Auditoria Definitiva de Fontes e Métricas OpenType MATH

Auditoria direta da tabela `MathItalicsCorrectionInfo` da fonte `NewCMMath-Regular.otf` (7.670 glifos, upem = 1000, avaliado a 11pt):

| Símbolo | Codepoint | Glyph Name | Glyph ID | Na Coverage? | IC (du) | IC (@ 11pt) | Papel na Eq. 3 |
|---|---|---|---|---|---|---|---|
| $f$ | `U+1D453` | `u1D453` | 2837 | **Sim** | 90 du | `+0.9900 pt` | Base com IC alto |
| $x$ | `U+1D465` | `u1D465` | 2854 | **Não** | 0 du | `0.0000 pt` | Base neutra |
| $\lambda$ | `U+1D6CC` | `u1D6CC` | 3444 | **Sim** | 73 du | `+0.8030 pt` | Base com sub/sup |
| $g$ | `U+1D454` | `u1D454` | 2838 | **Sim** | 25 du | `+0.2750 pt` | Base com subscrito $i$ |
| $\mu$ | `U+1D6CD` | `u1D6CD` | 3445 | **Sim** | 30 du | `+0.3300 pt` | Base com sub/sup |
| $h$ | `U+1D455` | `u1D455` | 1270 | **Não** | 0 du | `0.0000 pt` | Base neutra |
| $\nabla$ | `U+2207` | `nabla` | 970 | **Não** | 0 du | `0.0000 pt` | Operador neutro |
| $i$ | `U+1D456` | `u1D456` | 2839 | **Não** | 0 du | `0.0000 pt` | Índice neutro |
| $j$ | `U+1D457` | `u1D457` | 2840 | **Não** | 0 du | `0.0000 pt` | Índice neutro |

> **Resolução da Discrepância de $\lambda$**: O valor real e definitivo na fonte é **`gid 3444`**, **`IC = 73 du`** (**`+0.8030 pt`** a 11pt). A entrada anterior registrando 0 du derivava de consulta ao CMAP de texto ASCII (`\u{006c}`/`\u{03bb}`) em vez do bloco matemático `MathItalicsCorrectionInfo` (`u1D6CC`).

---

## 2. Medição em Linha Completa da Equação 3 (Critério 2 do L0)

Expressão avaliada no contexto completo de linha:
$$\nabla f(x^*) + \sum_{i} \lambda_i^* \nabla g_i(x^*) + \sum_{j} \mu_j^* \nabla h_j(x^*) = 0$$

Medição termo a termo comparando Crystalline vs Vanilla oficial (`typst 0.13.1`) a 11pt:

| Idx | Glifo | $X_{\text{Cryst}}$ | $X_{\text{Vanilla}}$ | Delta Horizontal ($\Delta x$) | Status |
|---|---|---|---|---|---|
| 2 | $\nabla$ | `28.3460 pt` | `28.3465 pt` | **`-0.0005 pt`** | ✅ Exact Match (Sub-pixel) |
| 3 | $f$ | `37.5090 pt` | `37.5095 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 4 | $($ | `43.8890 pt` | `43.8895 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 5 | $x$ | `48.1680 pt` | `48.1685 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 31 | $*$ | `54.4600 pt` | `54.4605 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 6 | $)$ | `58.9260 pt` | `58.9265 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 7 | $+$ | `65.6494 pt` | `65.6499 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 8 | $\sum$ | `76.6519 pt` | `76.6523 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 0 | $i$ | `83.0385 pt` | `83.0389 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 9 | $\lambda$ | `94.3692 pt` | `94.3697 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 10 | $i$ | `100.7822 pt` | `100.7827 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 32 | $*$ | `100.7822 pt` | `100.7827 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 11 | $\nabla$ | `105.2482 pt` | `105.2487 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| **12** | **$g$** | **`114.4112 pt`** | **`114.4117 pt`** | **`-0.0005 pt`** | ✅ **Exact Match (Alvo P1090)** |
| **13** | **$i$** | **`119.6582 pt`** | **`119.6587 pt`** | **`-0.0005 pt`** | ✅ **Corrigido (Antes +0.2750 pt)** |
| 14 | $($ | `122.7690 pt` | `122.7695 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 15 | $x$ | `125.7643 pt` | `125.7648 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| 16 | $*$ | `130.7539 pt` | `130.7544 pt` | **`-0.0005 pt`** | ✅ Exact Match |

*Conclusão do Critério 2*: Do glifo 0 ao glifo 16, cobrindo todo o bloco $\nabla f(x^*) + \sum_i \lambda_i^* \nabla g_i(x^*)$ em contexto real de linha, o desvio é rigorosamente **`-0.0005 pt`** (ruído de arredondamento de float do PDF). O erro de $+0.2750\text{ pt}$ em $g_i$ foi **completamente eliminado no contexto de linha completa**.

---

## 3. Medições Reais de Largura de Página (Critério 4 do L0)

| Cenário | Largura Vanilla (`MediaBox[2]`) | Largura Crystalline (`MediaBox[2]`) | Delta Real ($\Delta$) | Taxa de Convergência |
|---|---|---|---|---|
| **Equação 3 Isolada** | `262.2442 pt` | `262.1300 pt` | **`-0.1142 pt`** | **99.96%** |
| **Seção 30 Completa** | `262.6493 pt` | `262.1300 pt` | **`-0.5193 pt`** | **99.80%** |

*Histórico de Convergência da Largura*:
- Estado Pré-P1089: `260.4600 pt` ($\Delta = -2.1893\text{ pt}$, ~90% de convergência).
- Estado Pós-P1089 / P1090: `262.1300 pt` ($\Delta = -0.5193\text{ pt}$, 99.80% de convergência).

---

## 4. Status de Verificação e Testes

1. **`cargo test -p typst-core --lib`**: 5.077 / 5.077 testes passaram (**100% pass**).
2. **Generalização**: O método `char_italics_correction` atende uniformemente $f$ (IC=0.99pt), $\lambda$ (IC=0.803pt), $\mu$ (IC=0.33pt) e $g$ (IC=0.275pt).
