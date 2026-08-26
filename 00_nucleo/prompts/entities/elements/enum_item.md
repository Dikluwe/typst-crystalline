# Prompt L0 — `entities/elements/enum_item` — `EnumItemElem`
Hash do Código: f29b9a3a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/enum_item.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Campo
`numbering` adicionado em **P470**; campos `indent`/`body_indent`/`tight`
adicionados em **P505**. Trait, regras partilhadas e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P318).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumItemElem {
    pub number:       Option<u32>,
    pub body:         Content,
    pub numbering:    Option<EnumNumbering>,  // None = Decimal default
    /// **P505** — indentação do rótulo numérico em relação à margem esquerda.
    pub indent:       Option<Length>,
    /// **P505** — indentação do corpo do item em relação ao rótulo.
    pub body_indent:  Option<Length>,
    /// **P505** — `false` adiciona espaçamento de parágrafo entre itens;
    /// `true` (default) mantém os itens justapostos.
    pub tight:        Option<bool>,
}
```

`Content::EnumItem(Arc<EnumItemElem>)`.

Construtor sem numbering (retrocompatível):
`Content::enum_item(number: Option<u32>, body: Content)`
→ `EnumItemElem { number, body, numbering: None, indent: None, body_indent: None, tight: None }`.

Construtor com numbering (P470):
`Content::enum_item_with_numbering(number: Option<u32>, body: Content, numbering: EnumNumbering)`
→ `EnumItemElem { number, body, numbering: Some(numbering), indent: None, body_indent: None, tight: None }`.

Construtor completo (P505):
`Content::enum_item_full(number: Option<u32>, body: Content, numbering: Option<EnumNumbering>, indent: Option<Length>, body_indent: Option<Length>, tight: Option<bool>)`
→ `EnumItemElem { number, body, numbering, indent, body_indent, tight }`.

## `impl Element for EnumItemElem`

| método | comportamento |
|---|---|
| `plain_text` | `format!("{}{}", label, body.plain_text())` onde `label` usa `numbering.as_ref().unwrap_or(&EnumNumbering::Decimal).format(number.unwrap_or(1))` quando `number.is_some()`, senão `""` |
| `is_empty` | default `false` |
| `map_content` | **recursivo** no body; **preserva** `number`, `numbering`, `indent`, `body_indent`, `tight` |
| `map_text` | **recursivo** no body; **preserva** `number`, `numbering`, `indent`, `body_indent`, `tight` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

**Nota `plain_text`:** retrocompatibilidade — quando `number` é `Some` e `numbering` é `None`,
usa `Decimal` como antes: `format!("{}. ", n)`. Quando `numbering` é `Some`, usa
`numbering.format(n)`.

## `eq` estrutural

`#[derive(PartialEq)]` compara `number + body + numbering + indent + body_indent + tight`.

## Nota arquitetural P505

No Typst vanilla `indent`/`body-indent`/`tight` são propriedades do container
`enum`. No cristalino a função `enum(...)` expande para uma `Sequence` de
`EnumItemElem`; os valores do container são **replicados em cada item**.
Esta é uma divergência mecânica intencional (ADR-0107): a paridade é com a
**linguagem** (resultado visual e semântica dos argumentos), não com a estrutura
interna de dados.

> **Fonte de paridade (P1031)** — a premissa ("são propriedades do container `enum`") é
> literal no vanilla ratificado (`e0e8ca4d`): os três campos estão declarados em
> `crates/typst-library/src/model/enum.rs`, dentro de `pub struct EnumElem` — `tight` em
> `enum.rs:89-90` (`#[default(true)]`), `indent` em `enum.rs:149-150` (*"The indentation of
> each item."*), `body_indent` em `enum.rs:152-154` (*"The space between the numbering and
> the body of each item."*, `#[default(Em::new(0.5).into())]`). O sub-elemento de item
> (`EnumItem`) não os declara. Publicado em `typst.app/docs/reference/model/enum/`.
>
> **A divergência mecânica é legítima; o *valor* replicado é que não é.** A replicação
> container→item é estrutura de dados e diverge de propósito. Mas o default de
> `body_indent` do cristalino é `0pt` e o da linguagem é `0.5em`, o que **é** observável no
> resultado visual — precisamente o critério que este parágrafo invoca. Medido em
> 2026-08-13: documento `+ Um` / (linha em branco) / `+ Dois` dá `1. Um` / `2. Dois` no
> vanilla e `1.Um` / `2.Dois` no cristalino. Achado escalado e detalhado em
> `compiler/layout/enum_item.md` §"Resolve indentação" (mesma causa em
> `compiler/layout/list_item.md`). **Não implementado aqui.**

## Critério

- `plain_text` de `{ number: Some(3), body: "x", numbering: None, ... }` == `"3. x"`.
- `plain_text` de `{ number: None, body: "x", numbering: None, ... }` == `"x"`.
- `plain_text` de `{ number: Some(1), body: "a", numbering: Some(LowerAlpha), ... }` == `"a) a"`.
- `map_content`/`map_text` recursam no body e preservam `number`, `numbering`, `indent`, `body_indent`, `tight`.
- Igualdade estrutural inclui `indent`, `body_indent`, `tight`.
- `Content::enum_item_full` cria item com todos os campos propagados.
