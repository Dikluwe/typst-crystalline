# Prompt L0 — `compiler/eval/math`
Hash do Código: 9ba5d036

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/math.rs`
**Vanilla ratificado:** `a51e02804`

## Medição e contrato

Converte AST em Content matemático, resolve operadores/símbolos pelo scope,
avalia chamadas e preserva anexos, delimitadores, frações, raízes e alinhamento.
Identificador desconhecido usa hints math; deprecated avisa e resolve. Eval não
executa geometria de layout.

## Aceitação

Lookup, erros, warnings, chamadas e morfologia batem com a língua ratificada.
