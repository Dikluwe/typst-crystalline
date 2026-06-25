# Prompt L0 — `entities/elements/outline` — `OutlineElem`
Hash do Código: P457

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

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OutlineElem {
    pub title:  Option<Content>,
    pub depth:  usize,
    pub indent: bool,
}
```

- `title`: título customizado da TOC; `None` renderiza o default `"Índice"`.
- `depth`: profundidade máxima de headings listados (default `3`).
- `indent`: se `true` (default), entradas de nível > 1 são indentadas.

`Content::Outline` → `Content::Outline(Arc<OutlineElem>)`.
Construtores ergonómicos:
- `Content::outline()` → `outline_with(None, 3, true)`.
- `Content::outline_with(title, depth, indent)`.

**Deriva `Hash`/`PartialEq`**, mas **não `Eq`** porque `Content` não implementa
`Eq`.

## `impl Element for OutlineElem`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` |
| `is_empty` | default `false` |
| `map_content`/`map_text` | recursivos sobre `title`; preservam `depth` e `indent` |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Outline)` |
| `to_payload` | `Some(ElementPayload::Outline)` |

## `eq`

`#[derive(PartialEq)]` — comparação estrutural dos três campos (delegada ao
`PartialEq` de `Content` para `title`).
