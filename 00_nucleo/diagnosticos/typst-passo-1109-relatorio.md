# Relatório Oficial — Passo 1109: Fechamento Analítico de Acentos Duplos/Empilhados e Linhas Matemáticas na Secção 33

**Data:** 20 de Agosto de 2026  
**Status:** CONVERGÊNCIA TOTAL (11/11 Blocos de Glifos + Vetores)  
**Tolerância Estrita:** $\pm 0.0005\text{ pt}$ atingida (máximo residual obtido: $\pm 0.0001\text{ pt}$).

---

## 1. Resumo Executivo

O Passo 1109 realizou a validação e o fechamento analítico da **Secção 33** (`.typ/sec_33.typ`), correspondente a acentos empilhados/duplos (`hat(tilde(x))`, `tilde(hat(x))`, `dot(hat(x))`, `hat(x + y)`, `hat(a) + tilde(b) + dot(c)`) e linhas matemáticas de decorações aninhadas (`overline(underline(x))`).

### Resultados de Dimensão de Página:
- **MediaBox Vanilla:** `279.1261 × 202.6981 pt`
- **MediaBox Crystalline:** `279.1261 × 202.6981 pt` ($\Delta W = 0.0000\text{ pt}, \Delta H = 0.0000\text{ pt}$)

---

## 2. Causas Raízes Identificadas e Resolvidas

1. **Ausência de Handlers Matemáticos Dedicados para `overline` e `underline` (`mod.rs`):**
   - Elementos `Content::Overline` e `Content::Underline` dentro do modo matemático caíam no catch-all `other.plain_text()`, perdendo a renderização vetorial das linhas (`FrameItem::Line`) e o layout das métricas de OpenType MATH (`overbar_vertical_gap`, `underbar_vertical_gap`, etc.).
   - **Correção:** Implementados `layout_overline` e `layout_underline` em `MathLayouter` com a geometria e constantes canônicas de OpenType MATH (`overbar_rule_thickness`, `overbar_vertical_gap`, `overbar_extra_ascender`, `underbar_rule_thickness`, `underbar_vertical_gap`, `underbar_extra_descender`) e propagação em `apply_math_default`.

2. **Alinhamento da Fórmula de `layout_accent` (`accent.rs`):**
   - Ajustada a fórmula de `new_ascent` em `accent.rs` para espelhar a álgebra exata do Vanilla:
     $$\text{ascent} = \text{accent.ascent} + \text{base.ascent} - \min(\text{base.ascent}, \text{accent\_base\_height})$$
     garantindo a altura nominal e o empilhamento de acentos duplos sem perda de avanço vertical.

3. **Formatação Inteligente de MediaBox no PDF Builder (`builder.rs`):**
   - Implementado `format_dim(val)` para preservar 2 casas decimais em páginas padrão e 4 casas decimais em páginas com dimensões fracionárias (`202.6981 pt`), eliminando distorções de escala top-down no cálculo $y_{\text{top}} = H_{\text{page}} - y_{\text{pdf}}$.

---

## 3. Tabela de Medição Top-Down (Vanilla vs Crystalline)

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

- **`typst-core`**: 5.082 aprovados / 0 falhas
- **`typst-infra`**: 796 aprovados / 0 falhas
- **`typst-shell`**: 41 aprovados / 0 falhas
- **`main (eviction)`**: 2 aprovados / 0 falhas
- **`tests/cli.rs`**: 37 aprovados / 0 falhas
- **`tests/crystalline_lint.rs`**: 2 aprovados / 0 falhas
- **Total Real:** **5.960 testes aprovados / 0 falhas** (100% OK).
