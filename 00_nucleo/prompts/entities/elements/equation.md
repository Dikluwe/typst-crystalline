# Prompt L0 — `entities/elements/equation` — `EquationElem`
Hash do Código: cace93ea

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/equation.rs`
**Origem**: modelo D (ADR-0105), **Lote 10 P325** (por largura). Trait: ver
`entities/elements/_comum.md`. Contentor **assimétrico** — ver `map_*`.

> **Fronteira: LOCATÁVEL** (P186B, M6 eixo 2 ADR-0068). Absorve o braço de
> `extract_payload` no trait (precedente Heading/Lote 6): `element_kind` →
> `ElementKind::Equation`, `to_payload` → `ElementPayload::Equation { block,
> counter_update: Step }`. O consumo por `ElementPayload` (`from_tags` arm
> Equation, gate `block && numbering_active:equation`) é **inalterado**
> (matcheia o payload, não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EquationElem {
    pub body:  Content,            // era Box<Content>
    pub block: bool,
}
```

`Content::Equation { body, block }` → `Content::Equation(Arc<EquationElem>)`.
Construtor ergonómico: `Content::equation(body, block)`. **Deriva `Hash`**
(`Content` tem `impl Hash` manual; `bool: Hash`).

## `impl Element for EquationElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | **custom** (`content.rs:1736`): `if self.block { format!("\n{}\n", body.plain_text()) } else { body.plain_text() }` |
| `is_empty` | **default `false`** — `Equation` cai em `_ => false` no hub; **não delega** ao body. Preservar (não override). Precedente `Transform`/`Place` L9. |
| `map_content` | **recursivo** no `body`, preserva `block` (`content.rs:2107`) |
| `map_text` | **TERMINAL** — `Equation` está no arm `\|`-combinado de `map_text` (`content.rs:2466`): math structural não desce em texto. **Assimetria preservada**: `map_content` recursa, `map_text` devolve `self`. |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Equation)` |
| `to_payload` | `Some(ElementPayload::Equation { block: self.block, counter_update: CounterUpdate::Step })` (absorve `extract_payload.rs:90`) |

> **Assimetria `map_content` ≠ `map_text`**: única no roteiro até agora. O hub
> tinha arm próprio em `map_content` (recursa) e `Equation` no `\|`-terminal de
> `map_text`. O `…Elem` materializa ambos; o arm de `map_text` no hub
> permanece **combinado** (`{ .. }` → `(_)`, sem split — sem binding).

## `eq`

`#[derive(PartialEq)]` compara `body`/`block` (paridade `content.rs:1883`).
