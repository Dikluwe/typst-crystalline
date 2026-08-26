# Prompt L0 — `compiler/eval/flow`
Hash do Código: 09a57d6b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/flow.rs`

## Medição e contrato

`FlowEvent` transporta Break, Continue e Return com span, valor opcional e flag
condicional. `forbidden` emite exatamente as três mensagens inglesas medidas
para uso fora de loop/função.

## Aceitação

Payload, flag e mensagens são cobertos por testes unitários.
