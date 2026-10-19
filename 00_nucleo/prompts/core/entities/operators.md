# Prompt L0 — `entities/operators`
Hash do Código: 300fe547

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/operators.rs`
**Reference:** ADR-0004; parser/evaluator contracts remain separate owners.

## Evidence before precedence contract

Historical upstream e0e8ca4d scripting.typ:256-257 and its following language
table specify higher precedence binding more strongly. ast.rs:1810-1815 and
1930-1952 independently implement the same unary/binary levels;
ast.rs:1954 onward supplies associativity. Historical syntax/ast/code.rs and
foundations/ops.rs provide operator and semantic sources. This is language
precedence evidence, not a requirement for identical evaluation mechanics.
The old shared parser prompt header is superseded by this exclusive owner.

## Contract

UnOp, BinOp and Assoc derive Debug/Copy/Clone/Eq/PartialEq/Hash. Pure L1 data,
no evaluation logic/I/O/global state/external crates; only SyntaxKind.
Use qualified BinOp::/UnOp:: in eval, not wildcard imports that confuse V14.

UnOp variants Pos,Neg,Not expose from_kind(SyntaxKind)->Option<Self>,
precedence(self)->u8 and as_str(self)->&'static str.
BinOp variants Add,Sub,Mul,Div,And,Or,Eq,Neq,Lt,Leq,Gt,Geq,In,NotIn,
Assign,AddAssign,SubAssign,MulAssign,DivAssign expose the same methods plus
assoc(self)->Assoc. Assoc::{Left,Right}: all assignment variants associate
right, all other binary operators left.

| Level | Operators |
|---|---|
|7|Pos,Neg|
|6|Mul,Div|
|5|Add,Sub|
|4|Eq,Neq,Lt,Leq,Gt,Geq,In,NotIn, unary Not|
|3|And|
|2|Or|
|1|Assign,AddAssign,SubAssign,MulAssign,DivAssign|

NotIn has no single SyntaxKind; parser combines Not+In. BinOp::from_kind never
returns NotIn, but its precedence/associativity remain required.

## Acceptance

UnOp from Plus/Minus/Not yields Pos/Neg/Not and Star yields None; Pos level 7,
Not level 4. BinOp from Plus/EqEq/Eq yields Add/Eq/Assign; Not yields None.
Mul binds more strongly than Add, Add than And. Add associates Left, Assign
Right. NotIn level 4/Left. Keep colocated tests, all specified implementations
and canonical owner lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/operators.md`. Historical observations are not a new parity certification.
