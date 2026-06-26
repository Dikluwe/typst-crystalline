# Prompt L0 — `entities/elements/list_item` — `ListItemElem`
Hash do Código: (a calcular após P470)

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/list_item.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Campo
`marker` adicionado em **P470**. Trait e regras partilhadas: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P318).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ListItemElem {
    pub body:   Content,
    pub marker: Option<ListMarker>,  // P470: None = bullet padrão "•"
}
```

`Content::ListItem(Arc<ListItemElem>)`.

Construtor sem marcador (retrocompatível):
`Content::list_item(body: Content)` → `ListItemElem { body, marker: None }`.

Construtor com marcador (P470):
`Content::list_item_with_marker(body: Content, marker: ListMarker)`
→ `ListItemElem { body, marker: Some(marker) }`.

## `impl Element for ListItemElem`

| método | comportamento |
|---|---|
| `plain_text` | `format!("{} {}", self.marker.as_ref().map(|m| m.render()).unwrap_or("•"), self.body.plain_text())` |
| `is_empty` | default `false` |
| `map_content` | **recursivo** no body; **preserva** `marker` |
| `map_text` | **recursivo** no body; **preserva** `marker` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + marker`.

## Critério

- `plain_text` de `ListItemElem { body: "um", marker: None }` == `"• um"`.
- `plain_text` de `ListItemElem { body: "um", marker: Some(Custom("→")) }` == `"→ um"`.
- `map_content`/`map_text` recursam no body e preservam `marker`.
- Igualdade estrutural inclui `marker`.
