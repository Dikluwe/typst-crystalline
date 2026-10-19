## Nota de divergência arquitetural — P459

**Data:** 2026-06-25  
**Autor:** Auditoria pós-P459  
**Referência:** Relatório P459, nota arquitetural; P451, P454, P456.

---

### Divergência

P459 implementou table numbering via contador local `table_counter: usize` no
`Layouter`, em vez de via `CounterRegistry`/introspector com chave `"table"`.

### Padrão estabelecido (Trilha 1)

| Passo | Elemento | Contador | Camada |
|-------|----------|----------|--------|
| P451 | Heading | `CounterRegistry` / `Introspector` | Eval / Oráculo |
| P454 | Figure | `CounterRegistry` / `Introspector` | Eval / Oráculo |
| P456 | Equation | `CounterRegistry` / `Introspector` | Eval / Oráculo |
| **P459** | **Table** | **`table_counter: usize` no `Layouter`** | **Layout (local)** |

### Consequências

1. **Incoerência arquitetural:** Estado mutável no Layouter vs. oráculo selado.
2. **Bloqueio de Trilha 2:** Número de table não é locatable; `label`/`ref` e LoT não conseguem resolver.
3. **Bug de re-layout:** `table_counter` reinicia em re-layout; oráculo persiste.

### Correção

Passo 461 move contador para `CounterRegistry` com chave `"table"`, coerente
com heading/figure/equation. Para isso, `Content::Table` foi promovido a
locatable (`ElementKind::Table`, `ElementPayload::Table`), o walk de
introspecção popula o counter gated por `caption && table.numbering`, e o
layout consome via `flat_counter_at("table", current_location)`.

### Lição

A Cláusula 4 da ADR-0117 (verificar fronteiras/ADR vigentes antes de propor
estrutura) deveria ter impedido esta divergência. P456 aplicou a cláusula
corretamente (verificou P365 antes de propor campo em `MathElem`). P459 não
verificou o padrão de contadores estabelecido em P451/P454/P456 antes de
propor campo local no `Layouter`.
