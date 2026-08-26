# Prompt L0 — `entities/elements/math_attach` — `MathAttachElem`
Hash do Código: bc010300

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_attach.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAttachElem {
    pub base: Content,            // era Box<Content>
    pub tl:   Option<Content>,    // top-left  (pre-superscript)
    pub bl:   Option<Content>,    // bottom-left (pre-subscript)
    pub sub:  Option<Content>,    // bottom-right (subscript)
    pub sup:  Option<Content>,    // top-right (superscript)
}
```

`Content::MathAttach { base, tl, bl, sub, sup }` →
`Content::MathAttach(Arc<MathAttachElem>)`. Construtor ergonómico:
`Content::math_attach(base, tl, bl, sub, sup)`.

## `impl Element for MathAttachElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | concat `^tl _bl base _sub ^sup` (cada opcional só se `Some`; `content.rs:1616`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `base` + cada `Option` (`content.rs:2100`): `base.map_content(f)?`; cada `Option` via `.as_ref().map(map_content).transpose()?` |
| `map_text` | **terminal** (math structural; `content.rs:2622`): `Content::MathAttach(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara os 5 campos (paridade `content.rs:1817`).

## Critério

`plain_text` na ordem `^tl _bl base _sub ^sup`; `map_content` recurse em todos os
slots preservando `None`; `map_text` terminal; igualdade estrutural.
