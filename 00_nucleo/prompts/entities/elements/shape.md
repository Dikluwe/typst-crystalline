# Prompt L0 — `entities/elements/shape` — `ShapeElem`
Hash do Código: 5f932ed4

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/shape.rs`
**Origem**: modelo D (ADR-0105), **Lote 11 P326** (por largura). Trait: ver
`entities/elements/_comum.md`. **Não-locatável**. **Leaf** — `map_*` terminais
(sem body de conteúdo; geometria pura).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeElem {
    pub kind:   ShapeKind,
    pub width:  Option<Box<Value>>,      // mantém Box (precedente Image L7)
    pub height: Option<Box<Value>>,
    pub fill:   Option<Color>,
    pub stroke: Option<Stroke>,
}
```

`Content::Shape { kind, width, height, fill, stroke }` →
`Content::Shape(Arc<ShapeElem>)`. Construtor ergonómico:
`Content::shape(kind, width, height, fill, stroke)`.

> **`Hash` manual via Debug**: `Value` (`width`/`height`) carrega `f64`;
> `Color`/`Stroke` (geometria) idem → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva — `Box`
> compara por conteúdo (equivalente ao `.as_deref()` do hub, `content.rs:1907`).

## `impl Element for ShapeElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` — geometria sem texto (`content.rs:1778`) |
| `is_empty` | default `false` — `Shape` cai em `_ => false` no hub |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field`/`element_kind`/`to_payload` | default |

> **Arm `|`-combinado**: `Shape` está no arm terminal combinado de
> `map_content`/`map_text` com outros leaves. Terminal sem binding → fica **no
> mesmo arm** (`{ .. }` → `(_)`), **sem split**.

## `eq`

`#[derive(PartialEq)]` compara os 5 campos (paridade `content.rs:1905`).
