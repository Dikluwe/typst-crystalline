# Prompt L0 — `entities/elements/table_header` — `TableHeaderElem`
Hash do Código: 89eac155

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_header.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). Contentor de
prosa (body + `repeat`) — `map_*` recursam no body.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableHeaderElem {
    pub body:   Content,   // era Box<Content>
    pub repeat: bool,
}
```

`Content::TableHeader { body, repeat }` → `Content::TableHeader(Arc<TableHeaderElem>)`.
Construtor ergonómico preservado: `Content::table_header(body: Content, repeat: bool)`.

## `impl Element for TableHeaderElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1739`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1578`) |
| `map_content` | **recursivo** no body, preserva `repeat` (`content.rs:2327`) |
| `map_text` | **recursivo** no body (`content.rs:2627`), preserva `repeat` |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + repeat` (paridade `content.rs:1917`).

## Critério

`plain_text` do body; `is_empty` delega ao body; `map_*` recursam preservando
`repeat`; igualdade estrutural.
