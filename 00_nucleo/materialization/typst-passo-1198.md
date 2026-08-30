# P1198 — individualizar wiring e suas duas suítes

**Estado:** EXECUTADO — GREEN EM 2026-08-26
**Baseline condicionado:** P1197 GREEN; V15=9.

Manter `wiring.md` para `04_wiring/src/main.rs`; criar `wiring/tests/cli.md` e
`wiring/tests/crystalline_lint.md`. Nuclearizar observáveis CLI somente entre
main e testes CLI em `_nuclei/wiring/cli-observables.toml`. A suíte do linter
tem owner independente e não consome esse Núcleo por proximidade.

Gate V15 9→8, V26=0, três owners 1:1, V5 focal limpo, suítes wiring verdes,
build e sources idênticos fora da linhagem. Dry-run bloqueado por 8 V15.
Fechar `typst-p1198-saneamento-wiring-owners.md`.
