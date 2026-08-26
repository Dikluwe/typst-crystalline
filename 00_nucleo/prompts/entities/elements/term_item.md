# Prompt L0 — `entities/elements/term_item` — `TermItemElem`
Hash do Código: ebcdee0a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/term_item.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Trait, regras partilhadas
e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P318). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TermItemElem {
    pub term:        Content,   // era Box<Content>
    pub description: Content,   // era Box<Content>
}
```

`Content::TermItem { term, description }` → `Content::TermItem(Arc<TermItemElem>)`.
Construtor ergonómico: `Content::term_item(term: Content, description: Content)`.

## `impl Element for TermItemElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("{}: {}", term.plain_text(), description.plain_text())` (`content.rs:1758`) |
| `is_empty` | `self.term.is_empty() && self.description.is_empty()` (**override**; `content.rs:1571`) |
| `map_content` | **recursivo** em `term` e `description` (`content.rs:2119`) |
| `map_text` | **recursivo** em `term` e `description` (container; `content.rs:2453`) |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `term + description` (paridade `content.rs:1935`).

## Critério

`plain_text` `term: description`; `is_empty` quando ambos vazios; `map_*` recursam
em ambos; igualdade estrutural.
