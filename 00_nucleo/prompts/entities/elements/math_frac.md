# Prompt L0 — `entities/elements/math_frac` — `MathFracElem`
Hash do Código: 73155b7b

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_frac.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathFracElem {
    pub num: Content,   // era Box<Content>
    pub den: Content,   // era Box<Content>
}
```

`Content::MathFrac { num, den }` → `Content::MathFrac(Arc<MathFracElem>)`.
Construtor ergonómico: `Content::math_frac(num: Content, den: Content)`.

## `impl Element for MathFracElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("({})/({})", self.num.plain_text(), self.den.plain_text())` (`content.rs:1613`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `num`/`den` (`content.rs:2096`): `Content::MathFrac(Arc::new(MathFracElem { num: self.num.map_content(f)?, den: self.den.map_content(f)? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2621`): `Content::MathFrac(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `num + den` (paridade `content.rs:1815`).

## Critério

`plain_text` formato `(num)/(den)`; `map_content` recurse ambos; `map_text`
terminal; igualdade estrutural.
