# P1332 — nome de abs nos diagnósticos

## Recorte medido

O nome registrado de calc.abs era calc.abs no baseline P1331; vanilla
ratificado a51e02804 usa abs. A representação repr já publica abs e não
precisa de correção. A mudança implementada é somente o nome intrínseco
no registro de calc.rs:58, não uma substituição textual no dispatcher.
Chave, callable, assinatura, corpo de abs e todos os outros registros
permaneceram intactos. L0 foi atualizado antes do código.

Isso afeta traces de With/arguments e traces dos guards ainda divergentes.
Os diagnósticos de gradient.*(space: calc.abs) e show calc.abs também
interpolam esse nome. Seus demais textos/âncoras não são corrigidos e não
se declara paridade dessas operações. Mensagens de aridade que contêm
calc.abs() não são nomes de trace e devem continuar intactas.

O comentário de Func::name sugere somente apresentação, mas igualdade/hash
nativos usam o nome também. A checagem da fonte não encontrou outra função
produtiva registrada como abs. A validação exige preservar identidade da
linguagem por lookup/alias/import e distinção das outras funções, não um
valor fixo de hash Rust entre revisões.

## Proveniência

HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093, working tree não commitado.
Baseline p1332-baseline.json SHA-256
1e9b77f550d5e34c4d3b52df5306eae3588122a35262d19128561532c33a7d97
registra diff HEAD/stat, inventário exato, argv/horários e binários.
Efeitos indiretos: p1332-name-consumers-public.json SHA-256
2d8ea9bb87879b307d1a7bd35e35c72feb02f913470b935aa8355207f8002e48.
BASE é /tmp/p1331-target.rtY0la/release/typst, SHA-256
a8d6e2f4472fefc9e63a123783feac852445191dcb82ac269d366a6419ae1a47;
vanilla /usr/local/bin/typst, SHA-256
7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8.
Não usar a string de versão como identidade.

O target dedicado é /tmp/p1332-target.tTIpWy, cópia sem hardlinks;
/dev/shm não tinha espaço suficiente. Os recibos p1332-target-capacity.json
e p1332-target-free.json, com o estado exato e horários, registram cache
de 3.982.626.887 bytes contra 3.454.021.632 bytes livres na RAM temporária.
Nenhum target anterior foi apagado.
Os dados históricos P1331 são preservados e verificados por hash.

Skill tekt-materializacao-segregada: A/B, root autor L0/candidato,
p1332_tests autor de testes sem runtime/patch/outputs privados, e
p1332_review revisor sem editar entradas julgadas. Executado em filesystem
compartilhado sem atestação técnica de isolamento; sem selo de refinamento.
O revisor apontou impactos indiretos antes de C e o L0 os incorporou antes
do freeze. O status Git inicial expôs nomes históricos não rastreados;
nenhum arquivo histórico de materialization/context foi aberto. O recorder foi
restringido a pathspecs permitidos nos status seguintes.

## Resultado

Correção implementada e gates finais concluídos. O corpus distingue ganho
integral de correção parcial do nome; não é uma medição global da linguagem.
Sem stage, commit ou push nesta rodada.

### Testes e fronteiras congeladas antes do candidato

Manifesto p1332-manifest.json SHA-256
b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084;
norma ac26a9a0492f250e4a59175256b8b1b7a4a173546dc483131a704f01262b84ad.
Freeze B p1332-ab-freeze.json SHA-256
7e2e2350cefbf5316131cd8ff9ccc0baf53be0efb36d3a0a4ffe059990c21719,
UTC 2026-09-09T14:40:08.207550+00:00, pina nove entradas públicas e
oito artefatos de teste, além do ledger preciso dos sucessores.

Sete linhas de testes históricos migraram: quatro nomes esperados de
trace, dois metadados esperados de abs e a condição auxiliar que separa
abs/sqrt. As expressões originais, textos primários de erro, âncoras e
controles sqrt permaneceram intactos. Os snippets antigos não mudaram;
integração cega registrada em p1332-test-integration.json SHA-256
e77f14a829afa0e69dfe1dfb0df3315cd1bc62d3eb4c8f048576b6d2ad74f2c6.

O RED compilado teve 28 aprovados e sete falhas exclusivamente de nome,
confirmadas por p1332-review-red.md. Recibo p1332-unit-red.json SHA-256
ce2f87b33ee14a6798ffbca953b9ffe2b0fb5d4ee58b8f5151e077440ee08750.
Seis falhas eram de Tracepoint e uma de Func::name; não eram erros de
compilação ou construção. O GREEN executou os mesmos 35 testes, todos
aprovados, alcançando os demais casos dos loops interrompidos pelo RED.
Recibo p1332-unit-green.json SHA-256
5b65eb016388c38d812201925abfa9d077f8e09a0a109938e5736bac85e0f2af,
de 2026-09-09T14:45:01.914959+00:00 a
2026-09-09T14:46:53.949671+00:00. A revisão independente
p1332-review-candidate.md confirmou a reconstrução integral do delta
produtivo, restrito ao registro.

Hash canônico final do owner:
0f671ea97c0408df0bddeead179cf44a8787cef780c3fbe8bd25b4c7f21a9cec.
Binário construído em /tmp/p1332-target.tTIpWy/release/typst, SHA-256
dfe7c3ab89eaa145d79e5b56ac72c657e49c6c994f9a88a4338915808ec0e8ea.
Recibo build p1332-final-build.json SHA-256
70a3597189ca3fb04b7ea24feac5633b489b183f0e0628904e41656bef4349e1.

Formatação, diff check, linhagem bidirecional e V5/V15/V26 terminaram
com exit 0. Linter geral: zero erros, 240 warnings e 1.146 notes. Seu
multiconjunto de regra/nível/mensagem/arquivo é idêntico ao P1331;
nenhum achado foi acrescentado ou removido. Não confundir zero erros
com ausência de achados. Recibo p1332-final-lint.json SHA-256
2ecb01ebe2ab195631800f18241732870680df8d4e07a9192adbe54d1b8b9f5f.

Durante a autoria pré-C, a nova sonda
`{import calc: abs; $abs(-1)$}` mostrou mais uma dívida: BASE retorna
equation/math.delimited, enquanto vanilla rejeita content. Foi
classificada como dívida de resolução de math antes do freeze, com as
quatro saídas originais preservadas. O baseline bruto conserva a hipótese
inicial no rótulo, e o freeze documenta sua refutação; o hash do runner
mudou nessa classificação pré-C, não para acomodar candidato.
Essa dívida não deve desaparecer de contagens nem virar alegação de
paridade. A expressão e os outputs estão em p1332-ab-cli-baseline.json.

### Resultado dos gates e ganho real

Os onze gates finais terminaram com exit 0 e o mesmo inventário produtivo
antes/depois: GREEN, build release/locked, workspace release/locked,
fmt, diff check, lint geral, linhagem bidirecional, V5/V15/V26 e as três
execuções CLI. Recibos p1332-*.json guardam argv, UTC, HEAD, diff/stat e
hashes suficientes para reproduzir os números sobre esta árvore não commitada.

Workspace: **6.721 aprovados, zero falhas e três doctests ignorados**,
sem somar novamente o GREEN focal. Execução de
2026-09-09T14:47:36.357913+00:00 a
2026-09-09T14:51:13.067195+00:00. Recibo p1332-workspace-tests.json,
SHA-256 da78d61fa58af847c1556d9e4be85e48b75d8fad7d9d919f5bc27d88cd38c98d.

CLI: **616 observações em cada ordem, 1.848 verificações aprovadas**
(normal, repetição normal e reversa). São 154 casos em quatro perfis.
Exit/stdout/stderr completos coincidem literalmente com o congelado,
sem normalizar as saídas candidatas.

| Medida por execução | BASE P1331 | C P1332 |
| --- | ---: | ---: |
| Equivalência integral com vanilla, corpus ampliado de 616 | 444 | 520 |
| Diferenças restantes, corpus ampliado | 172 | 96 |
| Equivalência integral, somente as 504 observações históricas | 404 | 452 |
| Casos antes equivalentes que regrediram | — | 0 |

Foram alteradas 144 observações (84 históricas e 60 novas). O ganho
integral é de **76**, sendo 48 históricas e 28 novas; as outras **68**
mudaram apenas o nome e continuam divergentes: 52 de guards e 16 de
gradientes/show. Não contabilizar essas correções parciais como paridade.
As demais 472 observações preservaram literalmente o BASE.

As 96 diferenças restantes do corpus são:

| Pendência observada | Observações |
| --- | ---: |
| Named/aridade: mensagem, precedência e/ou origem | 52 |
| Outras funções: sqrt | 8 |
| Operação Float×Fraction anterior a abs | 4 |
| Parser do literal mínimo inteiro | 4 |
| path() sem argumento, anterior a abs | 4 |
| Resolução de abs importado em math | 4 |
| Gradientes: tipo inválido em space | 12 |
| Show: função não-elemento como seletor | 4 |
| Método where em função não-elemento | 4 |

Não são 96 problemas únicos: a contagem inclui quatro perfis por caso
e controles sobrepostos. NaN dimensional/construção e outras pendências
fora desse corpus continuam abertas. O próximo recorte exige nova medição;
este passo não legitima corrigir todos esses owners nem reabrir a fase.

Recibos CLI públicos (os recibos root correspondentes conservam o estado
produtivo completo):

- p1332-ab-cli-normal.json, início UTC 2026-09-09T14:48:26.841466+00:00,
  SHA-256 9d7b8bac08f04d498bfaf72d22b467858daae91dec5e80f8dba24e08f4a95d0e.
- p1332-ab-cli-repeat.json, início UTC 2026-09-09T14:48:34.825241+00:00,
  SHA-256 f9eb8befa9a9ba6fa6ae30fbe8f2d964b3e70cf6a7dc94c4e744bccc06cd9dec.
- p1332-ab-cli-reverse.json, início UTC 2026-09-09T14:48:40.473767+00:00,
  SHA-256 5bf55fb972768c8ad5d09bca078fa243e0957fcf99bbf4f8ddad808579077ba3.

O fechamento verificável p1332-closure.json exige ainda os pareceres
p1332-ab-receipt.md e p1332-review-final.md e pina seus hashes, os gates
e este relatório. Preservação histórica é revalidada nessa operação.
