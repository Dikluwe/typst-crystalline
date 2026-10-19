# L0 — Cardinal direction
Hash do Código: 07d9626b

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/dir.rs`

## Evidence

The consumer declares `Dir`; retained upstream evidence is
`lab/typst-original/crates/typst-library/src/layout/dir.rs:6-20` at the historical
`e0e8ca4d` snapshot. It establishes the four surface values and `stack(dir:)`,
not a fresh measurement against the current upstream pin.

## Contract

`Dir` is a closed `LTR | RTL | TTB | BTT` enum with `Debug`, `Clone`, `Copy`,
`PartialEq` and `Eq`. Default is `TTB`.
Pure methods `is_horizontal(self)`, `is_vertical(self)` and `is_reverse(self)`
return true respectively for LTR/RTL, TTB/BTT and RTL/BTT, in X-right/Y-down
coordinates. There is no internal `Auto` variant.

The entity's default must not silently define another consumer's automatic bidi
policy. The earlier inference that direct TTB is observationally equivalent to
upstream Smart direction remains unproved; an implicit-direction RTL stack
probe can refute it. Global/type-scope bindings belong to stdlib/eval owners,
and stack/page layout belongs to compiler owners, not this data type.
