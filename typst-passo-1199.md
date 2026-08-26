# P1199 — individualizar os quatro modos do lexer

**Estado:** EXECUTADO — GREEN EM 2026-08-26
**Baseline condicionado:** P1198 GREEN; V15=8.

`compiler/lexer/mod.md` permanece owner do hub; criar owners `lexer/code.md`,
`lexer/markup.md` e `lexer/math.md`. Extrair somente fronteiras/tokens comuns
para `_nuclei/lexer/mode-boundaries.toml`; regras específicas de cada modo não
são copiadas nem nuclearizadas.

Gate V15 8→7, V26=0, quatro owners 1:1, ausência focal V5, testes completos do
lexer e build. Dry-run bloqueado por 7 V15. Diagnóstico:
`typst-p1199-saneamento-lexer-owners.md`.
