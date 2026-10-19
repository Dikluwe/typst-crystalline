# Prompt L0 — `entities/ast/code`
Hash do Código: fcb0eeab

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/ast/code.rs`
**References:** ADR-0006, ADR-0017.

## Contract and complete interface

Expose code syntax for bindings, set/show, conditionals, loops and imports/
includes, without eval dependency or evaluation. Pure L1, no I/O; use Scanner,
is_ident and PackageSpec. ModuleImport::bare_name may use std::path::Path and
FromStr for pure string-path/package-name extraction.

```rust
node! { struct LetBinding }

pub enum LetBindingKind<'a> {
    Normal(Pattern<'a>),
    Closure(Ident<'a>),
}
impl LetBindingKind<'a> { fn bindings(self) -> Vec<Ident<'a>> }

impl LetBinding<'a> {
    fn kind(self) -> LetBindingKind<'a>
    fn init(self) -> Option<Expr<'a>>
}

node! { struct DestructAssignment }
impl DestructAssignment<'a> {
    fn pattern(self) -> Pattern<'a>
    fn value(self) -> Expr<'a>
}

node! { struct SetRule }
impl SetRule<'a> {
    fn target(self) -> Expr<'a>
    fn args(self) -> Args<'a>
    fn condition(self) -> Option<Expr<'a>>
}

node! { struct ShowRule }
impl ShowRule<'a> {
    fn selector(self) -> Option<Expr<'a>>
    fn transform(self) -> Expr<'a>
}

node! { struct Contextual }
impl Contextual<'a> { fn body(self) -> Expr<'a> }

node! { struct Conditional }
impl Conditional<'a> {
    fn condition(self) -> Expr<'a>
    fn if_body(self) -> Expr<'a>
    fn else_body(self) -> Option<Expr<'a>>
}

node! { struct WhileLoop }
impl WhileLoop<'a> {
    fn condition(self) -> Expr<'a>
    fn body(self) -> Expr<'a>
}

node! { struct ForLoop }
impl ForLoop<'a> {
    fn pattern(self) -> Pattern<'a>
    fn iterable(self) -> Expr<'a>
    fn body(self) -> Expr<'a>
}

node! { struct ModuleImport }

pub enum BareImportError { Dynamic, PathInvalid, PackageInvalid }

impl ModuleImport<'a> {
    fn source(self) -> Expr<'a>
    fn imports(self) -> Option<Imports<'a>>
    fn bare_name(self) -> Result<String, BareImportError>
    fn new_name(self) -> Option<Ident<'a>>
}

pub enum Imports<'a> { Wildcard, Items(ImportItems<'a>) }

node! { struct ImportItems }
impl ImportItems<'a> {
    fn iter(self) -> impl DoubleEndedIterator<Item = ImportItem<'a>>
}

pub enum ImportItem<'a> { Simple(ImportItemPath<'a>), Renamed(RenamedImportItem<'a>) }
impl ImportItem<'a> {
    fn path(self) -> ImportItemPath<'a>
    fn original_name(self) -> Ident<'a>
    fn bound_name(self) -> Ident<'a>
}

node! { struct ImportItemPath }
node! { struct RenamedImportItem }
impl RenamedImportItem<'a> {
    fn path(self) -> ImportItemPath<'a>
    fn original_name(self) -> Ident<'a>
    fn new_name(self) -> Ident<'a>
}

node! { struct ModuleInclude }
impl ModuleInclude<'a> { fn source(self) -> Expr<'a> }

node! { struct LoopBreak }
node! { struct LoopContinue }
node! { struct FuncReturn }
impl FuncReturn<'a> { fn body(self) -> Option<Expr<'a>> }
```

LetBindingKind distinguishes normal Pattern from named-closure Ident;
bindings returns their bound identifiers. init is None for a valueless let.
DestructAssignment exposes pattern/value. SetRule exposes target, Args and
optional condition after if. ShowRule selector before : is None for wildcard;
transform follows :. Contextual exposes body. Conditional if_body panics for
malformed syntax, else_body is absent without else. ForLoop exposes iteration
pattern, iterable after in (panic when malformed), and body; WhileLoop exposes
condition/body.

ModuleImport source is an expression; imports None means no colon. bare_name
maps "utils.typ"→"utils" and "@preview/algo:0.1.0"→"algo" or its precise
Dynamic/PathInvalid/PackageInvalid category. new_name handles as alias.
Imports distinguishes wildcard/items; ImportItem distinguishes simple/renamed
and exposes path/original/bound names. RenamedImportItem supplies new_name.
ModuleInclude exposes source. FuncReturn body is None for return without value.

## Acceptance

All three BareImportError variants exist; LetBinding/ModuleImport cast types
and SyntaxKind::LetBinding exist. Verify typed set target/args, show optional
selector/transform, destructuring Pattern/Expr, for iterable after in and
while condition/body. Keep all declared nodes and canonical lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/ast/code.md`. Historical observations are not a new parity certification.
