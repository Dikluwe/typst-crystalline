# P1192 — owner próprio para testes do layout matemático

**Estado:** EXECUTADO — GREEN EM 2026-08-25
**Baseline condicionado:** P1191 GREEN; V15=15.

Restringir `compiler/math/layout/_comum.md` a `math/layout/mod.rs`; criar
`compiler/math/layout/tests.md` para `tests.rs`. Nuclearizar apenas observáveis
matemáticos compartilhados em `_nuclei/math/layout-observables.toml`, sem mover
algoritmos, helpers ou fixtures ao Núcleo e sem enumerar milhares de testes.

Exigir V15 15→14, V26=0, dois pins válidos, ausência focal V5, bodies Rust
idênticos, testes math selecionados e build. O dry-run permanece bloqueado por
14 V15. Fechar `diagnosticos/typst-p1192-saneamento-math-layout-tests.md`.
