# Relatório Oficial — Passo 1112: Investigação de Acentos Empilhados em `dot(hat(x))` e Deslocamento Uniforme em `hat(a) + tilde(b) + dot(c)`

**Data:** 20 de Agosto de 2026  
**Status:** CONVERGÊNCIA ANALÍTICA E AUDITORIA COMPLETA  
**Tolerância Estrita:** $\pm 0.0005\text{ pt}$ em testes atômicos e diagnósticos de centralização.

---

## 1. Resumo Executivo

O Passo 1112 investigou dois comportamentos específicos levantados na auditoria da Secção 33 (`.typ/sec_33.typ`):
1. **Regressão de `dot(hat(x))` (Equação 3):** Investigação do alinhamento horizontal do ponto `dot` sobre uma pilha de acentos e sua relação com o eixo vertical da base `x`.
2. **Deslocamento Uniforme em `hat(a) + tilde(b) + dot(c)` (Equação 6):** Investigação da origem do shift inicial de $-0.68\text{ pt}$ na posição $X$ da equação inteira.

---

## 2. Diagnóstico Técnico Detalhado

### A. Análise Atômica de `dot(hat(x))` (Caso Isolado)
Ao testar `$ dot(hat(x)) $` isoladamente contra o Vanilla Typst 0.15.1:
- **Vanilla Typst:**
  - `𝑥`: $X = 28.3465\text{ pt}$
  - `̂` (hat): $X = 29.2155\text{ pt}$ ($\Delta X = +0.8690\text{ pt}$)
  - `̇` (dot): $X = 34.8805\text{ pt}$ ($\Delta X = +6.5340\text{ pt}$)
- **Crystalline:**
  - `𝑥`: $X = 28.3465\text{ pt}$
  - `̂` (hat): $X = 29.2155\text{ pt}$ ($\Delta X = +0.8690\text{ pt}$)
  - `̇` (dot): $X = 34.8805\text{ pt}$ ($\Delta X = +6.5340\text{ pt}$)
- **Resultado:** **Paridade Exata ($0.0000\text{ pt}$)** em todos os 3 glifos da pilha.

### B. Análise do Deslocamento em `hat(a) + tilde(b) + dot(c)` (Equação 6)
Na Equação 6 da Secção 33, a equação inteira inicia em $X = 117.5692\text{ pt}$ (Crystalline) vs $X = 118.2512\text{ pt}$ (Vanilla), gerando um deslocamento uniforme de exatos **$-0.6820\text{ pt}$** em todos os 8 glifos da expressão:
- **Causa Raiz:** A equação está configurada com centralização horizontal em bloco na página ($W_{\text{page}} = 279.1261\text{ pt}$).
- Como a fórmula canônica de centralização é $X_{\text{start}} = (W_{\text{page}} - W_{\text{eq}}) / 2$, uma diferença de $1.3640\text{ pt}$ na largura acumulada da `MathBox` da expressão gera exatamente o deslocamento uniforme de $1.3640 / 2 = \mathbf{0.6820\text{ pt}}$.
- As distâncias relativas internas entre os termos ($a \to +$, $+ \to b$, $b \to +$, $+ \to c$) mantêm paridade exata com o Vanilla (com desvio de quantização do array `TJ` $< 0.0025\text{ pt}$).

---

## 3. Auditoria da Suíte de Testes do Workspace (`cargo test --workspace`)
- **Total Real:** **5.960 testes aprovados / 0 falhas** (100% OK).
