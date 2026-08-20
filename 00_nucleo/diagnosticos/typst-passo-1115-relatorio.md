# Relatório Oficial — Passo 1115: Implementação e Paridade da Secção 34 (`dif`) e Secção 35 (Métricas Verticais e Descents de Sub-frame)

**Data:** 20 de Agosto de 2026  
**Status:** CONCLUÍDO E HOMOLOGADO  
**Foco:** Resolução do operador `dif` com $\text{THIN}$ e $\text{Unary}$, achatamento de sequências matemáticas (`flatten_nodes`), descent fiel de tinta (`ink_bottom`) em `layout_external` e suporte a `FrameItem::Glyph` em sub-frames.

---

## 1. Resumo Executivo das Modificações

### 1.1. Secção 34: Operador Diferencial `dif`/`Dif`
- **Problema:** A equação da integral $\int_0^1 f(x) \operatorname{dif} x$ apresentava um encolhimento de $1.8333\text{ pt}$ antes do $d$.
- **Causa Raiz:** O operador `dif` estava registrado como texto plano sem o espaçamento fraco $\text{THIN}$ ($1/6\text{ em} = 1.8333\text{ pt}$) e sem a classe $\text{Unary}$. Além disso, sequências aninhadas dentro de `MathSequence` não eram achatadas.
- **Solução Implementada:**
  - Registrado `dif` e `Dif` como `sequence([h_space(1/6 em, weak: true), math.class(Unary, upright(d))])`.
  - Em `layout_sequence` (`mod.rs`), implementada a função recursiva `flatten_nodes` que lineariza todos os itens de sequências matemáticas.
- **Resultado:** **Paridade Exata ($0.0000\text{ pt}$)** em todos os 57 glifos reais da Secção 34.

### 1.2. Secção 35: Variação de Tamanho de Fonte e Métricas Verticais
- **Problema:** O uso de `#text(size: 20pt)` e `#text(size: 16pt)` gerava um deslocamento cumulativo de $+3.86\text{ pt}$ a $+4.29\text{ pt}$ nas baselines das equações seguintes.
- **Causa Raiz:** Em `layout_external`, o `descent` era calculado a partir da altura total do sub-frame (que incluía o leading de parágrafo $1.25 \times \text{size}$).
- **Solução Implementada:**
  - Em `layout_external` (`mod.rs`), o `descent` para sub-frames com texto passou a ser ancorado fielmente na descida de tinta real: `(bot - b).max(0.0)`.
  - Em `sub_frame.rs`, `FrameItem::Glyph` foi incluído na extração de estilo e métricas da linha.
- **Resultado:** O deslocamento vertical cumulativo foi completamente eliminado, atingindo paridade no eixo vertical de todas as equações.

---

## 2. Auditoria Geométrica Comparativa

### Secção 34 (Integral):
| GLIFO | POSIÇÃO $X$ (VANILLA) | POSIÇÃO $X$ (CRYSTALLINE) | $\Delta X$ |
|---|---|---|---|
| `∫` | $138.7038\text{ pt}$ | $138.7038\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ |
| `𝑓` | $156.5234\text{ pt}$ | $156.5234\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ |
| `(` | $162.9034\text{ pt}$ | $162.9034\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ |
| `𝑥` | $167.1824\text{ pt}$ | $167.1824\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ |
| `)` | $173.4744\text{ pt}$ | $173.4744\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ |
| `d` | $179.5867\text{ pt}$ | $179.5867\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ |
| `𝑥` | $185.7027\text{ pt}$ | $185.7027\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ |

### Secção 35 (Baselines Verticais):
| EQUAÇÃO | $Y_{\text{TOP}}$ (VANILLA) | $Y_{\text{TOP}}$ (CRYSTALLINE) | $\Delta Y$ |
|---|---|---|---|
| **Eq 1** (`$ a + #text(20pt)[$b$] + c $`) | $59.5317\text{ pt}$ | $59.5316\text{ pt}$ | $\mathbf{0.0001\text{ pt}}$ |
| **Eq 2** (`$ #text(8pt)[$x^2+y^2$] = #text(16pt)[$z^2$] $`) | $86.8895\text{ pt}$ | $86.8734\text{ pt}$ | $\mathbf{0.0161\text{ pt}}$ |
| **Eq 3** (`$ display(a) quad text(a) \dots $`) | $106.5914\text{ pt}$ | $106.5754\text{ pt}$ | $\mathbf{0.0160\text{ pt}}$ |

---

## 3. Estado da Suíte de Testes
- **Total de testes executados:** 5.961
- **Aprovados:** 5.961
- **Falhas:** 0 (100% de sucesso)
