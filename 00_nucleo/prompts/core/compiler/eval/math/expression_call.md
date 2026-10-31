# L0 — compiler/eval/math/expression_call
Hash do Código: 4f50744e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/selected-symbol-accent-call.toml sha256:c6ef1be7fbad9f9513c172b144b3dbe314ba24a6d4c8c09a26a9bca6c6f8b48b

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/expression_call.rs`. Internal extraction measured at HEAD `ce27af795` with a clean tree at 2026-09-30 23:12:44 UTC. The source is this unit's parent consumer; extraction is neither a bilateral parity measurement nor performance evidence.

The extracted logic was in `01_core/src/compiler/eval/math/expression.rs:138-913` at the indicated HEAD. This is an internal behavior extraction, not a new parity decision.

## Contract

Own lexical shadowing and qualified callees in the `Expr::FuncCall` arm. Delegate ordered native math function dispatch to `00_nucleo/prompts/core/compiler/eval/math/native_dispatch.md` and unknown/global/symbol fallback to `00_nucleo/prompts/core/compiler/eval/math/call_fallback.md`. Preserve argument evaluation order, spans, content morphology and the static dispatch order defined by the parent math prompt. Preserve the public contract, pipeline phase, default behavior and compatibility.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains from structural separation.


## Explicit Code calls / rich cancel stroke — 2026-10-29

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/ retains
math-code-stroke-before/receipt.json (fifteen bilateral cases) and
math-code-namespace-before/receipt.json (eight boundary cases), with actual
immutable build5/source, HEAD/dirty stat, UTC, fixture and executable hashes.
The oracle is64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17.
Actual AST evidence equation-scene2/{receipt.json,code-versus-math-call-roots.log}
uses the existing public API and build5 library: embedded Code calls have Ident
roots, including field chains; ordinary math calls have MathIdent roots.
The diagnostic's empty-font World is not render or font evidence.

### Exclusive correction and acceptance

Before ordinary math-call dispatch, descend only callee FieldAccess targets to
classify an existing Code Ident root. Evaluate that actual FuncCall through the
existing code evaluator, exactly once, preserving its standard argument order,
value types, feature/field authentication and source spans. Display the returned
value through the existing value_to_display_content route, like other embedded
Code expressions. This is the same evaluation phase, not a new mode or parser.
MathIdent-rooted calls retain their current lexical/math path and return policy;
never recognize Code by function name, namespace spelling or source substring.

The product currently loses a box width Binary's Length and rejects nested calc
and str results as content. Parenthesized/literal/local-width/alias controls
succeed; namespace probes preserve invalid Code body/global/math boundaries.
Co-located source tests require Binary and nested-call widths, embedded return
values and unchanged bare-math/lexical controls. This does not add Symbol call
support: qualified accent failures remain separately measured in
math-accent-alias-before/receipt.json and are not silently certified here.

Internal ADR-0127 continuous parity flow: owning L0 first, genuine RED before
code, GREEN/full core/lint/build and unchanged prior visual/history/JPEG gates.
No public field/method/signature, compatibility break or phase change.


## Selected-symbol accent callability — readiness 2026-10-29

### Measurement before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/accent-callability-findings6.json`
binds actual release6 source/archive, HEAD 212aed0449150dd296ceea13fc2be665cc5db01a,
uncommitted exact dirty-file stat, UTC and pinned oracle64c5e8c0. The original
14-case receipt's single-letter a is notation in BOTH engines; only the corrected
long-name before6b cohort proves local alias callability. Bare grave/arrow render
symbol+parentheses in Tekt, native accents; qualified and Code Symbol calls reject
Func in Tekt. Long local symbols lose accents/options/arity; beta/string notation
and noncallable Code rejection remain controls. accent-effects-before6/receipt.json
measures one seen.push effect in valid math/native Code controls, with Code Tekt
blocked at callee. No source/font/PDF-byte equality is language acceptance.

Pinned typst-library/src/foundations/symbol.rs:133-137 dispatches selected Symbol
through its accent function; math/accent.rs:212-290 owns strict aliases and the
single-base wrapper. Existing Func::native/with and call_dispatch's With merger
already transport bound arguments, final call span and occurrence order.
This is an internal semantic/morphological parity repair under ADR-0127 continuous
flow, not a public API/default/phase change.

### Exclusive amendment and acceptance

For an actually resolved local or qualified raw Value::Symbol callee only, ask the accent owner's compiler-private helper for its existing Func wrapper at actual call.span before evaluating arguments. If recognized, use the existing Func/math-argument/application branch. Otherwise retain the original value and existing notation/error route. Ordinary strings, single-letter notation, arbitrary Content, lexical shadowing and unknown/error-before-argument priority stay. Explicit Code roots continue through existing Code eval; its invocation owner handles that normalization. No new lookup/table/callback execution here. Co-located source RED→GREEN covers bare/qualified/local/Code accent morphology, options/arity/effects and nonaccent controls; one-letter a is not alias evidence.

L0 precedes actual functional RED, then implementation/GREEN; preserve old tests,
source-bound bilateral PDF/SVG/error/effect controls, full core/workspace, build,
strict lint/Clippy/rustdoc/fmt and both lineage directions. Validate V15/V26 before
resealing. No completion/performance/JPEG qualification from a finite call fix.
