# P1205 — decompor os dezessete owners do layout geral

**Estado:** EXECUTADO — GREEN
**Baseline condicionado:** P1204 GREEN; V15=2.

Manter `compiler/layout.md` para `layout/mod.rs`; criar owners para cursor,
dynamic, grid, grid_placement, helpers, hyphenation, linebreak, metrics,
page_geometry, page_run, parbreak, placement, sequence, slicing, sub_frame e
tests. Extrair `_nuclei/layout/coordinates.toml` somente para os consumidores
que realmente compartilham coordenadas/rebase/regiões; testes ganham owner, não
ownership do motor.

Gate V15 2→1, V26=0, 17 owners, V5 focal limpo, suíte layout/core e build.
Dry-run ainda bloqueado pela última V15. Fechar
`typst-p1205-saneamento-layout-owners.md`.

## Resultado

Gate cumprido: V15=1, V26=0, 17 owners sem V5 focal, 788 testes de layout
GREEN, build e `git diff --check` GREEN. O dry-run foi bloqueado somente pela
última V15 e não escreveu. Evidência em
`00_nucleo/diagnosticos/typst-p1205-saneamento-layout-owners.md`.
