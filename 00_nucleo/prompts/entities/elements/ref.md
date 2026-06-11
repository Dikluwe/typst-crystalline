# Prompt L0 — `entities/elements/ref` — `RefElem`
Hash do Código: 8a6135c7

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/ref.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura). Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P323:
`locatable.rs` lista `Content::Ref` no bloco não-locatável; sem arm em
`extract_payload`). Leaf — `map_*` terminais.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefElem {
    pub target: Label,
}
```

`Content::Ref { target }` → `Content::Ref(Arc<RefElem>)`.
Construtor ergonómico: **`Content::reference(target: Label)`** — não `r#ref`
(o raw identifier é fricção em cada call-site; comentário de 1 linha no módulo
sobre a colisão com a keyword `ref`). **Deriva `Hash`/`Eq`** (`Label(String)`
deriva `Hash`/`Eq`).

## `impl Element for RefElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("@{}", self.target.0)` (`content.rs:1724`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `target` (paridade `content.rs:1888`:
`(Ref{target:ta}, Ref{target:tb}) => ta == tb`).
