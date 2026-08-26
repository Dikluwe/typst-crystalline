# Prompt L0 — `compiler/eval/markup`
Hash do Código: 1ef04de3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/markup.rs`

## Medição e contrato

Avalia strong, emph, heading, raw, link, list e enum item. Estilos locais são
empilhados em Engine reborrowed sem vazar; conteúdo atravessa show aplicável.
Morfologia e spans vêm do AST, sem antecipar layout.

## Aceitação

Corpo, destino, nível, marcador e estilo semântico preservam scopes e show.
