# P1204 — decompor os dez owners de eval

**Estado:** EXECUTADO — GREEN
**Baseline condicionado:** P1203 GREEN; V15=3.

Manter `compiler/eval.md` somente para `eval/mod.rs`; criar owners para
`bibliography`, `control_flow`, `flow`, `markup`, `math`, `modules`, `rules` e
`tests`. Repointar `stdlib/foundations/path.rs` ao owner de path já vigente,
após auditoria contra P1141–P1178, não a um novo owner eval. Extrair somente
engine/scopes/spans/FlowEvent compartilhados para `_nuclei/eval/core.toml` e
apenas aos prompts aplicáveis.

Gate V15 3→2, V26=0, dez relações 1:1, V5 focal limpo, suítes eval/path e
build. Dry-run bloqueado por 2 V15. Fechar
`typst-p1204-saneamento-eval-owners.md`.

## Resultado

Gate cumprido: V15=2, V26=0, dez V5 focais removidos, 1.284 testes de eval e
36 testes de path GREEN, `cargo build` e `git diff --check` GREEN. O dry-run
foi bloqueado pelos dois V15 previstos e não escreveu. Evidência completa em
`00_nucleo/diagnosticos/typst-p1204-saneamento-eval-owners.md`.
