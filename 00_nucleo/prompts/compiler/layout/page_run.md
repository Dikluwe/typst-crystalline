# Prompt L0 — layout lexical de page run
Hash do Código: bdd32431

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/page_run.rs`

## Contrato e aceitação

Aplicar deltas locais de PageRun, conservar páginas materiais vazias, fechar
boundaries necessárias e restaurar PageConfig/região/cursor em LIFO. Runs
aninhados não vazam configuração; pagebreak adjacente consome a transição.
