# L0 — html
Hash do Código: 2e73a56f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html.rs:27` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own HtmlSerializationMode, HtmlFrameResolver and the existing export_html,
export_html_with_serialization, export_html_with_serialization_and_pretty,
export_html_with_resolved_frames and mode method entrypoints. Preserve their public
paths/signatures and Crystalline/pretty=false defaults. Delegate document encoding
to the document child with the selected mode, resolver and formatting setting.

HTML is a semantic target, not conversion of PagedDocument. This façade does not
perform I/O, call World/layout or resolve frames; the caller supplies the existing
read-only resolver. Keep descendants private and current sibling visibility.
Each child below owns its specific behavior; do not duplicate its specification.

## Delegated owners

- `00_nucleo/prompts/infra/export/html/document.md` owns `03_infra/src/export/html/document.rs`.
- `00_nucleo/prompts/infra/export/html/encoding.md` owns `03_infra/src/export/html/encoding.rs`.
- `00_nucleo/prompts/infra/export/html/inline.md` owns `03_infra/src/export/html/inline.rs`.
- `00_nucleo/prompts/infra/export/html/pretty_serialization_tests.md` owns `03_infra/src/export/html/pretty_serialization_tests.rs`.
- `00_nucleo/prompts/infra/export/html/pretty.md` owns `03_infra/src/export/html/pretty.rs`.
- `00_nucleo/prompts/infra/export/html/table.md` owns `03_infra/src/export/html/table.rs`.
- `00_nucleo/prompts/infra/export/html/tags.md` owns `03_infra/src/export/html/tags.rs`.
- `00_nucleo/prompts/infra/export/html/tests.md` owns `03_infra/src/export/html/tests.rs`.
- `00_nucleo/prompts/infra/export/html/whitespace.md` owns `03_infra/src/export/html/whitespace.rs`.

## Owned test module declarations

This consumer declares the following separately owned test modules: `pretty_serialization_tests`. Their names describe their test responsibilities; each test module retains its own L0 owner. This module delegates the test bodies and does not absorb their ownership.
