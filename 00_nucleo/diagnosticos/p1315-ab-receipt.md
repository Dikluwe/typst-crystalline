# P1315 — recibo A/B independente

Resultado: **PASS no fragmento e controles congelados**, executado sem atestação
de isolamento técnico. Foram aprovadas 3.480 comparações completas de
`exit/stdout/stderr`, nas ordens normal, repetida e inversa; zero diferenças
inesperadas e zero Unknown. Não é certificado de refinamento, mutation score
ou declaração de paridade geral.

## Proveniência e cadeia causal

O baseline é o commit `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, executável
`/dev/shm/p1314-target.cswujn/release/typst`, SHA-256
`cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f`.
A referência vanilla é `/usr/local/bin/typst`, upstream ratificado
`a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
As identidades foram verificadas antes e depois das execuções.

O freeze precedeu a liberação do candidato:
`p1315-ab-freeze.json`, SHA-256
`c2cd0c90e4ea09082980b7dba5ed960dc8c9415bc4930fbeb6c68c2566ddac4f`.
Seu L0 normativo é `00_nucleo/prompts/compiler/stdlib/loading.md`, SHA-256
`9a67fcb0cd258ba661221b844e59d149061d4b673a52618bc62092054080b40a`,
excluindo somente uma linha canônica `Hash do Código`. Todos os outros bytes
normativos e todos os inputs protegidos foram revalidados na comparação.
Nenhum passo tático foi pinado.

O candidato é `/dev/shm/p1315-target.V6TWEF/release/typst`, SHA-256
`6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1`.
Execução de `2026-09-08T13:51:16.424820+00:00` a
`2026-09-08T13:52:30.430549+00:00`, HEAD do baseline, working tree não
commitado. O `git diff HEAD --stat` naquele momento era:

```text
00_nucleo/prompts/compiler/stdlib/loading.md | 57 +++++++++++++++++-
01_core/src/compiler/stdlib/loading.rs       | 86 ++++++++++++++++++++++++----
2 files changed, 129 insertions(+), 14 deletions(-)
```

O estado, status completo, UTC, argv, cwd e observações de cada execução
constam em `p1315-ab-candidate-runs.json`, SHA-256
`7091b03795a741fd6c2120506e8322b6b871c43b8d91a76f30e8f920f96efc54`.
O resultado verificável está em `p1315-ab-comparison.json`, SHA-256
`2ee04e8e626bb8d8032cbf3df66dbf4470e12096254184ff5de32fd1a164046d`.

## Contrato observado

São 290 casos nos perfis default, html, a11y e html+a11y. As 1.160
expectativas foram congeladas antes do candidato e executadas três vezes.

| Classe | Casos | Comparações candidatas |
|---|---:|---:|
| Ordinal UnequalLengths | 38 | 456 |
| Valores CSV válidos | 12 | 144 |
| Erros UTF-8 preservados | 6 | 72 |
| Replay literal P1314 | 234 | 2.808 |

Os casos de ordinal declaram manualmente `found X instead of Y fields in
line N` e conferem esse fragmento no vanilla. A expectativa completa deriva
do baseline, substituindo **somente os dígitos de N** na mensagem. X/Y,
texto circundante, traces, spans renderizados, stdout e código de saída
permanecem literais. Entre essas expectativas, 112 eram RED no baseline
(28 casos em quatro perfis) e ficaram GREEN nas três ordens; os restantes
casos de ordinal são controles contra alterações indevidas.

A suíte cobre falta/excesso, cabeçalho multilinha, campos citados multilinha,
linhas vazias iniciais/intermediárias, CRLF, aspas escapadas, registro vazio
com dois campos, erro tardio, delimiter alternativo e header de campo único.
Cada cenário focal usa array e dictionary. As rotas adicionais incluem
With, Args spread, Args.map, bytes, string path e rooted path.

Os 234 casos P1314 mantêm id de origem, expressão e cwd originais. A nova
medição baseline foi comparada literalmente às observações candidatas P1314;
todos passaram. Eles preservam casts, opções e sua precedência, erros de
I/O, outros loaders e demais comportamentos históricos. A evidência
histórica é referenciada por hash, sem copiar diffs históricos de código.

## Calibração e custo

A primeira tentativa de freeze bloqueou quatro controles UTF-8: a construção
`bytes(...) + bytes(...)` falhava antes de CSV no baseline. Isso foi Unknown,
não sucesso. A revisão única substituiu a construção por tuplas literais com
os mesmos octetos (`97,44,98,10,255,44,97` e
`34,97,10,98,34,44,99,10,255,44,97`, nos dois modos). Os argv antigos e novos
permanecem nos registros preliminar e focal. O recorte de 32 execuções
confirmou erros CSV UTF-8 nos dois produtos e nos quatro perfis: quatro
construções corrigidas, zero regressões nesse recorte. Só então foi repetida
a medição completa. Nenhum candidato foi consultado para essa revisão.

| Artefato | Processos | Tempo entre recibos UTC |
|---|---:|---:|
| `p1315-ab-baseline-runs.json` | 1.384 | 42,137406 s |
| `p1315-ab-focal-runs.json` | 32 | 0,480268 s |
| `p1315-ab-baseline-final.json` | 1.384 | 25,276561 s |
| `p1315-ab-candidate-runs.json` | 3.480 | 74,005729 s |

Total: 6.280 processos, 141,899964 s de janelas de execução, sem incluir
autoria, ferramentas e espera entre comandos. Cada artefato contém seu HEAD,
estado da árvore e timestamps; os três primeiros hashes estão no freeze.

## Capacidades e limites

Executor `/root/p1315_tests`, papel testador A/B independente. Leu somente
L0, fonte vanilla autorizada, measurement P1315 sem conteúdo diff, código do
recorder P1315, artefatos A/B P1314, fixtures, observações CLI e metadados git
HEAD/status/diff STAT. Não leu owner `loading.rs`, testes locais, patch
candidato ou recibo de build com diff embutido. Escreveu somente
`00_nucleo/diagnosticos/p1315-ab-*` e `/tmp/p1315-ab-fixtures`.

A skill e referências consultadas têm SHA-256:

- `SKILL.md`: `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48`.
- `references/papeis-e-capacidades.md`: `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417`.
- `references/artefatos-e-gates.md`: `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d`.

O filesystem é compartilhado; a segregação de entradas é operacional e não
atestada tecnicamente. O A/B não verifica pureza de código, chamadas internas
ao World ou carriers detached sintéticos de Rust. Sufixo de posição física,
formatação geral dos diagnósticos e spans vanilla continuam fora do escopo.
Não se afirma paridade desses eixos pela extração do fragmento.

Unknown bloqueia quando há inputs alterados, identidade ambígua, observações
faltantes/duplicadas, expressões inadequadas, timeout ou crash. A comparação
não converte Unknown em PASS.

## Reprodução

Executar no repositório e manter fixtures/cwd históricos. O runner está
congelado em SHA-256
`b5fa4f0fc4834ca122e9baaa765de2e2ae04ad3b66a0cb3ab837dd93b15b0532`;
os casos em `32b4e4980d7b0e790d072ca3a9f7e2a442eb97458ff890f6d61f8f1203c33d50`.
Usar nomes novos de saída, porque evidência existente é imutável:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1315-ab-runner.py run --binary /dev/shm/p1315-target.V6TWEF/release/typst --binary-sha256 6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1 --orders normal,repeat,reverse --output 00_nucleo/diagnosticos/p1315-ab-rerun.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1315-ab-runner.py compare --freeze 00_nucleo/diagnosticos/p1315-ab-freeze.json --measurement 00_nucleo/diagnosticos/p1315-ab-rerun.json --output 00_nucleo/diagnosticos/p1315-ab-recomparison.json
```
