# L0 — Explicit fallback font catalogs
Hash do Código: 0b381339

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fallback_fonts.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
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

## Cadeia comum de fallback — medição 2026-09-29

Medição: `lab/typst-original/crates/typst-library/src/text/mod.rs:1083-1101` acrescenta após as famílias declaradas, quando fallback está ativo, `Libertinus Serif`, `Twitter Color Emoji`, `Noto Color Emoji`, `Apple Color Emoji`, `Segoe UI Emoji`. A sonda `correcao-rtl-restante-2026-09-29/after-leading/hebrew-0-vanilla.pdf` incorpora Libertinus para `שלום עולם`; o PDF cristalino incorpora Noto Sans Hebrew. Ambos os catálogos listam essas faces. Inferência: a cadeia explícita anterior à busca global explica a diferença; refutar se a fonte incorporada persistir após a cadeia alinhada.

O catálogo comum tem exatamente a ordem da cadeia `text::families()` do pin vanilla. É distinto dos catálogos de recuperação por classe (usados quando nenhuma declarada resolve) e do catálogo matemático. Não escolhe a fonte por língua ou script.
