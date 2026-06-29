# Prompt L0 — `entities/elements/raw` — `RawElem`
Hash do Código: 7dd4d357

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/raw.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P322). Comportamento
idêntico ao braço atual.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawElem {
    pub text:  EcoString,
    pub lang:  Option<EcoString>,
    pub block: bool,
}
```

`Content::Raw { text, lang, block }` → `Content::Raw(Arc<RawElem>)`.
Construtor ergonómico preservado: `Content::raw(text, lang, block)`.
(P502 — `native_raw` passa a aceitar `lang:` e `block:` named; syntax highlighting real continua scope-out per ADR-0054.)
**Deriva `Hash`** (`EcoString`/`Option<EcoString>`/`bool`).

## `impl Element`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.text.to_string()` (`content.rs:1683`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (folha) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `text + lang + block` (paridade `content.rs:1857`).
