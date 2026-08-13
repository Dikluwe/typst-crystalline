# Prompt L0 — `entities/elements/image` — `ImageElem`
Hash do Código: 003f6fcd

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/image.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait e glossário (§A.0): ver
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
    pub fit:    EcoString,
}
```

`Content::Image { path, data, width, height, fit }` → `Content::Image(Arc<ImageElem>)`.
Construtor ergonómico: `Content::image(path, data, width, height, fit)`.
`fit` default `"cover"`; valores válidos `"contain"`, `"cover"`, `"stretch"`.
(P502 — paridade `image.fit` do vanilla 0.14.2; renderização do fit scope-out per ADR-0054.)
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
