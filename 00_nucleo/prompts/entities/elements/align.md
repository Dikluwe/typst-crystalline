# Prompt L0 — `entities/elements/align` — `AlignElem`
Hash do Código: 2cda91eb

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/align.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P322). Contentor de
prosa — `map_*` recursam no body (precedente Heading/Lote 3).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct AlignElem {
    pub alignment: Align2D,
    pub body:      Content,   // era Box<Content>
}
```

`Content::Align { alignment, body }` → `Content::Align(Arc<AlignElem>)`.
Construtor ergonómico: `Content::align(alignment, body)`. `Align2D` de
`entities::layout_types`.

> **`Hash` manual via Debug** (regra do modelo): `Align2D` deriva
> `Debug, Clone, Copy, PartialEq, Default` — **sem `Hash`** → `impl Hash {
> format!("{self:?}")… }` (paridade `content_hash`). `PartialEq` deriva.

## `impl Element`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1798`) |
| `is_empty` | default `false` (Align **não** delega — content-preserving) |
| `map_content` | **recursivo** no body, preserva `alignment` (`content.rs:2356`) |
| `map_text` | **recursivo** no body (`content.rs:2640`), preserva `alignment` |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `alignment + body` (paridade `content.rs:1958`).
