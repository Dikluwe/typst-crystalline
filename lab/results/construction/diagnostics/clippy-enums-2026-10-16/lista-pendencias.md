# Pendências após enums internos e traits aprovados

Inventário: **2 avisos Clippy**, medido 2026-10-16T18:03:38.128855+00:00 UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-enums-2026-10-16/validation-details.json) conserva estado/hashes/diffstat.

| Ordem | Caso | Decisão pendente |
|---|---|---|
| 1 | Tag | Auditar armazenamento/volume e clones; Box do payload público exige proposta e ratificação |
| 2 | Transformation | Auditar distribuição de Style e clones; Box pode piorar o caso Style; API exige proposta e ratificação |
| 3 | Tekt | 380 avisos de estrutura/contratos, com classificação por owner antes de qualquer exceção |

- [01_core/src/entities/tag.rs:18](/repos/Antigravity/typst-crystalline/01_core/src/entities/tag.rs:18) — `clippy::large_enum_variant`; L0 [00_nucleo/prompts/entities/tag.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/tag.md).

- [01_core/src/entities/show.rs:101](/repos/Antigravity/typst-crystalline/01_core/src/entities/show.rs:101) — `clippy::large_enum_variant`; L0 [00_nucleo/prompts/entities/show.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/show.md).
