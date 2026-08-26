# Prompt L0 — `entities/elements/list_item` — `ListItemElem`
Hash do Código: 1d12729a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/list_item.rs`
**Origem**: modelo D (ADR-0105), **Lote 3 P318** (família lista/termos). Campo
`marker` adicionado em **P470**; campos `indent`/`body_indent`/`tight` adicionados
em **P505**. Trait, regras partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`.
**Não-locatável** (confirmado P318).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ListItemElem {
    pub body:         Content,
    pub marker:       Option<ListMarker>,  // None = bullet padrão "•"
    /// **P504** — alinhamento do marker (Typst 0.15.0). Layout real é
    /// scope-out; o campo é aceite e preservado sem efeito visual.
    pub marker_align: Option<Align2D>,
    /// **P505** — indentação do marker em relação à margem esquerda.
    pub indent:       Option<Length>,
    /// **P505** — indentação do corpo do item em relação ao marker.
    pub body_indent:  Option<Length>,
    /// **P505** — `false` adiciona espaçamento de parágrafo entre itens;
    /// `true` (default) mantém os itens justapostos.
    pub tight:        Option<bool>,
}
```

`Content::ListItem(Arc<ListItemElem>)`.

Construtor sem marcador (retrocompatível):
`Content::list_item(body: Content)` → `ListItemElem { body, marker: None, marker_align: None, indent: None, body_indent: None, tight: None }`.

Construtor com marcador (P470):
`Content::list_item_with_marker(body: Content, marker: ListMarker)`
→ `ListItemElem { body, marker: Some(marker), marker_align: None, indent: None, body_indent: None, tight: None }`.

Construtor completo (P504/P505):
`Content::list_item_full(body: Content, marker: Option<ListMarker>, marker_align: Option<Align2D>, indent: Option<Length>, body_indent: Option<Length>, tight: Option<bool>)`
→ `ListItemElem { body, marker, marker_align, indent, body_indent, tight }`.

## `impl Element for ListItemElem`

| método | comportamento |
|---|---|
| `plain_text` | `format!("{} {}", self.marker.as_ref().map(|m| m.render()).unwrap_or("•"), self.body.plain_text())` |
| `is_empty` | default `false` |
| `map_content` | **recursivo** no body; **preserva** `marker`, `marker_align`, `indent`, `body_indent`, `tight` |
| `map_text` | **recursivo** no body; **preserva** `marker`, `marker_align`, `indent`, `body_indent`, `tight` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + marker + marker_align + indent + body_indent + tight`.

## Nota arquitetural P505

No Typst vanilla `indent`/`body-indent`/`tight` são propriedades do container
`list`. No cristalino a função `list(...)` expande para uma `Sequence` de
`ListItemElem`; os valores do container são **replicados em cada item**.
Esta é uma divergência mecânica intencional (ADR-0107): a paridade é com a
**linguagem** (resultado visual e semântica dos argumentos), não com a estrutura
interna de dados.

## Critério

- `plain_text` de `ListItemElem { body: "um", marker: None, marker_align: None, indent: None, body_indent: None, tight: None }` == `"• um"`.
- `plain_text` de `ListItemElem { body: "um", marker: Some(Custom("→")), ... }` == `"→ um"`.
- `map_content`/`map_text` recursam no body e preservam `marker`, `marker_align`, `indent`, `body_indent`, `tight`.
- Igualdade estrutural inclui `indent`, `body_indent`, `tight`.
- `Content::list_item_full` cria item com todos os campos propagados.
