# L0 — compiler/layout/grid/row_pagination
Hash do Código: 986974ed

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), o comportamento está em `grid/cell_emission.rs:67-91`. Trata-se de delimitação de código vigente; não há medição de desempenho nova.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/row_pagination.rs`. Decide a quebra antes de uma linha de grid. Se a linha couber numa página nova, descarrega os segmentos verticais abertos na página anterior e só então cria a página. Uma linha maior que a área útil conserva o overflow atual. Mantém as assinaturas públicas, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

Testes L1, build e crystalline-lint passam. Sem inferência de ganho de desempenho.
