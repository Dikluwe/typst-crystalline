# ADR-0126 — Production PDF streams and compact validation

**Status:** ACCEPTED
**Date:** 2026-08-03
**Decision owner:** Repository owner
**Consolidated:** 2026-11-24

## Evidence and amendment

The original exporter used per-item text blocks and relative placement. During P954/P955 the descriptions attached the labels “verbose” and “compact” to the wrong streams. In P956 the owner corrected the labels and designated verbose output as the production default. The current pipeline separately resolves stream mode, document options, serialization and accessible output. [00_nucleo/prompts/infra/pipeline/pdf.md](../prompts/infra/pipeline/pdf.md) · [00_nucleo/prompts/infra/export/stream.md](../prompts/infra/export/stream.md)

## Decision

The default stream mode is **verbose**. Its content operators mirror upstream semantics. Compare text placement matrices, graphics-state isolation, color transitions and text rendering mode directly with the pinned upstream PDF. For example, inspect `Tm`, the surrounding `q/cm/Q`, `cs/scn` and `Tr` in the circumstances where those operators apply.

The explicit **`--compact`** option selects the compact emitter. Validate it against the verified verbose output for the same document and settings. Compare glyph positions and ink, including any affected graphics-state transitions. The historical exporter used `Td` with an item-local text block. Compact encoding can evolve provided its overlay against verbose output and its own format contract hold. Global byte identity is not the acceptance measure for language parity. [00_nucleo/prompts/wiring/compile.md](../prompts/wiring/compile.md) · [00_nucleo/prompts/infra/export/stream/text.md](../prompts/infra/export/stream/text.md)

Accessibility and tagged PDF have independent requirements. Preserve the caller's distinction between absent, automatic and explicit choices when resolving tagging, page ranges, PDF standards and serialization. With a page selection, automatic tagging disables tags with the prescribed warning, explicit tagging fails, and explicit disabling stays silent at this resolution step. Accessible profiles impose their own restrictions after resolution. Keep these outcomes and the separate CLI diagnostics under their existing owners. A standard flag alone does not demonstrate full structural conformance. [00_nucleo/prompts/infra/pipeline/pdf.md](../prompts/infra/pipeline/pdf.md)

## PDF resource contracts

Font binding preserves the selected face and the glyph-to-CID mapping through subsetting and fallback. Paint support is specified per kind, including the chosen color space and the conic mesh representation. Reuse an allocation-backed resource only within its export lifetime and include any rendering context that changes the emitted resource in its key. The component owners carry the exact mappings and failure cases. [00_nucleo/prompts/infra/export/fonts.md](../prompts/infra/export/fonts.md) · [00_nucleo/prompts/infra/export/subset.md](../prompts/infra/export/subset.md) · [00_nucleo/prompts/infra/export/gradients/mod.md](../prompts/infra/export/gradients/mod.md) · [00_nucleo/prompts/infra/export/gradients/conic.md](../prompts/infra/export/gradients/conic.md) · [00_nucleo/prompts/infra/export/images/scan.md](../prompts/infra/export/images/scan.md)

## Alternatives and validation

Keeping only the compact emitter would remove the production reference for operator-level audits. Coupling accessibility to the compression choice would make a visual overlay appear to certify a document's semantic structure. Validate verbose against upstream first, compact against verbose next, and affected accessibility properties with their own tests. Changes to the default or public flags pass through ADR-0127.
