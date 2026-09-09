# P1333 — `calc.abs`: o erro do primeiro valor vence as sobras

## O que mudou

`calc.abs([x], 2)` agora rejeita content, em vez de acusar excesso de
argumentos. A mesma correção vale para outros tipos rejeitados, overflow
inteiro e comprimento misto, inclusive com named antes/depois do valor,
With, alias/import e spreads. Conservam-se a mensagem completa e a origem
do primeiro valor; não se aponta indevidamente para o argumento extra.

A causa era local: os guards named/aridade interrompiam a chamada antes
de processar o primeiro posicional. O vanilla ratificado converte esse
valor antes de validar sobras; sua conversão já inclui o módulo inteiro
e dimensional. O owner agora mantém essa precedência. Fórmulas, tipos
aceitos, registro/nome de abs e seleção de origem não foram alterados.
A auditoria reconstruiu o delta: fora de `calc_abs`, os bytes produtivos
são iguais ao baseline, descontando somente testes congelados e linhagem.

Dois limites foram preservados deliberadamente. Primeiro valor válido
com sobras e chamada sem posicional continuam usando os guards anteriores.
Além disso, `calc.abs(false, bad: panic("later"))` continua falhando no
panic: avaliar as expressões dos argumentos precede a entrada na nativa.
Não houve mudança de dispatcher, Args, API nem fase eval/layout.

## Ganho observado e o que falta

As métricas abaixo vêm de `p1333-metrics.json`, que identifica os recibos
e o inventário exato. São observações de um corpus delimitado, não
percentual de paridade de toda a linguagem.

| Mesmo corpus, antes/depois | Baseline P1332 | P1333 |
| --- | ---: | ---: |
| Observações integralmente iguais ao vanilla, em 712 | 532 | 608 |
| Diferenças restantes, em 712 | 180 | 104 |
| Equivalentes ao vanilla, somente as 616 históricas | 520 | 560 |
| Casos antes equivalentes que regrediram | — | 0 |

O ganho integral é de 76 observações: 40 históricas e 36 novas. As outras
636 preservam literalmente o baseline. Cada observação compara exit,
stdout e stderr completos; não se normalizou a saída candidata. O corpus
tem 178 expressões em quatro perfis: default, html, a11y e html+a11y.

As 104 diferenças restantes incluem repetições por perfil, não 104 defeitos
únicos. A expansão do corpus acrescentou fronteiras deliberadamente ainda
divergentes; por isso não se compara essa contagem diretamente às 96 do P1332.

| Pendência medida | Observações |
| --- | ---: |
| Ausência de posicional ou sobras após primeiro valor válido | 56 |
| Resolução de abs importado em math | 8 |
| Outras funções: sqrt | 8 |
| Operação Float×Fraction anterior a abs | 4 |
| Parser do literal mínimo inteiro | 4 |
| Construção path() anterior a abs | 4 |
| Gradientes: tipo inválido em space | 12 |
| Show: função não-elemento como seletor | 4 |
| Where em função não-elemento | 4 |

O próximo subconjunto de validação está explicitamente incompleto no L0
`compiler/stdlib/calc.md` e nomeado para P1334: mensagens/ordem das sobras,
ausência, named `value:` com hint e origem agregada da chamada. A fonte
mostrou que a ausência requer transporte da chamada inteira; não basta
inventar um span dentro de calc. Esses owners deverão ser medidos e
atualizados antes de implementar esse próximo recorte.

## Verificação

Testes congelados antes do candidato: seis snippets, 39 funções, com
migrações restritas à precedência especificada. O RED compilou e executou:
32 passaram, sete falharam na mensagem de primeiro valor versus guard.
O GREEN final passou os mesmos 39 testes. Workspace: 6.725 aprovados,
nenhuma falha e três doctests ignorados, sem somar novamente o focal.
Build release/locked, fmt, diff check, linhagem e V5/V15/V26 passaram.

Linter geral: zero erros, 240 warnings e 1.146 notas, mesmas contagens do
preflight. Nove notas mudaram somente o texto do padrão de abs para incluir
`..`; não se afirma que o multiconjunto textual ficou idêntico. A comparação
válida está em `p1333-lint-comparison-r1.json`. A tentativa anterior misturou
SARIF e texto e foi conservada como comparação inválida, não evidência.
O primeiro fmt detectou dois returns longos; após formatar e ressellar,
os gates finais foram repetidos com sufixo `-r1` e `unit-green-final`.

CLI normal, repetição normal e inversão: 712/712 aprovadas em cada ordem,
total de 2.136 verificações literais. Recibos públicos
`p1333-ab-cli-normal.json`, `p1333-ab-cli-repeat.json` e
`p1333-ab-cli-reverse.json`, com os recibos root correspondentes, identificam
o mesmo binário e a mesma árvore final. O fechamento verificável
`p1333-closure.json` exige os pareceres independentes
`p1333-ab-receipt.md` e `p1333-review-final.md`, além de conferir hashes
históricos, testes congelados e todos os gates sobre esse estado.

## Proveniência e limites da evidência

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
`p1333-baseline.json` registra o diff HEAD/stat, todos os arquivos alterados,
inventário e binários; SHA-256
`744983aa8c41877b21c2cd55abbc5c5ccb498a7bfbbb2fc5ea8dd893567ed8b0`.
Cada gate root conserva o estado antes/depois, argv, UTC e manifesto.
As alterações prévias foram preservadas; somente o par calc recebeu
mudanças produtivas nesta rodada. Sem stage, commit ou push.

Referência: vanilla ratificado upstream/main `a51e02804`, não a string
de versão. Binário `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline `/tmp/p1332-target.tTIpWy/release/typst`, SHA-256
`dfe7c3ab89eaa145d79e5b56ac72c657e49c6c994f9a88a4338915808ec0e8ea`.
Candidato `/tmp/p1333-target.MDDkou/release/typst`, SHA-256
`79470612fc121fa42a6898846f85d0b0325edf517c90a830c01cde947f748ffe`.
Target separado, copiado sem hardlinks; nenhum temporário anterior apagado.
RAM temporária insuficiente para esse cache, conforme target-capacity/free.

Norma congelada `517e1544fbd419a5b1c13e524c02570d3c22606a7b97d25562723c278bdd2e83`;
owner canônico `d4d546a7aef3c156edc42a309b5b1f88fe36e6d15a29f47852102fbd5afcc994`.
Manifesto `p1333-manifest.json`; freeze `p1333-ab-freeze.json`, SHA-256
`bdbadb8d0c91dc9823c085489ae2c058f0a20e0d1db844ae1346176a316c1199`.
GREEN final UTC 15:34:27–15:36:15, workspace 15:34:30–15:39:20, ambos em
2026-09-09; recibos incluem frações de segundo e listas exatas de arquivos.

A skill tekt-materializacao-segregada orientou autoria A/B e revisão
separadas: root escreveu L0/candidato; autor B recebeu somente entradas
públicas e testes históricos; revisor não editou entradas julgadas.
Executado sem atestação técnica de isolamento, em filesystem compartilhado;
sem selo de refinamento, mutation score ou alegação de equivalência geral.
Fixtures deliberadamente incoerentes de Args são somente ensaios sintéticos
de robustez, fora do domínio causal válido, e não legitimam writers stale.
O RED público coerente e a comparação CLI sustentam o ganho sem elas.
Timeouts de autorização automática precederam duas execuções, depois
autorizadas em repetição; não foram tratados como falhas do produto.
