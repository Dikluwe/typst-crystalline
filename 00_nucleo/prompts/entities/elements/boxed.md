# Prompt L0 — `entities/elements/boxed` — `BoxedElem`
Hash do Código: de6a7f03

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/boxed.rs`
**Origem**: modelo D (ADR-0105), **Lote 14 P329** (reclassificado da triagem
DEBT-58 — element-shaped denso, não primitivo). Trait: ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
no `body`. Família densa do L12 (box inline container).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BoxedElem {
    pub body:     Content,            // era Box<Content>
    pub width:    Option<Length>,
    pub height:   Option<Length>,
    pub inset:    Sides<Length>,
    pub baseline: Length,
    pub outset:   Sides<Length>,
    pub radius:   Corners<Length>,
    pub clip:     bool,
    pub fill:     Option<Color>,
    pub stroke:   Option<Stroke>,
}
```

`Content::Boxed { … }` → `Content::Boxed(Arc<BoxedElem>)`. Construtor ergonómico
preservado: `Content::boxed(body, width, height, inset, baseline)` — **cobre só
5 dos 10 campos** (defaults outset/radius/clip/fill/stroke). Logo as construções
de campo-completo (materialize, testes) usam **Arc-wrap** (regra C2):
`Content::Boxed(Arc::new(BoxedElem { … }))`.

> **`Hash` manual via Debug**: `Length`/`Sides<Length>`/`Corners<Length>`/`Color`/
> `Stroke` carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for BoxedElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1768`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1635`) |
| `map_content` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2054`) |
| `map_text` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2235`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 10 campos (paridade `content.rs:1904`).
