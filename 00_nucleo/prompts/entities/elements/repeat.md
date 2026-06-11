# Prompt L0 — `entities/elements/repeat` — `RepeatElem`
Hash do Código: 9b1a0fba

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/repeat.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P322). Contentor —
`map_*` recursam no body.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct RepeatElem {
    pub body:    Content,         // era Box<Content>
    pub gap:     Option<Length>,
    pub justify: bool,
}
```

`Content::Repeat { body, gap, justify }` → `Content::Repeat(Arc<RepeatElem>)`.
Construtor ergonómico preservado: `Content::repeat(body, gap, justify)`.

> **`Hash` manual via Debug** (precedente Lote 4/5): `Length` carrega `f64` e não
> implementa `Hash` → `impl Hash { format!("{self:?}")… }` (paridade
> `content_hash`). `PartialEq` deriva.

## `impl Element`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1829`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1659`) |
| `map_content` | **recursivo** no body, preserva `gap`/`justify` (`content.rs:2207`) |
| `map_text` | **recursivo** no body (`content.rs:2491`), preserva `gap`/`justify` |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `body + gap + justify` (paridade `content.rs:2016`).
