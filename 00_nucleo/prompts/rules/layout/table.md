# Prompt L0 — `rules/layout/table` — Layout de `Table`
Hash do Código: 0de7a090

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

## Scope-out

- Caption abaixo da table (vanilla default é acima).
- Cross-reference / label / ref (Trilha 2).
- List of tables.
