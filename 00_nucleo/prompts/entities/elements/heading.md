# Prompt L0 — `entities/elements/heading` — `HeadingElem`
Hash do Código: 184b6999

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/heading.rs`
**Origem**: modelo D (ADR-0105), lote piloto P316. Trait e regras partilhadas:
ver `entities/elements/_comum.md`. É o **teto** do piloto (locatável — prova a
absorção de `ElementKind`/`ElementPayload`; se quebrar, **parar e voltar ao L0**).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HeadingElem {
    pub level: u8,        // clamped 1..=6
    pub body:  Content,   // era Box<Content>; agora Content dentro do Arc
    pub outlined: bool,   // P493 — visível no outline; default true (paridade vanilla)
}
```

O `Content` passa de `Heading { level, body }` para `Heading(Arc<HeadingElem>)`.
Construtor preserva o clamp: `level.clamp(1, 6)` (`content.rs:1267`).

## `impl Element for HeadingElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1573`) |
| `is_empty` | `false` (structural; comportamento atual do braço Heading) |
| `map_content` | `Content::Heading(Arc::new(HeadingElem { level: self.level, outlined: self.outlined, body: self.body.map_content(f)? }))` |
| `map_text` | `Content::Heading(Arc::new(HeadingElem { level: self.level, outlined: self.outlined, body: self.body.map_text(f) }))` |
| `get_field` | `"body"` → `Some(Value::Content(self.body.clone()))`; `"level"` → `Some(Value::Int(self.level as i64))`; `"outlined"` → `Some(Value::Bool(self.outlined))`; outro → `None` (`content.rs:2021-2022`) |
| `element_kind` | `Some(ElementKind::Heading)` — **absorção** (era `element_kind.rs`) |
| `to_payload` | `Some(ElementPayload::Heading { depth: self.level, body_hash: hash_content(&self.body), counter_update: CounterUpdate::Step })` — **absorção** (era `extract_payload.rs:21`) |

## Absorção do locatável (A.1.3)

- `extract_payload.rs` arm `Content::Heading {..}` → `Content::Heading(h) => h.to_payload()`.
- `ElementKind`/`ElementPayload` **permanecem** (outras variantes ainda usam);
  só o braço Heading migra. Estado misto esperado (ADR-0105).

## Layout (inalterado, em `rules/`)

`engine/layout/mod.rs:669` arm `Content::Heading(h) => …` usa `h.level`/`h.body`
— mesma lógica, só destructuring.

## Critério

`plain_text`/`get_field`/`map_*` iguais ao braço inline; `to_payload()` produz
o mesmo `ElementPayload::Heading` que `extract_payload` produzia (test
`heading_produz_some_payload` passa sem alteração).
