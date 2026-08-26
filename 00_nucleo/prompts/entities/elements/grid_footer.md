# Prompt L0 — `entities/elements/grid_footer` — `GridFooterElem`
Hash do Código: d6e0e8e9

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/grid_footer.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). Contentor de
prosa (body + `repeat`) — par simétrico de `GridHeader`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct GridFooterElem {
    pub body:   Content,   // era Box<Content>
    pub repeat: bool,
}
```

`Content::GridFooter { body, repeat }` → `Content::GridFooter(Arc<GridFooterElem>)`.
Construtor ergonómico: `Content::grid_footer(body: Content, repeat: bool)`.

## `impl Element for GridFooterElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1720`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1564`) |
| `map_content` | **recursivo** no body, preserva `repeat` (`content.rs:2279`) |
| `map_text` | **recursivo** no body (`content.rs:2584`), preserva `repeat` |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + repeat` (paridade `content.rs:1890`).

## Critério

`plain_text` do body; `is_empty` delega ao body; `map_*` recursam preservando
`repeat`; igualdade estrutural.
