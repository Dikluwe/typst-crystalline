# L0 — Generic axis pair
Hash do Código: 6dec3154

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/axes.rs`

## Contract

`Axes<T>` has public `x: T` and `y: T`. `new(x, y)` preserves both values.
Debug, Clone, Copy and PartialEq are derived when supported by T; Eq requires
T: Eq. This pure generic pair does not acquire map/with helpers merely because
the upstream `lab/typst-original/crates/typst-library/src/layout/axes.rs`
has them. The assigned scope is the base carrier and constructor (ADR-0080).

Facade exposure belongs to `00_nucleo/prompts/core/entities/mod.md`. The former
`pub use axes::Axes` obligation is retained as an unresolved ownership/public
surface discrepancy in the sanitation diagnostic; absence in code does not
revoke it. This cleanup neither adds nor removes a reexport.

## Copy and Clone coverage

### Evidence before specification

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, with an uncommitted working
tree, `01_core/src/entities/axes.rs:43-49` exercises Copy and calls Clone only
with f64, which is already Copy. The derived signature at line 17 must also
support T: Clone without Copy. Sources, L0, UTC, status and diffstat are in
`lab/results/construction/diagnostics/simplificacoes-ci-2026-10-13/before.json.gz`.
The earlier axes filter passed; Clippy identified a redundant fixture, not a
confirmed defect in Clone implementation.

### Acceptance

Keep the Axes<f64> Copy case. In the same test, exercise Clone with non-Copy
Axes<String>: preserve both values and verify that changing the copy's x
changes neither the original nor y. Do not replace Clone coverage with Copy
assignment or alter derives, fields, constructor, API, or reexports. This tests
the existing generic contract; it creates no new parity or phase decision.
Revalidate the core suite, build, and lineage.
