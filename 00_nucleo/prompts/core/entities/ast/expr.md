# Prompt L0 — `entities/ast/expr`
Hash do Código: db528856

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/ast/expr.rs`
**References:** ADR-0006, ADR-0025.

## Contract and complete interface

Expr unifies markup/math/code views and atomic expressions. It derives
Debug/Copy/Clone/Hash, without Eq/PartialEq even though inner nodes can have
those traits. Pure L1, no I/O; Scanner decodes strings. The operator enum/API
below is a cross-owner dependency, not a second materialization owner;
`00_nucleo/prompts/core/entities/operators.md` owns those pure data types.

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
