# Prompt L0 — `entities/elements/math_class_override` — `MathClassOverrideElem`
Hash do Código: 91ed513c

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_class_override.rs`
**Origem**: modelo D (ADR-0105), **P772y** (`math.class(class, body)`). Trait e
regras partilhadas: ver `entities/elements/_comum.md`. **Não-locatável**
(paralelo P296/P298 — `MathAccent`/`MathCancel`/`MathOp`). Mecanismo vanilla:
`ClassElem` (`math/mod.rs`) — `#[elem(Mathy)] pub struct ClassElem { #[required]
pub class: MathClass, #[required] pub body: Content }`.

---

## Contexto

`math.class(class, body)` força a `MathClass` de `body` para efeitos de
espaçamento automático entre símbolos matemáticos adjacentes — override do
valor inferido automaticamente por `default_math_class`/
`rules/math/layout/spacing.rs::node_math_class`. Não afecta o layout/
renderização do `body` em si (que continua a ser layoutado normalmente via
`MathLayouter::layout_node` — ver `rules/math/layout/_comum.md`).

Caso de uso canónico (P772w, achado original): definir um símbolo
personalizado que se comporta tipograficamente como uma relação, por
exemplo `#let loves = math.class("relation", sym.suit.heart)`, de modo que
`$x loves y$` produza o mesmo espaçamento (THICK) que `$x = y$`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathClassOverrideElem {
    pub class: MathClass,
    pub body:  Content,
}
```

`Content::MathClassOverride(Arc<MathClassOverrideElem>)`.
Construtor ergonómico: `Content::math_class_override(class: MathClass, body: Content)`.

## `impl Element for MathClassOverrideElem`

| método | comportamento |
|---|---|
| `plain_text` | `self.body.plain_text()` |
| `is_empty` | `self.body.is_empty()` (delega — diferente do default `false` de `MathCancelElem`, pois `class` sem `body` visível não é conteúdo estrutural) |
| `map_content` | **recursivo** no `body`: `Content::MathClassOverride(Arc::new(MathClassOverrideElem { class: self.class, body: self.body.map_content(f)? }))` |
| `map_text` | **terminal** (paralelo `MathCancel`/`MathStyled` — família math não desce em `map_text`; `MathClassOverrideElem::map_text` existe pelo contrato do trait mas o dispatcher central `Content::map_text` clona directamente, tal como os restantes membros da família math) |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `class` e `body`.

## Despacho no hub (`content.rs`)

Mesmo padrão dos restantes membros da família math (mirror de `MathCancel`):
Debug/Display (`"math.class({:?})"`), constructor, `plain_text`, `PartialEq`,
`map_content` (recursivo), `map_text` (terminal — bloco de clonagem directa).
Também precisa de braço em `rules/introspect.rs` (2×, não-locatável/terminal),
`rules/introspect/locatable.rs` (não-locatável), `rules/layout/mod.rs`
(fallback de math fora de contexto — `layout_math_fallback`),
`rules/eval/repr.rs` (`class("<nome>", <repr(body)>)`) e
`03_infra/src/query_helpers.rs` (2×, terminal sem texto próprio).

## Critério

`plain_text`/`is_empty` transparentes ao body; `map_content` recurse o body
preservando `class`; `map_text` terminal (clona directamente, class
preservada); igualdade estrutural compara `class` e `body`.
