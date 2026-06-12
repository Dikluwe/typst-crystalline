# Prompt L0 — `entities/elements/bibliography` — `BibliographyElem`
Hash do Código: 99cb1cfd

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/bibliography.rs`
**Origem**: modelo D (ADR-0105), **Lote 10 P325** (por largura). Trait: ver
`entities/elements/_comum.md`. Contentor — `map_*` recursam no `title`.

> **Fronteira: LOCATÁVEL** (P181C, M6). Absorve o braço de `extract_payload` no
> trait (precedente Heading/Lote 6): `element_kind` → `ElementKind::Bibliography`,
> `to_payload` → `ElementPayload::Bibliography { entries }`. O consumo por
> `ElementPayload` (`from_tags` arm Bibliography → `BibStore`) é **inalterado**
> (matcheia o payload, não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct BibliographyElem {
    pub entries: Vec<BibEntry>,
    pub title:   Option<Content>,        // era Option<Box<Content>>
}
```

`Content::Bibliography { entries, title }` →
`Content::Bibliography(Arc<BibliographyElem>)`. Construtor ergonómico:
`Content::bibliography(entries, title)`. **Deriva `Hash`** (`BibEntry` deriva
`Eq + Hash` — `bib_entry.rs:80`; `Content` tem `impl Hash` manual).

## `impl Element for BibliographyElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | **custom** (`content.rs:1810`): `title` (se houver) + `\n`, depois cada entry formatada `"[{key}] {author}. {title} ({year}).\n"` |
| `is_empty` | **override**: `self.entries.is_empty() && self.title.is_none()` (`content.rs:1654`) |
| `map_content` | **recursivo** no `title`, preserva `entries` (`content.rs:2313`) |
| `map_text` | **recursivo** no `title`, preserva `entries` (`content.rs:2564`) |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Bibliography)` |
| `to_payload` | `Some(ElementPayload::Bibliography { entries: self.entries.clone() })` (absorve `extract_payload.rs:77`) |

## `eq`

`#[derive(PartialEq)]` compara `entries`/`title` (paridade `content.rs:1967`).
