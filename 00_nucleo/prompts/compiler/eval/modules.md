# Prompt L0 — `compiler/eval/modules`
Hash do Código: 23981ab0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/modules.rs`

## Medição e contrato

Imports/includes resolvem somente por World, detectam ciclos na Route e
preservam FileId. Import avalia em Engine/Scopes isolados e exporta bindings;
include produz conteúdo na rota filha. Bare, rename, wildcard e items seguem a
linguagem; path enraizado não é re-resolvido.

## Aceitação

Ficheiro, pacote, módulo, aliases, include, ausência e ciclos têm testes focais.
