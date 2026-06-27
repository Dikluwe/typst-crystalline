# Prompt L0 — `entities/elements/outline` — `OutlineElem`
Hash do Código: 713a71c0

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/outline.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura) + **P457** (campos settable).
Trait: ver `entities/elements/_comum.md`. Comportamento idêntico ao braço atual.

> **Modelo D**: `Content::Outline` encapsula `Arc<OutlineElem>`. A struct deixou
> de ser unit em P457 e passou a transportar os campos settable do vanilla:
> `title`, `depth`, `indent`. Migração aterrada neste módulo sem tocar o hub.
>
> `Outline` é elemento de utilizador (fora da classe DEBT-58).

> **Fronteira: LOCATÁVEL** (queryable desde P178, fecha lacuna #7). Absorve o
> braço de `extract_payload` no trait: `element_kind` → `ElementKind::Outline`,
> `to_payload` → `ElementPayload::Outline`. O consumo por `ElementPayload`
> (`introspect.rs`/`from_tags`) é inalterado (matcheia o payload, não o
> `Content`).

---

## `OutlineTarget` — P472

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OutlineTarget {
    #[default] Headings,
    Figures,
    Tables,
}
```

- `Headings` (default): TOC clássica de headings.
- `Figures`: List of Figures — lê `Introspector::figures_for_lof()`.
- `Tables`: List of Tables — lê `Introspector::tables_for_lot()`.

Usado internamente por `layout_outline.rs` para despachar para o arm correcto.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OutlineElem {
    pub title:  Option<Content>,
    pub depth:  usize,
    pub indent: bool,
    /// **P472** — distingue TOC / LoF / LoT. Default `Headings`.
    pub target: OutlineTarget,
}
```

- `title`: título customizado; `None` usa default do target (TOC→`"Índice"`, LoF→`"List of Figures"`, LoT→`"List of Tables"`).
- `depth`: profundidade máxima de headings (só relevante para `Headings`).
- `indent`: indentação (só relevante para `Headings`).
- `target`: qual lista gerar.

`Content::Outline` → `Content::Outline(Arc<OutlineElem>)`.
Construtores ergonómicos:
- `Content::outline()` → `outline_with(None, 3, true)` com `target: Headings`.
- `Content::outline_with(title, depth, indent)`.
- `Content::lof(title: Option<Content>)` → P472, `target: Figures`.
- `Content::lot(title: Option<Content>)` → P472, `target: Tables`.

**Deriva `Hash`/`PartialEq`**, mas **não `Eq`** porque `Content` não implementa
`Eq`.

## `impl Element for OutlineElem`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` |
| `is_empty` | default `false` |
| `map_content`/`map_text` | recursivos sobre `title`; preservam `depth`, `indent`, `target` |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Outline)` |
| `to_payload` | `Some(ElementPayload::Outline)` |

## `with_target` — P472

```rust
pub fn with_target(title: Option<Content>, target: OutlineTarget) -> Self {
    Self { title, depth: 3, indent: true, target }
}
```

Usado por `Content::lof` e `Content::lot`.

## `eq`

`#[derive(PartialEq)]` — comparação estrutural dos quatro campos (delegada ao
`PartialEq` de `Content` para `title`, `Copy` para `target`).
