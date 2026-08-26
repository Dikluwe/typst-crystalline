# Prompt L0 — `compiler/eval/tests`
Hash do Código: 5e10be01

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1 test-only
**Ficheiro alvo:** `01_core/src/compiler/eval/tests.rs`

## Medição e contrato

Fornece Worlds puros, helpers test-only e regressões linguísticas do eval.
Fixtures não usam filesystem real. Asserções observam semântica, sintaxe,
morfologia e mensagens, não mecânica Rust incidental.

## Aceitação

Regressões têm controles e proveniência do vanilla quando decidem paridade.
