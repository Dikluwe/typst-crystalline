# L0 — compiler/eval/math/arguments
Hash do Código: 73545190

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
