# Prompt L0 — `entities/ast` (base module)
Hash do Código: 74561396

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/ast/mod.rs`
**References:** ADR-0006, ADR-0015.

## Contract

AST nodes are typed lifetime-'a views into the existing SyntaxNode CST;
no AST node allocates. Data remains in SyntaxNode's Arc<[SyntaxData]>.
Depend only on L1 SyntaxNode, SyntaxKind and Span; no I/O.
Wrappers are repr(transparent), with the same layout as &SyntaxNode.

AstNode<'a>: Sized defines from_untyped(&'a SyntaxNode)->Option<Self>,
returning None on mismatched kind; to_untyped(self)->&'a SyntaxNode;
and span(self)->Span, delegating to to_untyped().span().
The declarative wrapper macro is:

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

SyntaxNode extensions preserve public is<'a,T:AstNode<'a>>(&'a self)->bool
and cast<'a,T:AstNode<'a>>(&'a self)->Option<T>, delegating to from_untyped.
Crate-private try_cast_first/try_cast_last with those generics return the
first/last convertible child or None. Crate-private cast_first/cast_last
return T and panic if absent. The historical promise to revisit malformed-tree
panics with placeholder() remains unresolved here; cleanup does not silently
replace the failure behavior.

## Acceptance and module boundary

For Source::detached("Hello *world*"), Markup::from_untyped(root) succeeds;
a Text node fails that cast. markup.to_untyped().kind() is Markup;
markup.span() equals the root span. Preserve the zero-copy representation.
Markup wrappers belong to ast/markup.rs (including Heading/Strong/Emph/Raw/Link/Label/Ref);
math wrappers to ast/math.rs (Equation/Math/MathAttach/MathFrac/MathRoot);
code wrappers to ast/code.rs (LetBinding/SetRule/ShowRule/Conditional/ForLoop);
Expr/Ident/Bool/Int/FuncCall/Closure/operators to ast/expr.rs.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/ast/mod.md`. Historical observations are not a new parity certification.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.
