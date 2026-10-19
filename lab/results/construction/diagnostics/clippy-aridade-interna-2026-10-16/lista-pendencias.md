# Pendências após aridade interna

Inventário atual validado em 2026-10-16T16:12:16.258718+00:00 UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/validation-details.json) conserva fontes/hashes e diffstat. Total: **22**.

| Regra | Casos |
|---|---:|
| `dead_code` | 3 |
| `clippy::too_many_arguments` | 5 |
| `clippy::should_implement_trait` | 5 |
| `clippy::inherent_to_string` | 3 |
| `clippy::large_enum_variant` | 5 |
| `clippy::vec_box` | 1 |

As cinco aridades ainda a corrigir são C014 (Grid/Table), C021 (indentação de outline) e C067/C069/C071 (sessão SVG). Continuam visíveis e não foram silenciadas. Contratos públicos novos exigem proposta concreta e ratificação ADR-0127; os defaults, flags e fases permanecem protegidos.

## dead_code

- [03_infra/src/export/subset.rs:113](/repos/Antigravity/typst-crystalline/03_infra/src/export/subset.rs:113) — function `subset_font` is never used
- [03_infra/src/pipeline/context_stabilization.rs:25](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/context_stabilization.rs:25) — field `final_styles` is never read
- [03_infra/src/pipeline/context_stabilization.rs:31](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/context_stabilization.rs:31) — field `parent` is never read

## clippy::too_many_arguments

- [01_core/src/compiler/layout/grid.rs:164](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/grid.rs:164) — this function has too many arguments (13/7)
- [01_core/src/compiler/stdlib/structural/outline.rs:340](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/outline.rs:340) — this function has too many arguments (9/7)
- [03_infra/src/export/svg/groups.rs:16](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/groups.rs:16) — this function has too many arguments (14/7)
- [03_infra/src/export/svg/navigation.rs:83](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/navigation.rs:83) — this function has too many arguments (12/7)
- [03_infra/src/export/svg.rs:227](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg.rs:227) — this function has too many arguments (9/7)

## clippy::should_implement_trait

- [01_core/src/compiler/lexer/mod.rs:124](/repos/Antigravity/typst-crystalline/01_core/src/compiler/lexer/mod.rs:124) — method `next` can be confused for the standard trait method `std::iter::Iterator::next`
- [01_core/src/entities/locator.rs:62](/repos/Antigravity/typst-crystalline/01_core/src/entities/locator.rs:62) — method `next` can be confused for the standard trait method `std::iter::Iterator::next`
- [01_core/src/entities/style.rs:270](/repos/Antigravity/typst-crystalline/01_core/src/entities/style.rs:270) — method `from_iter` can be confused for the standard trait method `std::iter::FromIterator::from_iter`
- [01_core/src/entities/decimal.rs:30](/repos/Antigravity/typst-crystalline/01_core/src/entities/decimal.rs:30) — method `from_str` can be confused for the standard trait method `std::str::FromStr::from_str`
- [01_core/src/entities/version.rs:83](/repos/Antigravity/typst-crystalline/01_core/src/entities/version.rs:83) — method `from_str` can be confused for the standard trait method `std::str::FromStr::from_str`

## clippy::inherent_to_string

- [01_core/src/entities/decimal.rs:45](/repos/Antigravity/typst-crystalline/01_core/src/entities/decimal.rs:45) — implementation of inherent method `to_string(&self) -> String` for type `entities::decimal::Decimal`
- [01_core/src/entities/duration.rs:97](/repos/Antigravity/typst-crystalline/01_core/src/entities/duration.rs:97) — implementation of inherent method `to_string(&self) -> String` for type `entities::duration::Duration`
- [01_core/src/entities/version.rs:72](/repos/Antigravity/typst-crystalline/01_core/src/entities/version.rs:72) — implementation of inherent method `to_string(&self) -> String` for type `entities::version::Version`

## clippy::large_enum_variant

- [01_core/src/compiler/eval/context_reads.rs:12](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/context_reads.rs:12) — large size difference between variants
- [01_core/src/compiler/layout/par_line_callbacks.rs:54](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/par_line_callbacks.rs:54) — large size difference between variants
- [01_core/src/compiler/math/layout/callbacks.rs:56](/repos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/callbacks.rs:56) — large size difference between variants
- [01_core/src/entities/tag.rs:18](/repos/Antigravity/typst-crystalline/01_core/src/entities/tag.rs:18) — large size difference between variants
- [01_core/src/entities/show.rs:101](/repos/Antigravity/typst-crystalline/01_core/src/entities/show.rs:101) — large size difference between variants

## clippy::vec_box

- [03_infra/src/pipeline/context_stabilization.rs:54](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/context_stabilization.rs:54) — `Vec<T>` is already on the heap, the boxing is unnecessary
