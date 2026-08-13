# Prompt L0 — `entities/elements/outline` — `OutlineElem`
Hash do Código: 243eb555

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/outline.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura) + **P457** (campos settable).
Trait e glossário (§A.0): ver `entities/elements/_comum.md`. Comportamento idêntico ao braço atual.

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

## `OutlineIndent` — P502

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum OutlineIndent {
    Auto,
    Bool(bool),
    Length(Length),
    Function(Func),
}
```

Representa os tipos aceites por `outline(indent:)` no vanilla 0.14.2
(`length | function | auto`) mais `bool` para compatibilidade reversa com
 código cristalino existente. O layout de headings consome `Auto`/`Bool(true)`
como indentação ativa e `Bool(false)` como inactiva; `Length`/`Function`
são aceites e armazenados, mas a renderização específica é scope-out per
ADR-0054.

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OutlineElem {
    pub title:  Option<Content>,
    pub depth:  usize,
    pub indent: OutlineIndent,
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
- `Content::outline()` → `outline_with(None, 3, OutlineIndent::Auto)` com `target: Headings`.
- `Content::outline_with(title, depth, indent)`.
- `Content::lof(title: Option<Content>)` → P472, `target: Figures`, `indent: Bool(false)`.
- `Content::lot(title: Option<Content>)` → P472, `target: Tables`, `indent: Bool(false)`.

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
pub fn with_target(
    title: Option<Content>,
    depth: usize,
    indent: OutlineIndent,
    target: OutlineTarget,
) -> Self {
    Self { title, depth, indent, target }
}
```

Usado por `Content::lof` e `Content::lot`, que fixam os defaults dos sumários de
figuras/tabelas: `depth = 1` e `indent = OutlineIndent::Bool(false)`
(`entities/content.rs:2153-2161` para `lof`; `lot` é idêntico com
`OutlineTarget::Tables`).

> **Correcção de exemplo inválido** (2026-08-13). A versão anterior deste bloco
> documentava `with_target(title, target)` com `depth: 3, indent: true` fixos no corpo.
> Duas coisas erradas, medidas contra `entities/elements/outline.rs:87-94`: a assinatura
> tem **quatro** parâmetros (`depth` e `indent` deixaram de ser fixos), e `indent: true`
> **não compila** — o campo é `OutlineIndent` (`Auto | Bool(bool) | Length | Function`),
> logo o valor válido é `OutlineIndent::Bool(true)`, não o `bool` cru. O bloco acima é
> agora transcrição literal do código, que compila como parte do crate.

## `eq`

`#[derive(PartialEq)]` — comparação estrutural dos quatro campos (delegada ao
`PartialEq` de `Content` para `title`, `Copy` para `target`).
