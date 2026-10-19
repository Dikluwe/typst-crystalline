# P1204 — saneamento dos owners de eval

## Resultado

O antigo `compiler/eval.md` possuía dez consumers. Seu conteúdo agregado foi
preservado integralmente em `typst-eval-l0-agregado-historico.md`, fora do
namespace materializável. O owner vigente ficou restrito a `eval/mod.rs`.

Foram estabelecidas dez relações 1:1:

- `compiler/eval.md` → `eval/mod.rs`;
- `compiler/eval/bibliography.md` → `eval/bibliography.rs`;
- `compiler/eval/control_flow.md` → `eval/control_flow.rs`;
- `compiler/eval/flow.md` → `eval/flow.rs`;
- `compiler/eval/markup.md` → `eval/markup.rs`;
- `compiler/eval/math.md` → `eval/math.rs`;
- `compiler/eval/modules.md` → `eval/modules.rs`;
- `compiler/eval/rules.md` → `eval/rules.rs`;
- `compiler/eval/tests.md` → `eval/tests.rs`;
- `compiler/stdlib/foundations/path.md` →
  `compiler/stdlib/foundations/path.rs`.

`_nuclei/eval/core.toml` contém apenas estado explícito, spans causais,
transporte de `FlowEvent` e despacho estático. Bibliography e path não o pinam:
o primeiro não usa o contexto de eval compartilhado e o segundo pertence à
fronteira P1141 de path/World. A auditoria P1141–P1154 confirmou que
`entities/path.md` continua exclusivamente proprietário de `entities/path.rs`;
o constructor/helpers ganhou owner próprio, sem duplicar o domínio público.

## Medições reproduzíveis

Medição em `2026-08-26T08:41:56-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1204 registrou 94 ficheiros, 384
inserções e 6.137 remoções; havia 186 entradas em `git status --short`. O stat
não contabiliza os novos ficheiros ainda untracked, inclusive o histórico
movido, mas o status os enumera.

- Baseline P1203: V15=3, V26=0, V5 global=366.
- Fechamento P1204: V15=2, V26=0, V5 global=356.
- Queda focal: dez V5; nenhum V5 nos dez consumers deste passo.
- Hash efetivo do núcleo:
  `e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58`.
- `cargo test -p typst-core compiler::eval::`: 1.284 passados, zero falhas.
- `cargo test -p typst-core path`: 36 passados, zero falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `crystalline-lint --fix-hashes --dry-run .`: bloqueado exatamente pelos dois
  V15 remanescentes; digest do diff antes/depois
  `951d7596dd5b559c6053383349a554a12fdf6d3eec20af9cefc7926aaacefc9b`,
  portanto zero escritas.
- `git diff --check`: GREEN.

## Decisão

P1204 fecha GREEN. Nenhum comportamento, contrato público, default ou fase do
pipeline foi alterado. Os dois V15 restantes pertencem a
`compiler/atomizacao_elementos.md` e `compiler/layout.md` e seguem para os
passos já escritos.
