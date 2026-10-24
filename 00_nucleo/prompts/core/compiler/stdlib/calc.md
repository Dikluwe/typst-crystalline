# L0 — stdlib/calc
Hash do Código: ea324e7d

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/calc.rs`

## Evidence before specification


`01_core/src/compiler/stdlib/calc.rs:56` (pub fn make_calc_module) -> Value {); `01_core/src/compiler/stdlib/calc.rs:133` (pub crate) fn calc_abs); `01_core/src/compiler/stdlib/calc.rs:265` (pub crate) fn calc_pow) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.



Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

After behavior atomization, this exclusive consumer owns make_calc_module composition and historical tests. Child owners under `00_nucleo/prompts/core/compiler/stdlib/calc/` implement abs/power-root/round/extrema/trig/hyperbolic/log/integer/combinatorics/norm/erf/helpers. The following numeric clauses are preserved composition boundary criteria, not extra productive ownership.

`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz` inventories uncalled bare_diagnostic at calc.rs:2835/p1334_tests with sources/owners/uses, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty state/UTC/stat; not new parity/performance evidence.

## Exclusive contract and numeric boundary criteria


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



Remove only uncalled bare_diagnostic from p1334_tests; used homonyms elsewhere stay. Keep inlined diagnostic assertions/fixtures/productive bindings, test names/bodies and productive paths. Internal ADR-0127 continuous maintenance changes no external API/default/phase.

## Acceptance

Preserve all stated semantics/syntax/morphology/diagnostics/spans/defaults/native identities, 1:1 owners and pins. Later scalar-owner revisions refine their own implementation obligations; boundary criteria cannot silently revoke approved later formulas. Revalidate workspace build/check all-targets/core suite/test inventory/Clippy/lineage without deleting expectations. No new signature/compatibility/default/phase is authorized by consolidation.


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
