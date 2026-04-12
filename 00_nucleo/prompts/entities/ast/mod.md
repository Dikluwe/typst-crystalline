# Prompt L0 — `entities/ast` (módulo base)

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/ast/mod.rs`
**Criado em**: 2026-03-25 (Passo 5)
**Atualizado em**: 2026-04-12 (restauro — expandido com `node!` macro, métodos do `SyntaxNode` e critérios completos)
**ADRs relevantes**: ADR-0006 (zero-copy AST via lifetime `'a`), ADR-0015

---

## Contexto e Objetivo

A AST do Cristalino é uma **vista tipada sobre a CST** (Concrete Syntax Tree
armazenada em `SyntaxNode`). Em vez de alocar novos nós, os nós da AST são
wrappers de lifetime `'a` que apontam para os `SyntaxNode` já existentes.

Este módulo define:
1. O trait `AstNode<'a>` — contrato base de toda a AST
2. A macro `node!` — gera wrappers de AST de forma declarativa
3. Extensões de `SyntaxNode` (`is`, `cast`, `try_cast_first`, etc.)

**ADR-0006**: zero-copy — nenhum nó da AST aloca memória; os dados vivem
exclusivamente na `Arc<[SyntaxData]>` do `SyntaxNode`.

---

## Restrições Estruturais

- Camada **L1**: zero I/O. Depende apenas de `SyntaxNode`, `SyntaxKind` e `Span` (L1).
- Os wrappers são `#[repr(transparent)]` — mesmo layout que `&SyntaxNode`.
- `from_untyped` retorna `None` graciosamente se o `SyntaxKind` não corresponder.
- `cast_first`/`cast_last` são `pub(crate)` e `panic!` em árvores malformadas
  (aceitável em Passo 5; revisitar em Passo 10 com `placeholder()`).

---

## Instrução

### Trait `AstNode<'a>`

```rust
pub trait AstNode<'a>: Sized {
    /// Converte um nó genérico para o tipo tipado.
    /// Retorna None se node.kind() != SyntaxKind::NomeTipo.
    fn from_untyped(node: &'a SyntaxNode) -> Option<Self>;

    /// Referência ao SyntaxNode subjacente.
    fn to_untyped(self) -> &'a SyntaxNode;

    /// Localização na fonte (delega a SyntaxNode::span).
    fn span(self) -> Span {
        self.to_untyped().span()
    }
}
```

### Macro `node!`

```rust
macro_rules! node {
    ($(#[$attr:meta])* struct $name:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        pub struct $name<'a>(pub(crate) &'a SyntaxNode);

        impl<'a> AstNode<'a> for $name<'a> {
            fn from_untyped(node: &'a SyntaxNode) -> Option<Self> {
                if node.kind() == SyntaxKind::$name {
                    Some(Self(node))
                } else {
                    None
                }
            }
            fn to_untyped(self) -> &'a SyntaxNode { self.0 }
        }
    };
}
```

### Extensões de SyntaxNode (adicionadas neste módulo)

```rust
impl SyntaxNode {
    // Verifica se o nó pode ser convertido para T
    pub fn is<'a, T: AstNode<'a>>(&'a self) -> bool

    // Tenta converter para T (delega a T::from_untyped)
    pub fn cast<'a, T: AstNode<'a>>(&'a self) -> Option<T>

    // Primeiro filho convertível para T (None se nenhum)
    pub(crate) fn try_cast_first<'a, T: AstNode<'a>>(&'a self) -> Option<T>

    // Último filho convertível para T (None se nenhum)
    pub(crate) fn try_cast_last<'a, T: AstNode<'a>>(&'a self) -> Option<T>

    // Primeiro filho T, panic se não encontrado
    pub(crate) fn cast_first<'a, T: AstNode<'a>>(&'a self) -> T

    // Último filho T, panic se não encontrado
    pub(crate) fn cast_last<'a, T: AstNode<'a>>(&'a self) -> T
}
```

---

## Critérios de Verificação

```
// from_untyped retorna Some para o kind correto
let src = Source::detached("Hello *world*");
Markup::from_untyped(src.root()) = Some(...)

// from_untyped retorna None para kind errado
Markup::from_untyped(text_node) = None   // text_node.kind() == Text

// roundtrip to_untyped
markup.to_untyped().kind() = SyntaxKind::Markup

// span delega ao SyntaxNode
markup.span() = src.root().span()
```

---

## Submodules

| Ficheiro | Conteúdo |
|----------|---------|
| `ast/markup.rs` | Nós de markup: `Markup`, `Heading`, `Strong`, `Emph`, `Raw`, `Link`, `Label`, `Ref`, etc. |
| `ast/math.rs` | Nós matemáticos: `Equation`, `Math`, `MathAttach`, `MathFrac`, `MathRoot`, etc. |
| `ast/code.rs` | Nós de código: `LetBinding`, `SetRule`, `ShowRule`, `Conditional`, `ForLoop`, etc. |
| `ast/expr.rs` | `Expr<'a>` (enum unificador), `Ident`, `Bool`, `Int`, `FuncCall`, `Closure`, operadores, etc. |

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-25 | Criação — Passo 5: trait AstNode, macro node!, extensões SyntaxNode | `ast/mod.rs` |
| 2026-04-12 | Restauro — expandido com macro node!, extensões, critérios, submodules | `ast/mod.md` |
