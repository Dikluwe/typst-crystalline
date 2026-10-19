# L0 — compiler/stdlib/structural/math/cases
Hash do Código: 36dd7dcc

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/cases.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-cases-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:724-942` contains `MathCasesOptions`, delimiter cast/matching, option application, child conversion and the native. `math/matrix.rs:20,131` consumes the same delimiter cast. The `p1430` test filter passed 22 tests before movement. This is a structural regression baseline, not a timing result or a new bilateral vanilla measurement.

## Language contract

Own `math.cases` with variadic children, including zero, and named `delim`, `reverse` and `gap`. Preserve defaults, order, source spans, duplicate detection, cast errors and explicit-presence bits. Unknown named and `children:` remain errors. Convert Content, LocatedContent, String, Symbol and None children to the existing semantic Content, including MathIdent-to-MathText normalization; other types receive the existing content-cast error. Construct the complete `Content::MathCases` carrier without layout or splitting `&`.

Own the delimiter cast shared by cases and matrix. Accept none, a single valid delimiter in a string or symbol, or an array of exactly two delimiter values. Validate a single Unicode character using the existing opening/closing/fence classification, infer its matching pair with the current table and preserve the side when a closing delimiter is supplied. Array entries validate individually; empty, multiple-character and invalid delimiters retain their current diagnostics. The matrix owner calls this function directly and does not duplicate its policy. The layout cases consumer continues through the existing stdlib bridge.

`reverse` accepts boolean only. `gap` accepts Relative, Ratio or Length and retains the relative and absolute components. Named presence marks its bit even when the value equals the default. Set rules still supply cascaded values without marking explicit presence. The parent `math.rs` owns only static binding registration; the structural hub reexports `MathCasesOptions`, `math_cases_delim` and `apply_math_cases_option` directly from this child. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

The P1430 regression filter and full core suite remain green. A local regression verifies that a closing delimiter maps to its opening partner and retains its side. `cargo build` and `crystalline-lint .` have no new blocking violations. Header ownership is 1:1. No speedup is claimed without separate measurement.
