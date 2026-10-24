# Prompt L0 — `entities/ast/math`
Hash do Código: 322f953d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/ast/math.rs`
**Reference:** ADR-0006 zero-copy AST.

## Contract and interface

Provide typed syntax-only access to math operands inside $...$, without
Layout, ttf-parser or typography dependencies. Pure L1; depend on
AstNode/SyntaxNode/SyntaxKind/Expr. Wrappers borrow &'a SyntaxNode and allocate
no nodes. MathShorthand::LIST is static without allocation. Preserve all
nodes and interfaces:

```rust
node! { struct Equation }
impl Equation<'a> {
    fn body(self) -> Math<'a>
    fn block(self) -> bool
}

node! { struct Math }
impl Math<'a> {
    fn exprs(self) -> impl DoubleEndedIterator<Item = Expr<'a>>
    fn was_deparenthesized(self) -> bool
}

node! { struct MathText }
pub enum MathTextKind<'a> {
    Grapheme(&'a str),
    Number(&'a str),
}
impl MathText<'a> { fn get(self) -> MathTextKind<'a> }

node! { struct MathIdent }
impl MathIdent<'a> {
    fn get(self) -> &'a str
    fn as_str(self) -> &'a str
}
impl Deref for MathIdent<'_> { type Target = str }

node! { struct MathShorthand }
impl MathShorthand<'_> {
    const LIST: &'static [(&'static str, char)]
    fn get(self) -> char
}

node! { struct MathAlignPoint }

node! { struct MathDelimited }
impl MathDelimited<'a> {
    fn open(self) -> Expr<'a>
    fn body(self) -> Math<'a>
    fn close(self) -> Expr<'a>
}

node! { struct MathAttach }
impl MathAttach<'a> {
    fn base(self) -> Expr<'a>
    fn bottom(self) -> Option<Expr<'a>>
    fn top(self) -> Option<Expr<'a>>
    fn primes(self) -> Option<MathPrimes<'a>>
}

node! { struct MathPrimes }
impl MathPrimes<'_> { fn count(self) -> usize }

node! { struct MathFrac }
impl MathFrac<'a> {
    fn num(self) -> Expr<'a>
    fn denom(self) -> Expr<'a>
}

node! { struct MathRoot }
impl MathRoot<'a> {
    fn index(self) -> Option<u8>
    fn radicand(self) -> Expr<'a>
}
```

Equation::block inspects Space immediately after opening $ and before closing
$ (children1 and n-2), without regex. body returns mathematical content.
Math::exprs is double-ended; was_deparenthesized records an original (math)
body. MathTextKind distinguishes one grapheme from a numeric literal.
MathIdent get/as_str/Deref expose str. Shorthand get looks up the nonempty
40-plus-entry LIST (e.g. !=→≠, <=→≤). MathAlignPoint represents &.
MathDelimited exposes opening Expr, inner Math and closing Expr.
MathAttach exposes base, optional bottom after Underscore, top after Hat,
and primes after base. MathPrimes count is apostrophe text length.
MathFrac num/denom use first/last expression casts. MathRoot index is Some(3)
for ∛,Some(4) for ∜,None for √; radicand is the first expression.

## Acceptance

Verify nonempty shorthand LIST; real parsing of "$ x $" yields Equation
and block spacing behavior. Real "a_1^2" parsing supplies base/top/bottom;
"x/2" supplies numerator/denominator. Check all three radical indices.
Equation::from_untyped(Text) returns None. Keep all documented nodes and
canonical lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/ast/math.md`. Historical observations are not a new parity certification.


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
