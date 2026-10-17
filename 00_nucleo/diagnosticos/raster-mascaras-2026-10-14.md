# Raster PNG: máscaras, contorno e alpha — 2026-10-14

Corrigidos recortes de Group e Image, fill-rule, geometria de contorno e
codificação de transparência no rasterizador. As máscaras agora intersectam
em coordenadas do canvas, alcançam formas/linhas/imagens/glifos shaped e não
vazam aos irmãos. Esta é uma dependência concreta do T2 de tiling aprovado;
não é a migração Shape.fill/stroke nem a implementação de patterns nativos.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-source,
red-state, red-behavior-state, candidate-final, pre-validation e workspace-pre
conservam UTC, fontes, SHA-256 e diff HEAD --stat; sondas incluem status,
fontes Typst, comandos, resoluções e hashes de ambos os binários. Resselo
restrito a `03_infra/src/export/render.rs` e seu L0 exclusivo. Sem commit,
push, PR ou publicação.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489; binário
`lab/typst-original/target/release/typst` SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
A primeira sonda sem PPI explícito gerou PNG vanilla de17280×14400 pixels
para120×100pt e foi interrompida pela PIL. before/interruption.json preserva
comando, dimensões e hash; não é resultado válido de paridade. As sondas
válidas usam --ppi144 em ambos:240×200 pixels. Essa divergência do default
não foi copiada para o Tekt. SVG é rasterizado a72dpi/zoom2 para a mesma grade.

## Medição, decisão e mudança

before-explicit-ppi/results.json.gz mede clip com19200 pixels vermelhos no
PNG Tekt contra3744 no SVG Tekt e vanilla; image-cover2048 contra768;
tilings com corpo maior que célula14300 contra4800. render.rs descartava
clip_mask/clip_rect, forçava Winding e Butt/Miter sem dash. A inspeção do
encoder identificou bytes premultiplicados passados diretamente a RGBA comum.
Os testes confirmaram alpha128 com vermelho128 em vez de255 na saída PNG.
São falhas no código; o L0 anterior registrava algumas como limitações.
Esta amostra não mede a proporção de falhas de todo o projeto.

L0 atualizado antes do código, correção interna/de paridade ADR-0127.
O estado privado conserva a máscara emprestada; Group usa T(pos)*matrix e
inner_width/inner_height, Image usa clip_rect no referencial parent. Máscara
vazia não libera desenho. A interseção antialiased é local à recursão;
shapes/lines/images passam a máscara ao tiny-skia e o compositor manual de
glifos multiplica a cobertura correspondente, mantendo os pixels interiores.
FillRule NonZero/EvenOdd e cap/join/dash/fase/miter chegam ao raster. Arrays
ímpares repetem o período e LineWidth resolve contra espessura. O encoder
desmultiplica RGB antes de escrever PNG, preservando alpha. API/defaults,
Paint declarativo, pipeline e outros exporters permanecem sem alteração.

red.log contém erros de fixture (Color::RED e FontList::default inexistentes).
red-behavior.log contém seis falhas semânticas: interseção, curva/empty clip,
Line/Image, EvenOdd, stroke e alpha. O teste de glifo inicialmente não tinha
família em style e falhava no próprio controle sem clip; esse resultado não é
RED de clipping. green-glyph-fixture.log conserva a falha residual até corrigir
a fixture. green.log: oito testes passam, incluindo glifo com fonte Noto Sans
fixa, recorte e igualdade dos pixels interiores. implementation-syntax.log
registra o cast sem parênteses corrigido antes da validação. Expectativas de
geometria e alpha não foram relaxadas.

A primeira suíte (8537/0/7) passou, mas a revisão do caminho de documento
composto encontrou RGBA decodificado usado como premultiplicado. first-pass
preserva aquele estado/validação, sem o apresentar como estado final.
stack-red.log mediu [191,255,255,255] contra [159,191,223,255] ao compor
uma página semitransparente sobre branco; stack-red-state registra o estado.
O decoder do empilhamento agora premultiplica antes de construir Pixmap.
O teste novo também verifica a segunda página, dimensões e gap branco.
A suíte e todas as validações foram repetidas após essa correção; after-final
contém o corpus de renders produzido pelo binário final.

## Render bilateral e limites

comparison.json conserva canvas integral, sem corte/alinhamento posterior,
comparando PNG Tekt com SVG Tekt e PNG vanilla. 12/13 casos
válidos coincidem integralmente com o PNG pinado: clip, no-clip, rounded,
nested, text, image-cover, dash, solid, tiling, EvenOdd/NonZero com contorno
e alpha. Isso é evidência localizada de imagem, não igualdade de bytes de
arquivo como definição de paridade. O recorte arredondado foi inspecionado.

| Controle | Vermelho antes | Depois | Pin |
|---|---:|---:|---:|
| clip |19200|3744|3744|
| rounded |19200|3610|3610|
| image-cover |2048|768|768|
| tiling |14300|4800|4800|
| EvenOdd com stroke |5764|4320|4320|

No-clip, Solid e NonZero mantêm pixels idênticos à entrada. SVG não foi
alterado. Rotate continua com diferença de4141 pixels >16 contra o pin;
contra SVG Tekt, são3 pixels >16, máximo17. Isso separa recorte raster da
geometria de layout já refutada em T3, sem elevar tolerância para aceitá-la.
Texto PNG coincide com PNG vanilla, mas difere do raster SVG em52 pixels >16;
não transformar rasterizações diferentes em obrigação de mecânica.

Quatro fontes curve com stroke:none/0pt são recusadas pelo Tekt antes do
raster; os diagnósticos foram mantidos como falhas abertas e não entram nos
13 renders válidos. Os controles com stroke:1pt permitem testar fill-rule
sem apagar essas divergências. Tempos individuais estão nos resultados, mas
não sustentam conclusão de desempenho: máscara tem tamanho do canvas e o
layout legado ainda clona células; não há prova de transporte limitado ou
amostragem universal de tiling subpixel.

## Verificação geral

Suíte completa de 2026-10-14T14:37:50.190162+00:00 a 2026-10-14T14:43:44.300861+00:00:
**8538 passaram, 0 falharam, 7 ignorados**;
fontes estáveis. Build offline/locked, rustdoc -D warnings, all-target check,
fmt/diff, V1/V5/V15/V26 estritos e linhagem reversa passaram.
Clippy 181 avisos (delta {}); arquitetura
0 erros, 382 warnings,
1629 notes. Inventário não equivale ao gate
CI -D warnings. CI remoto/release permanecem sem certificação.

## Continuidade

T1c ainda migra Shape.fill/stroke para FramePaint/FrameStroke; T2 prepara
células únicas e recursos/shaping, patterns PDF/SVG e textura raster; T3
fecha Parent afim e geometria Place/Rotate. O limite10000 e a clonagem legada
não foram alterados neste lote. Shape/Line paints complexos, Glyph direto,
transformações afins de glifos, EXIF, constructor curve(stroke:none/0), altura
Auto, decorações e demais pendências do objetivo integral continuam abertos.
A correção da máscara não certifica produção nem encerra o objetivo.
next-shape-syntax.json registra 55 arquivos referenciadores e150 construções
em33 arquivos, com UTC/HEAD/diffstat/SHA dos sources e owners, para iniciar T1c.
É inventário sintático, não prova de fechamento semântico dos consumers; o
scanner e seu lockfile estão preservados em next-shape-tool.

## Evidência

[evidence.json.gz](raster-mascaras-2026-10-14/evidence.json.gz): 404
entradas com caminho/SHA-256/base64, relidas e verificadas.
SHA-256 `2622a024713eb917d5b2a0b62c10e25a798e4dda12c1ec26dc228b2418e6aac7`.
