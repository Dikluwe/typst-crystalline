# Block Auto: alinhamento geométrico tardio — 2026-10-14

Corrigido o refutador de Align geométrico que desaparecia sob clipping após
reduzir corretamente a largura do Block Auto. O filho é posicionado usando a
largura final do corpo com inset. O corpo continua sendo executado uma vez.
Isso não completa o transporte tiling, os exporters nativos nem a preparação
para produção.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-source,
red-state, candidate-final, pre-validation e workspace-pre conservam UTC,
diff HEAD --stat, fontes e SHA-256; os registros de sonda conservam também
status, comandos, fontes Typst e identidade dos binários. Não houve commit,
push, PR ou publicação.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário
`lab/typst-original/target/release/typst` SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O original block-auto-width permanece histórico e não foi reescrito.

## Medição e correção

before/results.json.gz reproduz zero pixels vermelhos em right/center/link
com clip, contra 3840 no pin, bbox [48,40,128,88] em 2px/pt. Sem clip, right
fica em [260,40,320,88], cortado pela página. A largura disponível era usada
por placement.rs como se fosse a alocação final, conhecida só ao fechar Block.
No pin, flow/block.rs:278–287 distingue constraint de expansão;
flow/distribute.rs:646–684 determina o tamanho antes de posicionar frames.
A necessidade é a geometria da linguagem, sem copiar a mecânica do upstream.

L0 foi atualizado primeiro. Núcleo block-auto-alignment.toml, pin
`ecf965471eedd8599209f0575811dd5b456dce1c43baa26881251e24e8ec6c6d`.
A mudança é interna/de paridade, em fluxo contínuo ADR-0127, sem API pública.
Layouter conserva um flag privado por Block; checkpoint salva/restaura e
subframe isolado limpa/restaura. Align fora da rota flow_aligned mantém
posição intrínseca e registra o fixup existente; Block resolve os paths
locais contra a largura final antes de recortar e introduzir paint_root.
O deslocamento usa os helpers existentes de Group/Link/Semantic.
A rota textual/grid certificada pelo classificador anterior não foi alterada.

red.log reproduz duas falhas reais: centro x77 em vez de x24 pt, nested x127
em vez de x27 pt. Uma aplicação parcial de script parou numa guarda de
cardinalidade; partial-implementation-test.log é falha intermediária, não
sucesso. green.log registra cinco testes passando (dois novos e três de
largura causal), incluindo left/center/right, irmão maior, nested/fixed,
Link, underline sobre forma e restauração de estado. Expectativas mantidas.
O resselo alterou somente os seis owners do manifesto e seus seis L0.

## Validação bilateral e limites

Corpus before/after: 28 pares SVG, canvas integral 160×120pt em
320×240 pixels, rsvg a 72dpi e zoom2, sem corte/alinhamento posterior.
As caixas do retângulo coincidem em 20/20 controles que o contêm;
12/28 renders completos coincidem pixel a pixel. Essa última
medida é evidência localizada, não requisito de igualdade de bytes/Rust.
Com irmão de 70pt, right termina em [108,40,188,88] e center em
[78,40,158,88], como o pin; o restante do render ainda diverge em altura.
O caso simples com clipping foi inspecionado visualmente.

PDF-after contém sete pares, todos aprovados por qpdf. right/center/link e
underline externo sobre a forma coincidem integralmente; wider/nested/fixed
mantêm divergências de altura já conhecidas. O corpus Parent/Solid anterior
foi repetido: 36/36 renders Tekt PDF/SVG têm pixels
idênticos à entrada registrada no lote block-auto-width. Isso preserva o
controle PDF de gradiente parent corrigido naquele lote.

O texto simples deste corpus passa a coincidir porque o carrier composto
entra na rota geométrica; não inferir daí que a rota textual está completa.
Multilinha ainda difere em 153 pixels >16; underline interno em 78; matemática
em 8132 (canvas e estado acima). A matemática permanece inalterada nesta
rodada. A posição correta de uma forma sob underline não certifica linhas
de decoração: segmentos externos consumidos antes do fechamento exigem
transporte causal próprio. Place horizontal, nested Align em subframe,
altura Auto/backing, outset, Block fragmentado e Box fracionário seguem abertos.

## Verificação geral

Suíte completa de 2026-10-14T14:03:25.356796+00:00 a 2026-10-14T14:10:26.065454+00:00:
**8530 passaram, 0 falharam, 7 ignorados**.
Fontes ficaram estáveis. Build offline/locked, rustdoc -D warnings,
all-target check, fmt/diff, V1/V5/V15/V26 estritos e linhagem reversa passaram.
Clippy: 181 avisos, delta por regra {};
isso é inventário e não passa o gate CI -D warnings. Arquitetura:
0 erros, 382
warnings, 1629 notes. CI remoto e release
continuam sem certificação; não foi publicada release.

## Continuidade

Completar alinhamento por linha e matemática, transporte de decoração,
altura/outset e fragmentação com seus refutadores. T1c ainda migra
Shape.fill/stroke aos carriers aprovados; T2 implementa patterns PDF/SVG/PNG
e recursos recursivos; T3 completa geometria afim. As demais pendências do
objetivo integral permanecem abertas; este lote não redefine conclusão.

## Evidência

[evidence.json.gz](block-auto-alignment-2026-10-14/evidence.json.gz):
514 entradas com caminho/SHA-256/base64, relidas e verificadas.
SHA-256 `12509a69619a4c4fd7f09223de0362faecf0c4ba53865817e0c9fa060b0f052a`.
