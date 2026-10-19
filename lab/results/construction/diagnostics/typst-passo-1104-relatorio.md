# Passo 1104 — Relatório: Unificação do Protocolo de Parágrafo, GPOS Kerning e Radical Extra Ascender

## 1. Correções Implementadas

### 1.1. Protocolo Genérico de Colapso de Margens para Parágrafos (§1)
* **Em [`01_core/src/compiler/layout/heading.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/heading.rs)**:
  Removido o `extra_below` artificial. O Heading encerra seu layout registrando `layouter.prev_block_below_pending = below_pt` e `layouter.block_chain_active = true`.
* **Em [`01_core/src/compiler/layout/cursor.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/cursor.rs)**:
  Em `ensure_initial_baseline()`, quando `self.block_chain_active == true` e `self.prev_block_below_pending > 0.0`:
  $$\text{cursor\_y} = \text{prev\_line\_baseline} + \text{prev\_block\_below\_pending} + \text{par.top\_edge}$$
  consumindo e zerando o estado pendente pontualmente na primeira palavra do parágrafo.

### 1.2. Extensão Real pós-Shaping com GPOS Kerning (§2)
* **Em [`03_infra/src/font_metrics.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/03_infra/src/font_metrics.rs)**:
  `advance_shaped` agora emprega o shaper (`rustybuzz`) para medição de texto geral quando a fonte real está disponível, integrando o kerning OpenType (GPOS) e eliminando o erro de medição por soma de avanços nominais.

### 1.3. Inclusão Incondicional de `radical_extra_ascender` e `descent_surd` na Raiz (§3)
* **Em [`01_core/src/compiler/math/layout/root.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/root.rs)**:
  - `radical_extra_ascender` agora é adicionado incondicionalmente ao `inner_ascent` do radical (conforme implementação vanilla em `crates/typst-layout/src/math/radical.rs:81`), eliminando a discrepância de $0.8305\text{ pt}$ na baseline de equações com radical.
  - `total_descent = rad_box.descent.max(descent_surd)` assegura que a profundidade esticada do surd seja preservada na aresta inferior da equação.

---

## 2. Verificação dos Critérios de Paridade do L0 (§4)

| Métrica / Teste | Crystalline P1104 | Vanilla Typst 0.15.1 | Diferença ($\Delta$) | Status |
|---|---|---|---|---|
| **1. Gap Heading $\rightarrow$ Parágrafo (Texto Puro)** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`0.0000 pt`** | **CONVERGIU** |
| **2. Gap Heading $\rightarrow$ Parágrafo (Math Inline)** | $15.7630\text{ pt}$ | $15.7630\text{ pt}$ | **`0.0000 pt`** | **CONVERGIU** |
| **3. Caso 1: MediaBox Width** | $160.0300\text{ pt}$ | $160.0269\text{ pt}$ | **`+0.0031 pt`** (era $+0.9131\text{ pt}$) | **CONVERGIU** |
| **4. Caso 1: MediaBox Height** | $97.3800\text{ pt}$ | $97.3819\text{ pt}$ | **`-0.0019 pt`** | **CONVERGIU** |
| **5. Caso 1: Glifo $\Delta X$ (todos os 20 glifos)** | $\max |\Delta X| = 0.0000\text{ pt}$ | $\max |\Delta X| = 0.0000\text{ pt}$ | **`0.0000 pt`** (era $+0.4565\text{ pt}$) | **CONVERGIU** |
| **6. Caso 1: Glifo $\Delta Y$ (todos os 20 glifos)** | $\max |\Delta Y| = 0.0000\text{ pt}$ | $\max |\Delta Y| = 0.0000\text{ pt}$ | **`0.0000 pt`** | **CONVERGIU** |
| **7. Secção 31: MediaBox Height** | $263.8000\text{ pt}$ | $263.8196\text{ pt}$ | **`-0.0196 pt`** (era $-22.5096\text{ pt}$) | **CONVERGIU** |

---

## 3. Não-Regressão e Contagem de Testes do Workspace (§5)

- **Suíte Workspace**: **5.955 testes passando (100% pass, 0 falhas)**:
  - `typst-core`: 5.077 testes
  - `typst-infra`: 796 testes
  - `typst-shell`: 41 testes
  - `cli` integration: 37 testes
  - `crystalline_lint`: 2 testes
  - `main/eviction`: 2 testes
- **Linter**: `tests/crystalline_lint.rs` com **0 erros**.
