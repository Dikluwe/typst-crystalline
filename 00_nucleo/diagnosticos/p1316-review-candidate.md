# P1316 — revisão do candidato

Revisor `/root/p1316_review`, somente leitura de candidato; escritas restritas
aos recibos `p1316-review-*`. A/B sem atestação técnica de isolamento.

## Proveniência e medição

Auditoria `node 00_nucleo/diagnosticos/p1316-review-audit.cjs`, script SHA-256
`455abda2b982284ee50300f34a983b840a2a64fa6a9b92b1771ba59c1eb22d8b`,
em `2026-09-08T14:37:26.776Z`; HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 124 +++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 213 +++++++++++++++++++++++++--
 2 files changed, 322 insertions(+), 15 deletions(-)
```

Fonte candidata SHA-256
`2278316a90408fc6dae9ba2fd0f3d5479571055ef4bcde84019147a84b12d75a`;
L0 raw SHA-256
`a4e4a71be35e5cf9008a47badb8a51e830616b6a768d0c185c24f78d79bce7d3`.
Hash normativo permanece o do freeze
`200816280fc2e1c18934103574c6d33c3d972bdfb576221c4a74d3175d385e5f`;
somente a linha canônica `Hash do Código` é excluída dessa comparação.
Todos os 29 pins congelados e 28 artefatos P1315 continuam íntegros.

RED estável `p1316-unit-red-r1.json`, SHA-256
`41a1ec7626e8e37d61644af4608388b8a203e844a7320f966f35c830c48ec431`:
antes `2026-09-08T14:32:32.157202+00:00`, depois
`2026-09-08T14:34:28.298933+00:00`, source/L0 iguais nos dois extremos.
`cargo test -p typst-core --release p1316 --lib` terminou exit 101:
duas falhas de span e dois controles passando. A falha é a ausência da
origem pretendida, não erro de compilação ou fixture.

## Inspeção e decisão

Em `01_core/src/compiler/stdlib/loading.rs:1301`, a única alteração
produtiva P1316 é `map_err` sobre `decode_csv` no ramo Bytes. A busca usa
a primeira ocorrência sem nome e seu `value_span`; não usa named, span
agregado ou ocorrência posterior. Ausência de occurrences resulta detached.
O laço altera somente `error.span`, preservando todos os demais campos.

O código anterior a `native_csv` e os testes anteriores ao P1316 estão
preservados, exceto resselo de linhagem. A assinatura nativa, os casts,
a validação de opções, o caminho Path/Str e o decoder puro permanecem
inalterados. Não há I/O ou nova dependência no ramo Bytes.

Os testes novos distinguem `value_span` de `occurrence.span`, named
anterior, chamada e segundo positional. Também verificam Args sintético,
origem explicitamente detached, decoder puro, Path/Str e World proibindo I/O.
O teste de parsing+excesso exige o efeito normativo expressamente previsto
no L0, sem tratar a precedência divergente como paridade vanilla.

Sem achado acionável na revisão do candidato. Veredito final pendente dos
gates GREEN/build/lint e comparação A/B integral com repetição e reordenação.
