# P1205 — saneamento dos owners do layout geral

## Resultado

O antigo `compiler/layout.md`, compartilhado por 17 consumers, foi preservado
integralmente em `typst-layout-l0-agregado-historico.md`. O owner vigente ficou
restrito a `layout/mod.rs`, e cada uma das 16 subunidades ganhou Prompt L0
próprio: cursor, dynamic, grid, grid_placement, helpers, hyphenation,
linebreak, metrics, page_geometry, page_run, parbreak, placement, sequence,
slicing, sub_frame e tests.

`_nuclei/layout/coordinates.toml` contém somente as invariantes realmente
transversais de origem absoluta, delta incremental, rebase conjunto e save/
restore LIFO. Pinam o núcleo hub, cursor, grid, helpers, placement, slicing,
sub_frame e tests. Os outros nove owners conservam contratos próprios e não
receberam pins por proximidade nominal. `tests.rs` possui seu próprio owner e
verifica o motor, sem possuir o contrato produtivo.

## Medições reproduzíveis

Medição em `2026-08-26T08:47:49-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1205 registrou 112 ficheiros, 435
inserções e 8.795 remoções; havia 222 entradas em `git status --short`. O stat
não contabiliza os novos owners/históricos ainda untracked, mas o status os
enumera.

- Baseline P1204: V15=2, V26=0, V5 global=356.
- Fechamento P1205: V15=1, V26=0, V5 global=340.
- Dezessete consumers sem V5 focal; a queda global é 16 porque
  `layout/linebreak.rs` não possuía `@prompt-hash` antes e passou a possuir.
- Hash efetivo do núcleo:
  `2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748`.
- `cargo test -p typst-core compiler::layout::`: 788 passados, zero falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: bloqueado pela única V15
  remanescente; digest antes/depois
  `e400217c191da2e3095e076d6336c542805502c5ca8a0eb861b2497b131d6c0c`,
  portanto zero escritas.
- `git diff --check`: GREEN.

## Decisão

P1205 fecha GREEN sem alterar código funcional, contrato público, default ou
fase. A única colisão restante é `compiler/atomizacao_elementos.md`, destinada
ao passo seguinte já escrito.
