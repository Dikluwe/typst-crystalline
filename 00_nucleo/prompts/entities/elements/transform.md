# Prompt L0 — `entities/elements/transform` — `TransformElem`
Hash do Código: 6c5a47d7

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/transform.rs`
**Origem**: modelo D (ADR-0105), **Lote 9 P324** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TransformElem {
    pub matrix: TransformMatrix,
    pub body:   Content,            // era Box<Content>
}
```

`Content::Transform { matrix, body }` → `Content::Transform(Arc<TransformElem>)`.
Construtor ergonómico: `Content::transform(matrix, body)`.

> **`Hash` manual via Debug**: `TransformMatrix` é 6×`f64` (`a,b,c,d,tx,ty`) e
> não implementa `Hash` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for TransformElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1767`) |
| `is_empty` | **default `false`** — `Transform` cai em `_ => false` no hub (`content.rs:1685`); **não delega** a `body.is_empty()`. Preservar (não override). |
| `map_content` | **recursivo** no `body`, preserva `matrix` (`content.rs:2249`) |
| `map_text` | **recursivo** no `body`, preserva `matrix` (`content.rs:2531`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `matrix`/`body` (paridade `content.rs:1916`).
