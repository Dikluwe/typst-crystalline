# ADR-003: CST with Lazy AST Projection

## Status
**Accepted**

## Context

Parsing needs to support:
1. Compilation (needs semantic structure)
2. Syntax highlighting (needs all tokens including whitespace)
3. IDE functionality (needs parent/sibling access)
4. Error recovery (must not fail on invalid input)

A traditional AST discards whitespace and comments, making it unsuitable for IDE use cases.

## Decision

Implement a **Concrete Syntax Tree (CST)** as the primary parse result, with a **lazy typed AST view** on top.

### CST Properties
- Preserves all tokens (whitespace, comments)
- Untyped: nodes have `SyntaxKind` but no semantic meaning
- Never fails: syntax errors become error nodes
- Supports incremental reparsing

### AST Properties
- Typed wrapper structs over CST nodes (`struct Raw<'a>(&'a SyntaxNode)`)
- Lazy: materialized only when traversed
- Provides semantic accessors (`.body()`, `.target()`, etc.)

### Projection Pattern

```rust
pub trait AstNode<'a>: Sized {
    fn from_untyped(node: &'a SyntaxNode) -> Option<Self>;
    fn to_untyped(self) -> &'a SyntaxNode;
}
```

## Consequences

### Positive
- **Unified data structure**: One tree serves all use cases
- **Error resilience**: Parsing never fails
- **IDE-friendly**: Full source reconstruction possible
- **Lazy evaluation**: AST nodes created only when needed

### Negative
- **Memory**: CST larger than minimal AST
- **Indirection**: AST methods traverse children

## References
- [specs/typst-syntax/ast.md](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/specs/01_core/typst-syntax/ast.md)
