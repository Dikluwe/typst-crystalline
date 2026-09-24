# Passo 1489 — ressincro do vanilla e recalibração da busca de paridade

**Estado:** execução e diagnóstico P1489 em 2026-09-24. O relatório de medição é
`00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md`.

## Medir antes de decidir

O alvo anterior era upstream/main `586e1bd43fae6c9a973218163d3165c53ab8d16d`.
`git ls-remote upstream HEAD refs/heads/main` em 2026-09-24 devolveu
`9f2b6e8715237cb086899a42873660fe744622e8` para ambos. A nova árvore é
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`; há 29 commits entre os pins.
Versão nominal `0.15.1` não identifica o snapshot. O laboratório antigo tinha
três overlays locais do extrator P1140 e não tinha `.git` próprio.

## Decisão e execução

1. Fixar `9f2b6e87` como alvo **desta** busca de paridade, não seguir `main`
   móvel. Copiar a árvore upstream para `lab/typst-original` sem alterar os
   três overlays locais nem os `target/` ignorados; verificar a identidade da
   árvore salvo os overlays. Construir o vanilla com
   `TYPST_COMMIT_SHA=9f2b6e8715237cb086899a42873660fe744622e8` e um job.
2. Atualizar pins operacionais do catálogo e da matriz. Rerodar os seis perfis
   contra o novo binário, separar MATCH, diferenças de formato, casos desligados
   por feature, lacunas e metadados de símbolo com diferença verificada.
3. Adicionar sondas novas para mudanças de linguagem detectadas no delta, com
   estado esperado VIOLATED até que haja implementação. Não converter presença
   de binding ou sucesso de compilação em paridade sem testar o observável.
4. Manter medições dos pins anteriores como históricas. Não copiar bugs de
   execução do vanilla; paridade é semântica, sintaxe e morfologia da linguagem
   (ADR-0107/0108). Não remover aliases públicos só porque o upstream os retirou:
   uma quebra exigiria L0 causal e confirmação específica (ADR-0127).

Não se altera comportamento produtivo neste passo. A sonda direta de
`sys.version` confirma `version(0, 15, 1)` nos dois produtos, portanto a
constante pública permanece. Gaps recém-encontrados ficam abertos para passos
posteriores, com RED executável na matriz. O relatório registra pin, árvore,
receita, hashes de binários, proveniência do worktree e resultados por perfil.
