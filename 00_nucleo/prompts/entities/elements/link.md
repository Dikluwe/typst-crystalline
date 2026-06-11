# Prompt L0 — `entities/elements/link` — `LinkElem`
Hash do Código: 384cf9e4

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/link.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Trait e
regras partilhadas: ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P318). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinkElem {
    pub url:  EcoString,
    pub body: Content,        // era Box<Content>
}
```

`Content::Link { url, body }` → `Content::Link(Arc<LinkElem>)`.
Construtor ergonómico: `Content::link(url: impl Into<EcoString>, body: Content)`.

## `impl Element for LinkElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` — transparente (`content.rs:1637`) |
| `is_empty` | default `false` |
| `map_content` | **recursivo** no body, preserva `url` (`content.rs:2070`) |
| `map_text` | **recursivo** no body (container; `content.rs:2444`), preserva `url` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `url + body` (paridade `content.rs:1820`).

## Critério

`plain_text` transparente ao body; `map_content`/`map_text` recursam no body
preservando `url`; igualdade estrutural.
