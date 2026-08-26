# Prompt L0 — cursor e fechamento de linhas
Hash do Código: b90b2853

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/cursor.rs`

## Contrato

Gere palavras/chunks, wrap, baseline, `flush_line`, paginação e footnotes.
Avanço usa arestas tipográficas, leading e extensões inline; collectors e
pending geometry recebem a mesma translação dos itens. Página auto deriva
dimensão do conteúdo real, sem constantes posicionais de oracle.

## Aceitação

Wrap, RTL, linebreak, parbreak, footnotes, páginas auto e limites permanecem
geométrica e morfologicamente equivalentes ao vanilla ratificado.
