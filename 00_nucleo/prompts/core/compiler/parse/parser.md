# L0 — compiler/parse/parser
Hash do Código: 6fb361ec

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/parse/parser.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Declare Parser, Token, Newline, AtNewline and Marker, and compose the state,
checkpoints and diagnostics children. Preserve grammar-visible field access;
nl_mode and internal trivia metadata remain private to the parser module tree.
Do not implement child algorithms or introduce dynamic dispatch or grammar rules.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
