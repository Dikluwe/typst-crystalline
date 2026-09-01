# Prompt L0 — `compiler/layout/table` — Layout de `Table`
Hash do Código: b2e5158e

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/table.rs`
**Prompt pai**: `00_nucleo/prompts/compiler/layout.md`
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
   `columns`, `rows`, `children`, `stroke`, `fill`, e (**P772v**)
   `header`/`footer` — `layout_grid` extrai as células do body de
   `Content::TableHeader`/`Content::TableFooter` e cola-as antes/depois
   das células normais como row-group real, mesmo mecanismo de
   `grid.header`/`grid.footer` (P772i, ver `00_nucleo/prompts/compiler/layout.md`
   secção "`grid.header(...)`/`grid.footer(...)` como row-groups"). Antes de
   P772v, `header`/`footer` eram sempre passados como `None, None` — o
   `TableElem` não tinha esses campos e `table.header[...]`/`table.footer[...]`
   caíam no braço genérico de célula normal em `native_table`.

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

## §P661 — Limitação: fora do mecanismo `figure`

A numeração automática de P459 é implementada directamente no layout de
`Content::Table`. A tabela resultante **não é uma `figure`** e, por isso,
**não responde** a selectors/show rules que visam `figure` ou
`figure.where(kind: table)`. Exemplos que não afectam a tabela numerada:

```typst
#show figure.where(kind: table): set figure.caption(position: top)
#show figure: [prefixo]
```

Estas regras aplicam-se apenas a tabelas envolvidas explicitamente em
`#figure(table(...), caption: [...])`. Para estilizar tabelas numeradas via
P459, o utilizador deve recorrer a `#show table: ...` (quando suportado) ou a
outros mecanismos específicos da extensão.

Esta é uma limitação conhecida e aceite da extensão P459; não é um bug de
paridade com o vanilla, onde a numeração de tabelas passa sempre por `figure`.

## Scope-out

- Caption abaixo da table (vanilla default é acima).
- Cross-reference / label / ref (Trilha 2).
- Integração de `table.numbering` no mecanismo `figure` (P661 — limitação documentada).
- ~~List of tables~~ — **LoT com page numbers resolvido em P488**.

## P1288 — envelope semântico da árvore de tabela (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`01_core/src/compiler/layout/table.rs:70-87` delega diretamente a
`layout_grid` e não preserva identidade Table no frame. A fonte vanilla pinada
constrói `/Table` com summary em `typst-pdf/src/tags/context/table.rs:129-135`
e linhas/células em `:289-353`.

### Decisão proposta

Manter a Forma B: `layout/table.rs` continua free function descendente e o
dispatcher permanece estático/exaustivo. A função envolve o resultado visual
da tabela numa árvore semântica fechada que preserva summary, ordem de linhas,
ordem de células, header/footer e spans, sem executar novo layout nem alterar
frames filhos. Caption/numeração permanecem fora do envelope Table.

`table.header` deriva Header/Column no seu level somente para células Auto;
classificação explícita Header/Data vence: Data explícita dentro de header
permanece `TD`. `table.footer` deriva Footer para Auto.

A estrutura lógica medida tem três formas: tabela simples produz `Table >
TR*`; header automático uniforme produz um único `THead` com seu `TR`/`TH` e
um único `TBody` com `TR`/`TD`; linha explícita mista permanece `TR` direta
sob `Table`, sem grupo artificial. `level` não é atributo numérico de saída:
ele determina relações `Headers` entre níveis e faz cada dado apontar ao
header aplicável mais próximo. `scope` e spans determinam aplicabilidade.

Em multipágina, cópias visuais do header não duplicam o `TH`, `THead` nem suas
relações: uma única árvore lógica atravessa páginas, enquanto MCIDs são locais
e reiniciam por página.

Não criar `dyn`, vtable, dispatcher por strings, import entities→compiler ou
derivação por aparência. AT real, reflow e conformidade PDF/UA permanecem
Unknown; a estrutura positiva não os prova.
