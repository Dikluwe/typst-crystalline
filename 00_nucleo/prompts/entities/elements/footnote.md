# Prompt L0 — `entities/elements/footnote` — `FootnoteElem`
Hash do Código: f04714d3

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/footnote.rs`
**Origem**: modelo D (ADR-0105), **Lote 11 P326** (por largura). Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P326: P295 Fase 1
marker-only, scope-out per ADR-0054 graded; frente futura P295.X tornaria
locatável, mas hoje não). Contentor — `map_*` recursam no `body`.

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
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1958`).
