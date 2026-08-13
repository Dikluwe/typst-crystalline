# Prompt L0 — `entities/elements/math_styled` — `MathStyledElem`
Hash do Código: 3b81457b

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_styled.rs`
**Origem**: modelo D (ADR-0105), lote piloto P316. Trait, regras partilhadas e glossário (§A.0):
ver `entities/elements/_comum.md`. É o **meio** do piloto (campos + math layout;
os prompts stdlib/layout math já são finos desde P314). Mecanismo do variant:
ADR-0102/0103.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathStyledElem {
    pub kind:    Option<MathStyleKind>,
    pub bold:    Option<bool>,
    pub italic:  Option<bool>,
    pub body:    Content,        // era Box<Content>
    pub cramped: Option<bool>,
}
```

O `Content` passa de `MathStyled { kind, bold, italic, body, cramped }` para
`MathStyled(Arc<MathStyledElem>)`. Campos agrupados (compat-F, A.1.4).

## `impl Element for MathStyledElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` — transparente (`content.rs:1642`) |
| `is_empty` | default `false` |
| `map_content` | recursivo no body preservando kind/flags: `Content::MathStyled(Arc::new(MathStyledElem { kind: self.kind, bold: self.bold, italic: self.italic, body: self.body.map_content(f)?, cramped: self.cramped }))` (`content.rs:2145`) |
| `map_text` | **terminal** (math structural; `content.rs:2636`): devolve `Content::MathStyled(Arc::new(self.clone))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural (A.1.1.b)

`#[derive(PartialEq)]` compara `kind + bold + italic + body + cramped`
(paridade `content.rs:1826`). Via `Arc<MathStyledElem>: PartialEq` → estrutural.

## Layout (inalterado, em `rules/math/layout`)

`rules/math/layout/mod.rs:336` (handler) + `apply_math_style` (`:695`, composição
outer-wins via `Option::or`) usam os campos do elemento — **mesma lógica**, só
destructuring de `Arc<MathStyledElem>`. Ver `rules/math/layout/_comum.md`
(handler `Content::MathStyled`).

## Critério

`map_content` recurse e preserva kind/flags; `map_text` terminal; igualdade
estrutural; suíte math layout (~58 tests) passa sem alteração.
