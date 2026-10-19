# Prompt L0 — `entities/operators`
Hash do Código: 8b9a30b7

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

## Closed arity-map evolution

### Source evidence before classification

Inspection at clean HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
2026-10-19T10:22:03Z, reads `01_core/src/entities/operators.rs:22–29`
(UnOp::from_kind) and `:75–97` (BinOp::from_kind).
`01_core/src/entities/syntax_set.rs:136–142` independently lists their accepted
unary/binary tokens; `01_core/src/compiler/parse/code.rs:97–156` uses those
sets and composes Not+In separately. The local AST family in
`01_core/src/entities/ast/expr.rs:597–604,672–694` has the same current token
maps but remains a distinct declared type family owned by
`00_nucleo/prompts/core/entities/ast/expr.md`. Baseline occurrences are
V16-080/081 in
`lab/results/v16-cause-analysis-2026-10-19/entities-boundaries-inventory.json`.
This static cross-check supports an internal classification correction; it
neither measures new oracle parity nor infers intent from a neutral fallback.

### Preservation and exclusive decision

Close each from_kind SyntaxKind decision at its own site. The unary positive
map is Plus→Pos, Minus→Neg, Not→Not. The binary positive map is Plus→Add,
Minus→Sub, Star→Mul, Slash→Div, And→And, Or→Or, EqEq→Eq, ExclEq→Neq,
Lt→Lt, LtEq→Leq, Gt→Gt, GtEq→Geq, Eq→Assign, In→In,
PlusEq→AddAssign, HyphEq→SubAssign, StarEq→MulAssign, SlashEq→DivAssign.
Explicitly classify every other current SyntaxKind as not an operator of this
arity and return `None`. This domain includes ordinary AST/markup/math nodes,
literals, trivia, delimiters, end/error markers and code tokens serving other
roles. It is a valid negative arity result, not a global syntax rejection,
zero operator or justification to panic. Star remains nonunary and Not remains
nonbinary; neither map invents a single token for NotIn.

Preserve the existing precedence, spelling and assignment associativity
contract above, parser set admission and public types. The local AST maps may
share test expectations but must not be silently replaced by these types or
vice versa. This is ADR-0127 internal continuous flow with no new public/default/
phase/compatibility decision. Retain an explicit static closed match at each
site; no suppression, weaker contract or `if let`/`matches!`/helper evasion.
A current parser-admitted operator returning None, a nonoperator becoming
accepted, or changed language grouping refutes the preservation claim.

### Regression and exact-site enum-growth acceptance

Retain `unop_from_kind`, `unop_precedencia`, `binop_from_kind`,
`binop_precedencia_e_assoc` and `not_in_nao_tem_from_kind_directo`. Extend
coverage to all accepted mappings and independent negative End/Error/Space/
LineComment/Ident/Int/delimiter cases, with Star-unary and Not-binary controls.
Cross-check each accepted token with its arity's SyntaxSet and the AST-local
map using operator spellings/precedence rather than merging the public types.
Check language parsing of multiplication versus addition, unary Not versus
comparison, right-associating assignments and composed `not in` through the
existing parser entrypoint, with required successful AST extraction. Mapping
unit assertions alone do not establish preserved language grouping.

Use disposable complete copies of the actual workspace, retaining real
manifests, modules, enums, dependencies and function bodies. Add a fresh
SyntaxKind sentinel in each copy and compile the real crate. Do not extract
from_kind bodies, substitute enums, stub dependencies or use a toy harness.
Record that neither original wildcard reports an exhaustiveness failure at its
own site; after repair require independently attributable E0004 diagnostics
at both UnOp::from_kind and BinOp::from_kind matches. A failure at another
workspace match proves neither site, and a diagnostic at one from_kind match
cannot substitute for the other. If compilation cannot reach a required
diagnostic, that site's proof remains incomplete. No sentinel enters productive source. Record both workspace
snapshots, source digests, sentinel-only changes, compiler command/version and
exact site spans. Run regression controls on the original implementation before
changing it, then revalidate after repair. Require structural RED, both site
growth checks and valid ownership/nucleus preflight before restricted
resealing. Passing runtime controls alone do not prove closed-domain evolution.
These are enduring acceptance requirements, not executed results, and confer
no authority for global hash repair.

## Acceptance

UnOp from Plus/Minus/Not yields Pos/Neg/Not and Star yields None; Pos level 7,
Not level 4. BinOp from Plus/EqEq/Eq yields Add/Eq/Assign; Not yields None.
Mul binds more strongly than Add, Add than And. Add associates Left, Assign
Right. NotIn level 4/Left. Keep colocated tests, all specified implementations
and canonical owner lineage.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/operators.md`. Historical observations are not a new parity certification.
