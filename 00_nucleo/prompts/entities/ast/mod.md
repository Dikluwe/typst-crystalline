# Prompt L0 — `entities/ast/`

**Camada**: L1 — domínio puro
**Módulo**: `01_core/src/entities/ast/`
**Origem**: `lab/typst-original/crates/typst-syntax/src/ast.rs`

---

## Contexto

A AST tipada são wrappers com lifetime `'a` sobre `&'a SyntaxNode`.
Zero I/O. Usa apenas tipos já em L1: `SyntaxNode`, `SyntaxKind`,
`Span`, `SyntaxText`.

O ficheiro original tem 2462 linhas e 76 invocações da macro `node!`
→ estrutura em submódulos (> 2000 linhas).

---

## Decisões de migração

| Original | Substituição | ADR |
|----------|-------------|-----|
| `use ecow::EcoString` | `use crate::entities::syntax_text::SyntaxText` | 0015 |
| `use typst_utils::NonZeroExt` | remover; substituir `NonZeroUsize::ONE` por `NonZeroUsize::MIN` | 0016 |
| `use unscanny::Scanner` | `use crate::rules::lexer::scanner::Scanner` | 0014 |

`unscanny::Scanner` já existe em `01_core/src/rules/lexer/scanner.rs`.
`NonZeroUsize::MIN == 1` (stdlib desde Rust 1.67).

---

## Interface pública — trait central

```rust
pub trait AstNode<'a>: Sized {
    fn from_untyped(node: &'a SyntaxNode) -> Option<Self>;
    fn to_untyped(self) -> &'a SyntaxNode;

    fn span(&self) -> Span {
        self.to_untyped().span()
    }
}
```

A macro `node!` implementa `AstNode` para cada tipo concreto.
Cada tipo concreto é um newtype `struct Foo<'a>(&'a SyntaxNode)`.

---

## Estrutura de submódulos

```
01_core/src/entities/ast/
  mod.rs      — re-exports públicos, AstNode trait, macro node!
  markup.rs   — Markup, Text, Space, Linebreak, Parbreak, Escape,
                Shorthand, SmartQuote, Strong, Emph, Link,
                Heading, ListItem, EnumItem, TermItem, Raw, ...
  math.rs     — Math, MathIdent, MathAlignPoint, MathDelimited,
                MathAttach, MathPrimes, MathFrac, MathRoot,
                MathText, MathTextKind, ...
  code.rs     — Ident, Let, LetBindingKind, Set, Show, Contextual,
                Conditional, WhileLoop, ForLoop, Import, Imports,
                ImportItem, BareImportError, Include, Break,
                Continue, Return, ...
  expr.rs     — Expr, Code, Content, Array, ArrayItem, Dict,
                DictItem, Parenthesized, Unary, UnOp, Binary,
                BinOp, Assoc, FuncCall, Args, Arg, Closure,
                Params, Param, Pattern, DestructuringItem,
                Spread, Destructuring, FieldAccess, ...
```

Os tipos distribuem-se pelos submódulos consoante a categoria
semântica. `mod.rs` re-exporta todos com `pub use`.

---

## Critérios de verificação

```
Dado SyntaxNode com kind() == SyntaxKind::Markup
Quando Markup::from_untyped(&node) for chamado
Então Some(markup)

Dado SyntaxNode com kind() == SyntaxKind::Text
Quando Markup::from_untyped(&node) for chamado
Então None (kind errado)

Dado AstNode válido
Quando span() for chamado
Então mesmo span que to_untyped().span()

Dado Source::detached("= Heading")
Quando Markup::from_untyped(root) for chamado e children iterados
Então existe nó com kind() == SyntaxKind::Heading convertível em Heading

Dado Source::detached("*bold*")
Quando Strong::from_untyped for chamado num filho do root
Então Some(strong) com body() acessível
```

---

## Nota sobre a macro `node!`

A macro `node!` em `mod.rs` implementa `AstNode` e adiciona
métodos auxiliares a cada tipo. Verifica se migra directamente
para os tipos L1 (`SyntaxKind`, `SyntaxNode`, `Span`, `SyntaxText`)
sem alteração de lógica.

Ficheiro único seria possível mas 2462 linhas > 2000 — manter
submódulos para facilitar navegação e revisão.
