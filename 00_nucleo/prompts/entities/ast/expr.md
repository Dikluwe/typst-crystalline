# Prompt L0 — `entities/ast/expr`
Hash do Código: bdca75b8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/ast/expr.rs`
**Criado em**: 2026-03-26 (Passo 6)
**Atualizado em**: 2026-04-12 (restauro — prompt dedicado; antes coberto por `ast/mod.md`)
**ADRs relevantes**: ADR-0006 (zero-copy AST), ADR-0025 (coerção Int/Float)

---

## Contexto e Objetivo

Define o **enum unificador** `Expr<'a>` que cobre todas as expressões possíveis
nos três modos (Markup, Math, Code). É o tipo de retorno de `AstNode::from_untyped`
para o nível mais geral da AST.

Também define os nós de expressão atômica — literais (`Bool`, `Int`, `Float`,
`Numeric`, `Str`), `Ident`, operadores (`Unary`, `Binary`), chamadas de função
(`FuncCall`), closures e padrões de destructuring.

---

## Restrições Estruturais

- Camada **L1**: zero I/O. Usa `Scanner` para parsing de strings com escapes.
- `Expr<'a>` é `#[derive(Debug, Copy, Clone, Hash)]` — sem `Eq/PartialEq`
  (os inner nodes têm Eq, mas Expr como enum não garante).
- `UnOp`, `BinOp`, `Assoc` são tipos de dados puros sem dependências.
- `Int::get` suporta literais hex (`0x`), octal (`0o`) e binário (`0b`).
- `Str::get` resolve sequências de escape (`\n`, `\r`, `\t`, `\u{XXXX}`).

---

## Instrução

### `Expr<'a>` — 57 variantes

```rust
#[derive(Debug, Copy, Clone, Hash)]
pub enum Expr<'a> {
    // Markup
    Text(Text<'a>), Space(Space<'a>), Linebreak(Linebreak<'a>), Parbreak(Parbreak<'a>),
    Escape(Escape<'a>), Shorthand(Shorthand<'a>), SmartQuote(SmartQuote<'a>),
    Strong(Strong<'a>), Emph(Emph<'a>), Raw(Raw<'a>), Link(Link<'a>),
    Label(Label<'a>), Ref(Ref<'a>), Heading(Heading<'a>), ListItem(ListItem<'a>),
    EnumItem(EnumItem<'a>), TermItem(TermItem<'a>), Equation(Equation<'a>),
    // Math
    Math(Math<'a>), MathText(MathText<'a>), MathIdent(MathIdent<'a>),
    MathShorthand(MathShorthand<'a>), MathAlignPoint(MathAlignPoint<'a>),
    MathDelimited(MathDelimited<'a>), MathAttach(MathAttach<'a>),
    MathPrimes(MathPrimes<'a>), MathFrac(MathFrac<'a>), MathRoot(MathRoot<'a>),
    // Code — literais
    Ident(Ident<'a>), None(None<'a>), Auto(Auto<'a>), Bool(Bool<'a>),
    Int(Int<'a>), Float(Float<'a>), Numeric(Numeric<'a>), Str(Str<'a>),
    // Code — estruturas
    CodeBlock(CodeBlock<'a>), ContentBlock(ContentBlock<'a>),
    Parenthesized(Parenthesized<'a>), Array(Array<'a>), Dict(Dict<'a>),
    Unary(Unary<'a>), Binary(Binary<'a>), FieldAccess(FieldAccess<'a>),
    FuncCall(FuncCall<'a>), Closure(Closure<'a>),
    // Code — statements
    LetBinding(LetBinding<'a>), DestructAssignment(DestructAssignment<'a>),
    SetRule(SetRule<'a>), ShowRule(ShowRule<'a>), Contextual(Contextual<'a>),
    Conditional(Conditional<'a>), WhileLoop(WhileLoop<'a>), ForLoop(ForLoop<'a>),
    ModuleImport(ModuleImport<'a>), ModuleInclude(ModuleInclude<'a>),
    LoopBreak(LoopBreak<'a>), LoopContinue(LoopContinue<'a>), FuncReturn(FuncReturn<'a>),
}

impl<'a> Expr<'a> {
    // Nota: Space retorna None em from_untyped! (filtrado em Markup::exprs)
    // Use cast_with_space para incluir espaços.
    pub(crate) fn cast_with_space(node: &'a SyntaxNode) -> Option<Self>

    /// Pode ser embutido em markup com #?
    pub fn hash(self) -> bool

    /// É um literal?
    pub fn is_literal(self) -> bool
}
```

### Nós atômicos

```rust
node! { struct Ident }
impl Ident<'a> {
    fn get(self) -> &'a str    // text_str()
    fn as_str(self) -> &'a str
}
impl Deref for Ident<'_> { type Target = str }

node! { struct None }; node! { struct Auto }

node! { struct Bool }
impl Bool<'_> { fn get(self) -> bool }  // text == "true"

node! { struct Int }
impl Int<'_> { fn get(self) -> i64 }   // 0x, 0o, 0b, decimal

node! { struct Float }
impl Float<'_> { fn get(self) -> f64 }

node! { struct Numeric }
impl Numeric<'_> { fn get(self) -> (f64, Unit) }

pub enum Unit { Pt, Mm, Cm, In, Rad, Deg, Em, Fr, Percent }

node! { struct Str }
impl Str<'_> { fn get(self) -> String }  // resolve escapes (\n, \r, \t, \u{})
```

### Operadores

```rust
node! { struct Unary }
impl Unary<'a> {
    fn op(self) -> UnOp    // '+', '-', 'not'
    fn expr(self) -> Expr<'a>
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnOp { Pos, Neg, Not }
impl UnOp {
    fn from_kind(token: SyntaxKind) -> Option<Self>
    fn precedence(self) -> u8    // Pos/Neg=7, Not=4
    fn as_str(self) -> &'static str  // "+", "-", "not"
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
    fn precedence(self) -> u8  // Mul/Div=6, Add/Sub=5, Cmp/In=4, And=3, Or=2, Assign=1
    fn assoc(self) -> Assoc    // Assign* → Right, resto → Left
    fn as_str(self) -> &'static str
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Assoc { Left, Right }
```

### Chamadas e closures

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
    fn name(self) -> Option<Ident<'a>>   // None para closures anónimas
    fn params(self) -> Params<'a>
    fn body(self) -> Expr<'a>
}

// Patterns e destructuring
pub enum Pattern<'a> { Normal(Expr<'a>), Placeholder(Underscore<'a>),
                        Parenthesized(Parenthesized<'a>), Destructuring(Destructuring<'a>) }
impl Pattern<'a> { fn bindings(self) -> Vec<Ident<'a>> }
```

---

## Critérios de Verificação

```
// Space filtrado em from_untyped
Expr::from_untyped(space_node) = None

// Int literais
Int.get() para "0xff" = 255
Int.get() para "0o17" = 15
Int.get() para "0b101" = 5

// Str escape
Str.get() para "\"hello\\nworld\"" = "hello\nworld"

// BinOp precedência
BinOp::Mul.precedence() = 6
BinOp::Add.precedence() = 5
BinOp::Assign.precedence() = 1
BinOp::Assign.assoc() = Assoc::Right
BinOp::Add.assoc() = Assoc::Left

// UnOp
UnOp::Pos.as_str() = "+"
UnOp::Not.precedence() = 4

// hash() — pode ser embutido com #
Expr::FuncCall(..).hash() = true
Expr::Text(..).hash() = false
```

---

## Resultado Esperado

- `01_core/src/entities/ast/expr.rs` com todos os tipos documentados
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/ast/expr.md`)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-26 | Criação — Passo 6: Expr (57 variantes), Ident, Int, Str, operadores, FuncCall, Closure | `ast/expr.rs` |
| 2026-04-12 | Restauro — prompt dedicado; interface completa, precedências, BinOp.assoc, Int (hex/oct/bin), Str (escapes) | `ast/expr.md` |
