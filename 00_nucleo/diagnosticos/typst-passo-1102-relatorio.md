# Passo 1102 — Relatório: Correção de Captura Prematura de `offset_x` e Validação de Paridade da Secção 31

## 1. Implementação da Correção (§1)

Em [`01_core/src/compiler/layout/equation.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/equation.rs#L165):
A captura de `offset_x` para equações de bloco foi movida para **depois** da chamada de `self.flush_line()`.
Após o flush da linha anterior, `self.regions.current.cursor_x` é resetado para `self.regions.current.line_start_x` (a margem da página, `28.3465 pt`).

---

## 2. Verificação Individual dos 3 Blocos da Secção 31 (§2)

| Bloco de Equação | `applied_offset` Antes | `applied_offset` Depois | `eq_width` | Deslocamento de Centragem ($\Delta X$) |
|---|---|---|---|---|
| **1. `$ sum_(k=1)^n k^2 $`** | $462.5250\text{ pt}$ | **`28.3465 pt`** | $35.9450\text{ pt}$ | **`0.0000 pt`** (era $+16.1368\text{ pt}$) |
| **2. `$ a/b $`** | $282.5500\text{ pt}$ | **`28.3465 pt`** | $5.8190\text{ pt}$ | **`0.0000 pt`** (era $+16.1368\text{ pt}$) |
| **3. `$ sqrt(x+1) $`** | $300.6500\text{ pt}$ | **`28.3465 pt`** | $28.9000\text{ pt}$ | **`0.0000 pt`** (era $+16.1368\text{ pt}$) |

---

## 3. Medição Final da Secção 31 (.typ/sec_31.typ) (§3)

| Métrica | Crystalline P1102 | Vanilla Typst 0.15.1 | Diferença ($\Delta$) |
|---|---|---|---|
| **MediaBox Width** | $494.5500\text{ pt}$ | $494.5453\text{ pt}$ | **`+0.0047 pt`** (era $+32.2747\text{ pt}$) |
| **MediaBox Height** | $241.3100\text{ pt}$ | $263.8196\text{ pt}$ | **`-22.5096 pt`** |
| **Centragem de Blocos** | Alinhamento perfeito | Alinhamento perfeito | **`0.0000 pt`** em todos os 3 blocos |

* A cascata de centragem de $+16.1368\text{ pt}$ foi eliminada em todas as 3 equações de bloco da página.

---

## 4. Confirmação do Offset Vertical de $\sim 1.27\text{ pt}$ (§4)

- **Medição do Sobrescrito `n`**: $dY = -22.5346\text{ pt}$ vs linha $dY = -21.2630\text{ pt}$ (diferença líquida de $\mathbf{-1.2716\text{ pt}}$, rigorosamente inalterada pós-P1102).
- Isso comprova a total independência de causas entre a geometria horizontal de bloco e a elevação de scripts do shaper.

---

## 5. Não-Regressão e Contagem de Testes do Workspace (§5)

- **Suíte Workspace**: **5.955 testes passando (100% pass, 0 falhas)**:
  - `typst-core`: 5.077 testes
  - `typst-infra`: 796 testes
  - `typst-shell`: 41 testes
  - `cli` integration: 37 testes
  - `crystalline_lint`: 2 testes
  - `main/eviction`: 2 testes
- **Linter**: `tests/crystalline_lint.rs` com **0 erros**.
- **Passos P1086-P1101**: Paridade preservada nos casos de teste anteriores.
