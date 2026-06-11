# Prompt L0 — `entities/elements/math_underover` — `MathUnderoverElem`
Hash do Código: e4b4c104

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_underover.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait e regras
partilhadas: ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Variant agregado P297
(ADR-0054 graded; cristalino unifica o cluster vanilla under/over).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderoverElem {
    pub base:  Content,            // era Box<Content>
    pub under: Option<Content>,    // era Option<Box<Content>>
    pub over:  Option<Content>,    // era Option<Box<Content>>
}
```

`Content::MathUnderover { base, under, over }` →
`Content::MathUnderover(Arc<MathUnderoverElem>)`. Construtor ergonómico:
`Content::math_underover(base: Content, under: Option<Content>, over: Option<Content>)`.

## `impl Element for MathUnderoverElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | ordem visual `over + base + under` (`Option` vazio = `""`; `content.rs:1648`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** nos 3 campos preservando `None` (`content.rs:2139`): `base.map_content(f)?`; `under`/`over` via `.as_ref().map(map_content).transpose()?` |
| `map_text` | **terminal** (math structural; `content.rs:2635`): `Content::MathUnderover(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `base + under + over` (paridade `content.rs:1835`).

## Critério

`plain_text` `over‖base‖under`; `map_content` recurse os 3 preservando `None`;
`map_text` terminal; igualdade estrutural.
