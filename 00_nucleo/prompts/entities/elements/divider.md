# Prompt L0 — `entities/elements/divider` — `DividerElem`
Hash do Código: 14c39ac1

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/divider.rs`
**Origem**: modelo D (ADR-0105), lote piloto P316. Trait, regras partilhadas e glossário (§A.0):
ver `entities/elements/_comum.md`. É o **piso** do piloto (singleton sem campos
— prova o mecanismo).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct DividerElem;   // singleton estrutural (P154B); sem campos
```

O `Content` passa de `Divider` (unit variant) para `Divider(Arc<DividerElem>)`.

## `impl Element for DividerElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` — structural sem texto (`content.rs:1726`) |
| `is_empty` | `false` — singleton estrutural, **nunca vazio** (`content.rs:1512`) |
| `map_content` | terminal: devolve `Content::Divider(Arc::new(DividerElem))` (sem filhos) |
| `map_text` | terminal: devolve `Content::Divider(Arc::new(DividerElem))` |
| `get_field` | default `None` (sem campos nomeados) |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## Layout (inalterado, em `rules/`)

`compiler/layout/mod.rs` arm `Content::Divider(_)` emite
`FrameItem::Shape::Line` 0.5pt (P154B) — **mesma lógica**, só muda o
destructuring do braço.

## Critério

`Content::Divider(_).plain_text() == ""`; `!is_empty()`;
`Content::Divider(a) == Content::Divider(b)` sempre (singleton).
