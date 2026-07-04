# Prompt L0 — `entities/elements/columns` — `ColumnsElem`

Hash do Código: ae395740

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/columns.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura). Trait: ver
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
