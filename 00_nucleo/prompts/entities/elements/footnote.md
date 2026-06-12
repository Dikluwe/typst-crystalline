# Prompt L0 — `entities/elements/footnote` — `FootnoteElem`
Hash do Código: 759a4f1a

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
    pub body: Content,           // era Box<Content>
}
```

`Content::Footnote { body }` → `Content::Footnote(Arc<FootnoteElem>)`.
Construtor ergonómico: `Content::footnote(body)`. **Deriva `Hash`**
(`Content` tem `impl Hash` manual).

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
