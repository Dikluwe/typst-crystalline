# L0 — compiler/parse/parser/diagnostics
Hash do Código: c6b66b38

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:b6a95a7684745e45903417a81eabc019a17eb3b8fab9fe246e8840978d5d42c8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/parse/parser/diagnostics.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own expected/unexpected-token diagnostics, warnings/hints, empty-error trimming,
error wrapping, unclosed-delimiter diagnostics and depth protection. Suppress
consecutive expected errors; trim empty errors before unexpected tokens; report
keywords when Ident is expected; retain persistent balance failure. The depth
limit is 256; a guard increments depth and restores it on drop. Recovery consumes
at least one token, respects delimiter balance and stop sets, and preserves text
in an aggregated error. Warnings retain supplied severity and zero length.
Preserve diagnostic messages. Do not own grammar-specific diagnostics.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
