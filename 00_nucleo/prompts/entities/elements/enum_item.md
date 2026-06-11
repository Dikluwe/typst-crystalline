# Prompt L0 — `entities/elements/enum_item` — `EnumItemElem`
Hash do Código: d0587998

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/enum_item.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Trait e
regras partilhadas: ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P318: na lista exaustiva não-locatável de `introspect/locatable.rs`;
não é `ElementKind`). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumItemElem {
    pub number: Option<u32>,
    pub body:   Content,        // era Box<Content>
}
```

`Content::EnumItem { number, body }` → `Content::EnumItem(Arc<EnumItemElem>)`.
Construtor ergonómico: `Content::enum_item(number: Option<u32>, body: Content)`.

## `impl Element for EnumItemElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `"{n}. {body}"` com `n` só se `Some` (`content.rs:1633`): `format!("{}{}", number.map(\|n\| format!("{}. ", n)).unwrap_or_default(), body.plain_text())` |
| `is_empty` | default `false` (não está no match de `is_empty`) |
| `map_content` | **recursivo** no body, preserva `number` (`content.rs:2066`) |
| `map_text` | **recursivo** no body (container de prosa — desce, precedente Heading; `content.rs:2440`), preserva `number` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `number + body` (paridade `content.rs:1818`).

## Critério

`plain_text` com prefixo numérico opcional; `map_content`/`map_text` recursam no
body preservando `number`; igualdade estrutural.
