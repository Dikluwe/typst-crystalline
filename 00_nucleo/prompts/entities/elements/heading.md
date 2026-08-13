# Prompt L0 — `entities/elements/heading` — `HeadingElem`
Hash do Código: 262625ed

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/heading.rs`
**Origem**: modelo D (ADR-0105), lote piloto P316. Trait, regras partilhadas e glossário (§A.0):
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
    pub bookmarked: Option<bool>,  // P606 — None = seguir `outlined` (vanilla `auto`)
    pub set_fields: u8,   // P829 — máscara HEADING_SET_* (ver §P829 abaixo)
}
```

O `Content` passa de `Heading { level, body }` para `Heading(Arc<HeadingElem>)`.
Construtor preserva o clamp: `level.clamp(1, 6)` (`content.rs:1267`).

## §P829 — `set_fields`: campos explicitamente assentes (paridade `has`/`at`/`fields`)

O vanilla distingue campo **assente no constructor** de default resolvido pela
chain — medido (fixtures `temp/p829/`): `heading[H].has("level")` → false,
`heading(level: 2)[H].has("level")` → true, `heading[H].at("level")` → erro
`field "level" in heading is not known at this point and no default was
specified`; heading de markup (`= H`) assenta **`depth`**, não `level`
(`[= H].fields()` → `(depth: 1, body: [H])`).

- Bits: `HEADING_SET_LEVEL` (1) — `level` posicional int ou named `level:`;
  `HEADING_SET_DEPTH` (2) — origem markup (`HeadingElem::new` /
  `new_with_outlined`); `HEADING_SET_OUTLINED` (4) — named `outlined:`.
  `bookmarked` é auto-rastreado pelo `Option`. O `offset` do vanilla é
  scope-out (não modelado; `depth` exposto = `level`).
- Quem assente os bits: `native_heading` (P829 — `compiler/stdlib/structural.rs`)
  via `Content::heading_native`/`heading_numbered_native`. Construtores de
  markup (`new`, `Content::heading`) levam `DEPTH`.
- **Reconstruções preservam a máscara** (`with_body`): `map_content`,
  `map_text`, materialização de tempo (`compiler/introspect.rs`).
- O field access de show rules (`get_field`, `it.level`) **não** consulta a
  máscara — continua a devolver o valor baked (comportamento pré-P829
  inalterado). A máscara só é lida pelos métodos `has`/`at`/`fields`
  (`compiler/eval/bindings.rs`, ver `compiler/eval.md` §P829-B).

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

`compiler/layout/mod.rs:669` arm `Content::Heading(h) => …` usa `h.level`/`h.body`
— mesma lógica, só destructuring.

## Critério

`plain_text`/`get_field`/`map_*` iguais ao braço inline; `to_payload()` produz
o mesmo `ElementPayload::Heading` que `extract_payload` produzia (test
`heading_produz_some_payload` passa sem alteração).
