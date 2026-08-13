# Prompt L0 — `entities/elements/columns` — `ColumnsElem`

Hash do Código: 8c0598a4

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/columns.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
no `body`. **P552** — adicionada distinção entre `#columns(N)[...]` e
`#set page(columns: N)` via campo `page_columns`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnsElem {
    pub count:         usize,
    pub gutter:        Option<Length>,
    pub body:          Content,            // era Box<Content>
    /// **P552** — `true` quando este `ColumnsElem` foi produzido pela
    /// transformação `wrap_page_columns` de `#set page(columns: N)`.
    /// `false` para a forma-função `#columns(N)[...]`.
    pub page_columns:  bool,
}
```

`Content::Columns { count, gutter, body }` →
`Content::Columns(Arc<ColumnsElem>)`. Construtor ergonómico preservado:
`Content::columns(count, gutter, body)` (default `page_columns: false`).

> **Fonte de paridade da distinção `page_columns` (P1031)** — a diferença entre
> `#columns(N)[...]` e `#set page(columns: N)` **é** superfície de linguagem documentada, e
> não uma decisão interna. Citação literal do doc comment `#[func]` do vanilla ratificado
> (`e0e8ca4d`), `crates/typst-library/src/layout/columns.rs:28-35` — texto publicado em
> `typst.app/docs/reference/layout/columns/#page-level`:
>
> *"= Page-level columns — If you need to insert columns across your whole document, use the
> `{page}` function's `columns` parameter instead. This will create the columns directly at
> the page-level rather than wrapping all of your content in a layout container. As a
> result, things like pagebreaks, footnotes, and line numbers will continue to work as
> expected."*
>
> E do lado do `page`, `crates/typst-library/src/layout/page.rs:233-236`, campo
> `#[default(NonZeroUsize::ONE)] #[ghost] pub columns: NonZeroUsize`: *"How many columns the
> page has. If you need to insert columns into a page or other container, you can also use
> the `columns` function."*
>
> O propósito geral de `columns` está em `columns.rs:6-13`: *"Separates a region into
> multiple equally sized columns. […] By default, columns take up the height of their
> container or the remaining height on the page and are filled up one after another."*
>
> **Natureza**: literal. A frase *"rather than wrapping all of your content in a layout
> container"* nomeia exactamente o mecanismo que o campo `page_columns` distingue, e a lista
> *"pagebreaks, footnotes, and line numbers"* identifica os observáveis que separam as duas
> formas. A mesma citação sustenta a afirmação sobre notas de rodapé em
> `00_nucleo/prompts/passo-537b-set-page-columns.md` §2.

> **`Hash` manual via Debug** (precedente Lote 4/5/7): `Length` carrega `f64` e
> não implementa `Hash` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`; `…Elem` não é chave de mapa —
> revisitar se passar a ser). `PartialEq` deriva (`usize`/`Length`/`Content`
> implementam `PartialEq`). O campo `bool` é incluído automaticamente no `Debug`
> e portanto no hash manual.

## `impl Element for ColumnsElem`

| método | comportamento |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1834`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1665`) |
| `map_content` | **recursivo** no `body`, preserva `count`/`gutter`/`page_columns` (`content.rs:2143`) |
| `map_text` | **recursivo** no `body`, preserva `count`/`gutter`/`page_columns` (map_text) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `count`/`gutter`/`body`/`page_columns` (paridade
`content.rs:2020`).
