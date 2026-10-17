# Pendências após traits de domínio e Duration

Inventário: **9 avisos**, medido 2026-10-16T17:24:18.582974+00:00 UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/validation-details.json) conserva estado/hashes/diffstat.

| Regra | Quantidade | Encaminhamento |
|---|---:|---|
| should_implement_trait | 1 | FromIterator para Styles proposto; aguarda ratificação/materialização |
| inherent_to_string | 3 | Display/ToString propostos; aguarda ratificação/materialização |
| large_enum_variant | 5 | Medir alternativas; um enum interno e quatro contratos públicos |

- [01_core/src/compiler/eval/context_reads.rs:12](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/context_reads.rs:12) — `clippy::large_enum_variant`; L0 [00_nucleo/prompts/compiler/eval/context_reads.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval/context_reads.md).

- [01_core/src/compiler/layout/par_line_callbacks.rs:54](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/par_line_callbacks.rs:54) — `clippy::large_enum_variant`; L0 [00_nucleo/prompts/compiler/layout/par_line_callbacks.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/par_line_callbacks.md).

- [01_core/src/compiler/math/layout/callbacks.rs:56](/repos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/callbacks.rs:56) — `clippy::large_enum_variant`; L0 [00_nucleo/prompts/compiler/math/layout/callbacks.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/math/layout/callbacks.md).

- [01_core/src/entities/tag.rs:18](/repos/Antigravity/typst-crystalline/01_core/src/entities/tag.rs:18) — `clippy::large_enum_variant`; L0 [00_nucleo/prompts/entities/tag.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/tag.md).

- [01_core/src/entities/show.rs:101](/repos/Antigravity/typst-crystalline/01_core/src/entities/show.rs:101) — `clippy::large_enum_variant`; L0 [00_nucleo/prompts/entities/show.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/show.md).

- [01_core/src/entities/style.rs:270](/repos/Antigravity/typst-crystalline/01_core/src/entities/style.rs:270) — `clippy::should_implement_trait`; L0 [00_nucleo/prompts/entities/style.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/style.md).

- [01_core/src/entities/decimal.rs:49](/repos/Antigravity/typst-crystalline/01_core/src/entities/decimal.rs:49) — `clippy::inherent_to_string`; L0 [00_nucleo/prompts/entities/decimal.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/decimal.md).

- [01_core/src/entities/duration.rs:97](/repos/Antigravity/typst-crystalline/01_core/src/entities/duration.rs:97) — `clippy::inherent_to_string`; L0 [00_nucleo/prompts/entities/duration.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/duration.md).

- [01_core/src/entities/version.rs:72](/repos/Antigravity/typst-crystalline/01_core/src/entities/version.rs:72) — `clippy::inherent_to_string`; L0 [00_nucleo/prompts/entities/version.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/version.md).

Proposta pública: [review.md](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/clippy-traits-2026-10-16/review.md), manifesto SHA-256 `d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`. Não adicionar impls ou alterar métodos antes de ratificação. Enums não estão incluídos. A análise dos 13 casos iniciais está em [classificacao-13.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16/classificacao-13.json).
