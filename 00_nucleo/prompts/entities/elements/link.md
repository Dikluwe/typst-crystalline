# Prompt L0 — `entities/elements/link` — `LinkElem`
Hash do Código: e1bc668f

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/link.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Trait, regras partilhadas
e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**
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

## P422 (S) — Render visual de hiperlinks

**Decisão arquitetural (ADR-0107 / ADR-0109):**
- `LinkElem` permanece struct puro (`url`, `body`). Nenhuma lógica de layout é
  adicionada aqui.
- O layout real vive em `compiler/layout/link.rs` como free function `layout_link`
  (forma B).
- O output de layout usa um novo variant `FrameItem::Link { url, items }` em
  `entities/layout_types.rs`, preservando o URL como metadado.

**Scope-out P422:**
- Cor azul / sublinhado no texto — depende de infraestrutura de decoração de
  texto (FrameItem::Decoration) não disponível.
- Consumer PDF que emite annotation URI — scope-out; o `FrameItem::Link` é
  infraestrutura preparatória.
- Links internos (`#link("<label>")`) / inter-página — scope-out.
- Hover tooltip — renderizador-dependente, scope-out.

## Critério

`plain_text` transparente ao body; `map_content`/`map_text` recursam no body
preservando `url`; igualdade estrutural. Layout emite `FrameItem::Link` com body
renderizado e URL preservado.
