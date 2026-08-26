# Prompt L0 — `entities/elements/table_footer` — `TableFooterElem`
Hash do Código: 88b07686

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/table_footer.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). Contentor de
prosa (body + `repeat`) — par simétrico de `TableHeader`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableFooterElem {
    pub body:   Content,   // era Box<Content>
    pub repeat: bool,
}
```

`Content::TableFooter { body, repeat }` → `Content::TableFooter(Arc<TableFooterElem>)`.
Construtor ergonómico preservado: `Content::table_footer(body: Content, repeat: bool)`.

## `impl Element for TableFooterElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1740`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1579`) |
| `map_content` | **recursivo** no body, preserva `repeat` (`content.rs:2331`) |
| `map_text` | **recursivo** no body (`content.rs:2631`), preserva `repeat` |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + repeat` (paridade `content.rs:1920`).

## Critério

`plain_text` do body; `is_empty` delega ao body; `map_*` recursam preservando
`repeat`; igualdade estrutural.
