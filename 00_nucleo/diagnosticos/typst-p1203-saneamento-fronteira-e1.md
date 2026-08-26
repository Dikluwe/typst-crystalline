# P1203 — saneamento da fronteira E1

## Resultado

O antigo `entities/f_fronteira_e1.md` era inteiramente transversal e histórico,
portanto não recebeu owner representativo. Seu conteúdo integral foi movido
para `diagnosticos/typst-f-fronteira-e1-historico.md`. Os contratos vigentes
foram separados em cinco relações 1:1:

- `entities/element_registry.md` possui `element_registry.rs`;
- `entities/elements/dynamic.md` possui `elements/dynamic.rs`;
- `entities/elements/emph.md` possui `elements/emph.rs`;
- `entities/elements/strong.md` possui `elements/strong.rs`;
- `entities/elements/test_callout.md` possui `elements/test_callout.rs`.

`_nuclei/entities/element-boundary.toml` contém somente enum Content fechado,
porta Dynamic única, trait Element compartilhado, despacho nativo estático e
registro injetado. Cada owner conserva sua semântica específica.

## Medições reproduzíveis

Medição em `2026-08-26T08:33:13-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1203 registrou 83 ficheiros, 342
inserções e 2206 remoções; havia 164 entradas em `git status --short`. O stat
não contabiliza o destino histórico ainda untracked, mas `git status` o enumera.

- Baseline P1202: V15=4, V26=0.
- Fechamento P1203: V15=3, V26=0.
- V5 global=366, queda exata de cinco; nenhum V5 focal.
- Digest efetivo do núcleo:
  `cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b`.
- Hashes efetivos: registry `27d55d3b`; dynamic `bd726fde`; emph
  `1d6c7f95`; strong `753ccc32`; test_callout `f3abeada`.
- Sources sem `@prompt`/`@prompt-hash` são idênticos ao HEAD:
  registry `2db5ca448413a76e2c2d80a4a299977efc5c1fb4306358ab966315f1987442ea`;
  dynamic `0b807c99bf403aec0d094b92a391218c9ed71f4e72f0cea3c2ce008fb23556f8`;
  emph `97f1ea5b927f4e72807b9707eb4f3576a4111bfb6d09bb4ea9bffa7bec4a2421`;
  strong `653c777a9ca62c0e889be3096dd014b544e758796a4a974d8e6fcf33bbf66361`;
  test_callout `8a871137d312e394f888ddce268d51c025aa0d4b3055eec6304759ed2bac4467`.
- `cargo test -p typst-core entities::elements::`: 293 passados, 0 falhas.
- `cargo test -p typst-core entities::element_registry::`: 2 passados, 0 falhas.
- `cargo test -p typst-core entities::content::`: 204 passados, 0 falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: exit 2, bloqueado pelos 3 V15
  restantes; diff antes/depois idêntico
  (`921a751a65ec9b832c1461bf11bc058e9b88ee36fedda38a426e2ef18989077f`),
  portanto zero escritas.

## Decisão

P1203 fecha GREEN. A fronteira deixou de ser prompt 1:N sem perder seu registro
histórico, os corpos produtivos não mudaram e os três V15 restantes seguem para
os próximos passos.
