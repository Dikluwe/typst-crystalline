# Prompt L0 — `rules/eval/table` — `#set table(numbering:)`
Hash do Código: ffcfbdc5

**Camada**: L1 · **Alvo**: `01_core/src/rules/eval/rules.rs` (arm `target == "table"`)
**Prompt pai**: `00_nucleo/prompts/rules/eval.md`
**P459**: materializar numeração automática de tables via chain léxica,
análoga a `figure.numbering` (P454) e `equation.numbering` (P456).

---

## `#set table(numbering: ...)`

- Target: `"table"`.
- Argumento nomeado reconhecido: `numbering`.
- Comportamento:
  - `Value::Str(pattern)` → empurrar `("table.numbering", Value::Str(pattern))`
    na `StyleChain` (`engine.styles.push_custom`).
  - `Value::None` → empurrar `("table.numbering", Value::None)` para limpar
    no escopo léxico.
  - Outros tipos → ignorar (herdar).
- Retorna `Value::None`.

## `native_table`

- Aceita argumento nomeado `caption: content|string|none` (P459).
- Armazena `caption` em `TableElem.caption`.
- Não lê `numbering` directamente — o gate vem da chain via `#set table`.

## Verificação

- `#set table(numbering: "1.")` deve transportar `"table.numbering"` até
  `Content::Table` via `Content::Styled` (escopo léxico).
- `#table([...], caption: [...])` sem `#set table(numbering:)` não recebe prefixo.
