# P1328 — calc.abs: erro de conteúdo com origem correta

## Resultado concreto

O passo corrige a rejeição de conteúdo em `calc.abs`: a mensagem passa a
ser a do vanilla ratificado e o erro aponta para o argumento que recebeu
esse conteúdo. Isso inclui chamadas em math, markup, aliases, `With` e
spread. Não transforma conteúdo matemático em número nem amplia os tipos
numéricos aceitos pelo compilador.

Antes, `calc.abs([abc])` devolvia `calc.abs() requer Int ou Float, recebeu
content`, sem localização primária e com um trace adicional. Agora devolve
`expected integer, float, length, angle, ratio, fraction, or decimal, found
content`, apontando para `[abc]`. Em `$std.calc.abs(-1)$`, o argumento
continua sendo conteúdo da linguagem; a mesma rejeição agora localiza `-1`.

A mudança produtiva está somente no ramo de conteúdo em
`01_core/src/compiler/stdlib/calc.rs:145-156`. Ele cobre `Content` e
`LocatedContent`, usa o `value_span` da primeira ocorrência posicional e
mantém detached quando não há origem. Não usa o span agregado de Args nem
o span da ocorrência como substitutos. Named, aridade, casos numéricos,
outros tipos e outras funções calc conservam seus caminhos anteriores.

O L0 proprietário é `00_nucleo/prompts/compiler/stdlib/calc.md`; o plano
escrito é `00_nucleo/materialization/typst-passo-1328.md`. A correção segue
ADR-0127 em fluxo contínuo: não muda API, default ou fase do pipeline.

## O que os resultados demonstram — e o que não demonstram

Os recibos `p1328-ab-cli-normal.json`, `p1328-ab-cli-repeat.json` e
`p1328-ab-cli-reverse.json` contêm 112 observações cada, com comparação
literal de exit/stdout/stderr contra expectativas congeladas antes do
candidato. Todas as 336 comparações passaram, sem normalização de saída.

| Classe por execução | Observações | Interpretação |
|---|---:|---|
| Correção integral de conteúdo | 44 | Saída completa igual ao vanilla |
| Correção com dívida de nome do trace | 12 | Mensagem e origem corrigidas; nome externo preservado |
| Controle numérico já em paridade | 12 | Resultado anterior preservado e igual ao vanilla |
| Dívida fora do recorte | 44 | Saída integral anterior preservada, ainda diferente do vanilla |

Portanto, 56 das 112 saídas completas coincidem com o vanilla depois da
correção, contra 12 no baseline deste corpus. As demais não foram
reclassificadas como paridade. Esse corpus é dirigido ao diagnóstico e a
suas fronteiras, não uma estimativa de cobertura da linguagem.

Em `content-with-bound`, `content-with-nested` e `content-spread-args`, os
rastros externos ainda dizem `calc.abs`, enquanto o vanilla diz `abs`.
São as 12 observações da segunda linha. A política foi medida e congelada
antes de C, em `p1328-review-trace-policy.md`; o comparador não apaga ou
reescreve esse trecho durante a avaliação do candidato.

## O que ainda falta

As diferenças abaixo foram observadas no baseline e nos controles finais;
não são fechadas por este passo:

- `calc.abs(1pt)` e `calc.abs(-2deg)` continuam rejeitados, enquanto o
  vanilla aceita length e angle. Ratio e fraction também permanecem fora
  do suporte autorizado neste recorte. A lista de tipos na nova mensagem
  não deve ser confundida com suporte já implementado.
- O menor inteiro continua saturando para o maior inteiro; o vanilla
  rejeita o resultado grande demais.
- Ausência/excesso de argumentos e named inesperado conservam mensagens
  e prioridade anteriores, ainda diferentes do vanilla.
- String, symbol e diagnósticos de outras funções, como `calc.sqrt`, não
  receberam a correção de mensagem/origem deste passo.
- A dívida de nome nos traces externos descrita acima permanece aberta.

Não há alegação de paridade geral de `abs`, de `calc` ou do compilador.
Um próximo trabalho deve escolher e medir uma dessas superfícies, em vez
de considerar o nome da função como unidade já concluída.

## Verificação e proveniência

Todas as medições foram feitas sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093` com **working tree não
commitado**. Os recibos root citados abaixo guardam a lista exata de
arquivos, inventário SHA-256, `git diff HEAD --stat`, diff integral,
comando, target, instantes UTC e estados antes/depois. Somente o par
calc L0/owner mudou sobre o baseline P1327; os outros doze arquivos já
alterados foram preservados. Os artefatos anteriores são verificados pelo
inventário histórico encadeado ao fechamento P1327.

Baseline: `p1328-baseline.json`, SHA-256
`e049418db46ea039235b336254bfdbffd492253ac66c38782ac6bc56564f2ffb`,
início UTC `2026-09-09T11:35:18.901045+00:00`.

| Gate | Resultado | Recibo em diagnosticos |
|---|---|---|
| RED R2 | Compilou; cinco testes falharam pela mensagem antiga, dois controles passaram | `p1328-unit-red-r2.json` |
| GREEN dos mesmos testes | 7 passaram, zero falhas | `p1328-unit-green.json` |
| Build release locked | exit 0 | `p1328-final-build.json` |
| Workspace release locked | 6.693 passaram, zero falhas, três ignorados; 17 resumos | `p1328-workspace-tests.json` |
| CLI integral normal/repetida/invertida | 112 por ordem, zero divergências do contrato | `p1328-cli-normal.json`, `p1328-cli-repeat.json`, `p1328-cli-reverse.json` |
| Formatação e diff | exit 0 | `p1328-final-fmt.json`, `p1328-final-diff-check.json` |
| Lint geral | zero errors, 240 warnings, 1.141 infos | `p1328-final-lint.json` |
| V5/V15/V26 estritos | zero violations, exit 0 | `p1328-final-lineage-lint.json` |
| Linhagem canônica e snippet intacto | PASS | `p1328-final-lineage.json` |

O RED R2 ocorreu entre `11:51:23.818009` e `11:52:48.227681` UTC; o
GREEN, entre `11:54:30.153404` e `11:55:58.070887`. A suíte completa foi
executada entre `11:56:44.291836` e `11:59:01.388051` UTC, todos em
2026-09-09. Seu recibo tem SHA-256
`fc940d1a7b776ddc004f4de1ae8efb2ecfca29b5370b16af098e56c722d3ae3b`.
As contagens de testes somam os campos dos resumos Cargo; as do lint
contam seus registros de severidade. Os 240 warnings são os anteriores
(desconsiderando apenas deslocamentos de linhas para essa comparação);
há duas infos novas. Não se afirma lint geral sem avisos.

Binários usados, identificados por bytes, não por string de versão:

- BASE P1327: `/tmp/p1327-target.k9Mq0s/release/typst`, SHA-256
  `75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31`.
- Vanilla ratificado upstream/main `a51e02804`: `/usr/local/bin/typst`,
  SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato P1328: `/tmp/p1328-target.T7Tg57/release/typst`, SHA-256
  `94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9`.

Foi mantido o target temporário dedicado em `/tmp`, copiado sem hardlinks
do baseline. A autorização para RAM não foi tratada como obrigação de
recriar os targets existentes. Nenhum target anterior foi removido.

## Testes separados e limites da evidência

A skill `tekt-materializacao-segregada` orientou o regime A/B: root
redigiu L0 e implementação; `p1328_tests` escreveu testes/oráculos sem ler
o runtime calc ou recibos com código; `p1328_review` revisou os artefatos
sem editá-los. O ambiente é compartilhado: **executado sem atestação
técnica de isolamento**, sem selo de refinamento ou mutation score.

O manifesto `p1328-manifest.json` congela a norma SHA-256
`5d8d8333b2ed042cb5af404957d7c8895fcf6e499798dd863aa54a3b321caa4e`.
O hash canônico final do owner é
`33f8772fe163ae379a88b72b50ba2e4926104e84e03e3c9af7e11a399129e2e6`.
O único ajuste no L0 após o freeze foi seu metadado recíproco Hash do Código.

Dois incidentes instrumentais ficam preservados, não ocultados: a primeira
medição CLI usou posição de flag incompatível com o vanilla e foi
substituída por baseline R1 válido antes de C; o primeiro snippet passou
por sucessor R2 apenas para ordenar imports conforme edition 2021. O RED
R1 permaneceu válido e o RED R2 foi repetido antes da implementação.
Nenhuma expectativa funcional mudou para acomodar o candidato.

Os pareceres de fechamento são `p1328-ab-receipt.md` e
`p1328-review-final.md`; `p1328-closure.json` agrega os recibos e verifica
os hashes históricos, os oráculos congelados e o estado produtivo final.
Não houve stage, commit ou push, nem abertura do passo seguinte.
