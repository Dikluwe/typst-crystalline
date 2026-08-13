# Prompt L0 — `entities/elements/place` — `PlaceElem`
Hash do Código: 761d2d80

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/place.rs`
**Origem**: modelo D (ADR-0105), **Lote 9 P324** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceElem {
    pub alignment: Align2D,
    pub dx:        f64,
    pub dy:        f64,
    pub scope:     PlaceScope,
    pub float:     bool,
    pub clearance: Option<Length>,
    pub body:      Content,            // era Box<Content>
}
```

`Content::Place { alignment, dx, dy, scope, float, clearance, body }` →
`Content::Place(Arc<PlaceElem>)`. Construtor ergonómico: `Content::place(...)`
(7 campos + body, na ordem do enum).

> **`Hash` manual via Debug**: `dx`/`dy` são `f64`; `Align2D` e
> `Option<Length>` também não implementam `Hash` → `impl Hash {
> format!("{self:?}").hash(state) }` (paridade `content_hash`; ressalva
> `-0.0`≠`0.0`). `PartialEq` deriva (todos os campos implementam `PartialEq`).

## `impl Element for PlaceElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1824`) |
| `is_empty` | **default `false`** — `Place` cai em `_ => false` no hub; **não delega**. Preservar. |
| `map_content` | **recursivo** no `body`, preserva os 6 campos cosméticos (`content.rs:2346`) |
| `map_text` | **recursivo** no `body`, preserva os 6 campos (`content.rs:2612`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 7 campos (paridade `content.rs:1974`).
