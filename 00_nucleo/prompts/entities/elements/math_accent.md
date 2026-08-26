# Prompt L0 — `entities/elements/math_accent` — `MathAccentElem`
Hash do Código: 5c99c7dd

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_accent.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Mecanismo vanilla:
`AccentElem` minimal (P296; cosméticos `size`/`dotless` scope-out, inalterado).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAccentElem {
    pub base:   Content,   // era Box<Content>
    pub accent: Content,   // era Box<Content>
}
```

`Content::MathAccent { base, accent }` → `Content::MathAccent(Arc<MathAccentElem>)`.
Construtor ergonómico: `Content::math_accent(base: Content, accent: Content)`.

## `impl Element for MathAccentElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("{}{}", self.base.plain_text(), self.accent.plain_text())` (`content.rs:1645`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `base`/`accent` (`content.rs:2131`) |
| `map_text` | **terminal** (math structural; `content.rs:2631`): `Content::MathAccent(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `base + accent` (paridade `content.rs:1831`).

## Critério

`plain_text` concat `base+accent`; `map_content` recurse ambos; `map_text`
terminal; igualdade estrutural.
