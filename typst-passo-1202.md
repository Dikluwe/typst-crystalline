# P1202 — individualizar fachada e unidades do parser

**Estado:** EXECUTADO — GREEN
**Baseline condicionado:** P1201 GREEN; V15=5.

`compiler/parse.md` permanece owner de `parse/mod.rs`; criar owners para
`code`, `markup`, `math`, `parser`, `patterns` e `rules`. Nuclearizar somente
contratos compartilhados de cursor/span/mode em `_nuclei/parse/core.toml`.
Gramáticas específicas e algoritmos permanecem nos owners.

Gate V15 5→4, V26=0, sete owners, V5 focal limpo, suíte parse completa e
build; dry-run bloqueado por 4 V15. Fechar
`typst-p1202-saneamento-parse-owners.md`.

## Fechamento

Executado em 2026-08-26. V15=4, V26=0, V5 focal limpo, 18 testes parse GREEN
e `cargo build` GREEN. O dry-run foi bloqueado pelos quatro V15 restantes e
não escreveu. Evidências em
`00_nucleo/diagnosticos/typst-p1202-saneamento-parse-owners.md`.
