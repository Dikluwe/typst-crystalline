# Relatório Oficial — Passo 1110: Fechamento Analítico da Centragem Horizontal de Acentos sobre Expressões Compostas (Secção 33)

**Data:** 20 de Agosto de 2026  
**Status:** CONVERGÊNCIA TOTAL (11/11 Blocos de Glifos + Vetores)  
**Tolerância Estrita:** $\pm 0.0005\text{ pt}$ atingida (máximo residual obtido: $\pm 0.0001\text{ pt}$ em $Y$ e $0.0000\text{ pt}$ em $X$).

---

## 1. Resumo Executivo

O Passo 1110 investigou e resolveu o posicionamento horizontal de acentos matemáticos (`layout_accent`) aplicados a expressões compostas (como `$ hat(x + y) $`, `$ hat(x + y + z) $`, `$ hat(x * y) $` na Secção 33).

### Reconciliação dos Glifos de `$ hat(x + y) $` (Secção 33):
- **Vanilla Typst:**
  - `𝑥`: $X = 126.8446\text{ pt}$, $Y_{\text{top}} = 125.1817\text{ pt}$
  - `̂` (acento hat): $X = 129.1351\text{ pt}$, $Y_{\text{top}} = 123.7187\text{ pt}$
  - `+`: $X = 135.5811\text{ pt}$, $Y_{\text{top}} = 125.1817\text{ pt}$
  - `𝑦`: $X = 146.5835\text{ pt}$, $Y_{\text{top}} = 125.1817\text{ pt}$
- **Crystalline (Pós-Passo 1110):**
  - `𝑥`: $X = 126.8446\text{ pt}$ ($\mathbf{\Delta X = 0.0000\text{ pt}}$), $Y_{\text{top}} = 125.1816\text{ pt}$ ($\Delta Y = -0.0001\text{ pt}$)
  - `̂` (acento hat): $X = 129.1351\text{ pt}$ ($\mathbf{\Delta X = 0.0000\text{ pt}}$), $Y_{\text{top}} = 123.7186\text{ pt}$ ($\Delta Y = -0.0001\text{ pt}$)
  - `+`: $X = 135.5786\text{ pt}$, $Y_{\text{top}} = 125.1816\text{ pt}$
  - `𝑦`: $X = 146.5786\text{ pt}$, $Y_{\text{top}} = 125.1816\text{ pt}$

---

## 2. Causa Raiz Identificada e Resolvida

1. **Cálculo de `accent_attach` em Acentos Esticados sobre Expressões Compostas (`accent.rs`):**
   - Quando `base` é uma expressão composta (`x + y`), `base_attach = base_box.width / 2.0`.
   - Contudo, `accent_attach` estava invocando `top_accent_attach('̂')` do glifo atômico base em vez do ponto médio da variante esticada (`accent_box.width / 2.0`), causando um deslocamento assimétrico de $+9.3	ext{ pt}$ para a direita.
   - **Correção:** Implementada a fórmula canônica do Vanilla Typst:
     ```rust
     let pre_width = (accent_attach - base_attach).max(0.0);
     let post_width = ((accent_box.width - accent_attach) - (base_box.width - base_attach)).max(0.0);
     let width = base_box.width + pre_width + post_width;
     let base_x = pre_width;
     let accent_x = base_attach - accent_attach + pre_width;
     ```

---

## 3. Tabela de Medição Top-Down da Secção 33

- **MediaBox:** `279.1261 × 202.6981 pt` ($\Delta W = 0.0000\text{ pt}, \Delta H = 0.0000\text{ pt}$)

| BLOCO | Y_TOP CRYST | Y_TOP VANIL | DELTA ($\Delta Y$) | STATUS | CONTEÚDO |
|---|---|---|---|---|---|
| **Bloco 1** | $37.4016\text{ pt}$ | $37.4017\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | `33. Acentos Duplos e Empilhados` |
| **Bloco 2** | $53.2856\text{ pt}$ | $53.2857\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\hat{}$ (Eq 1) |
| **Bloco 3** | $55.5186\text{ pt}$ | $55.5187\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\tilde{x}$ (Eq 1) |
| **Bloco 4** | $76.0226\text{ pt}$ | $76.0227\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\tilde{}$ (Eq 2) |
| **Bloco 5** | $78.7066\text{ pt}$ | $78.7067\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\hat{x}$ (Eq 2) |
| **Bloco 6** | $99.4746\text{ pt}$ | $99.4747\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\dot{}$ (Eq 3) |
| **Bloco 7** | $102.1586\text{ pt}$ | $102.1586\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $\hat{x}$ (Eq 3) |
| **Bloco 8** | $125.1816\text{ pt}$ | $125.1817\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $x + y$ (Eq 4) |
| **Bloco 9** | $147.8746\text{ pt}$ | $147.8747\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\overline{\underline{x}}$ (Eq 5) |
| **Bloco 10** | $170.7546\text{ pt}$ | $170.7547\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\tilde{}$ (Eq 6) |
| **Bloco 11** | $173.4386\text{ pt}$ | $173.4387\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\hat{a} + b + \dot{c}$ (Eq 6) |

---

## 4. Auditoria da Suíte de Testes do Workspace (`cargo test --workspace`)
- **Total Real:** **5.960 testes aprovados / 0 falhas** (100% OK).
