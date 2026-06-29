# Prompt L0 — `rules/layout/table` — Layout de `Table`
Hash do Código: 39e6b909

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/table.rs`
**Prompt pai**: `00_nucleo/prompts/rules/layout.md`
**Atomização ADR-0109 P380**: braço `Content::Table` delega para este módulo.
**P459**: numeração automática de tables com caption posicionado acima.

---

## Layout de `Content::Table`

1. Ler `table.numbering` da `StyleChain` (`layouter.chain.custom("table.numbering")`).
   - Só `Value::Str(pattern)` activa numeração.
2. Se `TableElem.caption` é `Some` e há pattern:
   - Incrementar `layouter.table_counter`.
   - Formatar com `format_counter(&[table_counter], pattern)`.
   - Construir prefixo `Table {formatted}: `.
3. Renderizar caption:
   - Se prefixo existir: `Content::Sequence([text(prefix), caption])`.
   - Se não: caption como está.
   - Posicionar **acima** da table, seguido de `Content::linebreak()`.
4. Delegar o corpo da table a `layout_grid` (P157A/P224), preservando
   `columns`, `rows`, `children`, `stroke`, `fill`.

## §P488 — Registo de página para LoT

Quando `caption_prefix.is_some()` (tabela contada), após calcular `table_number`,
regista a página actual em `layouter.runtime.table_page_numbers`:

```rust
let page = layouter.current_page_number();
layouter.runtime.table_page_numbers.push(page);
```

Inserção em ordem de documento — positional match com `tables_for_lot`
no `layout_lot` (§P488 em `layout_outline.md`). Tabelas sem caption ou sem
numbering não são registadas.

## Scope-out

- Caption abaixo da table (vanilla default é acima).
- Cross-reference / label / ref (Trilha 2).
- ~~List of tables~~ — **LoT com page numbers resolvido em P488**.
