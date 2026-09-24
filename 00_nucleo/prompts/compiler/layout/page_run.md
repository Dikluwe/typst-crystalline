# Prompt L0 — layout lexical de page run
Hash do Código: bd9cfafa

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/page_run.rs`

## Contrato e aceitação

Aplicar deltas locais de PageRun, conservar páginas materiais vazias, fechar
boundaries necessárias e restaurar PageConfig/região/cursor em LIFO. Runs
aninhados não vazam configuração; pagebreak adjacente consome a transição.

## P1493 — vazio material durante captura HTML

Medição: o carrier privado de posição podia tornar
`current_line.is_empty()` falso sem tinta nem conteúdo. PageRun consulta o
predicado material comum: o carrier direto não fecha linha nem
altera boundary. Qualquer item real preserva o comportamento anterior.
