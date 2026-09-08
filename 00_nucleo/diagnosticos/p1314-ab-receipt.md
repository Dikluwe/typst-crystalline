# P1314 — recibo A/B independente

Resultado: **PASS**. 2808 comparações exatas,
0 divergências e 0 Unknown.
Suite de 234 expressões em default, html, a11y e html+a11y, executada nas
ordens normal, repeat e reverse. As 344 expectativas RED anteriores ao
candidato estão incluídas no mesmo contrato de 936 expectativas.

Regime: A/B executado sem atestação de isolamento técnico. Executor
`/root/p1314_tests`; não é protocolo completo, selo de refinamento, mutation
score ou prova de equivalência funcional geral. As capacidades e a exposição
incidental ao diff histórico embutido no baseline autorizado estão declaradas
no freeze; nenhum código candidato, owner loading.rs ou testes locais foi lido.

## Entradas e proveniência

- Freeze: `p1314-ab-freeze.json`, SHA-256 `41ac0730e01cd31cd9660b0e229e37d5e486a2c114927f3ac49c3475733756b9`.
- L0 normativo: `21231170b7092331ae09fafc656210bb94829658d822a15291de6ea0eb28dbfe`. Todos os bytes são
  protegidos, exceto a linha canônica `Hash do Código`; pins verificados.
- Baseline: `/dev/shm/p1313-target.keFg93/release/typst`,
  SHA-256 `cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce`.
- Vanilla ratificado: upstream `a51e02804`, `/usr/local/bin/typst`,
  SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato: `/dev/shm/p1314-target.cswujn/release/typst`,
  SHA-256 `cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f`.
- Build recebido: `p1314-build.json`, SHA-256 `742fc81148573f5a18935ef8a24695f16f3942ad0ff4e8a7461b37c8a988c1cd`.
- Runs: `p1314-ab-candidate-runs.json`, SHA-256 `e1aa3bf594dbcd95c2ed818ec3747e8cd10193ee341f00dc79e51250e1bf1155`.
- Comparação: `p1314-ab-comparison.json`, SHA-256 `c04d16bed5b824dc52db6dfb383d21fecb0d56bdd8fdaf9e1c981515a545f0da`.
- Intervalo: 2026-09-08T12:43:49.390258+00:00 — 2026-09-08T12:45:41.271557+00:00.
- HEAD: `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`; working tree não commitado, diff/stat
  integral abaixo. UTC, argv, cwd, tempo, exit/stdout/stderr por processo e
  estado final encontram-se em runs.

```text
 .../prompts/compiler/eval/bindings/field_access.md |  62 +-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 277 +++++++-
 01_core/src/compiler/eval/bindings/field_access.rs | 175 ++++-
 01_core/src/compiler/stdlib/loading.rs             | 765 +++++++++++++++++++--
 4 files changed, 1234 insertions(+), 45 deletions(-)
```

## Execução reproduzível

```sh
python3 00_nucleo/diagnosticos/p1314-ab-runner.py --candidate /dev/shm/p1314-target.cswujn/release/typst --candidate-sha256 cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f --orders normal,repeat,reverse --output <novo-runs.json>
python3 00_nucleo/diagnosticos/p1314-ab-freeze.py --measurement 00_nucleo/diagnosticos/p1314-ab-freeze.json --candidate-runs <novo-runs.json> --output <nova-comparacao.json>
```

RAM e fixtures são do namespace host; executar com a mesma capacidade de
acesso concedida à medição. Outputs devem ser novos; evidência histórica
nunca é sobrescrita.

## Escopo observado e limites

Os 144 casos P1313 usam expressões e cwd `/tmp/p1313-ab-fixtures` originais.
Antes do freeze, seus outputs atuais foram conferidos literalmente contra
P1313 em todos os perfis. Os 13 deltas de opções CSV estão identificados em
`historical_deltas` e `historical_case_mapping`; a ordem normal serve para
replay sem outra execução. Também há o sentinela P1312 exato
`csv("data.csv", delimiter: "ab")`.

Casos causais distinguem primeira ocorrência inválida, última válida,
delimiter inteiro antes de row-type, value span, With, Args, spread, sink,
filter e map detached. Há controles de defaults, valores CSV, unknown-named
antes de fonte/opções, cast da fonte, missing, excesso, duplicata sintática,
I/O, parsing, UTF-8 inválido, read, decoders, encoders e fields.

Delimiter Symbol continua rejeitado por obrigação normativa; a origem e
o trace medidos no vanilla são reutilizados sem alegar igualdade da coerção.
Row-type Symbol segue diagnóstico vanilla. Dívidas de unknown-named, parsing
e read encoding sobrescrito permanecem controles literais.

Ausência de arquivos discrimina erro de opção antes de leitura; zero chamadas
a World e Args sintético Rust exigem evidência do owner/revisor. O transporte
map da linguagem cobre origem detached pública. Não há teste independente
direto de carriers Rust nem atestação de isolamento do filesystem compartilhado.
