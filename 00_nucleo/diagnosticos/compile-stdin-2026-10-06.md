# Compilação de stdin com saída explícita — 2026-10-06

`compile - OUTPUT` agora adquire o documento de stdin e mantém seu Source em
RAM; não abre um arquivo físico chamado `-`. Output `-` publica os bytes em
stdout, tanto para entrada stdin quanto física. Imports/read, fontes, inputs,
CA e plugins conservam os builders existentes. Não há nova API, enum, unsafe,
execução do documento ou mudança de fase.

## Evidência anterior à decisão

`compile-stdin-2026-10-06/before.json.gz` conserva fontes de entrada, comandos,
cwd, UTC, HEAD, git diff HEAD --stat/status e hashes dos arquivos alterados e
binários. O arquivo físico `-` contém #unknown_physical; todos os controles de
stdin da primeira coleta chegam indevidamente a esse arquivo no Tekt.
`04_wiring/src/compile.rs:223-270` compunha SystemWorld::new para toda entrada.
O upstream documenta `-` como stdin em
`lab/typst-original/crates/typst-cli/src/args.rs:164`; seu world.rs:222-242
valida a raiz e seleciona stdin, e world.rs:282-290 adquire bytes.
Essa documentação sustenta intenção; aquisição lazy e FileId upstream são
mecânica, não requisito de paridade (ADR-0107/0108).

O owner `00_nucleo/prompts/wiring/compile.md` foi atualizado antes dos testes e
implementação. `00_nucleo/prompts/wiring/tests/cli.md` legitimou as regressões.
A classe é correção interna de paridade ADR-0127, usando a API for_stdin já
ratificada. Apenas esses dois owners foram selados; V15/V26 passaram antes do
resselo. `seal.py` e `seals.json` mantêm algoritmo, escopo e hashes; não houve
reparo global. Todo trabalho anterior permanece na árvore.

## RED, reparo e refutação

`red-state.json` e `red.log` registram os três testes novos falhando antes da
implementação. `green.log` conserva o RED residual de stdout: o documento já
vinha de stdin, mas o writer gravava o output `-` em disco. O L0 especificou o
encaminhamento a stdout no mesmo ponto de publicação; o teste também exige que
o arquivo físico `-` não seja sobrescrito.

`after.json.gz` refutou uma expectativa inicial incorreta: com --root project,
o vanilla apresenta project/<stdin> relativo à cwd, não <stdin> incondicional.
O teste foi corrigido pela sonda e falhou em `root-red.log`. O reparo final usa
o formatter existente e o caminho causal do World, preservando a identidade
stdin e as origens dos imports. `green-final.log` registra as três regressões
passando no estado entregue. Não foi projetado ou apagado o stderr divergente.

L4 valida a raiz, adquire bytes uma vez e compõe for_stdin com UTF-8 válido.
UTF-8 inválido e raiz ausente reproduzem status/mensagem/hint do pin medido.
Os bytes parciais em BrokenPipe seguem a regra explícita upstream; esse ramo
não foi provocado diretamente na coleta. Os arms de PDF/PNG/SVG/HTML/Bundle,
timings, opções e política de publicação permanecem os existentes.

## Proveniência da coleta final

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitada.
Início UTC `2026-10-06T17:08:32.829769+00:00`.
`compile-stdin-2026-10-06/final.json.gz` registra a lista exata de alterações
em git diff HEAD --stat/status, hashes dos arquivos produtivos/L0 alterados,
fontes, cwd, ambiente, comandos, retornos, stderr, conteúdo, PDF bbox e hashes
de artefatos. O oracle tem proveniência pinada P1489, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4` e commit upstream
`9f2b6e8715237cb086899a42873660fe744622e8`.

- Tekt release: `9598c078f72678f399171886afbd5448d0b68f2c30807c9a6b690f80146b2a09`.
- Oracle lab/typst-original/target/release/typst:
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- O baseline anterior foi congelado em /tmp/tekt-compile-stdin/typst-before;
  seu hash está em before.json.gz e before-expanded.json.gz. A coleta ampliada
  foi executada após editar as fontes, mas antes de rebuildar release:
  seu binário continua o baseline, identificado por esse hash e pela coleta
  anterior. Não atribuir os hashes de fonte candidatos dessa coleta ao build
  anterior nem somar rodadas.

## Validação e limites

`summary.json` e `summarize.py`: **15 dos 16 controles** passam nos critérios
explicitados. O único DIFF é input stdin sem output: o parser ainda deriva
-.pdf, enquanto vanilla exige <OUTPUT> e retorna 2. Essa validação de L2
permanece aberta; o recorte de saída explícita passou por inteiro.

Nos PDFs com texto, conteúdo, páginas e caixas coincidem dentro de 0,05pt;
maior delta medido 0,00036pt. HTML compara DOM do body. SVG certifica nesta
coleta somente disponibilidade/status/stderr, sem alegar paridade de geometria
ou tinta. PNG é controlado por assinatura e sucesso no teste CLI, não por
comparação raster nova. PDF completo byte a byte não é critério de aceitação.
Timings usa --timings-json no Tekt e --timings no oracle; compacto compara
conteúdo/caixas com o padrão do oracle. Não houve benchmark de desempenho.

Builds debug/release passaram (`build-final.log`, `release-final.log`).
Linter final (`lint-final.json`): **zero erros, 530 warnings, 1.578 notes**;
nenhum V15/V26 global nem V5/V6/V15/V26 nos owners alterados. Não se afirma
zero violations globais. `git diff --check` passou.

A primeira suíte CLI (`cli.log`) teve 113 passes e dois erros de permissão de
loopback em watch/HTTP, no binário anterior ao ajuste final da legenda de raiz.
A suíte foi repetida no estado entregue com acesso de loopback autorizado;
**115/115 testes CLI passaram** no estado entregue. O resultado final e sua
proveniência constam em cli-final.log/execution.json.gz.

A matriz macro histórica de 36 e os corpora anteriores não foram somados nem
reclassificados como nova medição deste recorte. Watch com retenção de recursos,
memoização incremental, matemática/morfologia/seletores e ordem de warnings
continuam em suas frentes próprias.
