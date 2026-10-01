# L0 — compiler/layout/sequence/fractional_preflight
Hash do Código: a4255e70

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/layout/sequence.rs:19-72,559-747`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/sequence/fractional_preflight.rs`. Calcular a parcela fracionária da região e medir a cauda fixa antes da emissão. Preservar os guards de região, a base herdada ou restante e a restauração do estado de medição. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.
