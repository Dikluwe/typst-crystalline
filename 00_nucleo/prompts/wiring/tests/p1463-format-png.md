# L0 — wiring/tests/p1463-format-png
Hash do Código: 079b8f39

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1463_format_png.rs`

## Current evidence

- `04_wiring/tests/p1463_format_png.rs:20` — `TempDir`.
- `04_wiring/tests/p1463_format_png.rs:25` — `new`.
- `04_wiring/tests/p1463_format_png.rs:42` — `drop`.
- `04_wiring/tests/p1463_format_png.rs:48` — `ProcessResult`.
- `04_wiring/tests/p1463_format_png.rs:55` — `Compilation`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1463_format_png.rs`.

## Owned contract

- Own format.png integration, reading IHDR directly: documentary default 144 PPI gives A4 1191 by 1684; both document and CLI resolve through effective_ppi/72, with explicit CLI precedence and equal-PPI equal dimensions.
- Cover integer/float/cascade/empty rule/errors/container rejection, finite zero/negative safe minimum dimensions, public compile/watch flag aliases and identical font/no-font resolution.
- Keep other formats/pretty/compact/tags/features neutral and html sibling feature/existence boundaries separate. Do not compare compressed bytes, metadata, file size or hashes, or risk OOM with nonfinite input. Signal/timeout/missing artifact is failure, not success.

## Verification and boundary

Run `cargo test -p typst --test p1463_format_png -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
