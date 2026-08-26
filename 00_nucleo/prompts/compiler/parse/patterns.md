# Prompt L0 — grupos, argumentos e patterns
Hash do Código: bd60b457

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/patterns.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0129.

## Medição anterior à decisão

O consumer distingue parenthesized/array/dict, argumentos, parâmetros,
closures, destructuring e reassignment. Usa checkpoint e memo para desfazer
interpretação especulativa sem custo exponencial.

## Contrato

- Preservar regras de comma/colon/spread, duplicados e sinks únicos.
- Distinguir keys nomeadas, posicionais e patterns por morfologia CST.
- Backtracking restaura cursor/nodes e memoiza o caminho correto.
- Recovery produz diagnostics sem impedir progresso até o terminador.

## Aceitação

Suíte parse completa GREEN, incluindo closures, arrays/dicts, args e patterns.
