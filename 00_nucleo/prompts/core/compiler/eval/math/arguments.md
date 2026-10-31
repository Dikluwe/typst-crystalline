# L0 — compiler/eval/math/arguments
Hash do Código: cc088667

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/arguments.rs`. At HEAD `e754ab364a221dbe344852178e84bd42b02c463e` with a clean tree, `01_core/src/compiler/eval/math.rs:10-16,251-575` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contract

Parse named arguments, ordinary arguments and matrix rows while preserving AST order and spans. Preserve signatures, language semantics and diagnostics from `00_nucleo/prompts/core/compiler/eval/math.md`; preserve pipeline phase, defaults and compatibility. Extraction remains internal to L1, without I/O or mutable global state.

## Acceptance

The L1 suite and math tests must pass; `crystalline-lint .` must validate ownership and hashes. Do not infer performance gains.

## Mathematical namespace and explicit Code arguments

### Evidence before decision

`lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/before.json.gz`
shows bare-none/bare-align-left/bare-center resolving through Code/global where
the pinned vanilla produces unknown variable with math hints. code-none and
code-align-left are positive controls. Historical upstream
`lab/typst-original/crates/typst-library/src/foundations/scope.rs:81–100` searches
lexical scopes and base.math, using global only for diagnostics. Local
compiler/eval/math/arguments.rs:40–75 reparsed none as a literal; :115–130
redirected MathIdent to globals. code-auto also shows a Code Auto literal
converted to Content.

Syntax/semantics correction: a singleton MathIdent wrapper remains math, never
a reparsed Code literal. Only Code Ident uses the globals route; MathIdent keeps
lexical/math resolution and hints. Explicit Auto, like None/Bool, uses eval_expr
in both authorized arms. Preserve prior Code expressions, locals, content and
callbacks. The inference that those two routes cause the negative controls is
refuted if they still diverge after correction. No public or phase change.
Require RED→GREEN for bare-none/left and bilateral bare/#/local controls.

### Typed lexical values: refutation and current contract

local-refuters-before.json.gz in that diagnostic directory measures local-left/
local-none accepted by vanilla but rejected as content by Tekt; local-auto/
local-boolean report auto/boolean cast errors in vanilla and content in Tekt.
The collection uses the first repair binary
`a62789ae46d2dbc3ab9ecd0137a94f3ed18c6d44a2b54869b9b77f600d36cdd5`,
with complete sources/UTC/state. This refutes that preserving the locals'
display route preserves argument type. Upstream foundations/scope.rs:81–100
returns the real lexical Value.

MathIdent resolving a local binding returns its cloned Value without display
coercion before the cast. A singleton MathIdent wrapper delegates to the same
argument route, preserving the math/Code boundary and missing-name hints.
Local Content remains Content; None/Auto/Align/Bool retain type. Do not alter
general mathematical text resolution or globals. Require RED→GREEN for typed
locals and bilateral controls for the four refuters, preserving measured #,
bare and local Content controls. Historical evidence remains qualified by its
recorded pin and code state.


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


## Typed mathematical inputs — measured 2026-10-29

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/
math-input-findings-before-decision.json binds actual build3/source, pinned
oracle64c5e8c0, UTC, HEAD/dirty stat, exact sources and binary hashes. The fifteen
math-input-values-before cases isolate Ratio, Code local/loop/Func and escaped
accent failures with Length/Relative/literal/Math local/Content and Bool controls.
math-escape-callback-values-before uses a CONTENT-returning callback: an escaped
argument has type content versus native symbol; String stays str. Its markup
escape function is text versus symbol, while literal A stays text. The earlier
raw-Type-return callback has an independent return-display failure and does not
authorize changing callback return policy. All affected owners remain identical
to the archived actual build3.

### Exclusive correction and acceptance

In eval_math_arg_value, a singleton synthetic Math wrapper whose real sole child
is Code Ident or Escape delegates to the same typed argument route, in addition
to the existing singleton MathIdent. Code Ident and Escape use the existing
code evaluator exactly once, retaining actual Value and original span until the
constructor cast. MathIdent keeps lexical/math lookup and missing-name hints.
Do not reparse ordinary math bodies as new Code expressions; multi-expression
and alignment-point bodies retain their existing Content paths. Remove only
the superseded display diversion for Code Ident. Preserve quoted/numeric and
all other existing argument/row/spread behavior, ordering and effects.

Co-located tests cover Ratio and Func bindings, each loop iteration, typed
math-local/literal controls, existing Content rejection, escaped accent and a
content-returning callback observing raw Symbol. Strings remain strings and
prose accent arguments remain rejected. No callback-return-policy expansion.

Internal measured parity correction follows ADR-0127 continuous flow: L0 before
actual functional RED, implementation only after RED, GREEN and source-bound
full tests/build/fmt/Clippy/Rustdoc/lint/lineage. Preserve all earlier positive
visual/history and JPEG criteria; no normalization, suppression or limit change.
