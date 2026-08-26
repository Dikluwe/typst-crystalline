# Prompt L0 — `entities/elements/stack` — `StackElem`
Hash do Código: a2c00e37

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/stack.rs`
**Origem**: modelo D (ADR-0105), **Lote 9 P324** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
em cada child.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StackElem {
    pub children: Arc<[Content]>,
    pub dir:      Dir,
    pub spacing:  Option<Length>,
}
```

`Content::Stack { children, dir, spacing }` →
`Content::Stack(Arc<StackElem>)`. Construtor ergonómico preservado:
`Content::stack(children: Vec<Content>, dir, spacing)` (`children.into()` →
`Arc<[Content]>`, clone O(1) ADR-0026).

> **`Hash` manual via Debug** (precedente Lote 4/5/7/8): `spacing: Option<Length>`
> carrega `f64` → `impl Hash { format!("{self:?}").hash(state) }` (paridade
> `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for StackElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.children.iter().map(\|c\| c.plain_text()).collect()` (`content.rs:1850`) |
| `is_empty` | **override**: `self.children.iter().all(\|c\| c.is_empty())` (`content.rs:1681`) |
| `map_content` | **recursivo** em cada child (collect `SourceResult`), preserva `dir`/`spacing` (`content.rs:2192`) |
| `map_text` | **recursivo** em cada child, preserva `dir`/`spacing` (`content.rs:2463`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `children`/`dir`/`spacing` (paridade
`content.rs:2023`).
