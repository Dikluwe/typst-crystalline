# L0 — compiler/layout/sequence/inline_prefix
Hash do Código: 44773c76

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/layout/sequence.rs:73-558`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/sequence/inline_prefix.rs`. Certificar prefixos inline e a reserva vertical antes do primeiro bloco fracionário. Preservar os limites de estilos, largura, wrap e boundary descritos em sequence.md. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.
