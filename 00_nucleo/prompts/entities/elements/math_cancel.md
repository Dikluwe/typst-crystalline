# Prompt L0 — `entities/elements/math_cancel` — `MathCancelElem`
Hash do Código: a68839da

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_cancel.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Mecanismo vanilla:
`CancelElem` minimal (P296; cosméticos `length`/`inverted`/`cross`/`angle`/
`stroke` scope-out, inalterado).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathCancelElem {
    pub body: Content,   // era Box<Content>
}
```

`Content::MathCancel { body }` → `Content::MathCancel(Arc<MathCancelElem>)`.
Construtor ergonómico: `Content::math_cancel(body: Content)`.

## `impl Element for MathCancelElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1646`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** no `body` (`content.rs:2135`): `Content::MathCancel(Arc::new(MathCancelElem { body: self.body.map_content(f)? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2632`): `Content::MathCancel(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1833`).

## Critério

`plain_text` transparente ao body; `map_content` recurse o body; `map_text`
terminal; igualdade estrutural.
