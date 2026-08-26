# Prompt L0 — `entities/elements/terms` — `TermsElem`
Hash do Código: 667514fc

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/terms.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Trait, regras partilhadas
e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P318). Comportamento idêntico ao braço atual do hub. Contém
tipicamente `Content::TermItem`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TermsElem {
    pub items: Vec<Content>,
}
```

`Content::Terms { items }` → `Content::Terms(Arc<TermsElem>)`.
Construtor ergonómico: `Content::terms(items: Vec<Content>)`.

## `impl Element for TermsElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `items` `plain_text` `join("\n")` (`content.rs:1754`) |
| `is_empty` | `self.items.is_empty()` (**override**; `content.rs:1570`) |
| `map_content` | **recursivo** em cada item (`content.rs:2114`) |
| `map_text` | **recursivo** em cada item (container; `content.rs:2450`) |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `items` (paridade `content.rs:1934`).

## Critério

`plain_text` junta itens com `\n`; `is_empty` quando sem itens; `map_*` recursam
em cada item; igualdade estrutural.
