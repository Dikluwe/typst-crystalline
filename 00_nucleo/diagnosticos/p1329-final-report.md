# P1329 — calc.abs passa a aceitar grandezas dimensionais

## O que mudou

`calc.abs` agora calcula o módulo de length, angle, ratio e fraction,
conservando o tipo da linguagem. Exemplos medidos no candidato final:

| Expressão | P1328 | P1329 e vanilla ratificado |
|---|---|---|
| `calc.abs(-2pt)` | Erro de tipo | `2pt` |
| `calc.abs(-2em)` | Erro de tipo | `2em` |
| `calc.abs(-2deg)` | Erro de tipo | `2deg` |
| `calc.abs(-2%)` | Erro de tipo | `2%` |
| `calc.abs(-2fr)` | Erro de tipo | `2fr` |

Comprimentos relativos não são resolvidos usando um tamanho de fonte.
Comprimentos com componentes absoluta e em simultaneamente não zero
continuam rejeitados, inclusive quando têm o mesmo sinal: por exemplo,
`calc.abs(2pt + 3em)`. Agora o erro é `cannot take absolute value of this
length`, localizado no argumento. A origem é mantida em aliases, With e
spread; ausência de origem sintética permanece detached.

A implementação mudou somente os braços dimensionais de
`01_core/src/compiler/stdlib/calc.rs:145-168` e imports necessários.
O diagnóstico de conteúdo P1328, Int/Float/Decimal, saturação inteira,
guards, demais tipos e outras funções calc permaneceram intactos.
O L0 proprietário é `00_nucleo/prompts/compiler/stdlib/calc.md`, com o
esclarecimento de domínio P1329-R2. O plano escrito é
`00_nucleo/materialization/typst-passo-1329.md`; ele não substitui o L0.

## Ganho observado, sem inflar a paridade

O corpus contém 63 casos em quatro perfis, totalizando 252 observações
por execução. Normal, repetida e invertida passaram nas mesmas 756
comparações literais de exit/stdout/stderr contra expectativas pré-C.
Além dessa sentinela CLI, os testes focais verificam tipo/magnitude,
origens e diagnósticos completos; não usam bytes de render como definição
de paridade da linguagem.

Esta é a classificação calculada diretamente das saídas BASE/CANDIDATE/
VANILLA por execução, não a soma dos nomes históricos das categorias:

| Resultado integral | Observações |
|---|---:|
| Mudou e passou a coincidir com vanilla | 128 |
| Mudou, mas mantém dívida de nome do trace externo | 12 |
| Já coincidia e foi preservado | 68 |
| Dívida anterior preservada integralmente | 44 |

Assim, 196 das 252 saídas completas coincidem com o vanilla, contra 68
no baseline deste mesmo corpus. As demais não são chamadas de paridade.
O corpus foi escolhido para esta função e suas fronteiras; esses números
não estimam a cobertura geral do compilador.

Todos os 28 casos CLI do P1328 foram repetidos. Suas 112 observações
conservaram expressão/perfil: 96 saídas permaneceram idênticas e somente
as 16 correspondentes aos quatro tipos dimensionais migraram para o
resultado vanilla. O módulo unitário antigo sofreu a mesma migração
limitada, com imports necessários; todas as outras asserções foram
preservadas. Snippets, expectativas e recibos históricos em diagnosticos
não foram reescritos. Prova: `p1329-historical-expectations.json`,
SHA-256 `0fa6b48daadbbe6efdae1614abe1e088ba0fb31e216e457821d822c489f5cfc0`.

## O que ainda falta

- Overflow de `i64::MIN`: o cristalino ainda satura; o vanilla rejeita.
- Guards de aridade/named, rejeições de string/symbol e diagnósticos de
  outras funções continuam com diferenças anteriores.
- Traces externos de certas chamadas With/arguments ainda usam
  `calc.abs`, enquanto o vanilla usa `abs`. A mensagem e o span primários
  de Length misto estão corrigidos; o nome do trace não foi alterado.
- `float × fraction` pode falhar antes de abs no cristalino, inclusive
  `calc.abs(-calc.inf * 1fr)`. Isso não é falha do novo braço Fraction.
- Há diferenças anteriores de construção de NaN dimensional e ausência
  de `float.nan` no cristalino. Elas não foram escondidas pelo novo suporte.
- O controle math com `-2pt` pode falhar antes de abs por variável `pt`
  desconhecida. O teste math com `-2deg` verifica conteúdo sem coerção
  numérica; este passo não muda a fase nem a avaliação de math.

### A ressalva NaN que obrigou a reabrir a norma antes de C

A fonte vanilla `typst-utils/src/scalar.rs:29-31` converte NaN em zero
ao construir Scalar, usado pelas grandezas dimensionais. A primeira
norma não distinguia suficientemente esse domínio da representação
cristalina. O trabalho foi suspenso antes da integração/implementação;
L0, manifesto e freeze receberam sucessores R2. Nenhuma expectativa de
valor ou diagnóstico foi ajustada depois de ver um candidato.

Não se trata apenas de entradas artificiais: `(calc.inf - calc.inf) *
1deg` e a variante `* 1%` produzem NaN no cristalino e zero no vanilla.
Depois deste passo, `calc.abs` sobre esses valores cristalinos conserva
NaN, enquanto o vanilla recebe a grandeza já normalizada e devolve zero.
Isso é uma **dívida de construção anterior à chamada**, não paridade
fechada pelo P1329. Os testes nativos NaN verificam somente a obrigação
local de módulo sobre valores já construídos. Inf é representável no
vanilla e foi distinguido de NaN; `calc.abs(-calc.inf * 1pt)` coincide.

As testemunhas estão em `p1329-nan-domain-probe-r2.json`, SHA-256
`8ca6f55fbcde70a3a4edcc521dccaa248caedd872650b6b735e113991ea84b9c`,
e no relatório de execução final `p1329-domain-final.json`, SHA-256
`0b0d4d99479590f9a57f259f92821103b8de549081a9e6a13ad2a2e816565717`.
Essas medições não entram no total de comparações A/B aprovadas como se
demonstrassem equivalência geral. Impressão de Length não foi usada para
inferir a identidade de componentes NaN.

## Gates e proveniência

HEAD de todas as medições:
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, **working tree não
commitado**. Os recibos root abaixo incluem a lista exata de arquivos,
diff HEAD integral, `git diff HEAD --stat`, inventário SHA-256, argv,
target e estados antes/depois. Somente o par calc L0/owner mudou sobre
o fechamento P1328; os demais arquivos previamente alterados foram
preservados. A cadeia histórica é conferida pelo fechamento P1329.

Baseline `p1329-baseline.json`, SHA-256
`d0e1787fac8b6264122ca6dcf5e29e4729552e8031e591ce6f4ee725e14cbd23`,
início `2026-09-09T12:08:08.824769+00:00`.

| Gate | Resultado | Recibo em diagnosticos |
|---|---|---|
| RED compilado | 8 passaram, 7 falharam pelas lacunas previstas | `p1329-unit-red.json` |
| GREEN dos mesmos testes | 15 passaram, zero falhas | `p1329-unit-green.json` |
| Build release locked | exit 0 | `p1329-final-build.json` |
| Workspace release locked | 6.701 passaram, zero falhas, três ignorados | `p1329-workspace-tests.json` |
| CLI normal/repetida/invertida | 252 por ordem; zero diferenças do contrato | `p1329-cli-normal.json`, `p1329-cli-repeat.json`, `p1329-cli-reverse.json` |
| Fmt e diff | exit 0 | `p1329-final-fmt.json`, `p1329-final-diff-check.json` |
| Lint geral | zero errors, 240 warnings, 1.145 infos | `p1329-final-lint.json` |
| V5/V15/V26 estritos | zero violations | `p1329-final-lineage-lint.json` |
| Linhagem bidirecional e oráculos intactos | PASS | `p1329-final-lineage.json` |

RED: `12:35:52.022315`–`12:37:43.196470` UTC; GREEN:
`12:40:05.108442`–`12:41:54.285617`; workspace:
`12:42:58.973751`–`12:46:05.617429`, todos em 2026-09-09. A contagem
do workspace soma seus 17 resumos Cargo. Recibo SHA-256
`14ac6ecdf719629aeb49714eaddd3ca5efa3df34a8da67276bad76405a484219`.
Os 240 warnings do lint são os anteriores, desconsiderados somente
deslocamentos de linhas nessa comparação; há quatro infos novas.
Não se afirma lint geral sem avisos.

Binários, identificados por bytes e não pela string de versão:

- BASE P1328: `/tmp/p1328-target.T7Tg57/release/typst`, SHA-256
  `94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9`.
- Vanilla upstream/main ratificado `a51e02804`: `/usr/local/bin/typst`,
  SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato: `/tmp/p1329-target.bg3p5A/release/typst`, SHA-256
  `9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec`.

Target dedicado copiado sem hardlinks em /tmp; a RAM do host tinha 3.3G
livres diante de um cache anterior de 3.8G. Targets anteriores preservados.

## Separação de autoria e fechamento

A skill `tekt-materializacao-segregada` orientou o regime A/B: root
escreveu L0/candidato e integrou os bytes congelados; `p1329_tests`
escreveu os testes e oráculos sem ler o runtime ou patch calc;
`p1329_review` revisou sem editar produto/oráculos. Execução em ambiente
compartilhado, **sem atestação técnica de isolamento e sem selo de
refinamento**. A revisão R2 de domínio ocorreu antes de RED e C;
snippets e expectativas mantiveram seus valores, mudando somente os
vínculos normativos do runner/freeze/metadata.

Manifesto vigente `p1329-manifest-r2.json`, SHA-256
`c7f5963d2735226ae3ddf653deb13856e187f723312fcfd7521c42ae5011fbe1`;
norma SHA-256 `07f83fc24dc13837f54a25f0bec6be20ff495e1f679c4975bec3f1fb583ef5ae`;
owner canônico `483d55d0fd0da9545f76aa3d63dd0d6f6f8f32603593476cf27cf511a051b503`.
Após o freeze R2, só o metadado recíproco Hash do Código mudou no L0.

Pareceres: `p1329-ab-receipt.md` e `p1329-review-final.md`. O agregador
`p1329-closure.json` exige os gates, verifica literalmente todas as células,
os hashes congelados, o estado produtivo e os artefatos históricos.
Sem stage, commit, push ou criação do próximo passo nesta execução.
