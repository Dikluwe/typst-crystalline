# L0 — compiler/stdlib/structural/math/frac
Hash do Código: 5b1e95ff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/frac.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree recorded in `lab/results/construction/diagnostics/atomizacao-stdlib-math-frac-2026-09-30-before.json.gz` contains the style cast, operand cast, constructor and native in `math.rs:291-391`; the static `math.frac` binding is later in the same parent. The P1427 filter passed 15 tests before movement. This is structural and regression evidence, not a new bilateral vanilla measurement or a timing claim. The historical language measurement remains in the parent L0 P1427 section.

## Language contract

Own the `math.frac(num, denom, style: vertical|skewed|horizontal)` argument validation and semantic `Content::MathFrac` construction. Require exactly two positional operands. Accept Content, LocatedContent, String converted to text, and None converted to empty content. Other operand types retain the current content-cast diagnostic. Validate `num:` and `denom:` as invalid named positionals before other named arguments, then reject unknown named arguments, duplicate `style`, missing/excess positional values, style values, and operand values in the existing order with unchanged spans and diagnostic wording.

`style` accepts only the three literal strings; absent remains absent, while an explicit `vertical` remains explicit in the complete carrier. Preserve the constructor's existing semantic flags and its morphology. No layout, dynamic dispatch or second runtime is introduced.

The parent `math.rs` owns only static binding registration for `math.frac`, calling this child's native function. The child retains the current native signature. The payload remains owned by `00_nucleo/prompts/core/entities/elements/math_frac.md`, with layout in `00_nucleo/prompts/core/compiler/math/layout/frac.md`. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

P1427 regressions and a local explicit-style test remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without measurement.


## R4 — measured public mathematical projection / qualified conversion

### Measurement before decision

`lab/results/math-fields-r4-2026-10-23/baseline.json` and `before/receipt.json`
retain UTC, HEAD, complete uncommitted diff stat, source map, archived sources
and binary SHA-256. The oracle is pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
(tree cac836c07e4dec2a89219370428292393a3216b5). The before probe records
fields/has/at and accepted/rejected constructor values rather than inferring
intent from implementation. Upstream math/matrix.rs:45–67,238–272 and
math/mod.rs:227–234 declare the already existing language fields. Local
content_methods.rs:151–158,378–380 omits these projections; frac.rs:43–53
and root.rs:27–38 reject supported operands. These are language semantics
and morphology, distinct from byte or Rust algorithm parity.

### Exclusive correction and gate

Delegate numerator and denominator conversion to the existing casts::math_lr_content_value; keep arity/style validation and semantic flags.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.


## Causal named style consumption — measured parity repair 2026-10-29

### Measurement before decision

math-frac-arguments-before2/receipt.json under readiness-fixes-2026-10-28 binds
immutable table-alignment-build1/source archive, HEAD/dirty inputs/UTC, sources
and pinned64c5e8c0 oracle. A real multi-letter frac alias with a bound vertical
style and call-site horizontal is accepted by native, rejected as duplicate by
Tekt; two successive with bindings have the same split. Literal duplicate style
is rejected by both. A bound boolean followed by a valid override must still
fail the boolean cast at its original expression: removing bound occurrences
would be wrong. The initial before probe's one-letter alias/named-math fixtures
fail with unknown-variable style in both and are not semantic evidence.

Source frac.rs:64 counts all causal style occurrences as literal duplicates.
Current parser math.rs:480 and patterns.rs:279 already reject duplicate literal
names inside each actual source argument list. call_dispatch/trace_plugin.rs:99
correctly appends With occurrences; entities/args.md requires keeping every value
and native foundations/args.rs:222 casts every named occurrence before retaining
the last. Language casting/order/morphology/errors govern this decision, not
copying Rust Args mechanics. No change to Args, Func, dispatch or parser is needed.

### Exclusive bounded correction

Keep current named-positionals/unknown/arity/operand validation. Remove only the
constructor-level duplicate style count: source syntax still rejects literal
duplicates before execution. Consume each style occurrence in its retained order,
cast it once through math_frac_style, fail the first invalid cast, and retain the
last valid style. Real value_span owns a failure; absent occurrence origin falls
back to the existing aggregate args.span, without inventing Source or offsets.
For legacy synthetic Args without occurrences use the existing occurrence_sequence
projection and same fallback; do not mutate Args or discard losing values.
Absent style remains None; explicit vertical stays explicit; default styling,
fields/morphology, operand conversion and native signatures remain unchanged.

This supersedes only the old constructor duplicate-count and style-value-origin
clauses. ADR-0127 continuous internal parity correction: no new public fields,
trait methods, signatures, dependency, mode/default or pipeline phase. L0 first,
co-located actual-source RED→GREEN for bound/chained overrides, invalid bound
origin, literal duplicate rejection and once-only argument evaluation. Preserve
all previous tests and general merge_with_args behavior. Rebuild the original
public frac document and retain any later geometry/other negative after removing
this rejection; full core, twelve gates, historical/JPEG and strict lineage/lint
are required on final stable sources. Other math constructors remain separate.
