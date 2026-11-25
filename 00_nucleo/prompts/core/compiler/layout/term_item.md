# L0 — term item
Hash do Código: 6557b6c1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/term_item.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Contract

Lay out a term item found outside a term list, from the line start of the active
region: strong term, separator, description. The default separator is a weak space of
0.6em; a list passes its own. Children follow `TermItemTerm` and
`TermItemDescription`.

## Acceptance

Existing item tests; an item in an inset region starts at that region.

## Evidence

Approved on 2026-11-24 (“Aprovo A–G”): `00_nucleo/diagnostics/lists-package-2026-11-24.pt-BR.md`. The previous text and its
measurements are in commit `f2c737125`.
