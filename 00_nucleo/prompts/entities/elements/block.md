# Prompt L0 — `entities/elements/block` — `BlockElem`
Hash do Código: 20c6319f

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/block.rs`
**Origem**: modelo D (ADR-0105), **Lote 15 P330** (lote tardio da triagem
DEBT-58; **o último lote do roteiro**). Trait: ver `entities/elements/_comum.md`.
**Não-locatável**. Contentor — `map_*` recursam no `body`. A **variante mais
densa** do roteiro (14 campos); família L12/`Boxed`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BlockElem {
    pub body:      Content,            // era Box<Content>
    pub width:     Option<Length>,
    pub height:    Option<Length>,
    pub inset:     Sides<Length>,
    pub breakable: bool,
    pub outset:    Sides<Length>,
    pub radius:    Corners<Length>,
    pub clip:      bool,
    pub fill:      Option<Color>,
    pub stroke:    Option<Stroke>,
    pub spacing:   Option<Length>,
    pub above:     Option<Length>,
    pub below:     Option<Length>,
    pub sticky:    bool,
}
```

`Content::Block { … }` → `Content::Block(Arc<BlockElem>)`. Construtor ergonómico
preservado: `Content::block(body, width, height, inset, breakable)` — **cobre só
5 dos 14 campos** (defaults outset/radius/clip/fill/stroke/spacing/above/below/
sticky). Logo as construções de campo-completo (materialize `..(**e).clone()`,
stdlib, testes) usam **Arc-wrap** (regra C2): `Content::Block(Arc::new(BlockElem
{…}))`.

> **`Hash` manual via Debug**: `Length`/`Sides<Length>`/`Corners<Length>`/`Color`/
> `Stroke` carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for BlockElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1737`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1604`) |
| `map_content` | **recursivo** no `body`, preserva os 13 cosméticos (`content.rs:1998`) |
| `map_text` | **recursivo** no `body`, preserva os 13 cosméticos (`content.rs:2166`) |
| `get_field`/`element_kind`/`to_payload` | default |

> **Introspecção**: o walk arm (`introspect.rs:1225`) só **desce no body** — não
> consome `Content::Block` para payload (contraste com `Labelled`/P195D). A
> migração só converte o lado `Content::Block` dos arms (walk, materialize).

## `eq`

`#[derive(PartialEq)]` compara os 14 campos (paridade `content.rs:1864`).
