# P1197 — separar catálogo de fallback do shaper

**Estado:** EXECUTADO — GREEN EM 2026-08-26
**Baseline condicionado:** P1196 GREEN; V15=10.

Manter `infra/shaper.md` para `shaper.rs`; criar `infra/fallback_fonts.md` para
`fallback_fonts.rs`. Extrair `_nuclei/fonts/fallback-selection.toml` apenas com
invariantes de seleção/ordem compartilhadas; descoberta/catálogo e shaping são
algoritmos de owners diferentes. Não alterar escolha de fonte ou morfologia.

Gate V15 10→9, V26=0, V5 focal limpo, testes fallback/shaper e build; dry-run
bloqueado por 9 V15. Diagnóstico `typst-p1197-saneamento-shaper-fallback.md`.
