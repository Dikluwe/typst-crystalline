# Prompt L0 — gramática de código
Hash do Código: bd270db1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/code.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0129.

## Medição anterior à decisão

O consumer possui sequências e expressões de código, precedência de operadores,
blocos code/content, código embutido e reparse de bloco. Delega statements a
`rules`, grupos a `patterns` e transições para markup/math aos owners próprios.

## Contrato

- Consumir expressões até o stop-set e recuperar tokens inesperados sem perder
  progresso.
- Preservar associatividade/precedência dos operadores e a morfologia dos nós.
- Entrar em modo Code para `{...}` e Markup para `[...]`, restaurando o estado.
- Reparse só aceita resultado balanceado que termine exatamente no range.

## Aceitação

Suíte parse completa GREEN, sem alterar gramática, diagnostics ou CST.
