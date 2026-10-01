# L0 — compiler/layout/columns/segmented
Hash do Código: f825a60a

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/layout/columns.rs:192-309,407-639`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/columns/segmented.rs`. Segmentação por colbreak, descoberta de direção e layout de regiões irmãs; preserva ordem, estado lógico e semântica de segmentos vazios. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.
