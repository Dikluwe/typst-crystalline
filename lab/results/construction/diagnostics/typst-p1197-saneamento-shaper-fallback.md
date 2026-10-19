# P1197 — saneamento shaper / catálogo de fallback

## Resultado

O catálogo de fallback ganhou owner 1:1 sem alteração de comportamento:

- `infra/shaper.md` permanece proprietário de `shaper.rs`;
- `infra/fallback_fonts.md` passa a ser proprietário de
  `fallback_fonts.rs`;
- `infra/font_metrics.md` mantém `font_metrics.rs` e pina o núcleo porque
  mede com as mesmas cadeias;
- `_nuclei/fonts/fallback-selection.toml` contém somente as invariantes
  compartilhadas de ordem, precedência, alinhamento medida/render e gatilho
  semântico matemático.

Os nomes e listas pertencem ao catálogo. Segmentação, coverage e scoring
continuam nos owners do shaper, das métricas e do `FontBook`.

## Medições reproduzíveis

Medição em `2026-08-26T07:04:54-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1197 registrou 49 ficheiros, 193
inserções e 222 remoções; artefactos novos são enumeráveis por
`git status --short`.

- Baseline P1196: V15=10, V26=0.
- Fechamento P1197: V15=9, V26=0.
- V5 global=393; nenhum V5 focal nos três consumers afetados.
- Digest efetivo do núcleo:
  `faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52`.
- Hashes efetivos: fallback catalog `5886c598`; shaper `1e92d4f2`;
  font metrics `bc598cdd`.
- Sources sem `@prompt`/`@prompt-hash` são idênticos ao HEAD:
  fallback `f7791d6c91da7c91b0f406be3e2906879cae628c57fded24baec2e236bf5ed96`;
  shaper `c3025cf8465783366b1ff0d122e75b79e821f357b42f6b3bd6d9aae033185907`;
  métricas `a92f8d3eb73366949560929af5b4fd3a1483c1abc2e3c40bf46952d3c2bee161`.
- Testes: P555 4/4; P783 2/2; P838 5/5; P875 1/1.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: exit 2, bloqueado pelos 9
  V15 restantes; diff antes/depois idêntico
  (`7be57e4e1b330b3c94a4c9ae645c9902b37f21c19315e41f7c21d0a830b2a6b2`),
  portanto zero escritas.

## Decisão

P1197 fecha GREEN. A colisão focal foi removida, o núcleo possui três
consumers verdadeiros, não há V26 e nenhuma escolha de fonte ou morfologia foi
alterada. Os 9 V15 restantes ficam para os passos seguintes.
