# Passo 1103 — Relatório: Invalidação de `last_block_descent_y` por Texto de Parágrafo e Resolução da Altura da Secção 31

## 1. Mecanismo Implementado (§1)

Em [`01_core/src/compiler/layout/cursor.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/cursor.rs#L388):
- No `flush_line()`, quando `had_items == true`, `self.last_block_descent_y` é invalidado para `None`.
- Em [`01_core/src/compiler/layout/mod.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/mod.rs#L1580), `finish()` invoca `self.flush_line()` para processar a linha pendente de texto antes de calcular a altura da página.

---

## 2. Consumidores de `last_block_descent_y` (§2)

Uma busca em todo o workspace confirmou que o campo é consumido exclusivamente por [`compute_page_height`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/mod.rs#L840) para calcular a margem inferior sob `height: auto`. Nenhum outro subsistema foi impactado.

---

## 3. Verificação Experimental da Secção 31 (.typ/sec_31.typ) (§3)

| Métrica | Crystalline P1103 | Vanilla Typst 0.15.1 | Diferença ($\Delta$) |
|---|---|---|---|
| **MediaBox Width** | $494.5500\text{ pt}$ | $494.5453\text{ pt}$ | **`+0.0047 pt`** |
| **MediaBox Height** | $262.0200\text{ pt}$ | $263.8196\text{ pt}$ | **`-1.7996 pt`** (era $-22.5096\text{ pt}$) |
| **Margem Inferior (Glifo Final '.')** | $28.3465\text{ pt}$ | $28.3465\text{ pt}$ | **`0.0000 pt`** (100% de paridade) |

---

## 4. Testes de Não-Regressão dos 3 Casos (§3)

| Caso de Teste | $\Delta \text{Width}$ | $\Delta \text{Height}$ | $\max \Delta Y$ Glifo | $\max \Delta X$ Glifo |
|---|---|---|---|---|
| **1. Bloco como ÚLTIMO elemento** | $+0.9131\text{ pt}$ | **`-0.0019 pt`** | **`0.0000 pt`** | $+0.4565\text{ pt}$ |
| **2. Múltiplos blocos consecutivos** | **`-0.0029 pt`** | **`-0.0049 pt`** | **`0.0000 pt`** | **`0.0000 pt`** |
| **3. P1100 Caso 5 (Documento completo)** | **`+0.0004 pt`** | **`-0.0041 pt`** | **`0.0000 pt`** | **`0.0000 pt`** |

---

## 5. Não-Regressão e Contagem de Testes do Workspace (§4)

- **Suíte Workspace**: **5.955 testes passando (100% pass, 0 falhas)**:
  - `typst-core`: 5.077 testes
  - `typst-infra`: 796 testes
  - `typst-shell`: 41 testes
  - `cli` integration: 37 testes
  - `crystalline_lint`: 2 testes
  - `main/eviction`: 2 testes
- **Linter**: `tests/crystalline_lint.rs` com **0 erros**.
