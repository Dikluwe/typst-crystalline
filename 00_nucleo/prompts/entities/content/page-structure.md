# L0 — entities/content/page-structure
Hash do Código: b30feba6

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/page_structure.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own structural page and column segmentation and `wrap_page_columns`.
Preserve source order, nested content, page markers, column markers, page
configuration, and degenerate-case normalization. The behavior is pure and
must not perform layout, measurement, rendering, or I/O.

## Acceptance

Existing page/column tests pass unchanged and the module includes local
tests.
