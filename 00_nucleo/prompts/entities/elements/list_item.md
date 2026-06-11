# Prompt L0 — `entities/elements/list_item` — `ListItemElem`
Hash do Código: 25871fb7

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/list_item.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Trait e
regras partilhadas: ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P318). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ListItemElem {
    pub body: Content,        // era o conteúdo do tuple Box<Content>
}
```

`Content::ListItem(Box<Content>)` → `Content::ListItem(Arc<ListItemElem>)`.
Construtor ergonómico (preserva assinatura): `Content::list_item(body: Content)`.

## `impl Element for ListItemElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("• {}", self.body.plain_text())` (`content.rs:1632`) |
| `is_empty` | default `false` |
| `map_content` | **recursivo** no body (`content.rs:2065`) |
| `map_text` | **recursivo** no body (container; `content.rs:2439`) |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1817`).

## Critério

`plain_text` com bullet `• `; `map_content`/`map_text` recursam no body;
igualdade estrutural.
