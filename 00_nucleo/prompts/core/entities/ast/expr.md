# Prompt L0 — `entities/ast/expr`
Hash do Código: 2745dfa7

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/ast/expr.rs`
**References:** ADR-0006, ADR-0025.

## Contract and complete interface

Expr unifies markup/math/code views and atomic expressions. It derives
Debug/Copy/Clone/Hash, without Eq/PartialEq even though inner nodes can have
those traits. Pure L1, no I/O; Scanner decodes strings. The AST-local operator
enums/accessors below belong to this exclusive consumer. The parallel pure data
types in `01_core/src/entities/operators.rs` belong only to
`00_nucleo/prompts/core/entities/operators.md`. Preserve both current public type
families; cross-check their language rules without transferring ownership or
unifying types in this internal repair.

```rust
#[derive(Debug, Copy, Clone, Hash)]
pub enum Expr<'a> {
    Text(Text<'a>), Space(Space<'a>), Linebreak(Linebreak<'a>), Parbreak(Parbreak<'a>),
    Escape(Escape<'a>), Shorthand(Shorthand<'a>), SmartQuote(SmartQuote<'a>),
    Strong(Strong<'a>), Emph(Emph<'a>), Raw(Raw<'a>), Link(Link<'a>),
    Label(Label<'a>), Ref(Ref<'a>), Heading(Heading<'a>), ListItem(ListItem<'a>),
    EnumItem(EnumItem<'a>), TermItem(TermItem<'a>), Equation(Equation<'a>),
    Math(Math<'a>), MathText(MathText<'a>), MathIdent(MathIdent<'a>),
    MathShorthand(MathShorthand<'a>), MathAlignPoint(MathAlignPoint<'a>),
    MathDelimited(MathDelimited<'a>), MathAttach(MathAttach<'a>),
    MathPrimes(MathPrimes<'a>), MathFrac(MathFrac<'a>), MathRoot(MathRoot<'a>),
    Ident(Ident<'a>), None(None<'a>), Auto(Auto<'a>), Bool(Bool<'a>),
    Int(Int<'a>), Float(Float<'a>), Numeric(Numeric<'a>), Str(Str<'a>),
    CodeBlock(CodeBlock<'a>), ContentBlock(ContentBlock<'a>),
    Parenthesized(Parenthesized<'a>), Array(Array<'a>), Dict(Dict<'a>),
    Unary(Unary<'a>), Binary(Binary<'a>), FieldAccess(FieldAccess<'a>),
    FuncCall(FuncCall<'a>), Closure(Closure<'a>),
    LetBinding(LetBinding<'a>), DestructAssignment(DestructAssignment<'a>),
    SetRule(SetRule<'a>), ShowRule(ShowRule<'a>), Contextual(Contextual<'a>),
    Conditional(Conditional<'a>), WhileLoop(WhileLoop<'a>), ForLoop(ForLoop<'a>),
    ModuleImport(ModuleImport<'a>), ModuleInclude(ModuleInclude<'a>),
    LoopBreak(LoopBreak<'a>), LoopContinue(LoopContinue<'a>), FuncReturn(FuncReturn<'a>),
}

impl<'a> Expr<'a> {
    pub(crate) fn cast_with_space(node: &'a SyntaxNode) -> Option<Self>

    pub fn hash(self) -> bool

    pub fn is_literal(self) -> bool
}
```

```rust
node! { struct Ident }
impl Ident<'a> {
    fn get(self) -> &'a str
    fn as_str(self) -> &'a str
}
impl Deref for Ident<'_> { type Target = str }

node! { struct None }; node! { struct Auto }

node! { struct Bool }
impl Bool<'_> { fn get(self) -> bool }

node! { struct Int }
impl Int<'_> { fn get(self) -> i64 }

node! { struct Float }
impl Float<'_> { fn get(self) -> f64 }

node! { struct Numeric }
impl Numeric<'_> { fn get(self) -> (f64, Unit) }

pub enum Unit { Pt, Mm, Cm, In, Rad, Deg, Em, Fr, Percent }

node! { struct Str }
impl Str<'_> { fn get(self) -> String }
```

```rust
node! { struct Unary }
impl Unary<'a> {
    fn op(self) -> UnOp
    fn expr(self) -> Expr<'a>
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnOp { Pos, Neg, Not }
impl UnOp {
    fn from_kind(token: SyntaxKind) -> Option<Self>
    fn precedence(self) -> u8
    fn as_str(self) -> &'static str
}

node! { struct Binary }
impl Binary<'a> {
    fn op(self) -> BinOp
    fn lhs(self) -> Expr<'a>
    fn rhs(self) -> Expr<'a>
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinOp {
    Add, Sub, Mul, Div, And, Or, Eq, Neq, Lt, Leq, Gt, Geq,
    Assign, In, NotIn, AddAssign, SubAssign, MulAssign, DivAssign,
}
impl BinOp {
    fn from_kind(token: SyntaxKind) -> Option<Self>
    fn precedence(self) -> u8
    fn assoc(self) -> Assoc
    fn as_str(self) -> &'static str
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Assoc { Left, Right }
```

```rust
node! { struct FieldAccess }
impl FieldAccess<'a> { fn target(self) -> Expr<'a>; fn field(self) -> Ident<'a> }

node! { struct FuncCall }
impl FuncCall<'a> { fn callee(self) -> Expr<'a>; fn args(self) -> Args<'a> }

node! { struct Args }
impl Args<'a> {
    fn items(self) -> impl DoubleEndedIterator<Item = Arg<'a>>
    fn trailing_comma(self) -> bool
}
pub enum Arg<'a> { Pos(Expr<'a>), Named(Named<'a>), Spread(Spread<'a>) }

node! { struct Closure }
impl Closure<'a> {
    fn name(self) -> Option<Ident<'a>>
    fn params(self) -> Params<'a>
    fn body(self) -> Expr<'a>
}

pub enum Pattern<'a> { Normal(Expr<'a>), Placeholder(Underscore<'a>),
                        Parenthesized(Parenthesized<'a>), Destructuring(Destructuring<'a>) }
impl Pattern<'a> { fn bindings(self) -> Vec<Ident<'a>> }
```

Expr::from_untyped filters Space; crate-private cast_with_space includes it.
hash() indicates embedding through #, is_literal tests literals. Ident get/
as_str/Deref expose text_str as str. Bool::get tests "true". Int supports
0x/0o/0b and decimal; Float yields f64; Numeric yields (f64,Unit) with the
listed units. Str returns owned decoded text with \\n,\\r,\\t and Unicode escapes.
Unary exposes op/expr; Binary exposes op/lhs/rhs. Operator levels/associativity
follow their canonical owner: Pos/Neg7,Not4; Mul/Div6,Add/Sub5,comparisons/In4,
And3,Or2,assignments1; assignments associate right, others left.
FieldAccess exposes target/field; FuncCall callee/args; Args iterates Pos/Named/
Spread in both directions and reports trailing comma. Anonymous Closure name
is None; parameters/body remain accessible. Patterns preserve normal,
placeholder,parenthesized/destructuring cases and return bound identifiers.

## Closed token and binding projections

### Source evidence before classification

Inspection at clean HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
2026-10-19T10:22:03Z, reads the complete local `UnOp::from_kind` at
`01_core/src/entities/ast/expr.rs:597–604`, `BinOp::from_kind` at `:672–694`,
and `Pattern::bindings` at `:904–911`. `Unary::op` at `:575–581` scans children
for an operator; `Binary::op` at `:625–638` separately composes Not then In.
Consequently ordinary child tokens legitimately return `None` from the arity
maps. `Pattern` is declared at `:876–881`, `Expr` at `:36–96`, and recursive
Destructuring binding collection at `:922–933` preserves item order and named/
spread handling. The two distinct operator families also exist in
`01_core/src/entities/operators.rs`; this inspection does not authorize
unifying their public types. Baseline occurrences are V16-071/072/073 in
`lab/results/v16-cause-analysis-2026-10-19/entities-boundaries-inventory.json`.
This is static evidence of the current projections, not a new bilateral parity
measurement or intent inferred from an implementation fallback.

### Preservation and exclusive decisions

Make both local SyntaxKind arity decisions exhaustive at their actual sites.
UnOp accepts exactly Plus→Pos, Minus→Neg and Not→Not. BinOp accepts exactly
Plus→Add, Minus→Sub, Star→Mul, Slash→Div, And→And, Or→Or, EqEq→Eq,
ExclEq→Neq, Lt→Lt, LtEq→Leq, Gt→Gt, GtEq→Geq, Eq→Assign, In→In,
PlusEq→AddAssign, HyphEq→SubAssign, StarEq→MulAssign and SlashEq→DivAssign.
All other current SyntaxKinds explicitly return `None` for that arity:
nonoperator children, trivia, delimiters, literals, syntax nodes and tokens of
a different operator role do not become errors. In particular Star is not
unary and Not alone is not binary; NotIn is still composed in `Binary::op`.
Cross-check these maps against `01_core/src/entities/syntax_set.rs:136–142`
and `01_core/src/compiler/parse/code.rs:97–156`, without copying the unary
complement into the binary map. Preserve existing precedence, spelling,
association and fallback behavior of the surrounding accessors.

Make Pattern exhaustive first: Placeholder produces no identifiers;
Parenthesized delegates to its pattern; Destructuring delegates to its ordered
items; Normal then exhaustively classifies Expr. Only Normal(Ident) yields
that one identifier. Every other current Expr under Normal contributes no
bindings because this projection identifies declarations, not free-variable
uses or arbitrary identifiers inside expressions. Do not recurse through
normal FieldAccess/FuncCall/Array/Closure to invent declarations, and do not
turn a literal's empty binding list into a new parser error. The existing
Parenthesized and Destructuring Pattern routes retain their explicit recursion,
including named patterns and spread sinks. This closes both the outer Pattern
and nested Expr evolution holes; `Normal(_)` is not a sufficient repair.

The changes are internal preservation under ADR-0127 continuous flow: no public
type unification, new grammar admission, defaults, phase or compatibility
changes. Keep the real decisions closed, without suppression or moving a
wildcard into `if let`, `matches!` or an unreviewed helper. A language pattern
whose existing binding order changes, an operator admitted by the parser but
now missed, or a new diagnostic on a current nonbinding pattern refutes this
prescription. Broader parser/AST interface discrepancies are outside this
bounded change and must not be silently corrected here.

### Regression and exact-site enum-growth acceptance

Add meaningful local tests for all positive arity mappings and independent
negative representatives End/Error/Space/LineComment/Ident/Int/LeftParen,
including Star-unary and Not-binary controls. Parse real unary/binary fixtures
and assert the required operator and operand accessors, precedence and the
composed `not in` operator; require the expected AST nodes instead of skipping
assertions when extraction fails. Keep existing `binop_as_str` and
`unop_precedence` controls. Compare spellings/precedence/association with the
parallel operator family through an explicit cross-check; retain both public
types rather than changing source ownership.

For bindings, assert exact ordered names from an identifier, nested
parenthesized/destructuring patterns, a named pattern, a named spread sink and
an anonymous spread/underscore. Literal and a normal nonbinding expression
must yield an empty vector without introducing parse rejection. Use real valid
parsed patterns where admitted; direct SyntaxNode fixtures may isolate the
existing Normal(non-Ident) projection but must not claim new language grammar
admission. Distinguish declaration names from identifiers merely used inside
an expression. No test passes by conditionally omitting its assertion.

Perform enum-growth checks in disposable complete copies of the actual
workspace, retaining its real manifests, modules, enums, dependencies and
function bodies, and compile the real crate. Do not extract methods, replace
enums, stub dependencies or build a toy harness. With a fresh SyntaxKind
sentinel require independently attributable E0004 diagnostics at the repaired
AST-local UnOp::from_kind and BinOp::from_kind matches; neither site's failure
can stand in for the other. In separate copies extend Pattern to require E0004
at the outer `bindings` decision, and extend Expr to require E0004 at its nested
Normal decision. Record that the original wildcard decisions lack those
site-specific exhaustiveness failures. Errors at unrelated derived/accessor
matches may coexist but do not prove these decisions; inability to obtain a
diagnostic at a required site leaves its proof incomplete. No sentinel enters
productive source. Record original/repaired workspace snapshots, source
digests, sentinel-only changes, compiler command/version and exact diagnostic
spans. Run the regression controls against the original implementation before
changing it, then repeat after repair. Require structural RED, these independent
growth checks, valid ownership/nucleus preflight and restricted resealing;
passing runtime controls do not replace the structural proof. These are
enduring acceptance requirements, not tests already run.

## Acceptance

Space returns None through from_untyped. Int parses0xff→255,0o17→15,0b101→5.
A string literal containing escaped newline between hello/world decodes to
an actual newline. Check Mul6,Add5,Assign1,Assign Right,Add Left,Pos.as_str()=="+",
Not4. Expr::FuncCall.hash() is true; Expr::Text.hash() false.
Preserve every enumerated variant and accessor, not the outdated heading's
unverified variant count. Keep canonical lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/ast/expr.md`. Historical observations are not a new parity certification.
