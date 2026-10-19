# L0 — compiler/parse/parser/checkpoints
Hash do Código: 2d5beeb0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:b6a95a7684745e45903417a81eabc019a17eb3b8fab9fe246e8840978d5d42c8

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
