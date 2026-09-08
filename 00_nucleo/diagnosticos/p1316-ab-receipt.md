# P1316 — recibo A/B independente

Resultado: PASS restrito ao fragmento congelado, executado sem atestação de
isolamento técnico. As 3.960 comparações completas de exit/stdout/stderr
passaram nas ordens normal, repetida e inversa, com zero diferenças inesperadas
e zero Unknown. Não constitui selo completo de refinamento ou paridade CSV geral.

## Identidades e proveniência

Baseline: P1315 não commitado sobre HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, executável
`/dev/shm/p1315-target.V6TWEF/release/typst`, SHA-256
`6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1`.
Vanilla ratificado upstream `a51e02804`: `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Os hashes de ambos foram conferidos antes e depois das execuções.

O freeze precedeu a liberação do candidato: `p1316-ab-freeze.json`, SHA-256
`200816280fc2e1c18934103574c6d33c3d972bdfb576221c4a74d3175d385e5f`.
O L0 proprietário `00_nucleo/prompts/compiler/stdlib/loading.md` tem SHA-256
normativo `f429ebf31bd6723cf8a69b7be82f2c223ced005bf367655ef17cadd26026b112`,
excluindo somente a linha canônica `Hash do Código`. O hash bruto no freeze
era `06faafea633ab0bd17a8d2c0862eff0123d6c23618ffff7367d8517e840c6bb1`.
Nenhum passo tático foi pinado como entrada normativa.

Candidato: `/tmp/p1316-target.1c6HK7/release/typst`, SHA-256
`1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba`.
Execução de `2026-09-08T14:42:25.757472+00:00` a
`2026-09-08T14:45:51.470755+00:00`, no mesmo HEAD, working tree não commitado.
O `git diff HEAD --stat` registrado no início foi:

```text
00_nucleo/prompts/compiler/stdlib/loading.md | 124 +++++++++++++++-
01_core/src/compiler/stdlib/loading.rs       | 213 +++++++++++++++++++++++++--
2 files changed, 322 insertions(+), 15 deletions(-)
```

Estado, status completo, UTC individual, argv, cwd, duração e observações
integrais estão em `p1316-ab-candidate-runs.json`, SHA-256
`5f784d7248a14d47ca449bbea99287a9bdb202aabd07e8d582a60c420fd26bc4`.
A comparação verificável está em `p1316-ab-comparison.json`, SHA-256
`d80d70ca2b1a598df9491a6eab6b716fd44191975f366c405ba626c9ccb2ccae`.
Todas as entradas protegidas e o L0 normativo foram revalidados; a mudança
posterior de metadata de linhagem não alterou o conteúdo normativo.

## Observável congelado

São 330 casos nos perfis default, html, a11y e html+a11y: 290 casos de replay
P1315 com expressões e cwd originais e 40 casos focais/controles. Antes da
transformação de expectativas, as 1.160 observações históricas foram
comparadas literalmente ao candidato P1315 registrado; todas coincidiram.

| Classe | Casos | Comparações candidatas |
|---|---:|---:|
| Origem parsing Bytes predeclarada | 69 | 828 |
| Preservação literal | 261 | 3.132 |

Nos casos de origem, a expectativa é a primeira linha completa do diagnóstico
baseline seguida do restante integral do diagnóstico vanilla. Isso preserva
literalmente a mensagem do decoder, incluindo texto UTF-8 e ordinal P1315,
e verifica a apresentação da origem primária e dos traces. Não remove sufixos
das observações para alegar igualdade dos produtos nem reescreve mensagens.

As 276 expectativas de origem incluem 16 controles Args.map detached,
que já coincidiam com a expectativa; as outras 260 eram RED no baseline e
ficaram GREEN em todas as três ordens. As 1.044 expectativas literais também
passaram nas três ordens. Não houve instabilidade por repetição/reordenação.

O recorte cobre UnequalLengths e UTF-8 em array/dictionary, named antes da
fonte, fonte pré-ligada em With, Args com named anterior, spread de array,
sink e Args.map. O replay acrescenta registros multilinha, vazios, CRLF,
delimiter, cabeçalho e erro tardio. Controles protegem Path/Str e erros de
arquivo, valores válidos, excesso com fonte válida, opções, casts e outros
loaders. O oráculo de Args.map confirma origem detached, sem range fabricado.

## Calibração e dívida preservada

A revisão prefreeze corrigiu o contador de diagnósticos do harness: o padrão
genérico `error: ` também contava `CSV parse error:` dentro da mensagem UTF-8.
O contador final exige início de linha. As mensagens esperadas não mudaram.

A mesma revisão corrigiu quatro controles de excesso: com primeira fonte
malformada, o baseline atinge parsing, enquanto o vanilla rejeita o segundo
positional. Essa dívida de ordem já existia e impossibilitava usar o diagnóstico
vanilla de outro estrato como oráculo de origem. Os controles finais usam fonte
válida e preservam integralmente o comportamento baseline. O L0 foi atualizado
antes do freeze para exigir que a colisão parsing+excesso mantenha o parsing
legado e receba a origem do primeiro Bytes, como efeito normativo local.

Os quatro casos malformados originais não foram apagados: estão em
`p1316-ab-cases.json` e `p1316-ab-baseline-runs.json`. A comparação bilateral
suplementar está em `p1316-ab-excess-debt.json`, SHA-256
`a6662c21ccd3c8c981f41834f0291ea10a3bef8a8ef158ee8821fd41a5c7be25`, com
16 observações baseline reutilizadas e 16 vanilla novas. Esses casos não
participam do oráculo bilateral de origem; a obrigação normativa de origem
primeiro-vs-segundo fica com o teste local, que este papel não inspecionou.

Após a revisão, somente o recorte afetado e controles UTF-8 foram reexecutados
(48 processos). `p1316-ab-baseline-final.json`, SHA-256
`965b2c57b5764681a85218bf7b84b25c031655078a86b3e94f097cd7b8e987ab`, é uma
composição explícita: conserva UTC/argv/cwd dos resultados iniciais, substitui
somente as linhas reexecutadas e verifica cada expressão e cwd final. Não é
uma nova execução integral. Registra os hashes de ambos os conjuntos e a
clarificação normativa anterior ao freeze. Nenhum candidato foi consultado
para calibrar, classificar os casos ou construir expectativas.

## Custo

Cada artefato abaixo conserva HEAD, diff/stat, status e timestamps próprios.
As durações são as janelas entre recibos UTC, excluindo autoria e espera.

| Artefato | Processos novos | Janela em segundos |
|---|---:|---:|
| baseline-runs | 1.596 | 54,738079 |
| focal-runs | 48 | 0,949636 |
| excess-debt | 16 | 0,205445 |
| candidate-runs | 3.960 | 205,713283 |

Total registrado: 5.620 processos e 261,606443 segundos de janelas de execução.
Houve ainda uma consulta exploratória vanilla isolada sobre excesso, repetida
no suplemento registrado; ela não é usada como prova independente de fechamento.
Uma revisão de calibração produziu testemunha nova e resolveu os bloqueios do
harness. A política previamente declarada interrompe duas revisões sem ganho;
esse limite não foi atingido. Não houve mudança ou nova calibração após freeze.

## Capacidades e limites

Executor `/root/p1316_tests`, papel testador A/B. Leu L0, fontes vanilla
autorizadas (loading/mod.rs, csv.rs, diag.rs e foundations/args.rs), artefatos
A/B P1315 e fixtures, observações CLI, skill e referências e metadados git
HEAD/status/diff STAT. Não leu owner `loading.rs`, testes locais, patch
candidato, recibo de build com diff embutido ou measurement com código.
Escreveu somente `00_nucleo/diagnosticos/p1316-ab-*` e
`/tmp/p1316-ab-fixtures`. Skills e demais entradas têm hashes no freeze.

O filesystem é compartilhado e as capacidades não têm isolamento técnico
atestado. A segregação é operacional. A CLI não constrói Args Rust sintético
sem occurrences nem permite distinguir occurrence.span de value_span em um
positional sintético. Também não atesta spans do decoder puro, ausência de
chamadas internas ao World, arquitetura, mutation score ou equivalência geral.
Essas obrigações exigem evidência dos papéis pertinentes.

Unknown bloqueia para entradas alteradas, identidade ambígua, dados ausentes
ou duplicados, construção sem suporte, timeout ou crash. Nenhum Unknown foi
convertido implicitamente em sucesso. O resultado refere-se exclusivamente
às identidades, expressões e observáveis congelados.

## Reprodução

Runner congelado `p1316-ab-runner.py`, SHA-256
`a7677d26111d90db39f8bec059642b07cfa6a29e02ffaaa4705f2a455ec02d9d`;
casos `p1316-ab-cases-r1.json`, SHA-256
`1bc61227bfd8e2f8e1188a8ce40343f60c7157c92b7346f4eec6ec4ce58372b8`.
Preservar fixtures e cwd históricos. Usar saídas novas, pois os arquivos de
evidência são imutáveis:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1316-ab-runner.py run --binary /tmp/p1316-target.1c6HK7/release/typst --binary-sha256 1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba --orders normal,repeat,reverse --output 00_nucleo/diagnosticos/p1316-ab-rerun.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1316-ab-runner.py compare --freeze 00_nucleo/diagnosticos/p1316-ab-freeze.json --measurement 00_nucleo/diagnosticos/p1316-ab-rerun.json --output 00_nucleo/diagnosticos/p1316-ab-recomparison.json
```
