# L0 — compiler/parse/parser/checkpoints
Hash do Código: 257c6e76

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/parse/parser/checkpoints.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own MemoArena, MemoKey, Checkpoint, PartialState and checkpoint/restore/memo
methods. A snapshot contains node_len, lexer cursor, SyntaxMode and a cloned
Token. Restore truncates nodes and restores partial state, but does not restore
depth, balanced or nl_mode. Memo keys are token-start offsets and arena entries
are node ranges. Hits append cached nodes without truncating current nodes and
restore lexer/lookahead; misses return a key and checkpoint. Preserve memo
lifetime and verify rollback and replay without losing prior error nodes.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
