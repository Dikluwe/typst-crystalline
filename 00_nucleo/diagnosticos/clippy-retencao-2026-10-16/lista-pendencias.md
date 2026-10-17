# Pendências após retenção e código sem uso

Inventário: **13 avisos**, medido 2026-10-16T16:59:31.836226+00:00 UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; fontes/hashes/diffstat em [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/validation-details.json). dead_code e vec_box zerados; aridade e expectativas continuam aprovadas.

| Regra | Quantidade |
|---|---:|
| `clippy::should_implement_trait` | 5 |
| `clippy::inherent_to_string` | 3 |
| `clippy::large_enum_variant` | 5 |

Próxima decisão: auditar semântica dos cinco métodos sugeridos como traits e dos três formatadores; medir tamanho/uso dos enums antes de decidir Box. Contratos externos, defaults e fase continuam sujeitos à ADR-0127. Não converter automaticamente Option em Result, Location em Option<Location> ou mudar campos públicos para Box só para satisfazer Clippy.

## clippy::should_implement_trait

- [01_core/src/compiler/lexer/mod.rs:124](/repos/Antigravity/typst-crystalline/01_core/src/compiler/lexer/mod.rs:124) — method `next` can be confused for the standard trait method `std::iter::Iterator::next`. L0: [00_nucleo/prompts/compiler/lexer/mod.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/lexer/mod.md).
- [01_core/src/entities/locator.rs:62](/repos/Antigravity/typst-crystalline/01_core/src/entities/locator.rs:62) — method `next` can be confused for the standard trait method `std::iter::Iterator::next`. L0: [00_nucleo/prompts/entities/locator.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/locator.md).
- [01_core/src/entities/style.rs:270](/repos/Antigravity/typst-crystalline/01_core/src/entities/style.rs:270) — method `from_iter` can be confused for the standard trait method `std::iter::FromIterator::from_iter`. L0: [00_nucleo/prompts/entities/style.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/style.md).
- [01_core/src/entities/decimal.rs:30](/repos/Antigravity/typst-crystalline/01_core/src/entities/decimal.rs:30) — method `from_str` can be confused for the standard trait method `std::str::FromStr::from_str`. L0: [00_nucleo/prompts/entities/decimal.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/decimal.md).
- [01_core/src/entities/version.rs:83](/repos/Antigravity/typst-crystalline/01_core/src/entities/version.rs:83) — method `from_str` can be confused for the standard trait method `std::str::FromStr::from_str`. L0: [00_nucleo/prompts/entities/version.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/version.md).

## clippy::inherent_to_string

- [01_core/src/entities/decimal.rs:45](/repos/Antigravity/typst-crystalline/01_core/src/entities/decimal.rs:45) — implementation of inherent method `to_string(&self) -> String` for type `entities::decimal::Decimal`. L0: [00_nucleo/prompts/entities/decimal.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/decimal.md).
- [01_core/src/entities/duration.rs:97](/repos/Antigravity/typst-crystalline/01_core/src/entities/duration.rs:97) — implementation of inherent method `to_string(&self) -> String` for type `entities::duration::Duration`. L0: [00_nucleo/prompts/entities/duration.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/duration.md).
- [01_core/src/entities/version.rs:72](/repos/Antigravity/typst-crystalline/01_core/src/entities/version.rs:72) — implementation of inherent method `to_string(&self) -> String` for type `entities::version::Version`. L0: [00_nucleo/prompts/entities/version.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/version.md).

## clippy::large_enum_variant

- [01_core/src/compiler/eval/context_reads.rs:12](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/context_reads.rs:12) — large size difference between variants. L0: [00_nucleo/prompts/compiler/eval/context_reads.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval/context_reads.md).
- [01_core/src/compiler/layout/par_line_callbacks.rs:54](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/par_line_callbacks.rs:54) — large size difference between variants. L0: [00_nucleo/prompts/compiler/layout/par_line_callbacks.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/par_line_callbacks.md).
- [01_core/src/compiler/math/layout/callbacks.rs:56](/repos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/callbacks.rs:56) — large size difference between variants. L0: [00_nucleo/prompts/compiler/math/layout/callbacks.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/math/layout/callbacks.md).
- [01_core/src/entities/tag.rs:18](/repos/Antigravity/typst-crystalline/01_core/src/entities/tag.rs:18) — large size difference between variants. L0: [00_nucleo/prompts/entities/tag.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/tag.md).
- [01_core/src/entities/show.rs:101](/repos/Antigravity/typst-crystalline/01_core/src/entities/show.rs:101) — large size difference between variants. L0: [00_nucleo/prompts/entities/show.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/show.md).
