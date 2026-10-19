# Pendências Clippy após Default

Inventário medido na validação 2026-10-16T14:59:40.108554+00:00 UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Fontes, hashes, diffstat e comandos: [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-default-2026-10-16/validation-details.json). Total: **91**. Ordem de triagem mantém Clippy local → decisões Tekt → estrutura/contratos; não autoriza alterações de API.

| Regra | Casos |
|---|---:|
| `clippy::derivable_impls` | 3 |
| `clippy::map_entry` | 1 |
| `clippy::items_after_test_module` | 2 |
| `clippy::unnecessary_literal_unwrap` | 1 |
| `dead_code` | 8 |
| `clippy::single_range_in_vec_init` | 4 |
| `clippy::mutable_key_type` | 1 |
| `clippy::too_many_arguments` | 57 |
| `clippy::should_implement_trait` | 5 |
| `clippy::inherent_to_string` | 3 |
| `clippy::large_enum_variant` | 5 |
| `clippy::vec_box` | 1 |

Os 57 too_many_arguments conservam a classificação anterior (11 correções propostas e 46 exceções justificadas); nenhuma API dessa frente foi alterada aqui. Os demais grupos exigem auditoria de seus owners antes de decidir correção ou exceção. Não transformar Range em seus elementos, nem remover retenção causal apenas porque um campo não tem leitura explícita.

## clippy::derivable_impls

- [03_infra/src/export/svg.rs:47](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg.rs:47) — this `impl` can be derived
- [03_infra/src/export/mod.rs:201](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:201) — this `impl` can be derived
- [03_infra/src/export/mod.rs:224](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:224) — this `impl` can be derived

## clippy::map_entry

- [03_infra/src/shaper/cache.rs:88](/repos/Antigravity/typst-crystalline/03_infra/src/shaper/cache.rs:88) — usage of `contains_key` followed by `insert` on a `HashMap`

## clippy::items_after_test_module

- [03_infra/src/pipeline/html.rs:147](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/html.rs:147) — items after a test module
- [03_infra/src/pipeline/svg.rs:160](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/svg.rs:160) — items after a test module

## clippy::unnecessary_literal_unwrap

- [03_infra/src/export/tests/mod.rs:6083](/repos/Antigravity/typst-crystalline/03_infra/src/export/tests/mod.rs:6083) — used `unwrap()` on `Some` value

## dead_code

- [03_infra/src/export/subset.rs:113](/repos/Antigravity/typst-crystalline/03_infra/src/export/subset.rs:113) — function `subset_font` is never used
- [03_infra/src/pipeline/context_stabilization.rs:25](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/context_stabilization.rs:25) — field `final_styles` is never read
- [03_infra/src/pipeline/context_stabilization.rs:31](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/context_stabilization.rs:31) — field `parent` is never read
- [03_infra/src/watch.rs:170](/repos/Antigravity/typst-crystalline/03_infra/src/watch.rs:170) — field `live` is never read
- [04_wiring/tests/p1469_format_pdf_standard.rs:32](/repos/Antigravity/typst-crystalline/04_wiring/tests/p1469_format_pdf_standard.rs:32) — field `stdout` is never read
- [04_wiring/tests/p1468_format_pdf_pages.rs:32](/repos/Antigravity/typst-crystalline/04_wiring/tests/p1468_format_pdf_pages.rs:32) — field `stdout` is never read
- [04_wiring/tests/p1467_format_pdf_tagged_pretty.rs:29](/repos/Antigravity/typst-crystalline/04_wiring/tests/p1467_format_pdf_tagged_pretty.rs:29) — field `stdout` is never read
- [03_infra/src/plugin_host/tests.rs:14](/repos/Antigravity/typst-crystalline/03_infra/src/plugin_host/tests.rs:14) — constant `I64` is never used

## clippy::single_range_in_vec_init

- [03_infra/src/fonts/coverage.rs:290](/repos/Antigravity/typst-crystalline/03_infra/src/fonts/coverage.rs:290) — an array of `Range` that is only one element
- [03_infra/src/fonts/coverage.rs:294](/repos/Antigravity/typst-crystalline/03_infra/src/fonts/coverage.rs:294) — an array of `Range` that is only one element
- [03_infra/src/fonts/coverage.rs:403](/repos/Antigravity/typst-crystalline/03_infra/src/fonts/coverage.rs:403) — a `Vec` of `Range` that is only one element
- [03_infra/src/fonts/coverage.rs:408](/repos/Antigravity/typst-crystalline/03_infra/src/fonts/coverage.rs:408) — an array of `Range` that is only one element

## clippy::mutable_key_type

- [01_core/src/entities/counter.rs:70](/repos/Antigravity/typst-crystalline/01_core/src/entities/counter.rs:70) — mutable key type

## clippy::too_many_arguments

- [01_core/src/compiler/eval/mod.rs:688](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:688) — this function has too many arguments (8/7)
- [01_core/src/compiler/eval/mod.rs:715](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:715) — this function has too many arguments (9/7)
- [01_core/src/compiler/eval/mod.rs:740](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/mod.rs:740) — this function has too many arguments (10/7)
- [01_core/src/compiler/introspect/walk.rs:91](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:91) — this function has too many arguments (8/7)
- [01_core/src/compiler/introspect/walk.rs:121](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/walk.rs:121) — this function has too many arguments (10/7)
- [01_core/src/compiler/introspect/heading.rs:147](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/heading.rs:147) — this function has too many arguments (8/7)
- [01_core/src/compiler/layout/convergence.rs:9](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/convergence.rs:9) — this function has too many arguments (8/7)
- [01_core/src/compiler/layout/page_geometry.rs:9](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/page_geometry.rs:9) — this function has too many arguments (8/7)
- [01_core/src/compiler/layout/grid.rs:164](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/grid.rs:164) — this function has too many arguments (13/7)
- [01_core/src/compiler/layout/paragraph_select.rs:94](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/paragraph_select.rs:94) — this function has too many arguments (8/7)
- [01_core/src/compiler/layout/break_selection.rs:27](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/break_selection.rs:27) — this function has too many arguments (8/7)
- [01_core/src/compiler/layout/mod.rs:1911](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/mod.rs:1911) — this function has too many arguments (8/7)
- [01_core/src/compiler/realize/mod.rs:127](/repos/Antigravity/typst-crystalline/01_core/src/compiler/realize/mod.rs:127) — this function has too many arguments (8/7)
- [01_core/src/compiler/stdlib/structural/outline.rs:340](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/outline.rs:340) — this function has too many arguments (9/7)
- [01_core/src/entities/content/construction/layout.rs:177](/repos/Antigravity/typst-crystalline/01_core/src/entities/content/construction/layout.rs:177) — this function has too many arguments (9/7)
- [03_infra/src/export/gradients/mod.rs:320](/repos/Antigravity/typst-crystalline/03_infra/src/export/gradients/mod.rs:320) — this function has too many arguments (9/7)
- [03_infra/src/export/render.rs:349](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:349) — this function has too many arguments (9/7)
- [03_infra/src/export/render.rs:388](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:388) — this function has too many arguments (8/7)
- [03_infra/src/export/render.rs:839](/repos/Antigravity/typst-crystalline/03_infra/src/export/render.rs:839) — this function has too many arguments (10/7)
- [03_infra/src/export/stream/text.rs:340](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:340) — this function has too many arguments (9/7)
- [03_infra/src/export/stream/text.rs:365](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:365) — this function has too many arguments (9/7)
- [03_infra/src/export/stream/text.rs:390](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:390) — this function has too many arguments (10/7)
- [03_infra/src/export/stream/text.rs:531](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:531) — this function has too many arguments (9/7)
- [03_infra/src/export/stream/text.rs:717](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:717) — this function has too many arguments (9/7)
- [03_infra/src/export/stream/text.rs:742](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:742) — this function has too many arguments (9/7)
- [03_infra/src/export/stream/text.rs:767](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:767) — this function has too many arguments (10/7)
- [03_infra/src/export/stream/text.rs:893](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream/text.rs:893) — this function has too many arguments (9/7)
- [03_infra/src/export/stream.rs:222](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:222) — this function has too many arguments (9/7)
- [03_infra/src/export/stream.rs:253](/repos/Antigravity/typst-crystalline/03_infra/src/export/stream.rs:253) — this function has too many arguments (11/7)
- [03_infra/src/export/svg/geometry.rs:31](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/geometry.rs:31) — this function has too many arguments (9/7)
- [03_infra/src/export/svg/glyphs.rs:286](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/glyphs.rs:286) — this function has too many arguments (9/7)
- [03_infra/src/export/svg/groups.rs:16](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/groups.rs:16) — this function has too many arguments (14/7)
- [03_infra/src/export/svg/images.rs:14](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/images.rs:14) — this function has too many arguments (8/7)
- [03_infra/src/export/svg/navigation.rs:83](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg/navigation.rs:83) — this function has too many arguments (12/7)
- [03_infra/src/export/svg.rs:233](/repos/Antigravity/typst-crystalline/03_infra/src/export/svg.rs:233) — this function has too many arguments (9/7)
- [03_infra/src/export/mod.rs:445](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:445) — this function has too many arguments (8/7)
- [03_infra/src/export/mod.rs:554](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:554) — this function has too many arguments (8/7)
- [03_infra/src/export/mod.rs:671](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:671) — this function has too many arguments (8/7)
- [03_infra/src/export/mod.rs:809](/repos/Antigravity/typst-crystalline/03_infra/src/export/mod.rs:809) — this function has too many arguments (8/7)
- [03_infra/src/pipeline/bundle.rs:383](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/bundle.rs:383) — this function has too many arguments (10/7)
- [03_infra/src/pipeline/pdf.rs:121](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:121) — this function has too many arguments (8/7)
- [03_infra/src/pipeline/pdf.rs:376](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:376) — this function has too many arguments (12/7)
- [03_infra/src/pipeline/pdf.rs:548](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:548) — this function has too many arguments (9/7)
- [03_infra/src/pipeline/pdf.rs:578](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:578) — this function has too many arguments (10/7)
- [03_infra/src/pipeline/pdf.rs:614](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:614) — this function has too many arguments (8/7)
- [03_infra/src/pipeline/pdf.rs:641](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:641) — this function has too many arguments (9/7)
- [03_infra/src/pipeline/pdf.rs:669](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:669) — this function has too many arguments (10/7)
- [03_infra/src/pipeline/pdf.rs:702](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:702) — this function has too many arguments (8/7)
- [03_infra/src/pipeline/pdf.rs:733](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:733) — this function has too many arguments (9/7)
- [03_infra/src/pipeline/pdf.rs:760](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:760) — this function has too many arguments (10/7)
- [03_infra/src/pipeline/pdf.rs:793](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:793) — this function has too many arguments (8/7)
- [03_infra/src/pipeline/pdf.rs:878](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:878) — this function has too many arguments (8/7)
- [03_infra/src/pipeline/pdf.rs:945](/repos/Antigravity/typst-crystalline/03_infra/src/pipeline/pdf.rs:945) — this function has too many arguments (8/7)
- [03_infra/src/shaper/shaping.rs:124](/repos/Antigravity/typst-crystalline/03_infra/src/shaper/shaping.rs:124) — this function has too many arguments (8/7)
- [01_core/src/compiler/math/layout/matrix.rs:22](/repos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/matrix.rs:22) — this function has too many arguments (8/7)
- [01_core/src/compiler/stdlib/tests/support.rs:228](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/tests/support.rs:228) — this function has too many arguments (8/7)
- [01_core/src/compiler/stdlib/tests/support.rs:272](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/tests/support.rs:272) — this function has too many arguments (10/7)

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
