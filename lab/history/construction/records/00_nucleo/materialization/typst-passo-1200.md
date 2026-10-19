# P1200 — individualizar constructors primitivos

**Estado:** EXECUTADO — GREEN
**Baseline condicionado:** P1199 GREEN; V15=7.

Manter `primitives-constructors.md` para o hub e criar owners para
`decimal.rs`, `duration.rs` e `version.rs`. Nuclearizar somente convenções de
chamada/coerção realmente comuns em `_nuclei/stdlib/primitive-calls.toml`;
semântica e erros próprios ficam em cada L0. Respeitar gates públicos já
registrados, sem implementar contratos pendentes.

Gate V15 7→6, V26=0, V5 focal limpo, testes dos três constructors e build;
dry-run bloqueado por 6 V15. Fechar `typst-p1200-saneamento-primitives.md`.

## Fechamento

Executado em 2026-08-26. V15=6, V26=0, V5 focal limpo, 47 testes focais
GREEN e `cargo build` GREEN. O dry-run foi bloqueado pelos seis V15 restantes
e não escreveu. Evidências em
`00_nucleo/diagnosticos/typst-p1200-saneamento-primitives.md`.
