# P1200 — saneamento dos owners dos constructors primitivos

## Resultado

O owner conjunto foi separado em quatro relações 1:1:

- `compiler/stdlib/primitives-constructors.md` possui o hub
  `primitives_constructors.rs`;
- `primitives-constructors/decimal.md` possui `decimal.rs`;
- `primitives-constructors/duration.md` possui `duration.rs`;
- `primitives-constructors/version.md` possui `version.rs`.

`_nuclei/stdlib/primitive-calls.toml` contém somente ABI nativa, pureza L1,
consumo explícito dos argumentos e resultado tipado. Formas, coerções e erros
específicos permanecem nos três owners. Nenhum contrato público pendente foi
implementado.

## Medições reproduzíveis

Medição em `2026-08-26T07:43:44-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1200 registrou 63 ficheiros, 269
inserções e 570 remoções; havia 133 entradas em `git status --short`, que
enumera também os artefactos novos não incluídos pelo stat.

- Baseline P1199: V15=7, V26=0.
- Fechamento P1200: V15=6, V26=0.
- V5 global=383; nenhum V5 focal nos quatro módulos.
- Digest efetivo do núcleo:
  `761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736`.
- Hashes efetivos: hub `461f6aaa`; decimal `b2bd7d10`; duration `535135bf`;
  version `a87eeef2`.
- Sources sem `@prompt`/`@prompt-hash` são idênticos ao HEAD:
  hub `5b727702946ddb99bf98c9c6fbc52054161c4fb8a89e05c6a116d310a9ff30cf`;
  decimal `c12c862a8c96a74525eea335033bad0fb5ff27579c2a252efc4cf6c94d782d43`;
  duration `8404f04d5d9bd59eb6fb77c3a584764f6643ac6d20196091d66ace1155f2ce41`;
  version `26a6edd6c0f00ef1630257860553a19dd57304428f797edb2352d6dfad36f163`.
- `cargo test -p typst-core compiler::stdlib::primitives_constructors::`: 47
  passados, 0 falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: exit 2, bloqueado pelos 6 V15
  restantes; diff antes/depois idêntico
  (`713289946a2b84276307ad8a61ee80cefe4e857d8945dfa662d5c6a5ccb875a0`),
  portanto zero escritas.

## Decisão

P1200 fecha GREEN. A colisão focal foi removida sem alterar corpos produtivos,
o núcleo possui os quatro consumers verdadeiros e os seis V15 restantes ficam
para os próximos passos.
