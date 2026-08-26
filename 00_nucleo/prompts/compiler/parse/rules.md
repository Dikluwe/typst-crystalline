# Prompt L0 — statements e rules de código
Hash do Código: 8d43205a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/rules.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0129.

## Medição anterior à decisão

O consumer é dono de let/set/show/context, if/while/for, import/include e
break/continue/return. Expressões, blocos, args e patterns são delegados aos
owners correspondentes.

## Contrato

- Envolver cada statement no `SyntaxKind` correspondente e preservar sua ordem.
- Aplicar diagnostics/hints vigentes para colon, patterns, imports e blocos.
- Respeitar newline mode em listas de import e estruturas compostas.
- Não avaliar nem resolver nomes; este módulo produz somente CST.

## Aceitação

Suíte parse completa GREEN sem alteração de gramática ou diagnostics.
