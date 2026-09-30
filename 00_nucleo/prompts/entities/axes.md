# L0 — Generic axis pair
Hash do Código: fd124da2

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/axes.rs`

## Contract

`Axes<T>` has public `x: T` and `y: T`. `new(x, y)` preserves both values.
Debug, Clone, Copy and PartialEq are derived when supported by T; Eq requires
T: Eq. This pure generic pair does not acquire map/with helpers merely because
the upstream `lab/typst-original/crates/typst-library/src/layout/axes.rs`
has them. The assigned scope is the base carrier and constructor (ADR-0080).

Facade exposure belongs to `00_nucleo/prompts/entities/mod.md`. The former
`pub use axes::Axes` obligation is retained as an unresolved ownership/public
surface discrepancy in the sanitation diagnostic; absence in code does not
revoke it. This cleanup neither adds nor removes a reexport.
