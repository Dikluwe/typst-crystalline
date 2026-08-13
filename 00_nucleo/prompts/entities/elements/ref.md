# Prompt L0 — `entities/elements/ref` — `RefElem`
Hash do Código: 8a6135c7

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/ref.rs`
**Origem**: modelo D (ADR-0105), **P462**. Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (leaf).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RefElem {
    pub name: EcoString,
    pub supplement: Option<Content>,
}
```

`Content::Ref { name, supplement }` → `Content::Ref(Arc<RefElem>)`.
Construtores ergonómicos em `Content`:
- `Content::reference(name: impl Into<EcoString>)` — supplement `None`.
- `Content::reference_with_supplement(name, supplement: Option<Content>)`.

A keyword `ref` exigiria raw identifier em cada call-site — fricção
 desnecessária; o módulo e os construtores usam `reference`.

## `impl Element for RefElem`

| método | comportamento |
|---|---|
| `plain_text` | `format!("@{}", self.name)` |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field`/`element_kind`/`to_payload` | default |

## Notas P462

- `RefElem` não guarda o número resolvido — guarda apenas o nome do label.
- A resolução do número acontece no layout via `Introspector` (oráculo de
  counters).
- `supplement` opcional: se `None`, o layout usa o supplement default do tipo
  referenciado (`"Fig. "` para figure, `"Table "` para table, nenhum para
  heading/equation). Se `Some`, o conteúdo é prefixado ao número formatado.
