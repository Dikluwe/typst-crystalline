# L0 — entities/color/conversion
Hash do Código: 3edffe02

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/entities/color.rs:280-714`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/entities/color/conversion.rs`. Converter cores entre espaços de processo e representar sRGB em bytes e floats. Preservar arredondamento, fórmulas e identidade spot; manter as assinaturas públicas existentes. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.
