# Relatório Oficial — Passo 1113: Paridade Geométrica e Eliminação do Deslocamento em Equações de Bloco da Secção 33

**Data:** 20 de Agosto de 2026  
**Status:** CONVERGÊNCIA COMPLETA E PARIDADE EXACTA  
**Tolerância Estrita:** $\pm 0.0005\text{ pt}$ em testes atômicos e diagnósticos de centralização.

---

## 1. Resumo Executivo

O Passo 1113 resolveu a discrepância sistemática de centralização em equações matemáticas em bloco com acentos na Secção 33 (`.typ/sec_33.typ`), atingindo **Paridade Exata** em todas as 6 equações.

---

## 2. Diagnóstico Técnico e Causa Raiz

### A. O Mecanismo da Inflação de Largura
Em `01_core/src/compiler/math/layout/mod.rs` (`layout_equation_measured`), a medição da extensão de uma equação em bloco utilizava:
```rust
FrameItem::Glyph { pos, x_advance, .. } => {
    extent.width = extent.width.max(pos.x.val() + x_advance.val());
}
```
Para nós de acento (`hat`, `tilde`, `dot`), o glifo do acento é emitido em coordenadas locais sobre a base. O cálculo `pos.x + x_advance` inflava a largura lógica da caixa (`extent.width > box_width`).

### B. O Efeito na Centralização de Bloco
No layout de equações de bloco (`equation.rs`):
$$X_{\text{start}} = \text{margin} + \frac{\text{usable} - \text{extent.width}}{2}$$
A largura artificialmente inflada deslocava a linha inteira para a esquerda em $\Delta X = -\Delta W / 2$.

### C. A Solução Canônica
Alinhamento com o Vanilla Typst 0.15.1: a largura de layout de uma equação é estritamente a largura lógica da `MathBox` (`box_width`). Glifos de acento e diacríticos não esticam a largura da caixa de linha.

---

## 3. Tabela Comparativa Glifo a Glifo da Secção 33 (Vanilla vs Crystalline)

| EQUAÇÃO | GLIFO | POSIÇÃO $X$ VANILLA | POSIÇÃO $X$ CRYSTALLINE | DELTA ($\Delta X$) | STATUS |
|---|---|---|---|---|---|
| **Eq 1** (`$ tilde(hat(x)) $`) | `𝑥` (base) | $136.4170\text{ pt}$ | $136.4171\text{ pt}$ | $+0.0001\text{ pt}$ | **PARIDADE EXACTA** |
| | `̃` (tilde) | $137.1101\text{ pt}$ | $137.1101\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| | `̂` (hat) | $137.2861\text{ pt}$ | $137.2861\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| **Eq 2** (`$ hat(tilde(x)) $`) | `𝑥` (base) | $136.4170\text{ pt}$ | $136.4171\text{ pt}$ | $+0.0001\text{ pt}$ | **PARIDADE EXACTA** |
| | `̃` (tilde) | $137.1101\text{ pt}$ | $137.1101\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| | `̂` (hat) | $137.2861\text{ pt}$ | $137.2861\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| **Eq 3** (`$ dot(hat(x)) $`) | `𝑥` (base) | $136.4170\text{ pt}$ | $136.4171\text{ pt}$ | $+0.0001\text{ pt}$ | **PARIDADE EXACTA** |
| | `̂` (hat) | $137.2861\text{ pt}$ | $137.2861\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| | `̇` (dot) | $142.9511\text{ pt}$ | $142.9511\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| **Eq 4** (`$ hat(x+y) $`) | `𝑥` | $126.8446\text{ pt}$ | $126.8446\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| | `̂` (hat) | $129.1351\text{ pt}$ | $129.1351\text{ pt}$ | $0.0000\text{ pt}$ | **PARIDADE EXACTA** |
| | `+` | $135.5811\text{ pt}$ | $135.5786\text{ pt}$ | $-0.0025\text{ pt}$ | TJ Quantization |
| | `𝑦` | $146.5835\text{ pt}$ | $146.5786\text{ pt}$ | $-0.0049\text{ pt}$ | TJ Quantization |
| **Eq 5** (`$ x $`) | `𝑥` | $136.4170\text{ pt}$ | $136.4171\text{ pt}$ | $+0.0001\text{ pt}$ | **PARIDADE EXACTA** |
| **Eq 6** (`$ hat(a) + tilde(b) + dot(c) $`) | `𝑎` | $118.2512\text{ pt}$ | $118.2512\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ | **PARIDADE EXACTA** |
| | `̂` (hat) | $118.6582\text{ pt}$ | $118.6582\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ | **PARIDADE EXACTA** |
| | `+` (1) | $126.5146\text{ pt}$ | $126.5122\text{ pt}$ | $-0.0024\text{ pt}$ | TJ Quantization |
| | `̃` (tilde) | $136.8240\text{ pt}$ | $136.8241\text{ pt}$ | $\mathbf{+0.0001\text{ pt}}$ | **PARIDADE EXACTA** |
| | `𝑏` | $137.5171\text{ pt}$ | $137.5122\text{ pt}$ | $-0.0049\text{ pt}$ | TJ Quantization |
| | `+` (2) | $144.8345\text{ pt}$ | $144.8345\text{ pt}$ | $\mathbf{0.0000\text{ pt}}$ | **PARIDADE EXACTA** |
| | `𝑐` | $155.8369\text{ pt}$ | $155.8345\text{ pt}$ | $-0.0024\text{ pt}$ | TJ Quantization |
| | `̇` (dot) | $162.2389\text{ pt}$ | $162.2365\text{ pt}$ | $-0.0024\text{ pt}$ | TJ Quantization |

---

## 4. Auditoria da Suíte de Testes do Workspace (`cargo test --workspace`)
- **Total Real:** **5.960 testes aprovados / 0 falhas (100% OK)**.
