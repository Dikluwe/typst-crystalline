# Passo 1100 — Relatório: Calibração da Geometria de `box()` / `layout_external` e Validação de Não-Regressão

## 1. Diagnóstico e Causa Raiz

A divergência observada em `box()` embutido em equações foi rastreada a duas causas fundamentais no compilador:
1. **Vertical**:
   - Em `01_core/src/compiler/layout/boxed.rs`, o topo da caixa (`pos.y`) utilizava uma aproximação `cursor_y - line_h` (que posicionava o topo da caixa $4.5155\text{ pt}$ abaixo do correto). Corrigido para `cursor_y - top_edge - inset_top - outset_top`.
   - Em `01_core/src/compiler/layout/equation.rs`, a baseline inicial no topo da página foi ancorada em `self.page_config.margin + ext.ascent`.
   - Em `01_core/src/compiler/math/layout/mod.rs`, `extent.ascent` e `extent.descent` passam a incorporar os limites exatos do frame exterior da caixa.
2. **Horizontal**:
   - O `overhang` decorativo de `Stroke` em `boxed.rs` expandia espuriamente `outer_w` por $+2 \times (\text{thickness}/2) = +0.5\text{ pt}$ por caixa. O Typst Vanilla desenha o stroke centralizado na borda sem alterar o avanço do frame inline.
   - Em `01_core/src/compiler/layout/equation.rs`, foi adicionada a integração e propagação de `FrameItem::TextShaped` e `FrameItem::Glyph` em equações de bloco.

---

## 2. Verificação Experimental dos 5 Casos (§2 e §3)

### Medição Pós-Correção vs Vanilla

| Caso Testado | Linha / Expressão | $Y_{\text{Cryst}}$ | $Y_{\text{Vanil}}$ | $\Delta Y$ Alvo | $\Delta Y$ Real | $\max \Delta X$ |
|---|---|---|---|---|---|---|
| **1. Eq 1 isolada** | `$ boxed(a) + boxed(b) = boxed(c) $` | $31.3465\text{ pt}$ | $31.3465\text{ pt}$ | $0.0000\text{ pt}$ | **`+0.0000 pt`** | **`0.0000 pt`** |
| **2. Eq 2 isolada** | `$ boxed(a) + boxed(a) + boxed(a) + boxed(a) $` | $31.3465\text{ pt}$ | $31.3465\text{ pt}$ | $0.0000\text{ pt}$ | **`+0.0000 pt`** | **`0.0000 pt`** |
| **3. Heading + Eq 1** | `== 40. Reuso`<br>`$ boxed(a) + boxed(b) = boxed(c) $` | $50.1095\text{ pt}$<br>$31.3465\text{ pt}$ | $50.1095\text{ pt}$<br>$31.3465\text{ pt}$ | $0.0000\text{ pt}$<br>$0.0000\text{ pt}$ | **`+0.0000 pt`**<br>**`+0.0000 pt`** | **`0.0000 pt`**<br>**`0.0000 pt`** |
| **4. Heading + Eq 2** | `== 40. Reuso`<br>`$ boxed(a) + boxed(a) + boxed(a) + boxed(a) $` | $50.1095\text{ pt}$<br>$31.3465\text{ pt}$ | $50.1095\text{ pt}$<br>$31.3465\text{ pt}$ | $0.0000\text{ pt}$<br>$0.0000\text{ pt}$ | **`+0.0000 pt`**<br>**`+0.0000 pt`** | **`0.0000 pt`**<br>**`0.0000 pt`** |
| **5. Documento completo** | `== 40. Reuso`<br>`$ boxed(a) + boxed(b) = boxed(c) $`<br>`$ boxed(a) + boxed(a) + boxed(a) + boxed(a) $` | $76.8225\text{ pt}$<br>$58.0595\text{ pt}$<br>$31.3465\text{ pt}$ | $76.8225\text{ pt}$<br>$58.0595\text{ pt}$<br>$31.3465\text{ pt}$ | $0.0000\text{ pt}$<br>$0.0000\text{ pt}$<br>$0.0000\text{ pt}$ | **`+0.0000 pt`**<br>**`+0.0000 pt`**<br>**`+0.0000 pt`** | **`0.0000 pt`**<br>**`0.0000 pt`**<br>**`0.0000 pt`** |

---

## 3. Não-Regressão e Contagem de Testes do Workspace (§4)

- **Seção 30**: $\Delta Y = \mathbf{0.000000\text{ pt}}$ em todos os 115 glifos (paridade $\le 10^{-6}\text{ pt}$).
- **Seção 35**: $\Delta Y = -0.0160\text{ pt}$ (resíduo decimal constante de cabeçalho).
- **Contagem Total de Testes no Workspace**: **5.955 testes passando (100% pass)**:
  - `typst-core`: 5.077 testes
  - `typst-infra`: 796 testes
  - `typst-shell`: 41 testes
  - `cli` integration: 37 testes
  - `crystalline-lint`: 2 testes
  - `main/eviction`: 2 testes
- **Linter**: `tests/crystalline_lint.rs` com **0 erros**.
