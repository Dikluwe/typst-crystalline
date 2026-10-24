# L0 — compiler/stdlib/structural/math/binom
Hash do Código: 95d988c8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/binom.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `math.binom(upper, ..lower)` and bare bridge. Require upper plus at least one lower; reject named arguments in current order with the existing positional hint. Cast each value through the shared `p1293_math_content` cast in `00_nucleo/prompts/core/compiler/stdlib/structural/math/casts.md`. Join lower values with the current mathematical comma-space, construct an unlined fraction inside parenthesized MathDelimited, preserve current diagnostics and morphology. No layout.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.


## R4 supplementary measurement and exclusive correction

`lab/results/math-fields-r4-2026-10-23/before-expanded/receipt.json` retains
UTC/source-map/HEAD/diff stat and binary hashes against pinned bd5ebec2.
Binom lower None/integer/boolean and raw Symbol slots of attach differ;
upstream math/frac.rs:134–150 explicitly uses Value::display for lower.
These are language semantics/morphology, not Rust equality certification.

Use the existing p1293_math_content for the required upper operand. For variadic lower operands use existing eval::value_to_display_content(value.clone()).unwrap_or(Content::Empty), matching the measured display conversion for None, String, Symbol, integer and boolean. Preserve arity/named validation and semantic delimiter/fraction construction. The display helper retains its own bounded scope; do not certify arbitrary unmeasured Value variants.

This supersedes conflicting historical clauses only. Existing public fields,
trait declaration/signatures, defaults and pipeline phases stay unchanged;
ADR-0127 continuous parity flow, L0 before behavioral RED→GREEN, full validation
and lineage/full lint zero, preserving all received changes.
