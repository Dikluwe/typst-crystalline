# Prompt L0 — `entities/syntax_kind`
Hash do Código: c9290ebf

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_kind.rs`

## Vocabulary and complete domain

SyntaxKind is the parser/lexer vocabulary for terminals,keywords,symbols and
AST nodes used by SyntaxNode,SyntaxSet,operators and parsing. Pure L1,no I/O/
global state/external dependencies. Historical source:
lab/typst-original/crates/typst-syntax/src/kind.rs.
Keep a unit enum deriving Debug/Copy/Clone/Eq/PartialEq/Hash with repr(u8);
discriminants must remain≤127 for SyntaxSet. Preserve the explicitly listed
domain; the archived heading's93-variant count is not a separate authority:

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    End, Error,

    Shebang, LineComment, BlockComment,

    Markup, Text, Space, Linebreak, Parbreak, Escape, Shorthand, SmartQuote,
    Strong, Emph, Raw, RawLang, RawDelim, RawTrimmed,
    Link, Label, Ref, RefMarker,
    Heading, HeadingMarker,
    ListItem, ListMarker, EnumItem, EnumMarker, TermItem, TermMarker,
    Equation,

    Math, MathText, MathIdent, MathShorthand, MathAlignPoint,
    MathDelimited, MathAttach, MathPrimes, MathFrac, MathRoot,

    Hash, LeftBrace, RightBrace, LeftBracket, RightBracket,
    LeftParen, RightParen,
    Comma, Semicolon, Colon,
    Star, Underscore, Dollar,
    Plus, Minus, Slash, Hat, Dot,
    Eq, EqEq, ExclEq, Lt, LtEq, Gt, GtEq,
    PlusEq, HyphEq, StarEq, SlashEq,
    Dots, Arrow, Root, Bang,

    Not, And, Or, None, Auto,

    Let, Set, Show, Context, If, Else, For, In, While,
    Break, Continue, Return, Import, Include, As,

    Code, Ident, Bool, Int, Float, Numeric, Str,
    CodeBlock, ContentBlock, Parenthesized,
    Array, Dict, Named, Keyed,
    Unary, Binary, FieldAccess, FuncCall, Args, Spread,
    Closure, Params,
    LetBinding, SetRule, ShowRule, Contextual,
    Conditional, WhileLoop, ForLoop,
    ModuleImport, ImportItems, ImportItemPath, RenamedImportItem,
    ModuleInclude, LoopBreak, LoopContinue, FuncReturn,
    Destructuring, DestructAssignment,
}
```

## Public classification methods

All take self; boolean classifiers are O(1).
is_trivia: Shebang/LineComment/BlockComment/Space/Parbreak.
is_keyword: Not/And/Or/None/Auto/Let/Set/Show/Context/If/Else/For/In/While/
Break/Continue/Return/Import/Include/As.
is_error: Error only. is_grouping: six brace/bracket/parenthesis kinds.
is_terminator: End/Semicolon/RightBrace/RightParen/RightBracket.
is_block: CodeBlock/ContentBlock.
is_stmt: LetBinding/SetRule/ShowRule/ModuleImport/ModuleInclude.
name(self)->&'static str returns readable diagnostics, including text,
syntax error and keyword `let`.

## Acceptance

Let/For/Return/Import are keywords; Text/Ident/Int are not.
Space/LineComment/Parbreak are trivia; Text is not. Error is_error true,
Text false. LeftBracket/RightParen group; Comma does not.
Text.name()=="text", Error.name()=="syntax error",
Let.name()=="keyword `let`". Compile-time SyntaxSet assertion enforces
<128 discriminants. Keep all documented variants/methods and colocated tests
with canonical lineage; do not drop variants to match an obsolete count.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/syntax_kind.md`. Historical observations are not a new parity certification.
