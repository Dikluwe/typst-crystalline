:warning: **Prompt L0 — `entities/elements/equation` — `EquationElem`**
Hash do Código: d3748575

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/equation.rs`
**Origem**: modelo D (ADR-0105), **Lote 10 P325** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. Contentor **assimétrico** — ver `map_*`.

> **Fronteira: LOCATÁVEL** (P186B, M6 eixo 2 ADR-0068). Absorve o braço de
> `extract_payload` no trait (precedente Heading/Lote 6): `element_kind` →
> `ElementKind::Equation`, `to_payload` → `ElementPayload::Equation { block,
> counter_update: Step }`. O consumo por `ElementPayload` (`from_tags` arm
> Equation, gate `block && numbering_active:equation`) é **inalterado**
> (matcheia o payload, não o `Content`).
>
> **P456 (Numeração de equações):** o padrão de numeração vive **só na chain**
> (`custom("equation.numbering") = Value::Str(pattern)`), transportado por
> `Content::Styled` — análogo a `figure.numbering` (P365/P454). `EquationElem`
> **não** tem campo `numbering`; o gate efetivo é `block && pattern.is_some()`
> no consumidor (`introspect.rs` + `layout/equation.rs`).

> **Fonte de paridade (P1031)** — doc comments do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/math/equation.rs`, publicados em
> `typst.app/docs/reference/math/equation/`:
>
> - **O gate `block &&` é literal.** `equation.rs:63-64`, campo `numbering`:
>   *"How to number **block-level** equations. Accepts a numbering pattern or function
>   taking a single number."* A restrição a block-level está no próprio texto da
>   documentação — não é inferência do cristalino.
> - **`block`** — `equation.rs:59-61`: *"Whether the equation is displayed as a separate
>   block."*, `#[default(false)]`. Confere com o campo `pub block: bool` do struct.
> - **`pattern.is_some()`** — `pub numbering: Option<Numbering>` (`equation.rs:75`); o
>   exemplo do doc comment (`equation.rs:66-74`) mostra a forma de superfície
>   `#set math.equation(numbering: "(1)")`, que é o que a chain transporta.
>
> **Divergência mecânica declarada (não é achado)**: o vanilla guarda `numbering` como campo
> do `EquationElem`; o cristalino guarda-o só na chain. É estrutura de dados, logo diverge
> de propósito (ADR-0107) desde que a superfície `#set math.equation(numbering: …)` e o gate
> block-level se mantenham. O que a documentação sustenta é o **gate**, não a localização do
> dado — distinção registada para não a confundir com paridade estrutural.

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

Forma de transporte numerada (P364/P456):
`Content::equation_numbered(body, block)` produz `Content::Styled` com
`custom("equation.numbering", Value::Str("(1)"))`.

## P1140.3-B — produtor público e incompletude faseada

O elemento passa a ter dois produtores públicos da mesma morfologia: a
sintaxe `$...$` e `math.equation(body, block: false)`. Ambos delegam a
`Content::equation(body, block)`. `body` é `Content` posicional obrigatório;
`block` é booleano e tem default `false`. O `repr` omite `block` quando falso
e o inclui antes de `body` quando verdadeiro.

Esta fase é **deliberadamente incompleta**: `numbering`, `number-align`,
`supplement` e `alt` serão completados pelo **P1140.4**, com representação e
consumidores próprios. Até lá, a nativa deve rejeitá-los explicitamente; não
pode descartá-los nem assá-los no `body`. O transporte de `numbering` por
`Content::Styled` para set-rules permanece uma divergência mecânica autorizada.

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
