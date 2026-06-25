# Relatório — Passo 459: Table numbering

## Resumo

Materializou-se a numeração automática de tables (`#table[...]`) com contador
independente e caption prefixado, completando a **Trilha 1 — Numeração**
(heading P451, figure P454, equation P456, TOC P457, table P459).

A implementação segue o padrão estabelecido para `figure.numbering` e
`equation.numbering`: o padrão vive na `StyleChain` (`table.numbering`), o
`native_table` aceita `caption` e o layout computa o número e posiciona o
caption **acima** da table (vanilla default para tables).

## Alterações

### 1. `01_core/src/entities/elements/table.rs`

- `TableElem` ganhou campo `caption: Option<Content>`.
- `plain_text` passou a incluir o caption quando presente.
- `is_empty` passou a considerar `caption`.
- `map_content` e `map_text` preservam e recursam sobre `caption`.
- Prompt L0 `00_nucleo/prompts/entities/elements/table.md` actualizado.

### 2. `01_core/src/entities/content.rs`

- Construtor `Content::table(...)` mantém `caption: None` para backward
  compatibilidade.
- Adicionado `Content::table_with_caption(columns, rows, children, caption)`
  para construção programática com caption.
- Testes existentes que constroem `TableElem` manualmente foram actualizados
  com `caption: None`.

### 3. `01_core/src/rules/stdlib/structural.rs`

- `native_table` passou a aceitar argumento nomeado `caption: content|string|none`.
- Validação de argumentos nomeados actualizada para permitir `caption`.
- `TableElem` construído com `caption` extraído.

### 4. `01_core/src/rules/eval/rules.rs`

- Adicionado arm `target == "table"` em `eval_set_rule` para
  `#set table(numbering: ...)`.
- Empurra `("table.numbering", Value::Str(pattern))` na `StyleChain`;
  `Value::None` limpa no escopo léxico.
- Prompt L0 `00_nucleo/prompts/rules/eval/table.md` criado.

### 5. `01_core/src/rules/layout/mod.rs`

- Adicionado campo `table_counter: usize` ao `Layouter`.
- Inicializado a `0` em `Layouter::new`.

### 6. `01_core/src/rules/layout/table.rs`

- Layout de `Content::Table` lê `table.numbering` da `StyleChain`.
- Se houver caption e pattern, incrementa `table_counter`, formata com
  `format_counter` e prefixa o caption com `Table {formatted}: `.
- Caption renderizado **acima** da table, seguido de `Content::linebreak()`.
- Delegação ao motor `layout_grid` preservada.
- Prompt L0 `00_nucleo/prompts/rules/layout/table.md` criado.

### 7. Testes

- **L1 (eval)** — `rules/eval/tests.rs`:
  - `p459_set_table_numbering_assa_via_chain`
  - `p459_set_table_escopo_lexical_nao_vaza`
- **L2 (layout)** — `rules/layout/tests.rs`:
  - `p459_table_caption_numbering_prefixo_acima`
  - `p459_table_sem_caption_sem_prefixo`
- **L3 (E2E)** — `rules/eval/tests.rs`:
  - `p459_table_source_sequencia_numerada`
  - `p459_table_source_pattern_romano`

## Verificação

```bash
RUST_MIN_STACK=16777216 cargo test --workspace
# Resultado: todos os crates passaram (3234 + 491 + 24 + 2 + 21 + 2 + doc-tests).

crystalline-lint --fail-on error .
# Resultado: exit 0; restam apenas warnings pré-existentes de prompts órfãos
# (adr-stub-vs-fallback, show-regex) mais o novo prompt eval/table.md que
# ainda não foi referenciado por um arquivo dono único.
```

## Notas

- O padrão `"1."` produz prefixo `"Table 1.: "` porque `format_counter`
  preserva o ponto do pattern; isso espelha o comportamento de `format_counter`
  usado para heading/figure/equation.
- A decisão arquitectural de calcular o número no layout (via contador local
  `table_counter`) em vez de no eval via `CounterRegistry` mantém a coerência
  com o caminho real do `figure_progress`/`introspector` e evita tornar
  `TableElem` locatable neste passo (cross-reference continua scope-out).
- Table inline, label/ref, List of Tables, caption abaixo e i18n do prefixo
  permanecem explicitamente fora de escopo, conforme o roteiro P459.
