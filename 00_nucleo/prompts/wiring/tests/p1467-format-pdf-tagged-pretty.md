# L0 — wiring/tests/p1467-format-pdf-tagged-pretty
Hash do Código: 82dadb97

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs`

## Current evidence

- `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs:27` — `ProcessResult`.
- `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs:34` — `success`.
- `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs:39` — `typst_binary`.
- `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs:45` — `shm_root`.
- `04_wiring/tests/p1467_format_pdf_tagged_pretty.rs:55` — `fixture_paths`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1467_format_pdf_tagged_pretty.rs`.

## Owned contract

- Own real-PDF integration for documentary tagged/pretty and CLI precedence only, not accessibility namespace, pages/standards or full conformity.
- Without decisions Auto/no-pages enables tags and compact serialization. Documentary true/false controls StructTreeRoot/MarkInfo; explicit pdf-tagged in either direction and legacy no-pdf-tags win.
- Documentary pretty and explicit CLI pretty affect relative structural whitespace only. tagged=false plus pretty=true remain independent; nested block rules fail with `format set rules are not allowed inside of containers`.
- Inspect pertinent PDF structure/relative newline behavior, not total bytes. Use explicit TMPDIR beneath /dev/shm with no disk fallback, serial test execution and finite subprocess timeouts; missing TMPDIR is harness configuration failure.

## Verification and boundary

Run `cargo test -p typst --test p1467_format_pdf_tagged_pretty -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
