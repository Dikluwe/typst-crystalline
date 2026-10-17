from pathlib import Path
p=Path('00_nucleo/prompts/infra/export/render.md');s=p.read_text()
s=s.replace('The shape path uses Winding fill and its existing Butt/Miter stroke settings;\nline-cap support does not imply equivalent Shape stroke or fill-rule support.','The shape path consumes the transported NonZero/EvenOdd fill rule and all resolved\nstroke geometry: cap, join, miter limit, dash lengths and phase. Odd dash arrays\nrepeat once to form even pairs; LineWidth entries resolve against thickness.')
s=s.replace('Groups compose position and matrix and recurse, but currently do\nnot apply clip_mask. Image clip_rect is not consumed by this dispatcher.','Groups compose position and matrix and intersect clip_mask with the inherited\ndevice-space mask before recursing, using the supplied inner dimensions.\nImage clip_rect intersects in the current parent coordinates before image placement.\nEmpty clips paint nothing; unmasked siblings retain their original state.')
s=s.replace('Preserve PNG encoding\nand failure behavior;', 'Encode straight RGBA from the premultiplied canvas, preserving alpha and existing\nencoding failure behavior;')
s=s.replace('Unsupported clips/direct glyphs/complex\npaint remain recorded limitations, not validated equivalence.','Unsupported direct glyphs/complex paint remain recorded limitations, not validated\nequivalence. Clipping must apply to every currently rendered primitive, including\nmanual TextShaped glyph compositing, with antialiased intersection and no state leak.')
s+='''

## Máscaras raster, fill-rule e contorno — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/raster-mascaras-2026-10-14/before-source.json.gz`
e `before-explicit-ppi/results.json.gz` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, diffstat,
fontes, argv e SHA dos binários. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
Com --ppi144 e canvas120×100pt, clip pinta19200 pixels vermelhos em PNG Tekt,
contra3744 no SVG Tekt e vanilla; image-cover pinta2048 contra768; tiling com
célula maior que size pinta14300 contra4800. render.rs:229/584 ignora clip_mask;
:208 ignora clip_rect; :443 força Winding; :454–458 força Butt/Miter e perde
dash. :619 transporta bytes premultiplicados para ImageRgba8 sem conversão,
o que altera a cor ao recompor transparência. Essas observações justificam
correções de geometria/alpha da linguagem, não igualdade de PNG comprimido.
A primeira sonda, sem PPI explícito, foi interrompida: o pin gerou17280×14400
para a mesma página. Não copiar esse default; o contrato Tekt já usa PPI/72.
Os controles curve(stroke:none/0pt) falham antes do raster e são mantidos como
refutadores do constructor, não contabilizados como sucesso do exporter.

Fonte pinada typst-render/src/lib.rs:226–262 intersecta máscaras após transformar
clips; shape.rs:55–77 e :131–137 conserva fill-rule/contorno e pares de dash.
A biblioteca local tiny-skia0.12.0/src/mask.rs:253,346 implementa cobertura e
interseção em pixels; a máscara pertence ao canvas, não ao box do filho.
Hipótese refutável por transformações aninhadas, empty clips, glyph/image/line,
siblings não recortados, furos EvenOdd, dash/cap/join e recomposição alpha.

### Responsabilidade exclusiva

Estado privado conserva Option<&Mask> junto da transformação; a máscara tem
vida local à recursão e tamanho do canvas. Compor T(pos)*matrix antes do clip
Group. Usar inner_width/inner_height e o path completo, não um bbox aproximado.
Interseção antialiased conserva clips anteriores. Clip vazio/inválido não libera
o desenho dos filhos. Image clip_rect usa suas coordenadas parent, sem somar
pos do bitmap outra vez. Shapes/Lines/Images encaminham a mesma máscara aos
primitivos tiny-skia; glifos já rasterizados multiplicam cobertura pelo valor
correspondente da máscara, preservando a rota sem máscara. A máscara local não
vaza aos irmãos nem ao frame seguinte.

Mapear FillRule e todos os campos geométricos do Stroke. DashLength::LineWidth
usa espessura atual; arrays ímpares repetem o período; preservar fase. Não mudar
Paint declarativo para cor nova nem afirmar suporte a gradiente/tiling nativo.
No encoder, desmultiplicar RGB antes de gerar PNG RGBA; alpha não muda.

### Aceitação e limites

Correção interna/de paridade ADR-0127, L0 primeiro e RED→GREEN; sem API pública,
flag, default ou fase nova. Testes devem medir pixels/visibilidade de clip real,
composição e isolamento, fill-rule, contorno e alpha, com fontes locais fixas.
Revalidar CLI PNG/SVG contra pin em resolução explícita, suíte/build/doc/fmt,
linhagem e inventários sem ampliar tolerâncias para aceitar falhas.
Máscaras são requisito de T2 do tiling aprovado. Esta etapa não migra
Shape.fill/stroke para FramePaint nem implementa patterns nativos: T1c/T2/T3
continuam abertos, inclusive repetição sem clones, recursos e geometria afim.
Não certificar PNG universal: Glyph direto, transformações afins de glyphs,
EXIF, paints complexos e constructor de curve conservam seus refutadores.
'''
p.write_text(s)
