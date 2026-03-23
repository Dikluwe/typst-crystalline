# SyntaxNode — nó da árvore sintática concreta

## Contexto

`SyntaxNode` é o tipo central da CST (Concrete Syntax Tree) do Typst.
Três representações internas:
- **Leaf**: token terminal com `SyntaxKind`, texto (`EcoString`) e `Span`
- **Inner**: nó interior com filhos (`Vec<SyntaxNode>`), `Arc`-partilhado
- **Error**: nó de erro com `SyntaxError` e texto original

`Arc` partilha nós entre revisões incrementais sem cópia.
`EcoString` é `Arc<str>` com ergonomia extra — autorizado em L1 (ADR-0004).

## Origem

`lab/typst-original/crates/typst-syntax/src/node.rs`

Dependências: `ecow` (autorizado), `std` (Arc, Rc, Range), L1 (FileId, Span, SyntaxKind).

## Tipos migrados

| Tipo | Visibilidade | Descrição |
|------|-------------|-----------|
| `SyntaxNode` | `pub` | nó da CST |
| `SyntaxError` | `pub` | erro sintáctico com span, mensagem, hints |
| `LinkedNode<'a>` | `pub` | nó em contexto com offset e parent |
| `LinkedChildren<'a>` | `pub` | iterador sobre filhos de LinkedNode |
| `Side` | `pub` | Before/After para leaf_at |
| `NumberingResult` | `pub(crate)` | usado pelo parser (Passo 4) |
| `Unnumberable` | `pub(crate)` | usado pelo parser (Passo 4) |

## O que não migra

- Tests que usam `Source::detached()` → bloqueados até Passo 4
- `reparser.rs` → Passo 4
- Construção de `SyntaxNode` a partir do parser → Passo 4

## Critérios de correcção (sem Source)

```
SyntaxNode::leaf(SyntaxKind::Text, "hi").kind()  == SyntaxKind::Text
SyntaxNode::leaf(SyntaxKind::Text, "hi").text()  == "hi"
SyntaxNode::leaf(SyntaxKind::Text, "hi").len()   == 2
SyntaxNode::inner(SyntaxKind::Markup, vec![leaf]).children().count() == 1
SyntaxNode::error(SyntaxError::new("oops"), "x").errors().len() == 1
SyntaxNode::error(...).erroneous() == true
LinkedNode::new(&leaf).kind() == SyntaxKind::Text
```

## Notas pub(super) → pub(crate)

`numberize`, `is_leaf`, `descendants`, `children_mut`,
`replace_children`, `update_parent`, `upper`,
`convert_to_kind`, `convert_to_error`, `expected`, `unexpected`
são `pub(super)` no original (usado pelo parser no mesmo crate).
Em L1 ficam `pub(crate)` — serão usados pelo parser no Passo 4.
