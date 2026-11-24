# ADR-0128 — HTML as a semantic target

**Status:** ACCEPTED
**Date:** 2026-08-23
**Feature amendment approved:** 2026-08-25 under ADR-0127
**Consolidated:** 2026-11-24

## Evidence

The initial implementation evaluated every document as paged. The reference distinguished the HTML target during evaluation and required explicit access to its experimental feature. The HTML pipeline and bundle owners now define separate evaluation, feature checks, contextual preparation and publication. [00_nucleo/prompts/infra/pipeline/html.md](../prompts/infra/pipeline/html.md) · [00_nucleo/prompts/infra/pipeline/bundle.md](../prompts/infra/pipeline/bundle.md)

## Decision

Compile an HTML document by evaluating its source with the HTML target, expanding semantic content and serializing HTML. Existing paged evaluation APIs keep their paged target. Converting a whole paged document, wrapping pages in SVG or PDF, or rasterizing the document discards semantic information and cannot implement this target. A supported variant preserves its language semantics. An unsupported variant produces a diagnostic. Translate the surface incrementally through owning L0s. [00_nucleo/prompts/infra/export/html.md](../prompts/infra/export/html.md)

An intrinsic `html.frame` is a local layout island inside semantic HTML. Preserve occurrence order, effective styles and nested-frame handling. Local rendering can produce an inline SVG and supply an intrinsic position for a captured location. It does not create a public HTML page. The complete document remains an HTML target even if it contains such islands. In a bundle, paged bodies advance the page offset, while HTML and assets consume no pages. [00_nucleo/prompts/infra/pipeline/html.md](../prompts/infra/pipeline/html.md) · [00_nucleo/prompts/infra/tests/html_frame.md](../prompts/infra/tests/html_frame.md) · [00_nucleo/prompts/infra/pipeline/bundle.md](../prompts/infra/pipeline/bundle.md)

Feature selection is independent of target and serialization. Features default to empty. An HTML extension, output format or serialization choice cannot enable `Feature::Html`. The public HTML binding, expansion and export require explicit enablement. A bundle containing HTML requires the applicable bundle and HTML features. The approved amendment requires `info` to report effective feature state. [00_nucleo/prompts/_nuclei/compiler-feature-gates.toml](../prompts/_nuclei/compiler-feature-gates.toml)

## Feature reporting and verification

The 2026-11-24 review observed divergent `features.html` reports for
`TYPST_FEATURES=html`. The info owners now project recognized names from the
environment snapshot while preserving the serialized field names. The compile,
watch and eval commands govern their own explicit feature selection. The
follow-up found that compilation accepts `--features html` and still ignores
`TYPST_FEATURES=html`. Its environment activation policy is a separate
compatibility decision. Verify disabled and enabled compilation, effective
reporting, unsupported variants and the intrinsic-frame exception through
their public entry points. [Prompt review and controls](../diagnostics/prompt-review-after-adr-consolidation-2026-11-24.pt-BR.md) · [00_nucleo/prompts/shell/info.md](../prompts/shell/info.md) · [00_nucleo/prompts/wiring/info.md](../prompts/wiring/info.md)
