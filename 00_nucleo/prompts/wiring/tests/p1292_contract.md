# L0 — wiring/tests/p1292_contract
Hash do Código: 51235a5d

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1292_contract.rs`

## Current evidence

- `04_wiring/tests/p1292_contract.rs:16` — `Scratch`.
- `04_wiring/tests/p1292_contract.rs:19` — `new`.
- `04_wiring/tests/p1292_contract.rs:32` — `drop`.
- `04_wiring/tests/p1292_contract.rs:37` — `eval`.
- `04_wiring/tests/p1292_contract.rs:44` — `diagnostic`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1292_contract.rs`.

## Owned contract

- Own the black-box language/geometry suite only. Use isolated fixtures, propagate subprocess channels on failure and never turn Unknown into success.
- Preserve callbacks/cross/background/spans/no-provisional-output controls. Observe identity, morphology, dimensions, positions, presence and order; JSON/SVG/PDF are transports, not total-byte oracles.
- math.underline equality with underline must be JSON boolean false. SVG automatic-page geometry applies every ancestor translate/matrix in affine order once; no glyph/family/fixture-specific constants or defs/id-order comparisons.
- measure tests execute contextual measurement by rendering a uniquely colored shape, separating finite and height-auto regions; query/metadata/root are not execution substitutes. Preserve delimiter/alignment/repr/default/error controls.
- place.flush observes page, bbox and unique ordered tokens with explicit failure if pdftotext is absent. Cover prefix/suffix, no-flush, top/bottom floats, active-layouter nesting and no-float no-op. Keep tokens out of flow and remove fixtures.

## Verification and boundary

Run `cargo test -p typst --test p1292_contract -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
