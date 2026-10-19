# P1201 — saneamento dos owners de linguagem

## Resultado

O owner conjunto foi separado em cinco relações 1:1:

- `compiler/lang.md` possui `lang/mod.rs`;
- `compiler/lang/equation_supplement.md` possui `equation_supplement.rs`;
- `compiler/lang/figure_supplement.md` possui `figure_supplement.rs`;
- `compiler/lang/outline_title.md` possui `outline_title.rs`;
- `compiler/lang/quotes.md` possui `quotes.rs`.

`_nuclei/lang/defaults.toml` contém somente lookup exato, fallback
determinístico sem ambiente, default inglês e ownership local das tabelas. O
hub não pina o núcleo porque não gera texto; os quatro geradores são seus
consumers verdadeiros. Locale, textos, glyphs, espaços e capitalização
específicos permanecem nos owners.

## Medições reproduzíveis

Medição em `2026-08-26T08:18:21-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1201 registrou 69 ficheiros, 297
inserções e 731 remoções; havia 141 entradas em `git status --short`, que
enumera também artefactos novos não incluídos pelo stat.

- Baseline P1200: V15=6, V26=0.
- Fechamento P1201: V15=5, V26=0.
- V5 global=378; nenhum V5 focal nos cinco módulos.
- Digest efetivo do núcleo:
  `c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9`.
- Hashes efetivos: hub `87c26a33`; equation `aa5a3747`; figure
  `e7e4ec4d`; outline `4bc9b2ed`; quotes `07e3cb92`.
- Sources sem `@prompt`/`@prompt-hash` são idênticos ao HEAD:
  hub `03fdeecbd275456470bbf55edaf639a98423bcd778449763b3a9ac1632e40d5a`;
  equation `f5eda98f4f87776ae6267481293e80d2c230401a87e3b727ee328e66b5b135a2`;
  figure `95b77ec980a2a98b6eb3bc5770d3076c9b55b7710d6d7169a93be1032b75f666`;
  outline `2c0ab67908d9491fd04be3dcf8725ab3310a1918d1cfbe4da65c12e4f1afc70a`;
  quotes `230f9c20116f1b6cf689ee18a9b7d83bc9c483e0ad0fb2ff978717ba255b7acd`.
- `cargo test -p typst-core compiler::lang::`: 21 passados, 0 falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: exit 2, bloqueado pelos 5 V15
  restantes; diff antes/depois idêntico
  (`62e16f2fe7fff3bb7e19252ace7d07ba43a607535f8bf2c6e56af72f459415f4`),
  portanto zero escritas.

## Decisão

P1201 fecha GREEN. A colisão focal foi removida sem mudar outputs ou corpos
produtivos, e os cinco V15 restantes ficam para os próximos passos.
