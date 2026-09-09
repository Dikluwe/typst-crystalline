# P1330 — calc.abs deixa de saturar o menor inteiro

## Resultado e limite

O módulo de um inteiro deve caber no próprio tipo. O caso
`calc.abs(-9223372036854775807 - 1)` agora retorna o erro
`the result is too large` na expressão do argumento, em vez do número
9223372036854775807. Inteiros vizinhos, zero e sinais continuam exatos;
não há promoção para Float, wrap ou panic. Float e Decimal não herdam o
limite inteiro. O suporte dimensional P1329 permanece intacto.

A mudança produtiva está somente no braço Int de
`01_core/src/compiler/stdlib/calc.rs:141-149`. A origem é o value_span da
primeira ocorrência posicional; ausência ou detached permanecem detached.
Não se usa span agregado, ocorrência named ou origem inventada.
O L0 proprietário é `00_nucleo/prompts/compiler/stdlib/calc.md`, seção
P1330. O passo escrito é `00_nucleo/materialization/typst-passo-1330.md`;
ele coordena execução, não legitima o código.

### O que ainda falta

- O literal direto `-9223372036854775808` diverge antes de abs: vira Float
  no cristalino, mas é erro de parsing no vanilla. Usar essa grafia não
  testa o novo caso Int. A expressão de subtração acima constrói o inteiro.
- Guards de named e quantidade continuam precedendo a avaliação do caso
  de overflow no despacho nativo cristalino; o vanilla pode dar overflow
  antes de rejeitar esses argumentos. A diferença foi preservada.
- With e arguments podem manter o nome externo `calc.abs` no trace,
  enquanto vanilla usa `abs`. Mensagem e origem primárias de overflow
  estão corrigidas; não se declara equivalência integral nessas rotas.
- Permanecem as dívidas P1329 de construção dimensional NaN, ausência de
  `float.nan`, multiplicação Float×Fraction e certas grafias de math, além
  das diferenças de outros tipos e funções. Nenhuma foi mascarada por abs.

O alvo é a linguagem do vanilla ratificado upstream/main `a51e02804`,
não uma tag inferida de `--version`. Testes nativos observam espécie,
magnitude, mensagens, severidade, hints, traces, warnings e origem; a CLI
integral é sentinela complementar, não critério de bytes de render.

## Evidência e proveniência

Estado de partida: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`,
**working tree não commitado**. A medição começa em
`2026-09-09T13:04:16.518942+00:00`. O baseline
`p1330-baseline.json`, SHA-256
`a3f732bfb2fda2f177dbf3b33caddcd84b219af6fac22f96eae9644e784b6987`,
contém a lista exata dos arquivos alterados, diff HEAD/stat integral,
inventários e argv/horários de cada sonda. Os recibos de execução guardam
esses mesmos dados antes e depois dos gates. Sobre o estado P1329,
somente o par calc L0/owner mudou; as alterações anteriores são preservadas.

Binários pinados por conteúdo:

- BASE P1329: `/tmp/p1329-target.bg3p5A/release/typst`, SHA-256
  `9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec`.
- Vanilla: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato: `/tmp/p1330-target.f0lmDu/release/typst`, SHA-256
  `6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0`.

O target novo em /tmp é cópia sem hardlinks. O cache ocupava 3,8 GB,
acima dos 3,2 GB livres medidos em /dev/shm; targets anteriores não foram
removidos. Não houve stage, commit ou push.

## Como a verificação foi separada

A skill `tekt-materializacao-segregada` levou à execução A/B com root
como autor do L0/candidato, `p1330_tests` como autor dos testes/oráculos
sem ler runtime/patch calc e `p1330_review` como revisor sem editar produto
ou oráculos. **Sem atestação técnica de isolamento**, sem selo de
refinamento e sem alegação de paridade geral de abs ou calc.

Antes de congelar os testes, a auditoria encontrou duas asserções antigas
para a mesma saturação, e não apenas a nativa: havia também a expressão
avaliada. L0/manifesto receberam uma precisão R2 antes de C; a obrigação
de valor/erro não mudou. Só essas duas asserções receberam sucessoras.
Snippets e evidências históricas permanecem imutáveis, assim como todo o
módulo P1329. O corpus histórico mantém 248 células, migrando apenas as
quatro de overflow nos quatro perfis; prova em
`p1330-historical-expectations.json`, SHA-256
`fb3b2d075e51ff3f581a028a262a7c5c0574acd28eb3bdf97c7d5d8117633439`.

Um incidente de integração foi detectado antes de RED/C: contexto curto
do patch colocou o módulo novo antes de P1329, não no final do arquivo.
A asserção de montagem exata recusou essa posição. O módulo foi movido
intacto para o final e comparado integralmente com baseline + snippets
congelados. Não houve alteração de expectativa nem falha de compilação
nesse incidente. Registro: `p1330-integration-placement-incident.json`.

Manifesto vigente `p1330-manifest-r2.json`, SHA-256
`fa90e9d05855842467eb7baac85d81a157af03bb285b9c1d40df0faf1f660acf`.
Norma SHA-256
`9a5d734e086297d49d80d567bdb5527919049de19b7aea11f43b4467a972d2db`;
owner canônico
`30ef9f29b20e45b4ed51a368bfee566469df96db1dde5705755f8ca96b97794e`.
Depois do freeze, o L0 mudou somente na metadata recíproca Hash do Código.

## Gates finais

A CLI reúne 83 casos em quatro perfis (332 observações por execução).
As ordens normal, repetida e invertida passaram nas mesmas **996
comparações literais**, sem diferenças do contrato congelado. Isso não
significa que todas as saídas coincidem com vanilla: há dívidas previstas.

| Resultado integral por execução, calculado de BASE/CANDIDATE/VANILLA | Observações |
|---|---:|
| Mudou e passou a coincidir com vanilla | 32 |
| Mudou, mas mantém a dívida do nome no trace externo | 12 |
| Já coincidia e foi preservado | 220 |
| Dívida anterior preservada integralmente | 68 |

Assim, 252 das 332 saídas completas coincidem com vanilla, contra 220
no baseline deste mesmo corpus. A amostra foi escolhida para overflow e
suas fronteiras; não estima cobertura geral. Os nomes históricos de
categorias não foram somados como se fossem ganho novo.

| Gate | Resultado | Recibo em diagnosticos |
|---|---|---|
| RED antes de C | 23 executados: 17 passaram, seis falharam por saturação | `p1330-unit-red.json` |
| GREEN dos mesmos testes | 23 passaram, zero falhas | `p1330-unit-green.json` |
| GREEN no estado de linhagem final | 23 passaram, zero falhas | `p1330-final-unit-green.json` |
| Build release locked | exit 0 | `p1330-final-build.json` |
| Workspace release locked | 6.709 passaram, zero falhas, três ignorados | `p1330-workspace-tests.json` |
| CLI normal/repetida/invertida | 332 por ordem; zero diferenças do contrato | `p1330-cli-normal.json`, `p1330-cli-repeat.json`, `p1330-cli-reverse.json` |
| Fmt e diff | exit 0 | `p1330-final-fmt-resealed.json`, `p1330-final-diff-check.json` |
| Lint geral | zero errors, 240 warnings, 1.145 notes | `p1330-final-lint.json` |
| V5/V15/V26 estritos | zero violations | `p1330-final-lineage-lint.json` |
| Linhagem recíproca e oráculos intactos | PASS | `p1330-final-lineage.json` |

RED em `13:16:49.575816`–`13:18:36.686130` UTC; GREEN em
`13:20:14.215749`–`13:22:00.331156`; workspace em
`13:23:01.335138`–`13:25:57.117804`, todos em 2026-09-09.
A contagem do workspace soma seus 17 resumos Cargo. Recibo SHA-256
`44b3297431bf29c1557009446742214c62569d3a97e5988d7a226e9ded90a5a1`.
O GREEN foi repetido após mudar apenas a metadata Hash do Código para
conferir o mesmo inventário final dos demais gates; nenhuma expectativa
ou implementação foi ajustada entre as execuções.

Os diagnósticos do lint mantiveram severidade, regra, mensagem, arquivo,
multiplicidade e ordem do P1329; só as linhas de calc acompanharam o
acréscimo local. Auditoria em `p1330-review-lint-preservation.md`.
Não se afirma lint sem avisos; colunas não foram comparadas porque o
recibo textual anterior não as continha.

Os recibos públicos CLI têm SHA-256, respectivamente:
`082b6437d31dcc65ee61d9159ab903d9a84f24b04bfae6f15b71daacc0743402`,
`295c4ff6ca975d76b6dcaeed618206aa82acc119c24ecb98c1e7187ab3daf610`,
`f62c5c2de1a8fe9f8af98a449af9f5759a41b5c753b40794d340466330aff77e`.
Os wrappers root da tabela conservam a proveniência completa de execução.

Pareceres de fechamento: `p1330-ab-receipt.md` e
`p1330-review-final.md`. O agregador `p1330-closure.json` somente fecha
com ambos aprovados, todos os gates, comparação literal de cada célula,
hashes congelados e histórico preservado. Sem commit nem próximo passo.
