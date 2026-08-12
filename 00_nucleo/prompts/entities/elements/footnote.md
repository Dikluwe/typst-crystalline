# Prompt L0 — `entities/elements/footnote` — `FootnoteElem`
Hash do Código: 860488b2

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/footnote.rs`
**Origem**: modelo D (ADR-0105), **Lote 11 P326** (por largura). Trait: ver
`entities/elements/_comum.md`. **Locatável desde P1016** — o scope-out de P326
(P295 Fase 1 marker-only, ADR-0054 graded) foi revogado: `to_payload` emite
`ElementPayload::Footnote`, o counter flat `"footnote"` avança uma vez por nota
e `counter(footnote)` resolve, em paridade com o vanilla. Ver
`compiler/introspect.md` §P1016. Contentor — `map_*` recursam no `body`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FootnoteElem {
    pub body:      Content,           // era Box<Content>
    pub numbering: Option<EcoString>,
}
```

`Content::Footnote { body, numbering }` → `Content::Footnote(Arc<FootnoteElem>)`.
Construtores ergonómicos:
- `Content::footnote(body)` → `numbering: None`.
- `Content::footnote_with_numbering(body, numbering)` (P502).
**Deriva `Hash`** (`Content` tem `impl Hash` manual).
(P502 — `native_footnote` aceita `numbering:` named; uso no marcador de rodapé scope-out per ADR-0054.)

## `impl Element for FootnoteElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1814`) |
| `is_empty` | **default `false`** — `Footnote` tem arm explícito `=> false` no hub (marker `[N]` sempre observable; `content.rs:1660`); **não delega** ao body. Preservar (não override). |
| `map_content` | **recursivo** no `body` (`content.rs:2299`) |
| `map_text` | **recursivo** no `body` (`content.rs:2543`) |
| `get_field`/`element_kind` | default |
| `to_payload` | **P1016 — override**: `Some(ElementPayload::Footnote { counter_update: CounterUpdate::Step })`. Toda a nota conta (sem gate de `caption`/`numbering`, ao contrário de `TableElem`). Torna `Content::Footnote` locatable — ver `compiler/introspect.md` §P1016. |

## `eq`

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1958`).
