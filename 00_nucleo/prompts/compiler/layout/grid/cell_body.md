# L0 — compiler/layout/grid/cell_body
Hash do Código: 175d6b61

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/layout/grid/cell_emission.rs:95-583`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/cell_body.rs`. Dispor uma célula posicionada, com inset, alinhamento, subframe, decoração e caudas de overflow. Preservar save/restore do estado, ordem dos itens e índices de alinhamento adiados. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.
