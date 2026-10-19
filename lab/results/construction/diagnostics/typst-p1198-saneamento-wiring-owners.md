# P1198 — saneamento dos owners de wiring

## Resultado

Os três consumers de `wiring.md` foram individualizados:

- `wiring.md` possui `04_wiring/src/main.rs`;
- `wiring/tests/cli.md` possui `04_wiring/tests/cli.rs`;
- `wiring/tests/crystalline_lint.md` possui
  `04_wiring/tests/crystalline_lint.rs`.

`_nuclei/wiring/cli-observables.toml` é consumido somente por main e suíte
CLI. A suíte V14 não consome o núcleo por proximidade.

## RED→GREEN colateral

A suíte do linter inicialmente ficou RED porque esperava que a mensagem V14
repetisse `ecow::EcoMap`. A allowlist type-level estava funcionando: `EcoMap`
era corretamente rejeitado, mas o linter atual emite a mensagem normativa por
pacote (`'ecow'`). O L0 da suíte foi atualizado antes do teste, e apenas esse
oracle textual mudou. Duas hipóteses intermediárias sobre manifesto e path do
fixture foram testadas, refutadas e integralmente revertidas.

## Medições reproduzíveis

Medição em `2026-08-26T07:24:57-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1198 registrou 53 ficheiros, 213
inserções e 230 remoções; artefactos novos são enumeráveis por
`git status --short`.

- Baseline P1197: V15=9, V26=0.
- Fechamento P1198: V15=8, V26=0.
- V5 global=391; nenhum V5 focal nos três owners.
- Digest efetivo do núcleo:
  `0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece`.
- Hashes efetivos: main `7178f5a6`; suíte CLI `48a0b9e1`; suíte do linter
  `d55d39b8`.
- `cargo test -p typst-wiring --test cli`: 70 passados, 0 falhas.
- `cargo test -p typst-wiring --test crystalline_lint`: 2 passados, 0 falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `main.rs` e `tests/cli.rs` são idênticos ao HEAD fora da linhagem. A suíte
  do linter difere somente no oracle RED→GREEN descrito acima.
- `crystalline-lint --fix-hashes --dry-run .`: exit 2, bloqueado pelos 8 V15
  restantes; diff antes/depois idêntico
  (`a00523c68d585f31b950ec1f02d684ca2674f99821d370fdd0d4547d2469471b`),
  portanto zero escritas.

## Decisão

P1198 fecha GREEN. A colisão focal foi removida, o núcleo possui apenas os
dois consumers verdadeiros, a suíte independente permanece independente e os
8 V15 restantes ficam para os próximos passos.
