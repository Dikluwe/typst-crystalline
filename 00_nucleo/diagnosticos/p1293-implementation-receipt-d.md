# P1293 — receipt de implementação produtiva do lote D

## Regime, papel e limites

- Regime: protocolo Tekt completo, segregado por capacidades e artefactos, sem
  alegação de isolamento técnico de leitura no filesystem compartilhado.
- Executor: `/root/implementador_d_p1293`, papel exclusivo de implementador.
- Escrita produtiva autorizada: `01_core/src/compiler/eval/mod.rs`, incluindo
  somente testes próprios P1293 D no mesmo ficheiro.
- Escrita documental autorizada: este receipt.
- Não foram lidos nem executados o teste protegido
  `04_wiring/tests/p1293_contract.rs`, receipts/oráculos protegidos, mutações,
  ataques ou artefactos de veredito. O contrato, manifesto, selo, L0, aliases
  flat e produtos A/B/C não foram editados.
- Este receipt não aprova o lote D nem emite veredito do P1293.

## Entradas congeladas e proveniência

Leitura integral concluída para o L0 (172 linhas, 8.335 bytes), manifesto
(2.867 linhas, 186.676 bytes) e selo (5.212 linhas, 318.045 bytes). Os dois
JSON foram carregados integralmente e validados antes da escrita produtiva.

| Entrada | SHA-256 validado |
|---|---|
| `00_nucleo/prompts/compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` |
| `00_nucleo/diagnosticos/p1293-manifest.json` | `c0f0044add8ce5e7d334746988aa4f2cef6d602b5f24b4cd2ad51d4aab11878d` |
| `00_nucleo/diagnosticos/p1293-contract-seal.json` | `06f3b436e065d06ba4573e51457e1bffc902114279164677cc52bfc830a6f3ba` |
| bloco canónico `$.serial_lot_d_short_namespace_names` | `7c1eaeb13674e1a0102e8fc3d8ce7477f5d7501f4d6fc13891e824694d29a8c3` |
| preimage `01_core/src/compiler/eval/mod.rs` | `84888144e8722b93611ee35a3067df1de99a3d22c33f859679e68091a5a396d0` |

Medição inicial: `2026-09-02T16:02:07-03:00`, HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, branch `Tekt`, working tree
compartilhada e não commitada. A lista exata de paths modificados nesse
estado é a lista congelada pelo manifesto/selo mais o mesmo owner D já
marcado como modificado pelo resselo de linhagem; o conjunto retornado por
`git status --porcelain=v1` tem SHA-256
`80878944a9b9d91ce6c5cdac62d38255503b6567a4d9a78b71e0646e61e8feac`.
O `git diff HEAD --stat` imediatamente antes deste receipt tem SHA-256
`e2bd5fca7571970a1f73a3246db084496ac31960b10e4f7f4a21b769b8fc9c3c`.
Para a decisão do lote, o estado relevante é fixado sem ambiguidade pelo
preimage do único owner, pelo manifesto, pelo selo e pelo bloco canónico
acima. Nenhum owner gap foi encontrado: `compiler/eval.md ↔ eval/mod.rs`
permanece 1:1 e o header recebido já apontava a `79a8abaf`.

## RED próprio antes do produto

Foram adicionados três testes internos, sem consultar o oráculo protegido:

- `p1293_d_grid_namespace_uses_short_names_direct_and_with`;
- `p1293_d_table_namespace_uses_short_names_direct_and_with`;
- `p1293_d_flat_aliases_keep_historical_names_and_lines_stay_namespaced`.

Comando:

```text
cargo test -p typst-core p1293_d_ -- --nocapture
```

Resultado no preimage mais os testes próprios: compilação concluída, 3 testes
executados, 2 REDs semânticos e 1 controlo GREEN. Os REDs testemunharam
`Some("grid_cell") != Some("cell")` e
`Some("table_cell") != Some("cell")`; o controlo confirmou os seis aliases
flat históricos e a ausência dos quatro aliases flat de linha. Resultado do
processo: exit 101, sem timeout, crash ou `Unknown`.

## Implementação autorizada

Foram trocados exclusivamente os primeiros argumentos textuais de dez
instâncias namespaced de `Func::native`:

| Namespace | scope key | antes | depois | pointer preservado |
|---|---|---|---|---|
| grid | cell | `grid_cell` | `cell` | `native_grid_cell` |
| grid | header | `grid_header` | `header` | `native_grid_header` |
| grid | footer | `grid_footer` | `footer` | `native_grid_footer` |
| grid | hline | `grid_hline` | `hline` | `native_grid_hline` |
| grid | vline | `grid_vline` | `vline` | `native_grid_vline` |
| table | cell | `table_cell` | `cell` | `native_table_cell` |
| table | header | `table_header` | `header` | `native_table_header` |
| table | footer | `table_footer` | `footer` | `native_table_footer` |
| table | hline | `table_hline` | `hline` | `native_table_hline` |
| table | vline | `table_vline` | `vline` | `native_table_vline` |

As dez scope keys, os dez pointers nativos, `Func::native_with_namespace`
dos containers, argumentos, retornos e payloads ficaram intactos. Os seis
aliases flat continuam com nomes underscored e nenhum alias flat hline/vline
foi introduzido. Aridade e diagnósticos não foram alterados.

SHA-256 final do owner, incluindo os testes próprios e formatação focal:
`cafdcbf690f5ad8020bbe3da4457a759397c29ac091dc1624f33e14f5127c5b4`.
Medição final do produto: `2026-09-02T16:10:59-03:00`, no mesmo HEAD e
working tree congelados acima.

## GREEN e gates executados

| Gate/comando | Resultado |
|---|---|
| `cargo test -p typst-core p1293_d_ -- --nocapture` | PASS: 3/3, 0 falhas, 0 ignorados, 5.413 filtrados |
| `cargo test -p typst-core p1292_d_place_and_with_share_the_flush_namespace -- --nocapture` | PASS: 1/1, 0 falhas, 5.415 filtrados |
| `rustfmt --edition 2021 01_core/src/compiler/eval/mod.rs` | aplicado somente ao owner autorizado |
| `rustfmt --edition 2021 --check 01_core/src/compiler/eval/mod.rs` | PASS |
| `crystalline-lint --fail-on warning --checks v5 .` | PASS: zero violations |
| `crystalline-lint --fail-on warning --checks v15 .` | PASS: zero violations |
| `crystalline-lint --fail-on warning --checks v26 .` | PASS: zero violations |
| `crystalline-lint --fix-hashes --dry-run .` | PASS: `Nothing to fix` |
| `git diff --check` | PASS |

Revalidação pós-produto confirmou que L0, manifesto e selo preservam os
SHA-256 recebidos e que o bloco canónico D continua em `7c1eaeb...`. Os
warnings preexistentes emitidos pela compilação focal não alteraram o exit
GREEN. Não se executaram gates protegidos, campanha de mutações, ataques ou
aprovação/veredito D; esses passos permanecem para autoridade independente.
