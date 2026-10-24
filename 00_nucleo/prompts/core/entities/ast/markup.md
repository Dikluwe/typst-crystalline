# Prompt L0 — `entities/ast/markup`
Hash do Código: 85e4f255

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/ast/markup.rs`
**Reference:** ADR-0006.

## Contract and complete interface

Typed markup views borrow &'a SyntaxNode without allocating nodes; SyntaxNode
owns ordering/nesting. node! generates AstNode implementations. Pure L1,
no I/O; depend on AstNode/SyntaxNode/SyntaxKind/Scanner.
Markup::exprs uses Expr::cast_with_space to include interstatement spaces and
filter post-statement Space according to is_stmt.

```rust
node! { struct LineComment }
impl LineComment<'a>  { fn text(self) -> &'a str }

node! { struct BlockComment }
impl BlockComment<'a> { fn text(self) -> &'a str }

node! { struct Markup }
impl Markup<'a> {
    fn exprs(self) -> impl DoubleEndedIterator<Item = Expr<'a>>
}

node! { struct Text }
impl Text<'a>       { fn get(self) -> &'a str }

node! { struct Space }
node! { struct Linebreak }
node! { struct Parbreak }

node! { struct Escape }
impl Escape<'_>     { fn get(self) -> char }

node! { struct Shorthand }
impl Shorthand<'_>  {
    const LIST: &'static [(&'static str, char)]
    fn get(self) -> char
}

node! { struct SmartQuote }
impl SmartQuote<'_> { fn double(self) -> bool }

node! { struct Strong }
impl Strong<'a>     { fn body(self) -> Markup<'a> }

node! { struct Emph }
impl Emph<'a>       { fn body(self) -> Markup<'a> }

node! { struct Raw }
impl Raw<'a> {
    fn lines(self) -> impl DoubleEndedIterator<Item = Text<'a>>
    fn lang(self) -> Option<RawLang<'a>>
    fn block(self) -> bool
}
node! { struct RawLang }
impl RawLang<'a>    { fn get(self) -> &'a str }
node! { struct RawDelim }

node! { struct Link }
impl Link<'a>       { fn get(self) -> &'a str }

node! { struct Label }
impl Label<'a> {
    fn get(self) -> &'a str
}

node! { struct Ref }
impl Ref<'a> {
    fn target(self) -> &'a str
    fn supplement(self) -> Option<ContentBlock<'a>>
}

node! { struct Heading }
impl Heading<'a> {
    fn body(self) -> Markup<'a>
    fn depth(self) -> NonZeroUsize
}

node! { struct ListItem }
impl ListItem<'a>   { fn body(self) -> Markup<'a> }

node! { struct EnumItem }
impl EnumItem<'a> {
    fn number(self) -> Option<u64>
    fn body(self) -> Markup<'a>
}

node! { struct TermItem }
impl TermItem<'a> {
    fn term(self) -> Markup<'a>
    fn description(self) -> Markup<'a>
}

node! { struct ContentBlock }
impl ContentBlock<'a> { fn body(self) -> Markup<'a> }
```

LineComment::text strips //; BlockComment::text strips /* and */.
Text::get returns terminal text. Linebreak and Parbreak represent explicit
line break and blank line. Escape::get resolves escaped characters and Unicode
escapes. Shorthand::LIST is static (e.g. ...→… and ~→NBSP); get looks it up.
SmartQuote::double identifies a double quote. Strong/Emph bodies are Markup
without delimiters. Raw::lines is double-ended Text; lang is None with fewer
than three delimiter backticks; block requires at least three and a newline.
RawDelim retains crate-private len. RawLang/Link expose language/URL text.
Label::get strips leading < and trailing >; Ref::target strips @ from RefMarker
and supplement is optional ContentBlock. Heading depth is the nonzero count
of = in HeadingMarker. EnumItem number is None without an explicit number;
list/enum/content-block bodies and term/description expose Markup.

## Acceptance

Real parsing of *bold* finds Strong and its delimiter-free Markup body.
"= Heading" depth is 1, "== Sub" depth 2. Parsing hello yields Text::get
Some("hello"). Label exists and strips delimiters; Markup::from_untyped(Text)
returns None. Preserve all documented nodes/accessors and canonical lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/ast/markup.md`. Historical observations are not a new parity certification.


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
