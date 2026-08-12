# Prompt L0 — `rules/eval/table` — `#set table(numbering:)` e namespace `table.*`
Hash do Código: PENDENTE_HUMAN_CALC

**Camada**: L1 · **Alvo**: `01_core/src/compiler/eval/rules.rs` (arm `target == "table"`) e `01_core/src/compiler/stdlib/structural.rs`
**Prompt pai**: `00_nucleo/prompts/compiler/eval.md`
**P459**: materializar numeração automática de tables via chain léxica, análoga a `figure.numbering` (P454) e `equation.numbering` (P456).
**P493b**: namespace anexado em `table` para `table.header`, `table.footer`, `table.cell`.

---

## 1. `#set table(numbering: ...)`

- Target: `"table"`.
- Argumento nomeado reconhecido: `numbering`.
- Comportamento:
  - `Value::Str(pattern)` → empurrar `("table.numbering", Value::Str(pattern))` na `StyleChain` (`engine.styles.push_custom`).
  - `Value::None` → empurrar `("table.numbering", Value::None)` para limpar no escopo léxico.
  - Outros tipos → ignorar (herdar).
- Retorna `Value::None`.

## 2. `native_table`

- Aceita argumento nomeado `caption: content|string|none` (P459).
- Armazena `caption` em `TableElem.caption`.
- Não lê `numbering` directamente — o gate vem da chain via `#set table`.

## 3. Namespace anexado `table.*` (P493b)

A função `table` expõe sub-funções via field access:

| Campo | Função nativa | Propósito |
|---|---|---|
| `table.header` | `native_table_header` | Cabeçalho de tabela. |
| `table.footer` | `native_table_footer` | Rodapé de tabela. |
| `table.cell`   | `native_table_cell`   | Célula de tabela. |

### Construção

`native_table` é registada no scope global como `Func::native_with_namespace`:

```rust
let mut table_ns = Scope::new();
table_ns.define("header", Value::Func(native_table_header));
table_ns.define("footer", Value::Func(native_table_footer));
table_ns.define("cell", Value::Func(native_table_cell));

let table_func = Value::Func(Func::native_with_namespace(
    "table",
    native_table,
    Arc::new(table_ns),
));
```

### Field access

Quando o eval encontra `table.header`, resolve `field = "header"` no namespace anexado a `table` e devolve o `Value::Func` correspondente. Ver `00_nucleo/prompts/compiler/eval.md`, secção «Field Access em tipos primitivos, namespaces e módulos» (subsection `Func` com namespace anexado).

### Semântica das sub-funções

- `table.header(body, repeat: true)` → `Content::TableHeader`.
- `table.footer(body, repeat: true)` → `Content::TableFooter`.
- `table.cell(body, colspan: 1, rowspan: 1, fill: none, stroke: none)` → `Content::TableCell`.

Os detalhes de `native_table_header`/`native_table_footer`/`native_table_cell` permanecem como definidos em `entities/elements/table_header.md`, `table_footer.md` e `table_cell.md`.

---

## 4. Limitação conhecida (P661)

A numeração automática de P459 é uma extensão do cristalino: a tabela
numerada não é transformada numa `figure` e, consequentemente, **não**
dispara show rules de `figure.where(kind: table)` nem herda estilos aplicados
a `figure`. Ver `00_nucleo/prompts/compiler/layout/table.md` §P661.

## 5. Verificação

- `#set table(numbering: "1.")` deve transportar `"table.numbering"` até `Content::Table` via `Content::Styled` (escopo léxico).
- `#table([...], caption: [...])` sem `#set table(numbering:)` não recebe prefixo.
- `table.header[Nome][Idade][Cidade]` deve avaliar para `Content::TableHeader`.
- `table.footer[Total][2][—]` deve avaliar para `Content::TableFooter`.
- `table.cell[Conteúdo]` deve avaliar para `Content::TableCell`.
- `table(...)` sem header/footer continua a funcionar (não-regressão).
- Show rules de `figure.where(kind: table)` não afectam tabelas numeradas por P459 (P661).
