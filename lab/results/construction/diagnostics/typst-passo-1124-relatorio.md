# Relatório de Implementação e Fechamento — Passo 1124

## 1. Causa Raiz Isolada da Secção 35 (Sub-Frame Ascent em `layout_external`)

- **Diagnóstico Preciso**:
  - Em `layout_external` (`01_core/src/compiler/math/layout/mod.rs:872`), quando um sub-frame continha texto com tamanho aumentado (`#text(size: 20pt)[$b$]`), o `first_baseline` retornava $8.866\text{ pt}$ (o topo padrão da primeira linha criada pelo Layouter a $11\text{ pt}$).
  - A tinta superior do glifo de $20\text{ pt}$ estendia-se até `ink_top` $= -5.014\text{ pt}$ (acima do topo da caixa).
  - O código anterior atribuía `ascent = first_baseline` ($8.866\text{ pt}$) de forma cega, ignorando a extensão real de tinta superior `ink_top` quando `first_baseline` estava presente.
  - **Correção Aplicada**:
    $$\text{ascent} = (\text{anchor} - \text{ink\_top}).\text{max}(0.0) = 8.866 - (-5.014) = \mathbf{13.88\text{ pt}}$$
    $$\text{descent} = (\text{ink\_bottom} - \text{anchor}).\text{max}(0.0) = \mathbf{4.38\text{ pt}}$$
- **Resultado na Secção 35**:
  - Vanilla: $396.17050 \times 135.05891\text{ pt}$
  - Cristalino: $396.17050 \times 135.05890\text{ pt}$
  - **$\Delta W = 0.00000\text{ pt}, \Delta H = 0.00001\text{ pt}$ (Paridade Total de 100%)**.

---

## 2. Status Geral das Secções Testadas

- **Secção 19**: $\Delta W = 0.00002\text{ pt}, \Delta H = \mathbf{0.00000\text{ pt}}$ (100% de paridade).
- **Secção 26**: $\Delta W = 0.00001\text{ pt}, \Delta H = \mathbf{0.00002\text{ pt}}$ (100% de paridade).
- **Secção 27**: $\Delta H = \mathbf{0.00001\text{ pt}}$ (100% de paridade de altura).
- **Secção 28**: $\Delta W = 0.00001\text{ pt}, \Delta H = \mathbf{0.00001\text{ pt}}$ (100% de paridade).
- **Secção 32**: $\Delta W = 0.00000\text{ pt}, \Delta H = \mathbf{0.00002\text{ pt}}$ (100% de paridade).
- **Secção 35**: $\Delta W = 0.00000\text{ pt}, \Delta H = \mathbf{0.00001\text{ pt}}$ (100% de paridade).

---

## 3. Estado da Suíte de Testes e Lints

- `cargo test --workspace`: **100% PASS** (5.958 testes aprovados, 0 falhas).
- `crystalline-lint .`: **0 violações**.
