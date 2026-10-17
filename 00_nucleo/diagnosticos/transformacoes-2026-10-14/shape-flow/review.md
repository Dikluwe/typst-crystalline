# Fronteira vertical de Shape e espaço forte

Correção implementada no fluxo principal de Shape e na manutenção do fim
confirmado após VSpace forte. A integração geral de Block e espaços fracos
continua incompleta e refutada; não é uma certificação de produção.

## Proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Os JSONs preservam UTC, status/diffstat exatos e hashes de fontes/binários.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, receita P1489,
árvore 6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
`before-sources.json.gz` preserva bytes dos três consumers e owners antes
 desta etapa; `focused.patch` separa o lote do restante da árvore suja.

## Medição, decisão e reparo

`shape-terminal-complete` mede o fundo indevido da forma isolada, a segunda
forma deslocada pela cap-height e leitores usando prev_line_baseline antiga.
`shape-flow-before` amplia a 32 documentos, incluindo espaço forte/fraco,
texto, transformações e página explícita. A tentativa `shape-terminal-before`
abortou por uma suposição errada do harness sobre o template de saída da CLI;
não foi usada como corpus completo. A CLI Tekt conserva literalmente {p}
na saída SVG desse caso; o harness corrigido registra o caminho efetivo.

L0 atualizado antes de código. As quatro regressões em
`implementation/shape-flow-red.log` falharam; `shape-flow-green.log` passou
as quatro. Shape passou a publicar fim físico em prev_line_baseline e
last_block_descent_y; o topo seguinte usa esse fim e o gap, sem descontar
cap-height. A transição de página recupera a origem física. Subframes,
dimensões, pintura e estilos mantêm seus caminhos. VSpace forte mantém o
fundo confirmado e desloca a baseline da cadeia pelo mesmo amount; não
soma o below pendente como conteúdo. Nenhum contrato público ou fase mudou.

## Evidência posterior e limites

`shape-flow-after` mede os mesmos 32 documentos em SVG/PNG; `shape-flow-pdf`
cobre PDF. Forma isolada, texto adjacente, duas ordens Shape/Rotate, espaço
forte terminal/intermediário, parbreak e quebra explícita recuperaram a
geometria esperada. `shape-dim-flow-after` conserva os 20 controles de
dimensões e corrige também a página auto de 150×150pt do caso unbounded.
As 74 consultas/diagnósticos e renders anteriores estão em `after-shape-flow`;
seus erros esperados não são falhas novas.

Não exigir pixels idênticos como substituto de geometria: formas consecutivas
coincidem em SVG/PDF, mas o PNG vanilla encaixa as bordas em pixels inteiros
onde Tekt aplica suavização. O registro `raster-edge-observation.json`
preserva os pixels lidos; essa divergência não motivou alteração no rasterizador.
Elipses conservam diferenças anteriores. Alguns PDFs têm diferença de um
nível por canal; não se afirma igualdade binária com vanilla.

Pendências confirmadas: weak VSpace entre elementos ainda tem prioridade de
colapso errada; VSpace negativo é rejeitado; dimensões None/Auto e identidade
circle/square continuam incompletas; RTL permanece aberto. Block deixa o
fundo do filho escapar em vez de publicar o fim de sua própria alocação:
`shape-block-terminal-before` mede 20 controles de Shape/Text/Empty/Rotate,
com inset e altura explícita. Shape em Block height40 termina página em30pt,
quando a alocação exige60pt; inset5 produz35pt em vez40pt. Isso também ocorre
com Rotate e deve ser reparado no owner Block. Não encerrar B2b nem usar
suíte verde como prova de correção desses refutadores.

## Testes, snapshots e CI

A execução completa de 2026-10-14T19:58:54.544154+00:00 a 2026-10-14T20:05:53.776296+00:00 terminou com
8570 testes aprovados, duas falhas de snapshots e
7 ignorados. As fontes ficaram estáveis. Os nove comandos
em validation/ passaram: build, rustdoc estrito, check all-targets, inventário
Clippy, fmt, linhagem estrita, reversa, relatório arquitetural e diffcheck.
Clippy permanece com 181 avisos; Tekt com
380 warnings,
1662 notes e zero errors. Não é CI -D warnings verde.
Os 16 testes Python de tools/ci também passaram.

Somente snapshots 04/07 foram regenerados segundo o contrato já vigente de
export-fixtures, após justificação registrada e backup. Duas gerações foram
idênticas; qpdf passou, palavras extraídas foram preservadas e objetos/streams
foram comparados. Os outros sete PDFs/fontes/harness/L0 ficaram intactos.
Os nove snapshots passaram sem UPDATE. Essa reexecução dirigida resolve as
duas falhas de referência; não se apresenta como uma nova suíte completa
com todos os testes executados após a manutenção. MANIFEST registra os hashes.

A meta geral permanece aberta. Não houve commit, push ou publicação/release.
