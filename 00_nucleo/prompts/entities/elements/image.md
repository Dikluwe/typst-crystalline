# Prompt L0 — `entities/elements/image` — `ImageElem`
Hash do Código: 490eb825

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/image.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P322). Comportamento
idêntico ao braço atual.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ImageElem {
    pub path:   String,
    pub data:   PtrEqArc<Vec<u8>>,
    pub width:  Option<Box<Value>>,
    pub height: Option<Box<Value>>,
}
```

`Content::Image { path, data, width, height }` → `Content::Image(Arc<ImageElem>)`.
Construtor ergonómico: `Content::image(path, data, width, height)`.
`PtrEqArc`/`Value` de `entities`.

> **`Hash` manual via Debug** (precedente Lote 4/6): `Value` carrega `f64` e
> `PtrEqArc` não implementam `Hash` → `impl Hash { format!("{self:?}")… }`
> (paridade `content_hash`). **`PartialEq` deriva** (`PtrEqArc` tem `impl
> PartialEq` via ptr_eq; `Value` tem `PartialEq`).

## `impl Element`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1740`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (folha) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `path + data + width + height` (paridade
`content.rs:1896`; `data` via `PtrEqArc` ptr_eq).
