# Prompt L0 — `entities/elements/grid_header` — `GridHeaderElem`
Hash do Código: 63582830

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/grid_header.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). Contentor de
prosa (body + `repeat`) — `map_*` recursam no body (precedente Heading/Lote 3).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct GridHeaderElem {
    pub body:   Content,   // era Box<Content>
    pub repeat: bool,
}
```

`Content::GridHeader { body, repeat }` → `Content::GridHeader(Arc<GridHeaderElem>)`.
Construtor ergonómico: `Content::grid_header(body: Content, repeat: bool)`.

## `impl Element for GridHeaderElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1719`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1563`) |
| `map_content` | **recursivo** no body, preserva `repeat` (`content.rs:2275`) |
| `map_text` | **recursivo** no body (`content.rs:2580`), preserva `repeat` |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + repeat` (paridade `content.rs:1888`).

## Critério

`plain_text` do body; `is_empty` delega ao body; `map_*` recursam preservando
`repeat`; igualdade estrutural.
