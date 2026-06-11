# Prompt L0 — `entities/elements/math_delimited` — `MathDelimitedElem`
Hash do Código: cf51574d

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_delimited.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait e regras
partilhadas: ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathDelimitedElem {
    pub open:  char,
    pub body:  Content,   // era Box<Content>
    pub close: char,
}
```

`Content::MathDelimited { open, body, close }` →
`Content::MathDelimited(Arc<MathDelimitedElem>)`. Construtor ergonómico:
`Content::math_delimited(open: char, body: Content, close: char)`.

## `impl Element for MathDelimitedElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("{}{}{}", self.open, self.body.plain_text(), self.close)` (`content.rs:1629`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** só no `body`, `open`/`close` preservados (`content.rs:2111`): `Content::MathDelimited(Arc::new(MathDelimitedElem { open: self.open, body: self.body.map_content(f)?, close: self.close }))` |
| `map_text` | **terminal** (math structural; `content.rs:2624`): `Content::MathDelimited(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `open + body + close` (paridade `content.rs:1822`).

## Critério

`plain_text` `{open}{body}{close}`; `map_content` recurse só o body; `map_text`
terminal; igualdade estrutural.
