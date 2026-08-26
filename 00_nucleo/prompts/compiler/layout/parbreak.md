# Prompt L0 — fronteira visual de parágrafo
Hash do Código: 844ce71c

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/parbreak.rs`

## Contrato e aceitação

Fechar a linha anterior, aplicar spacing/colapso vigente e emitir marcador
ParbreakBoundary transparente somente quando existe linha material anterior.
O marcador não altera desenho, texto, cursor ou paginação.
