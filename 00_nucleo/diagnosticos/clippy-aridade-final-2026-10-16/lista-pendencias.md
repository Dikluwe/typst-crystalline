# Lista atual de pendências

Inventário Clippy: **17 avisos**, medido 2026-10-16T16:34:50.638757+00:00 UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Fontes/hashes/diffstat em [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16/validation-details.json). Aridade: zero avisos, com 48 expectativas nominais satisfeitas e 9 refatorações nos 57 casos originais; duas das expectativas exigiram retirar antes um argumento morto.

| Regra | Quantidade |
|---|---:|
| `dead_code` | 3 |
| `clippy::should_implement_trait` | 5 |
| `clippy::inherent_to_string` | 3 |
| `clippy::large_enum_variant` | 5 |
| `clippy::vec_box` | 1 |

Ordem sugerida: auditar dead_code/lifetimes; decidir semântica das traits/conversões; depois representação de enums e retenção. Remoção de campo retentor, Box por tamanho e troca de to_string/FromStr não são reparos automáticos. Mudanças públicas precisam de proposta concreta e gate ADR-0127.

## dead_code

- [03_infra/src/export/subset.rs:113](/repos/Antigravity/typst-crystalline/03_infra/src/export/subset.rs:113) — function `subset_font` is never used
- [03_infra/src/pipeline/context_stabilization.rs:25](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/context_stabilization.rs:25) — field `final_styles` is never read
- [03_infra/src/pipeline/context_stabilization.rs:31](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/context_stabilization.rs:31) — field `parent` is never read

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
