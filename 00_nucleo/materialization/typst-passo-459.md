# P459 — Table numbering

> **Passo:** 459  
> **Data:** 2026-06-25  
> **Foco:** Materializar numeração automática de tables (`#table[...]`) com contador independente e caption prefixado, completando a Trilha 1 (numeração).  
> **Trilha:** 1 — Numeração restante (depende de P451 heading numbering, P454 figure numbering, P456 equation numbering, P457 TOC).  
> **ADR-0109:** Reaproveita `CounterRegistry` e `format_counter` (P451/P454/P456); contador de table é independente de heading, figure e equation.

---

## Contexto

O Typst vanilla numera tables automaticamente quando `set table(numbering: "1.")` está activo. O cristalino tem `Content::Table` (entidade) e rendering básico, mas **não tem** contador de table nem caption prefixado com número. Este passo materializa o mecanismo de contador + numeração para tables, reaproveitando a infraestrutura `CounterRegistry` (P451) e `format_counter` (P451/P454/P456), e completando a Trilha 1 de numeração.

**Nota:** Com P459, a Trilha 1 (numeração) fica completa: heading (P451), figure (P454), equation (P456), TOC (P457), table (P459). Isso desbloqueia a Trilha 2 (label/ref), que é a maior lacuna estrutural restante.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Table` existe? | Sim — entidade com `rows: Vec<Vec<Content>>`, `columns: usize` | ✅ |
| Rendering de table existe? | Sim — layout básico via `FrameItem::Group` (grid-like) | ✅ |
| `CounterRegistry` existe? | Sim — P451/P177/P184C | ✅ |
| `format_counter` existe? | Sim — P451 (`entities/counter_format.rs`) | ✅ |
| `numbering` como parâmetro de table? | Não — `native_table` (se existe) não aceita `numbering` | ❌ |
| Caption de table existe? | Parcial — table pode ter caption como `Content` separado? | 🟡 |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S (~20 min; contador de table + formatação + caption prefixado + tests).

---

## Toques pontuais

### 1. Contador de table no `CounterRegistry`

Reaproveitar `CounterRegistry` (P451) com chave `"table"`:

```rust
// Em eval, ao encontrar Content::Table com table.numbering=Some(pattern):
let table_number = counter_registry.step("table");
let formatted = format_counter(&table_number, pattern.as_deref());
```

- Contador de table é **independente** de heading, figure e equation.
- `step("table")` incrementa contador de 1 dimensão (não hierárquico).
- `display` usa `format_counter` com pattern `"1."`, `"I."`, `"(a)"`, etc. (mesmo subset do P451/P454/P456).

### 2. Padrão na `StyleChain` (análogo a figure/equation)

**Decisão:** Usar a `StyleChain` com chave `table.numbering` (análogo a `figure.numbering` em P454/P365 e `math.equation.numbering` em P456), não campo em `TableElem`. Isto é coerente com a arquitectura estabelecida em P365 e evita contradizer a ADR-0117 cláusula 4.

### 3. `TableElem` — verificar se já tem caption

```rust
pub struct TableElem {
    pub rows: Vec<Vec<Content>>,
    pub columns: usize,
    pub caption: Option<Content>,  // VERIFICAR se existe
}
```

- Se `caption` não existe, adicionar `Option<Content>`.
- Se existe, reaproveitar.
- **Não** adicionar campo `numbering` — padrão vive na `StyleChain`.

### 4. `native_table` com caption (`rules/stdlib/structural.rs` ou `rules/stdlib/layout.rs`)

```rust
fn native_table(
    rows: Array,        // ou Vec<Vec<Content>>
    columns: usize,
    caption: Option<Content>,  // NOVO ou existente
) -> Content {
    Content::Table(TableElem { rows, columns, caption })
}
```

- Registar no stdlib scope como `"table"`.
- Se `caption` é `Some`, e `table.numbering` na chain é `Some(pattern)`, o eval computa número e prefixa caption.

### 5. Eval de table com caption numerada (`rules/eval/rules.rs`)

- Ao encontrar `Content::Table`:
  1. Verificar `table.numbering` na `StyleChain`.
  2. Se `Some(pattern)` e `table.caption` é `Some`:
     - `counter_registry.step("table")`.
     - `format_counter(&table_number, pattern)`.
     - Prefixar caption: `"Table {formatted}: " + caption_body`.
  3. Passar para layout como `TableElem` enriquecido (ou computar no layout, coerente com P454/P456).

**Decisão:** Eval computa o número e passa para o layout (coerente com P454 figure numbering e P456 equation numbering). Layout renderiza caption prefixado.

### 6. Layout de table com caption (`rules/layout/table.rs` ou `rules/layout/mod.rs`)

- Ao encontrar `Content::Table`:
  1. Renderizar células como `FrameItem::Group` (grid existente).
  2. Se caption numerada:
     - Renderizar caption como `FrameItem::Text` prefixado com número.
     - Posicionar caption **acima** da table (comportamento vanilla default para tables; figure é abaixo, table é acima).
  3. Envolver table + caption em `FrameItem::Group`.

**Posicionamento do caption:**
- Table: caption **acima** (default vanilla).
- Figure: caption **abaixo** (P454 já fez).
- Equation: número **à direita** (P456 já fez).

### 7. Pattern suportados (subset)

Mesmo subset de P451/P454/P456:
- `"1."` → `1.`, `2.`, `3.`
- `"I."` → Romanos maiúsculos
- `"(a)"` → Letras minúsculas
- `"A."` → Letras maiúsculas

### 8. Tests

- **L1 (eval):** 2 testes — `native_table` com `caption: Some(...)` e `table.numbering = Some("1.")` computa número; `table.numbering = None` omite.
- **L2 (layout):** 2 testes — table com caption numerada posiciona caption acima; table sem caption não tem prefixo.
- **L3 (E2E):** 2 testes — duas tables sequenciais numeram "Table 1", "Table 2"; table com pattern `"[I]"` usa romanos.

### 9. Spec L0

- `00_nucleo/prompts/entities/table.md` — `TableElem` com `caption` (se não existir).
- `00_nucleo/prompts/rules/layout/table.md` — table com caption numerada posicionada acima.
- `00_nucleo/prompts/rules/eval/table.md` — leitura de `table.numbering` da chain.

---

## Scope-out explícito

- **Table inline (não block)** — scope-out; apenas tables block são numeradas.
- **Cross-reference (`@tab1`)** — scope-out; depende de `label`/`ref` (Trilha 2, P460+).
- **List of tables (LoT)** — scope-out; depende de TOC infraestrutura + label/ref.
- **Caption posicionada abaixo** — scope-out; apenas acima (comportamento vanilla default para tables).
- **Sub-tables (a, b, c)** — scope-out; contador hierárquico de 2 dimensões.
- **Table alignment (left/center/right)** — scope-out; assume center.
- **i18n prefixo ("Table" vs "Tabela" vs "Tabelle")** — scope-out; usar "Table " fixo (ou "Tabela " se o cristalino já localiza).

---

## Critério de fecho

- [ ] Eval lê `table.numbering` da `StyleChain` ao processar `Content::Table`.
- [ ] Table com `caption` e `numbering = Some(pattern)` computa número via `CounterRegistry` (chave `"table"`).
- [ ] Layout renderiza caption prefixado com número **acima** da table.
- [ ] Table sem caption ou sem numbering não tem prefixo.
- [ ] 6 tests verdes (2 L1 + 2 L2 + 2 L3).
- [ ] Spec L0 actualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] Trilha 1 marcada como **COMPLETA** no roteiro de conclusão.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| Campo `numbering` em `TableElem` | Rejeitado em P454/P365 em favor da chain. Aplicar a `TableElem` violaria ADR-0117 cláusula 4. |
| Contador de table no layout (não no eval) | Layout não tem acesso à ordem sequencial de tables; eval tem. Coerente com P451/P454/P456. |
| Caption abaixo da table | Comportamento vanilla default para tables é caption acima; figure é abaixo. Divergência de paridade. |
| `native_table(rows, columns, caption?, numbering?)` | O Typst vanilla não expõe `table` como função nativa com parâmetro `numbering`; é um `set` rule. Divergência de paridade. |

---

## Próximo passo (Trilha 2 desbloqueada)

Com P459, a Trilha 1 (numeração) está completa. A Trilha 2 (label/ref + referências cruzadas) é a maior lacuna estrutural restante e está desbloqueada. O próximo passo natural é:
- **P460** — `label<x>` (tabela de destinos nomeados no documento)
- **P461** — `ref<x>` / `@x` (resolução de destino + texto da referência)
- **P462** — PDF `/Dests` + links internos `/GoTo`

**Aguardando sua indicação:**

1. **Executar o P459** (table numbering, ~20 min)?
2. **Escrever o P460** (Trilha 2: label/ref — maior impacto estrutural)?
3. **Pivotar para outra trilha** (Trilha 3: Selector::Where, Trilha 4: visuais, Trilha 6: bibliografia Fase 2, Trilha 8: refinos)?
4. **Ajustar o escopo** do P459?
