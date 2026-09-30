# L0 — Page parity
Hash do Código: a3ef6c07

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/parity.rs`

## Contract

`Parity` is the pure closed enum `Even | Odd`, with Debug/Clone/Copy/PartialEq/Eq
and the existing hash support. `matches(self, page_number: usize) -> bool`
compares one-based page parity: page 1 is odd, page 2 even.
Even uses remainder zero modulo 2; Odd uses remainder one.
There is no internal Auto variant; an optional parity in a caller represents
automatic/no-adjustment versus an explicit parity. No ordering, page mutation
or running-matter algorithm is owned here.

The upstream architectural reference is
`lab/typst-original/crates/typst-library/src/layout/page.rs::Parity`;
Smart versus Option is representation mechanics, not a new parity measurement.
Verify both variants, copy/equality and matches on consecutive pages.
