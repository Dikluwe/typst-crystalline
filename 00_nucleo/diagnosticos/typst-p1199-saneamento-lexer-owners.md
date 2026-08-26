# P1199 — saneamento dos owners do lexer

## Resultado

O owner conjunto do lexer foi separado em quatro relações 1:1:

- `compiler/lexer/mod.md` possui o hub `lexer/mod.rs`;
- `compiler/lexer/code.md` possui `lexer/code.rs`;
- `compiler/lexer/markup.md` possui `lexer/markup.rs`;
- `compiler/lexer/math.md` possui `lexer/math.rs`.

`_nuclei/lexer/mode-boundaries.toml` contém somente dispatch único, prefixo já
consumido, controle de modo pelo parser, precedência universal e proibição de
vazamento entre modos. Tokens e regras específicas permanecem nos owners dos
modos.

## Medições reproduzíveis

Medição em `2026-08-26T07:38:05-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1199 registrou 58 ficheiros, 230
inserções e 240 remoções; artefactos novos são enumeráveis por
`git status --short`.

- Baseline P1198: V15=8, V26=0.
- Fechamento P1199: V15=7, V26=0.
- V5 global=387; nenhum V5 focal nos quatro módulos.
- Digest efetivo do núcleo:
  `aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f`.
- Hashes efetivos: hub `ac49ec6b`; Code `8d0b3688`; Markup `0cc32222`;
  Math `21de7596`.
- Sources sem `@prompt`/`@prompt-hash` são idênticos ao HEAD:
  hub `68511f3b030b8ea6553c2ac08b6a12235468b3bf8eb527e0d244fe80b441701d`;
  Code `4e5e6346efc5f01f234863ee30e89787d4504f74de26a65abc87167c3b518158`;
  Markup `4a7cda64420c4a450c606fb47da77dbf6d97a6a25e3eab314b90e53f11e7f597`;
  Math `433b3b4b1e7f3e97dbf1dc9197a8aac6a3cb90b876bd508ada597bd614b0047a`.
- `cargo test -p typst-core compiler::lexer::`: 40 passados, 0 falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: exit 2, bloqueado pelos 7 V15
  restantes; diff antes/depois idêntico
  (`f5ba246a0981e945d99fedf97235119373876d4387827f845f69f1cbd3a5288c`),
  portanto zero escritas.

## Decisão

P1199 fecha GREEN. A colisão focal foi removida, o núcleo possui os quatro
consumers verdadeiros, a morfologia lexical permaneceu inalterada e os 7 V15
restantes ficam para os próximos passos.
