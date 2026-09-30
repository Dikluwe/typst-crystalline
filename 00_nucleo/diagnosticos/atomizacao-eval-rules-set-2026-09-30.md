# Atomização dos branches `page` e `text` em `eval/rules/set` — 2026-09-30

## Proveniência

O estado anterior é o snapshot `atomizacao-eval-rules-2026-09-30-after.json.gz`, capturado antes dos commits de integração e preservado no commit `7f806c8d3`. A extração atual foi feita na working tree sobre esse commit. O estado final, incluindo HEAD, status, diff stat, SHA-256 e linhas dos owners, está em `atomizacao-eval-rules-set-2026-09-30-after.json.gz`.

## Medição e decisão

O dispatcher `rules/set.rs` tinha 1939 linhas após P1499. Os braços contíguos `page` e `text` foram movidos, sem mudança de corpo algorítmico, para `rules/set/page.rs` e `rules/set/text.rs`. O dispatcher ficou com 995 linhas; page tem 553 e text tem 391. A ordem dos branches, o encaminhamento estático, os defaults, o primeiro erro e a fase eval permanecem. Cada novo arquivo possui Prompt L0 exclusivo e os pins de política relevantes.

`page` isola geometria e SetPageDelta. `text` isola validação e transporte lexical `text.*`. Os demais targets de `set` continuam no dispatcher, que permanece disponível para subdivisões futuras por comportamento. Não há afirmação de ganho de desempenho.

## Verificação

- `cargo test -p typst-core --lib --offline --locked --quiet` — 6401 passaram, 2 ignorados (`/tmp/eval-set-branches-final-test.log`).
- `cargo build --offline --locked` — exit 0 (`/tmp/eval-set-branches-build.log`).
- `crystalline-lint .` — exit 0 (`/tmp/eval-set-branches-lint.log`).
- `git diff --check` — exit 0.

A suíte verifica o comportamento coberto; não substitui medição bilateral contra o vanilla pinado.
