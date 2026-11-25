# L0 — list item
Hash do Código: ff9282d0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/list_item.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Contract

Lay out a bullet item found outside a list container. The origin is the line start of
the active region. The item keeps its own marker, indents and tightness. Grouped items
are laid out by `compiler/layout/terms.md`. The body follows plan edge `ListItemBody`.

## Acceptance

Existing item tests; an item in an inset region starts at that region.

## Evidence

Approved on 2026-11-24 (“Aprovo A–G”): `00_nucleo/diagnostics/lists-package-2026-11-24.pt-BR.md`. The previous text and its
measurements are in commit `f2c737125`.
