# L0 — wiring/tests/p1469-format-pdf-standard
Hash do Código: 5222e0fe

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1469_format_pdf_standard.rs`

## Current evidence

- `04_wiring/tests/p1469_format_pdf_standard.rs:30` — `ProcessResult`.
- `04_wiring/tests/p1469_format_pdf_standard.rs:37` — `success`.
- `04_wiring/tests/p1469_format_pdf_standard.rs:42` — `typst_binary`.
- `04_wiring/tests/p1469_format_pdf_standard.rs:48` — `shm_root`.
- `04_wiring/tests/p1469_format_pdf_standard.rs:58` — `fixture_paths`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1469_format_pdf_standard.rs`.

## Owned contract

- Own only standard language/version resolution and tagged/pages interaction in real-PDF subprocesses, not structural PDF/A/UA conformity or version-specific PDF rewrites.
- Check names/conflicts, explicit version, profile maximum version when absent, CLI over document and default 1.7. Examples: documentary 1.4 or a-1b gives header 1.4; a-2b gives 1.7; CLI 2.0/a-4 gives 2.0.
- Accessible standards with false tags preserve distinct documentary ``cannot set `pdf(tagged: false)` when exporting a {name} document`` versus precompile CLI `cannot disable PDF tags when exporting a PDF/A-1a document`. Ua1 plus CLI pages without explicit tagged uses the UA-1 variant and hint `using --pages implies --pdf-tagged=false`, exit 1.
- Conflicting CLI 1.4/1.7 fails before compilation with `PDF cannot conform to PDF 1.4 and PDF 1.7 at the same time`. Observe header prefix and diagnostics, never equality of the remaining bytes.
- Do not claim XMP profile identification, color/font/transparency rules or external conformity validation from these tests. Keep standalone other-format/tagged/pretty/pages suites separate. Use explicit /dev/shm TMPDIR without disk fallback, serial tests and finite process limits.

## Verification and boundary

Run `cargo test -p typst --test p1469_format_pdf_standard -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
