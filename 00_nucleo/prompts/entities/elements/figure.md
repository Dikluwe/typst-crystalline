# Prompt L0 — `entities/elements/figure` — `FigureElem`
Hash do Código: 8937a844

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/figure.rs`
**Origem**: modelo D (ADR-0105), **Lote 13 P328** (o último element-shaped).
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. Contentor — `map_*` recursam em
`body` **e** `caption` (precedente `Quote` L8; **simétrico**, sem assimetria).

> **Fronteira: LOCATÁVEL** (M1, junto de Heading/Cite). Absorve o braço de
> `extract_payload` no trait (precedente Heading/Lote 6): `element_kind` →
> `ElementKind::Figure`, `to_payload` → `ElementPayload::Figure { kind,
> counter_update: Step, is_counted }`. O consumo por `ElementPayload::Figure`
> (`from_tags`/walk) é **inalterado** (matcheia o payload, não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FigureElem {
    pub body:      Content,             // era Box<Content>
    pub caption:   Option<Content>,     // era Option<Box<Content>>
    pub kind:      Option<String>,
    pub numbering: Option<String>,
}
```

`Content::Figure { body, caption, kind, numbering }` →
`Content::Figure(Arc<FigureElem>)`. Construtor ergonómico (cobre os 4 campos
— logo o transformador usa `Content::figure(…)`, não Arc-wrap):
`Content::figure(body, caption, kind, numbering)`. **Deriva `Hash`** (`Content`
tem `impl Hash` manual; `Option<String>` deriva).

## `impl Element for FigureElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | **custom** (`content.rs:1706`): junta `body.plain_text()` e `caption.plain_text()` com espaço; cada um omitido se vazio |
| `is_empty` | **override**: `self.body.is_empty() && self.caption.as_ref().is_none_or(\|c\| c.is_empty())` (`content.rs:1573`) |
| `map_content` | **recursivo** em `body` e `caption`, preserva `kind`/`numbering` (`content.rs:1996`) |
| `map_text` | **recursivo** em `body` e `caption`, preserva `kind`/`numbering` (`content.rs:2206`) — **simétrico** com `map_content` |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Figure)` |
| `to_payload` | `Some(ElementPayload::Figure { kind: self.kind.clone(), counter_update: CounterUpdate::Step, is_counted: self.numbering.is_some() && self.caption.is_some() })` (absorve `extract_payload.rs:24`) |

## `eq`

`#[derive(PartialEq)]` compara os 4 campos (paridade `content.rs:1837`).

## Interação `figure_image.rs`

`infer_kind_from_body` (tocado no Lote 7) matcheia o **body** (`Content::Image(_)`/
`Content::Raw(_)`) — **inalterado** (matcheia o conteúdo, não `Figure`). Só a
**construção** em `native_figure` (`figure_image.rs:92`) muda para
`Content::figure(…)`.
