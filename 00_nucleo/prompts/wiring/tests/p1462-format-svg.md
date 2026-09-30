# L0 — wiring/tests/p1462-format-svg
Hash do Código: 37d50b05

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1462_format_svg.rs`

## Current evidence

- `04_wiring/tests/p1462_format_svg.rs:20` — `TempDir`.
- `04_wiring/tests/p1462_format_svg.rs:25` — `new`.
- `04_wiring/tests/p1462_format_svg.rs:42` — `drop`.
- `04_wiring/tests/p1462_format_svg.rs:48` — `ProcessResult`.
- `04_wiring/tests/p1462_format_svg.rs:55` — `Compilation`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1462_format_svg.rs`.

## Owned contract

- Own format.svg.pretty external-process tests: compact default, true/false/root/empty-rule cascade, explicit caller precedence, rejected containers/unknown named/type/direct construction.
- Check valid XML and unchanged semantic morphology between compact/pretty, without complete SVG hashes. Keep PNG/PDF/HTML/frame neutrality.
- Without html, sibling format.html produces its public feature gate; with html, only existence belongs here and its detailed surface belongs to `00_nucleo/prompts/wiring/tests/p1465-format-html.md`. Use isolated bounded fixtures; no administrative oracles.

## Verification and boundary

Run `cargo test -p typst --test p1462_format_svg -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
