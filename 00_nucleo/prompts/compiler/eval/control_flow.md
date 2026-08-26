# Prompt L0 — `compiler/eval/control_flow`
Hash do Código: c841c155

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/control_flow.rs`

## Medição e contrato

Avalia `if`, `while` e `for` com estado explícito. Condições exigem bool; loops
acumulam por `join`, respeitam o limite, consomem break/continue e propagam
return. `for` desestrutura cada item em scope lexical; return condicional
mantém a marca causal.

## Aceitação

Loops aninhados, eventos, joins, iteráveis e erros seguem `a51e02804`.
