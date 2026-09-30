# L0 — wiring/tests/p1468-format-pdf-pages
Hash do Código: ff46fc58

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1468_format_pdf_pages.rs`

## Current evidence

- `04_wiring/tests/p1468_format_pdf_pages.rs:30` — `ProcessResult`.
- `04_wiring/tests/p1468_format_pdf_pages.rs:37` — `success`.
- `04_wiring/tests/p1468_format_pdf_pages.rs:42` — `typst_binary`.
- `04_wiring/tests/p1468_format_pdf_pages.rs:48` — `shm_root`.
- `04_wiring/tests/p1468_format_pdf_pages.rs:58` — `fixture_paths`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1468_format_pdf_pages.rs`.

## Owned contract

- Own real-PDF page-selection integration: documentary integer/range, CLI list and explicit precedence, filtered page count and interactions with tagging. A beyond-end selection must not die by signal; its count is not fixed by this contract.
- Documentary explicit true with pages fails with `cannot enable tagged PDF and export a page range`; CLI true plus pages fails before compilation with distinct `cannot enable PDF tags when exporting a page range`, exit 1.
- Documentary Auto plus pages emits `exporting a page range disables PDF tagging` and the two canonical inaccessibility/silencing hints, producing no struct tree. Explicit false remains silent.
- Included headings survive native outline filtering; excluded headings disappear and included descendants reparent to the nearest included ancestor. An included outline must not become empty merely because pages are selected. Links to removed pages lose their annotation instead of redirecting; keep no-selection control.
- Inspect parsed counts/tree/titles/annotations, not full PDF bytes. Standard/conformity/PageLabels and PNG/SVG multi-output are outside this suite. Use explicit /dev/shm TMPDIR, no disk fallback, serial execution and bounded subprocesses.

## Verification and boundary

Run `cargo test -p typst --test p1468_format_pdf_pages -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
