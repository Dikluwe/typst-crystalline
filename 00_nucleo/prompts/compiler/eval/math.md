# L0 — compiler/eval/math
Hash do Código: a995795f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/math.rs`
**Atomização interna:** a fachada mantém imports e acesso de `eval_math_content`; os Prompts L0 individuais em `00_nucleo/prompts/compiler/eval/math/` possuem lookup, entrada, callee, argumentos, construção, expressão e delimitadores. O contrato abaixo permanece comum.
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math.rs:10` — `fn extract_math_named<'a>`.
- `01_core/src/compiler/eval/math.rs:53` — `fn lookup_math_op`.
- `01_core/src/compiler/eval/math.rs:66` — `fn lookup_math_catalog_submodule`.
- `01_core/src/compiler/eval/math.rs:98` — `fn unknown_variable_math`.
- `01_core/src/compiler/eval/math.rs:130` — `fn eval_math_content`.
- `01_core/src/compiler/eval/math.rs:177` — `fn eval_math_callee`.
- `01_core/src/compiler/eval/math.rs:260` — `fn eval_math_arg_value`.
- `01_core/src/compiler/eval/math.rs:369` — `fn eval_p1293_math_args`.
- `01_core/src/compiler/eval/math.rs:434` — `fn eval_p1431_mat_args`.
- `01_core/src/compiler/eval/math.rs:579` — `fn apply_p1293_math_constructor`.
- `01_core/src/compiler/eval/math.rs:600` — `fn eval_math_expr`.
- `01_core/src/compiler/eval/math.rs:1555` — `fn parse_delim_val`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/scope.rs:55-98`.
- `foundations/scope.rs:80-93`.
- `lab/typst-original/crates/typst-eval/src/call.rs:412-460`.
- `lab/typst-original/crates/typst-library/src/math/matrix.rs:18-68`.
- `typst-eval/src/call.rs:149-157`.
- `typst-eval/src/call.rs:149–157`.
- `typst-eval/src/call.rs:93-109`.
- `typst-eval/src/math.rs:45-54`.
- `typst-eval/src/math.rs:47-74`.
- `typst-library/src/foundations/scope.rs:80-92`.

## Contract

Own conversion of mathematical AST to Content, lexical math lookup and call
adapters. Preserve operator/symbol lookup, unknown-identifier math hints,
deprecation warnings, spans, attachments, delimiters, roots, fractions and
alignment. Never execute geometric layout in eval.

Syntax-absent attachment slots produce Omitted; actual attachments/primes produce
Present(content), including Empty. Syntax does not invent ExplicitNone where no
value exists. Consume the pinned tri-state attachment nucleus without inferring
presence from content. Syntax ^/_ uses t/b; primes directly attached to the base
use tr. Expr::MathPrimes constructs MathPrimes(count), not text. Preserve the AST
nesting for a primed base with top, primes inside top and primes after top,
rather than concatenating primes with top.
Grapheme math text constructs MathIdent; numeric math text constructs MathText.

Bare limits preserves causal inline_explicit: omitted inline uses true/false-bit,
explicit inline (including true) sets the bit; scripts has false bit. Bare root
requires index and radicand: empty index is Some(Empty)/explicit, evaluated none
is None/explicit, while sqrt is None/not-explicit. root(x) is not a sqrt alias.

eval_math_arg_value sends authorized code/scalar expressions, including Closure
and Contextual, through eval_expr without executing their deferred bodies.
A synthetic unit Math wrapper may be reparsed in Code mode only when exactly one
authorized code expression is recovered, anchored at the original span. Zero,
multiple or content expressions stay mathematical content. Preserve MathAlignPoint
in wrapped arguments of cases; the reparse must not consume & or classify it away.
Callback execution (such as cancel angle) stays with its existing pipeline owner.

Before hardcoded/native dispatch of a bare MathIdent callee, respect lexical
bindings. A local Func is called once through the canonical math argument builder
and retains the existing Content-only function-result contract. A local nonfunction
is displayed followed by positional mathematical arguments in parentheses; named
and spread fail with `named-argument syntax can only be used with functions` or
`spread-argument syntax can only be used with functions`, with the respective hints
``to render the colon as text, escape it: `foo\: 1` `` and
``to render the dots as text, add a space: `.. xs` ``. Preserve mathematical morphology
(the recorded witness is 7(1, 𝑥), not ASCII x). The separate vanilla span annotation
that a callee is not a function is not represented by the simplified diagnostic
carrier; do not fabricate a second hint or claim integral diagnostic parity.

Without a local binding, keep the existing native math routes. The catalog lookup
uses the unshadowed standard math namespace, so a local binding named math does
not hide chess/keyboard/ordinal. Lexically bound modules remain traversable;
the historical restriction on bare global modules does not reject them.
Standalone module roots do not turn into invented symbols/text. Bare global sym
still requires # or std.sym. Retain the legacy symbol aliases as explicit
crystalline compatibility extensions, with deprecations delegated to sym; absent
roots do not warn, invalid modifiers of deprecated roots preserve root warnings,
and the un-deprecated tack names remain silent.

Build Args from ordered occurrences with individual argument/value spans and the
real aggregate span, without evaluating expressions twice or reconstructing views.
Qualified and generic scope-resolved calls reuse transport_native_call_span from
00_nucleo/prompts/compiler/eval/call_dispatch.md for its closed typed identity set;
do not duplicate native identity recognition or encoder validation. Generic
builders without spread support do not acquire that support by metadata
transport alone. Canonical constructor adapters that already support spreads
retain them; do not generalize this into universal math spread parity.

Bare and qualified element calls converge by delegation to their canonical
stdlib builders, preserving casts, arity, unknown/duplicate named rejection,
spreads where supported and first-error/evaluation order. Do not silently discard
arguments or manufacture Empty for invalid arity. In particular:

- vec uses MathVec (not MathMatrix), ordered variadic children including zero/one;
  & stays in a child until layout. delim accepts the canonical pair/none/symbol/
  string forms, align is horizontal with Start/End retained, gap is relative.
  Defaults and presence are provided by its canonical constructor.
- attach/binom use their pure canonical constructors and retain all slots/lower
  children. frac uses the common parser; slash is left-associative and alone
  carries one causal bit per removed exterior parenthesis pair. Bare/qualified
  frac do not invent those bits.
- accent and symbolic wrappers (hat/tilde/dot/dot.double) use the common accent
  constructor/normalization; wrappers require a base and accept only size/dotless.
- cases preserves variadic children and MathAlignPoint, not pre-split rows or
  silently ignored casts. mat uses the common row algorithm: zero/scalar-only
  positionals form one row; any array requires every positional row to be array;
  shorter rows are padded with Empty. Cascades never manufacture explicit bits.
- lr is variadic with the canonical comma joining and optional size. Preserve
  nested lr rather than rewriting it prematurely into MathDelimited. mid requires
  its one body, without searching for fences or stretching during evaluation.
- norm/round/abs delegate with one body and optional size, producing MathLr with
  the canonical ‖/‖, ⌊/⌉ or |/| body sequence. floor/ceil/bar retain their own
  existing routes rather than acquiring this correction by generalization.
- overbrace/overbracket/overparen/overshell and their under counterparts retain
  each public variant, body, optional positional annotation and causal presence;
  no permanent reduction to MathUnderover or glyph-based identity. Mathematical
  overline delegates to the mathematical body-only constructor, not text overline.
- stretch delegates to its typed constructor; #size code reaches the relative
  cast while an ordinary mathematical body is not reinterpreted as a length.

Keep parser, AST, Args/Value public signatures, default target and pipeline phase
unchanged. Constructor schemas, render geometry and general public repr belong
to their respective owners, not duplicated parsers here.
