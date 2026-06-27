# Prompt L0 — `entities/elements/enum_item` — `EnumItemElem`
Hash do Código: 5d030429

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/enum_item.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Campo
`numbering` adicionado em **P470**. Trait e regras partilhadas: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P318).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumItemElem {
    pub number:    Option<u32>,
    pub body:      Content,
    pub numbering: Option<EnumNumbering>,  // P470: None = Decimal ("N.")
}
```

`Content::EnumItem(Arc<EnumItemElem>)`.

Construtor sem numbering (retrocompatível):
`Content::enum_item(number: Option<u32>, body: Content)`
→ `EnumItemElem { number, body, numbering: None }`.

Construtor com numbering (P470):
`Content::enum_item_with_numbering(number: Option<u32>, body: Content, numbering: EnumNumbering)`
→ `EnumItemElem { number, body, numbering: Some(numbering) }`.

## `impl Element for EnumItemElem`

| método | comportamento |
|---|---|
| `plain_text` | `format!("{}{}", label, body.plain_text())` onde `label` usa `numbering.as_ref().unwrap_or(&EnumNumbering::Decimal).format(number.unwrap_or(1))` quando `number.is_some()`, senão `""` |
| `is_empty` | default `false` |
| `map_content` | **recursivo** no body; **preserva** `number` e `numbering` |
| `map_text` | **recursivo** no body; **preserva** `number` e `numbering` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

**Nota `plain_text`:** retrocompatibilidade — quando `number` é `Some` e `numbering` é `None`,
usa `Decimal` como antes: `format!("{}. ", n)`. Quando `numbering` é `Some`, usa
`numbering.format(n)`.

## `eq` estrutural

`#[derive(PartialEq)]` compara `number + body + numbering`.

## Critério

- `plain_text` de `{ number: Some(3), body: "x", numbering: None }` == `"3. x"`.
- `plain_text` de `{ number: None, body: "x", numbering: None }` == `"x"`.
- `plain_text` de `{ number: Some(1), body: "a", numbering: Some(LowerAlpha) }` == `"a) a"`.
- `map_content`/`map_text` recursam no body e preservam `number` e `numbering`.
- Igualdade estrutural inclui `numbering`.
