# L0 — Explicit fallback font catalogs
Hash do Código: 0b381339

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fallback_fonts.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:300b35e6b7c70e79742d5edc4cda0f3419b15a41662e194fd1516b33b8cae1bb
```

## Current evidence

- `03_infra/src/fallback_fonts.rs` — the three catalog constants, `fallback_font_list_for` and `math_fallback_font_list`.
- `03_infra/src/shaper/font_selection.rs` and `03_infra/src/font_metrics/resolution.rs` — distinct selection consumers.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own ordered family catalogs and the prose class lookup only. Do not open font files, query coverage, score candidates, infer mathematical semantics or shape text.

| Catalog | Ordered families |
| --- | --- |
| Serif | `Liberation Serif`, `DejaVu Serif`, `Bitstream Vera Serif`, `FreeSerif` |
| Sans/default | `DejaVu Sans`, `Noto Sans`, `Liberation Sans`, `FreeSans`, `Arial` |
| Math | `New Computer Modern Math`, `Libertinus Serif`, `Twitter Color Emoji`, `Noto Color Emoji`, `Apple Color Emoji`, `Segoe UI Emoji` |
| Common text | `Libertinus Serif`, `Twitter Color Emoji`, `Noto Color Emoji`, `Apple Color Emoji`, `Segoe UI Emoji` |

`fallback_font_list_for(name)` lowercases the name and selects serif when it contains `serif`; every other name, including unknown and empty names, selects sans. `math_fallback_font_list()` returns the math catalog without filtering or reordering. Preserve `FreeSerif` last and the documented New Computer Modern family spelling.

`00_nucleo/prompts/infra/shaper/font_selection.md` owns run segmentation and candidate selection; `00_nucleo/prompts/infra/font_metrics/resolution.md` owns metric resolution. FontBook coverage/scoring remain L1. Shared obligations are supplied by the pinned fallback-selection nucleus, not duplicated here.

## Verification boundary

Check serif/sans classification and exact catalog order through the existing inline tests. No font availability or upstream-version assumption is needed to verify these constants.

## Common fallback-chain evidence

Upstream `lab/typst-original/crates/typst-library/src/text/mod.rs:1083–1101`
appends Libertinus Serif, Twitter Color Emoji, Noto Color Emoji, Apple Color Emoji
and Segoe UI Emoji after declared families when fallback is enabled. The historical
`correcao-rtl-restante-2026-09-29/after-leading/hebrew-0-vanilla.pdf` probe embeds
Libertinus for `שלום עולם`; Crystalline embeds Noto Sans Hebrew. Both catalogs
contain those faces. Inference: the explicit chain before global search explains
the difference, refuted if the embedded face persists after chain alignment. This
historical probe does not certify the current ratified oracle.

The common catalog preserves that exact text::families() order, separately from
class-recovery catalogs used when no declared family resolves and from the math
catalog. It does not select by language or script.
