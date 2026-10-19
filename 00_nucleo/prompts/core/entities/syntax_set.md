# Prompt L0 — `entities/syntax_set`
Hash do Código: 75bc8564

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_set.rs`

## Contract

SyntaxSet is a u128 bitset of SyntaxKind u8 discriminants for O(1) parser/error
recovery membership. Supported discriminants are 0–127, checked with a
compile-time assertion. No I/O, heap allocation or external crates; depend
only on SyntaxKind. Inspired by rust-analyzer TokenSet; historical source:
`lab/typst-original/crates/typst-syntax/src/set.rs`.

`pub struct SyntaxSet(u128)` derives Default/Copy/Clone. All methods are const:
new()->Self yields 0; add(self,kind)->Self ORs a bit; remove(self,kind)->Self
ANDs its complement; union(self,other)->Self ORs sets;
contains(&self,kind)->bool checks a nonzero masked bit.
syntax_set! accepts comma-separated kind identifiers with an optional trailing
comma and constructs compile-time constants without runtime cost.

## Required constants

| Constant | Contents |
|-----------|---------|
| `STMT` | `Let, Set, Show, Import, Include, Return` |
| `MATH_EXPR` | Tokens that begin mathematical expressions |
| `CODE_EXPR` | `CODE_PRIMARY ∪ UNARY_OP` |
| `ATOMIC_CODE_EXPR` | `ATOMIC_CODE_PRIMARY` |
| `CODE_PRIMARY` | `ATOMIC_CODE_PRIMARY ∪ {Underscore}` |
| `ATOMIC_CODE_PRIMARY` | `Ident, LeftBrace, LeftBracket, LeftParen, Dollar, Let, Set, Show, Context, If, While, For, Import, Include, Break, Continue, Return, None, Auto, Int, Float, Bool, Numeric, Str, Label, Raw` |
| `UNARY_OP` | `Plus, Minus, Not` |
| `BINARY_OP` | `Plus, Minus, Star, Slash, And, Or, EqEq, ExclEq, Lt, LtEq, Gt, GtEq, Eq, In, PlusEq, HyphEq, StarEq, SlashEq` |
| `ARRAY_OR_DICT_ITEM` | `CODE_EXPR ∪ {Dots}` |
| `ARG` | `CODE_EXPR ∪ {Dots}` |
| `PARAM` | `PATTERN ∪ {Dots}` |
| `DESTRUCTURING_ITEM` | `PATTERN ∪ {Dots}` |
| `PATTERN` | `PATTERN_LEAF ∪ {LeftParen, Underscore}` |
| `PATTERN_LEAF` | `ATOMIC_CODE_EXPR` |


## Acceptance

new contains no kind; add(k) contains k; add(k).remove(k) does not.
Union of {Plus} and {Minus} contains both and not Star. Copying {Text} leaves
both original and copy containing Text. STMT contains Let/Set/Import and not
Text/Ident. UNARY_OP contains Not/Plus and not Star. CODE_EXPR contains
Ident/Not/Int. Preserve all constants and colocated #[cfg(test)] checks,
with lineage pointing to this owner.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/syntax_set.md`. Historical observations are not a new parity certification.
