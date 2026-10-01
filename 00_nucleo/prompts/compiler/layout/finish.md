# L0 — compiler/layout/finish
Hash do Código: edc259cf

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/layout/mod.rs:1957-2281`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/finish.rs`. Composição terminal do documento paginado; preserva flush, fixups, camadas e transporte do resultado. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.
