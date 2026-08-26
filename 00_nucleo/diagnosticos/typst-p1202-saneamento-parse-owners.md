# P1202 — saneamento dos owners do parser

## Resultado

O owner conjunto foi separado em sete relações 1:1:

- `compiler/parse.md` possui `parse/mod.rs`;
- `compiler/parse/code.md` possui `code.rs`;
- `compiler/parse/markup.md` possui `markup.rs`;
- `compiler/parse/math.md` possui `math.rs`;
- `compiler/parse/parser.md` possui `parser.rs`;
- `compiler/parse/patterns.md` possui `patterns.rs`;
- `compiler/parse/rules.md` possui `rules.rs`.

`_nuclei/parse/core.toml` contém somente progresso do cursor, cobertura
lossless de spans/texto, markers contíguos e transições scoped de mode. Regras,
precedências, recovery e diagnostics específicos permanecem nos owners.

## Medições reproduzíveis

Medição em `2026-08-26T08:28:20-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1202 registrou 77 ficheiros, 332
inserções e 952 remoções; havia 151 entradas em `git status --short`, incluindo
artefactos novos não contados pelo stat.

- Baseline P1201: V15=5, V26=0.
- Fechamento P1202: V15=4, V26=0.
- V5 global=371, queda exata de sete; nenhum V5 focal.
- Digest efetivo do núcleo:
  `ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0`.
- Hashes efetivos: hub `98b2a80c`; code `124cb7a7`; markup `6f941372`;
  math `f634e022`; parser `57f554f3`; patterns `71d47814`; rules `087890b6`.
- Sources sem `@prompt`/`@prompt-hash` são idênticos ao HEAD:
  hub `52cf579f8f303d08cd7905b69f4fdd436ba3a53388a2d82ad6c6ba2dc84f3e66`;
  code `369d0e4734f6bed655f113bc8e54627088c2a6e36dcb6542cb033a394f89db84`;
  markup `eb2188a4c85226f91ab2fb7c52b2b89b6d59a6262234e0a4dd5c036634e086a3`;
  math `28d3f96bc330d027890c7dece3a3cc14b6bfdb2b6bbb0be862ed9e2c7a7ab654`;
  parser `5d4d5750707d127fe371a54ec4c049d0961203201815c12e978d172fb7c8f001`;
  patterns `e8a23e0225dc4109e9cd2429d9fbf789dc6237ab9d9249ef5469de44aa1712eb`;
  rules `f1df34df304d0c4a2dbd4dfede645dd61b40d54fe61bb931c061b2188362376f`.
- `cargo test -p typst-core compiler::parse::`: 18 passados, 0 falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: exit 2, bloqueado pelos 4 V15
  restantes; diff antes/depois idêntico
  (`3483c0261c979ff6aa468ee9bd2b5927c1f82b2e01924392dcafb84bc6313030`),
  portanto zero escritas.

## Decisão

P1202 fecha GREEN. A colisão focal foi removida sem alterar gramática, outputs
ou corpos produtivos; os quatro V15 restantes ficam para os próximos passos.
