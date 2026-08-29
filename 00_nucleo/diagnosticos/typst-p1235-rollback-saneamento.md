# P1235 — saneamento do rollback Linear/Radial SVG

**Veredito:** `ACCEPTED_WITH_EXTERNAL_V5`.

## Proveniência

A verificação foi executada em `2026-08-27T22:18:41-03:00`, HEAD
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, working tree não commitada. O
snapshot exato de `git status --short`, `git diff HEAD --stat`, HEAD e horário
está em `p1235-execution-provenance.txt`, SHA-256
`5e3185033edffcdae0102ef93b2c7143d73a0a50621b9c1273bd98ac32ab9fc8`.

## Ownership e L0

A decisão de whitelist Linear/Radial pertence exclusivamente ao consumer
`03_infra/src/export/svg.rs` e permanece especificada em
`00_nucleo/prompts/infra/export/svg.md`. A enumeração P1235 foi retirada de
`infra/export/gradients/adaptive.md`, que volta a possuir somente o contrato do
helper adaptativo. O consumer `adaptive.rs` foi ressellado com
`@prompt-hash fcca06aa`; nenhum fix global de hashes foi aplicado.

O código vigente de `paint_is_svg_native` admite somente `ColorSpace::Srgb`
para `Gradient::Linear` e `Gradient::Radial`. Linear/Oklab,
Linear/LinearRgb e Radial/Hsv recebem fallback explícito
`gradient-color-space`; nenhuma combinação P1231 foi promovida.

## Gates reproduzidos

`cargo test -p typst-infra p1235` executou três testes: `3 passed; 0 failed`.
`crystalline-lint --checks v15,v26 --fail-on warning .` respondeu `No
violations found`. `git diff --check` terminou sem diagnósticos.

O gate V5 global ainda registra duas derivas preexistentes fora dos owners
P1235: `01_core/src/compiler/stdlib/visualize.rs` e
`01_core/src/entities/tiling.rs`. Os consumers `svg.rs` e `adaptive.rs` não
recebem warning V5. O saneamento preserva as duas derivas externas sem
atribuí-las ou corrigi-las.

Não foi preservado recibo reproduzível do RED contra a whitelist anterior nem
dos três mutantes alegados originalmente. Ambos permanecem
`Unknown-not-persisted`; nenhum mutation score é alegado. O recibo canônico dos
gates atuais está em `p1235-gates.tsv`.

Regime: correção documental e verificação focal, não materialização Tekt
segregada. `EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.
