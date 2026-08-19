# Passo 1093 — Relatório: Correção de Literais Truncados de Conversão de Unidade

## 1. Problema

Os factores de conversão de centímetros e milímetros para pontos tipográficos estavam truncados a 3–4 casas decimais:

| Unidade | Valor Truncado | Valor Exacto (razão inteira) | Erro por unidade |
|---|---|---|---|
| cm → pt | `28.346` | `3600 / 127 = 28.346456692913385…` | `+0.000457 pt` |
| mm → pt | `2.8346` | `360 / 127 = 2.8346456692913385…` | `+0.0000457 pt` |

Com margem `1cm` em cada lado da página, o offset total era:

$$\Delta_{\text{margem}} = 2 \times 0.000457\text{ pt} = 0.000914\text{ pt}$$

Este offset de `~0.0005pt` foi observado **sistematicamente** em todas as medições glifo-a-glifo da investigação P1089–P1092.

## 2. Auditoria e Correção

Grep completo executado em todo o repositório (`01_core`, `03_infra`, `04_wiring`):

**Locais de produção encontrados (4, todos corrigidos):**

| Ficheiro | Linha | Literal Antigo | Substituição |
|---|---|---|---|
| `layout_types.rs` | 948 | `v * 28.346` (cm) | `v * Self::PT_PER_CM` |
| `layout_types.rs` | 952 | `v * 2.8346` (mm) | `v * Self::PT_PER_MM` |
| `eval/mod.rs` | 1135 | `value * 2.8346` (mm) | `value * Length::PT_PER_MM` |
| `eval/mod.rs` | 1138 | `value * 28.346` (cm) | `value * Length::PT_PER_CM` |

**Polegada (In):** Confirmada correcta — usa `72.0` exacto ($9144 / 127$).
**Ponto (Pt):** Confirmado correcto — usa `1.0` directo.

**Constantes centralizadas definidas** em `Length`:
```rust
pub const PT_PER_CM: f64 = 3600.0 / 127.0;  // 28.346456692913385…
pub const PT_PER_MM: f64 = 360.0 / 127.0;   // 2.8346456692913385…
pub const PT_PER_IN: f64 = 72.0;             // exacto
```

## 3. Validação Experimental — Offset de Margem Eliminado

### Equação 3 (medida glifo a glifo no PDF)

| Glifo | $\Delta x$ Antes (P1092) | $\Delta x$ Depois (P1093) |
|---|---|---|
| $\nabla$ (primeiro) | `-0.000460 pt` | **`+0.000003 pt`** |
| $f$ | `-0.000460 pt` | **`+0.000003 pt`** |
| $($ | `-0.000460 pt` | **`+0.000002 pt`** |
| glifos 7–29 | `-0.000460 pt` | **`0.000000 pt`** |

**O offset sistemático de `−0.0005pt` foi 100% extinto.**

### Equação 2 (défice de `0.619pt`)

| Métrica | Valor |
|---|---|
| Crystalline MediaBox Width | `262.030 pt` |
| Vanilla MediaBox Width | `262.649 pt` |
| Delta | `-0.619 pt` |

**Confirmado que o défice de `0.619pt` não foi afectado** — é o `space_after_script` trailing, causa separada.

## 4. Testes Automatizados

| Suíte | Testes Aprovados | Falhas |
|---|---|---|
| `typst-core` | 5.077 | 0 |
| `typst-infra` | 796 | 0 |
| `typst-shell` | 41 | 0 |
| `typst-wiring` | 2 | 0 |
| CLI / Integração | 37 | 0 |
| Doc-tests | 2 | 0 |
| **Total Workspace** | **5.955** | **0** |
