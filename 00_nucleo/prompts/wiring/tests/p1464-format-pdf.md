# L0 — wiring/tests/p1464-format-pdf
Hash do Código: 42bd80e3

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1464_format_pdf.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
```

## Current evidence

- `04_wiring/tests/p1464_format_pdf.rs:16` — `Profile`.
- `04_wiring/tests/p1464_format_pdf.rs:38` — `ProcessResult`.
- `04_wiring/tests/p1464_format_pdf.rs:45` — `success`.
- `04_wiring/tests/p1464_format_pdf.rs:51` — `Checks`.
- `04_wiring/tests/p1464_format_pdf.rs:56` — `require`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1464_format_pdf.rs`.

## Owned contract

- Own public format.pdf namespace integration only, across default, a11y-extras, html and combined profiles. Verify type function, repr pdf, defaults pages none/standard ("1.7",)/tagged auto/pretty false, rejected manual construction and alias/std identity.
- attach/artifact are the same short-named functions as pdf.attach/pdf.artifact. The accessibility trio table-summary/header-cell/data-cell is jointly gated by a11y-extras, with exact disabled-feature message, ordered hints, field-only span and empty stdout; direct/with/alias/std paths preserve the gate.
- Enabled members remain discoverable through with. A same-named user closure does not receive special feature diagnostics. Global pdf is function with repr pdf, not the obsolete module morphology, retaining its members/gates.
- Keep svg/png defaults and html sibling gate/existence controls; do not test documentary export, PDF/A/UA conformity, pages/tagging/pretty export here. Private module/foreign-function pointer identity and insertion order are not oracles.
- Use finite subprocess limits and isolated task fixtures under /dev/shm without disk fallback. Missing capacity is an explicit inability to verify, never success.

## Verification and boundary

Run `cargo test -p typst --test p1464_format_pdf -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
