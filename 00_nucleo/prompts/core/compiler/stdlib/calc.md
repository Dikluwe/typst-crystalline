# L0 — stdlib/calc
Hash do Código: 18654592

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/calc.rs`

Após a atomização de 2026-09-30, este consumer possui a composição de `make_calc_module` e os testes históricos. Os Prompts L0 individuais em `00_nucleo/prompts/compiler/stdlib/calc/` possuem os comportamentos de abs, potência/raiz, arredondamento, extremos, trigonometria, hiperbólicas, logaritmos, aritmética inteira, combinatória, norma, erf e helpers numéricos. As cláusulas abaixo permanecem o contexto normativo compartilhado, sem mudar a linguagem.

## Evidence and classification

`01_core/src/compiler/stdlib/calc.rs:56` (pub fn make_calc_module) -> Value {); `01_core/src/compiler/stdlib/calc.rs:133` (pubcrate) fn calc_abs); `01_core/src/compiler/stdlib/calc.rs:265` (pubcrate) fn calc_pow) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own make_calc_module and its static bindings; the child prompts own scalar native implementations and numeric helpers, including return kinds, argument validation and diagnostics. Preserve the current module bindings and native identities; calc.abs's intrinsic name is abs through import/alias/With, not calc.abs. Keep deliberate crystalline log10/deg/rad extensions; log10 requires a positive number and deg/rad convert plain numeric radians/degrees. Removing them requires a compatibility decision.

### Absolute value and diagnostic precedence

Process the first positional value before rejecting leftovers. Int uses checked_abs, with overflow `the result is too large`; Float/Decimal keep their kind. Angle/Ratio/Fraction take scalar magnitude in the same kind, without context resolution or angle normalization. Length is supported only when its absolute or em component is zero; otherwise `cannot take absolute value of this length`, including same-sign mixed lengths. Abs does not use guard_float: constructed scalar NaN/infinities follow the local magnitude rule. Divergent construction of dimensional non-finites is not a parity witness.

Content/LocatedContent and all unsupported types fail with `expected integer, float, length, angle, ratio, fraction, or decimal, found TYPE`; String/Bool use string/boolean, with current long names for the other types. No numeric-string or content coercion. Value failures use only the first positional occurrence's value_span, including detached; never substitute args.span, occurrence.span, text search or a function-name position.

Without a positional, select the first named value occurrence even after another named: `the argument `value` is positional`, on its complete argument span, with the single hint `try removing `value:``. Otherwise `missing argument: value` uses the supplied args.span. After a valid positional, exclude only that occurrence and reject the first remaining occurrence in joint causal order: `unexpected argument` or `unexpected argument: NAME`, on its argument span. Synthetic None origins use positional-before-named views and detached individual spans. Preserve duplicate/order coherence; do not mutate Args. Eager argument-expression failures still precede native entry. Outer traces remain a dispatcher responsibility; no native trace/hints are added except the positional-name hint.

### Scalar functions and kinds

| Family | Contract |
| --- | --- |
| sin/cos/tan | One Int/Float/Angle, radians for numbers; Float result, existing guarded result policy. |
| asin/acos/atan | One number; Angle result. asin/acos require [-1,1]. |
| atan2 | Two numbers in the existing x,y public order, Angle result from the current y.atan2(x) mapping. |
| sinh/cosh/tanh/asinh | One number, Float result. |
| acosh/atanh | acosh requires x>=1; atanh requires -1<x<1; Float result. |
| exp/ln/log | ln/log require x>0. log defaults to base 10; named base and legacy second positional are mutually exclusive; finite base>0 and !=1. Exact e/2/10 branches use ln/log2/log10; otherwise ln(x)/ln(base). |
| pow | Reject 0^0; checked i32 integer exponent, non-normal Float exponent except zero rejected. Int/nonnegative Int gives checked Int; negative Int gives Float; Decimal/Int gives checked Decimal; Decimal/Float has its dedicated error+hint. Other numeric pairs use the guarded floating path. |
| sqrt/root | sqrt rejects negative input. root takes radicand then integer index; zero index fails; negative radicand requires odd index and yields a real negative root. |
| floor/ceil/trunc | Int identity, Float to Int by the existing conversion, Decimal to checked Int. |
| round/fract | Preserve Int/Float/Decimal kinds. round digits defaults to 0; integer positive precision is identity, negative precision and Float/Decimal use the current away-from-zero rounding. fract(Int) is Int zero; others retain their kind. |
| min/max/clamp | Nonempty numerical extrema; current mixed Int/Float promotion. clamp rejects reversed bounds. |
| rem/rem-euclid/div-euclid/quo | Preserve type/result and zero-divisor diagnostics. rem follows dividend sign; Euclidean operations keep their distinct definition; quo is floored, with checked Float range. |
| even/odd/gcd/lcm | Integer-only parity; gcd(0,0)=0; lcm with zero is zero and checked overflow. |
| fact/perm/binom | Nonnegative integers and checked arithmetic; k>n gives zero for perm/binom. |
| norm | Numerical variadic p-norm, default p=2. |
| erf | Existing finite approximation; input NaN fails, infinities give +/-1, signed zero is preserved. |
| pi/tau/e/inf | Existing Float constants, not functions. |

Reject unsupported arity, named arguments and types per each native; abs's origin-sensitive rules must not replace every helper's diagnostic policy. ADR-0101's categorical non-finite policy remains: only functions actually routed through guard_float reject NaN/Inf results. No blanket IEEE policy is introduced for operators or conversions.

The old round prose that converted all Float results to Int conflicts with its later kind-preserving contract; the later rule is retained. The old atan2/tan boundary examples are not renewed proof. Preserve exact current diagnostics and explicitly recorded residual math-resolution/constructor debts, rather than claiming general calc parity.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/stdlib/calc.rs` nas linhas 2835 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Remover somente bare_diagnostic sem chamadas no módulo de teste p1334_tests. Homônimos usados em outros módulos permanecem. Preservar testes, assertivas de diagnóstico inlined, fixtures e bindings produtivos.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.
