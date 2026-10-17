# L0 — infra/export/render
Hash do Código: 4bb8e999

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/render.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/render.rs:40` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Raster contract

Own RenderOptions, FontKey, raster entrypoints, rendering state, recursive item
dispatch, glyph/image/shape rasterization and PNG encoding. Inputs are resolved Page
or PagedDocument values and caller-supplied fonts; outputs are in-memory PNG bytes.
Do not read files, fetch fonts, interpret CLI/PPI or run layout.

RenderOptions defaults to pixel_per_pt=2.0 and render_bleed=false. The scale is
pixels per typographic point, not PPI; the caller resolves physical units.
Use max(pixel_per_pt, 0.01), round each scaled dimension and require at least one
pixel. Without bleed use the logical page; with bleed use the physical canvas and
translate by bleed.left/top once. Auto fill is white, None retains transparency,
and Paint uses the existing to_color projection. Paint background, body and foreground
in order. The current allocation path may panic with "tamanho validado acima";
no independent maximum allocation bound is implemented here.

TextShaped renders only with a matching FontList/FontVariant/FontVariations and a
valid supplied face. Preserve glyph advances, offsets, position, scale and alpha;
the current pixglyph path uses its existing raster/outline support. Plain Text and
direct Glyph are omitted by dispatch, even with fonts; do not promise tofu or
complete mathematical glyph coverage.

Line maps Butt/Round/Square to the corresponding tiny-skia cap without changing
thickness, color, transforms or coordinates. Shape consumes resolved ShapeKind<Pt>
and all four Corners<Pt> directly, never resolving Length in a neutral context.
Preserve paths, ellipse/rounded geometry and current solid Paint projection.
The shape path consumes the transported NonZero/EvenOdd fill rule and all resolved
stroke geometry: cap, join, miter limit, dash lengths and phase. Odd dash arrays
repeat once to form even pairs; LineWidth entries resolve against thickness.

Images use existing decoding/texture preparation on the resolved box. EXIF
orientation remains transported but unconsumed by this renderer. Groups compose position and matrix and intersect clip_mask with the inherited
device-space mask before recursing, using the supplied inner dimensions.
Image clip_rect intersects in the current parent coordinates before image placement.
Empty clips paint nothing; unmasked siblings retain their original state.
Link and Semantic are visually transparent containers: recurse without painting
target, kind, placement or alt. No PDF tagged structure or accessible text layer
is produced by raster.

render_document_to_png uses the existing fontless per-page route, stacks successfully
decoded pages vertically with the rounded gap and white composite canvas, and keeps
its 1x1 allocation fallback for an empty/invalid composite. Encode straight RGBA from the premultiplied canvas, preserving alpha and existing
encoding failure behavior; do not claim compressed PNG bytes as language parity.

## Verification

Check IHDR dimensions, bleed/fill/layers, supported shaped text, image orientation,
resolved radii, and deterministic line endpoint coverage. Compare visible geometry
and color/alpha, not PNG compression bytes. Unsupported direct glyphs/complex paint remain recorded limitations, not validated
equivalence. Clipping must apply to every currently rendered primitive, including
manual TextShaped glyph compositing, with antialiased intersection and no state leak.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/render.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator não pinta pixels; advance não tem tinta.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


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

### Recomposição de páginas após alpha correto

Inspeção posterior à primeira validação: render_document_to_png em render.rs:154–180
decodifica PNG para RGBA comum e usa Pixmap::from_vec como se fossem canais
premultiplicados. A primeira suíte completa (8537/0/7), preservada em first-pass,
não exercitava recomposição semitransparente e não prova esse caso. Antes da
conversão para Pixmap, premultiplicar explicitamente cada pixel decodificado;
conservar canvas branco, dimensões, gaps e ordem. RED→GREEN com duas páginas
semitransparentes e gap opaco, seguido de nova validação no estado final.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Consumir FramePaint/FrameStroke em Shape. Rasterizar uma célula preparada em textura periódica transparente com clip local, dobrar overlap de spacing negativo na ordem de pintura e aplicar shader Repeat a fill e stroke, mantendo cap/join/dash/alpha/máscaras. A resolução da textura deriva do pitch transformado; reduzir por média premultiplicada quando subpixel, sem cor representativa. Propagar transformação afim relativa à raiz e reiniciar em paint_root; a célula tem raiz própria. Reutilizar fontes e imagens, não clonar uma grade proporcional à área de destino.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.

A sonda native-v1/dense-open mostra blur introduzido por supersampling indiscriminado: célula de 2 pixels foi ampliada para 16 e reduzida por Triangle. Renderizar diretamente na resolução do período quando ele ocupa pelo menos um pixel; somente períodos subpixel usam supersampling e média de área premultiplicada. Não suavizar as bordas inteiras alinhadas por redução redundante.

## Higiene da migração nativa

Medição: verified-v3/followup-clippy-inventory.json e followup-architecture.log, com candidate-final e workspace-pre próprios, registram quatro f32::hypot proibidos, um argumento adicional, dois itens produtivos depois de módulos de teste e dois wildcards neutros novos.

Decisão interna em fluxo contínuo (ADR-0127): Usar libm::hypotf pinado para a norma f32 das colunas da matriz (sx,ky) e (kx,sy), calculada uma vez por eixo; conservar ceil e limiar subpixel. Transportar a raiz lógica já atualizada em State ao chamar render_group, preservando transformação física/máscaras sem argumento booleano adicional. Não elevar limites nem adicionar allow. Validar suíte e renders antes/depois, além de formato, doc e linhagem.

## Gradientes raster — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/png-gradientes-2026-10-14/before-final/results.json.gz`
preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC, status e diffstat
da árvore não commitada, hashes das fontes e binários, comandos e 28 controles
bilaterais a 144 PPI. O pin é 9f2b6e8715237cb086899a42873660fe744622e8.
O controle sólido coincide; o linear tem 4440 pixels com diferença máxima de
canal acima de 16; radial 4785, cônico 4426, célula Parent 4200 e fundo 44800.
Esse limiar descreve o inventário, não constitui tolerância de aceitação.
Os diretórios before/before-valid conservam fixtures inválidas rejeitadas pelos
dois binários; não usar tais erros como prova de defeito no rasterizador.
render.rs:515–517 projeta Gradient para SolidColor e :120 faz a mesma projeção
no fundo. State:56–60 não conserva dimensões da raiz. O pin, em
typst-render/src/paint.rs:135–248 e typst-library/src/visualize/gradient.rs:969–1057,
amostra posição normalizada, aspecto, centro/foco, ângulo e cores; Self usa bbox
e Parent a raiz rígida. shape.rs:55–77 inclui contorno no bbox do gradiente de
stroke. São observáveis de cor/geometria da linguagem, não igualdade de bytes.

### Responsabilidade e decisão

Correção interna de paridade em fluxo contínuo ADR-0127: sem nova API pública,
flag, default ou mudança de fase. Esta cláusula supersede a projeção sólida de
Gradient em Shape e PageFill. O layout continua dono da preparação de células.
Rasterizar Linear, Radial e Conic por posição, preservando offsets/espaço de
cor/alpha via sampling existente de L1. Não usar cor representativa nem converter
prematuramente alpha para byte. Linear corrige aspecto e orientação; radial usa
os dois círculos, foco e raio focal, incluindo centro de raio zero sem NaN;
cônico conserva centro, ângulo, aspecto e volta. Usar libm para trigonometria.

O estado privado conserva tamanho da raiz e densidade de amostragem. Página,
Group.paint_root e célula preparada estabelecem raízes próprias; grupos técnicos
apenas compõem a matriz. Parent usa a inversa afim entre raiz e shape incluindo
posição, sem inferir raiz por clip. Self usa limites geométricos, incluindo o
contorno para stroke; curvas usam extremos da curva/contorno, não pontos de
controle ou o canvas inteiro. Retângulos negativos espelham orientação, mantendo
limites positivos para raster. Dash, cap, join, miter, fill-rule e máscaras seguem
o contrato anterior. Fill e stroke são independentes; ausência de textura não
deve impedir a pintura do outro papel.

Textura de gradiente cobre o bbox/raiz em resolução explícita, com extensão Pad
e amostragem Nearest; não repetir gradiente como tiling. Respeitar anti_alias do
gradiente. Amostrar cores em coordenadas de pixels inteiras na textura e converter
float RGBA, clamp de gamut, premultiplicação e só então bytes. Fundo usa canvas
físico quando bleed ativo; conteúdo mantém raiz lógica. Tiling conserva Repeat,
composição do período e recursos existentes; seu corpo permite gradientes sem
fallback. Não acrescentar cache global nem mudar interpolação pública de L1.

### Aceitação e refutadores

RED→GREEN por pixels para três geometrias, alpha, Self/Parent, raiz de grupo,
tiling, contorno, fundo e retângulo negativo. Repetir corpus bilateral, incluindo
ângulo negativo, foco deslocado, stops, sharp, transforms, clips e overlap.
Não aumentar tolerância nem realinhar imagens para esconder divergências;
registrar separadamente falhas anteriores de layout ou interpolação encontradas.
Revalidar suite/build/doc/fmt, linhagem direta/reversa e inventários de avisos.
Não certificar fontes diretas, ICC/CMYK, limites de alocação, performance ou toda
a paridade PNG por esses controles; esses eixos exigem suas próprias medições.

O teste `png_gradients_tiling_root_and_anisotropic_sampling`, preservado em
anisotropic-red.log e anisotropic-red-source.rs.txt, mede (149,0,106) onde a
composição geométrica exige (128,0,128): usar proporção de pixels sem compensar
densidades diferentes altera 45° para outro ângulo. Corrigir aspecto por
(largura_px/densidade_x)/(altura_px/densidade_y). A densidade pertence à
amostragem, não muda o ângulo ou centro autoral. Esse teste refuta uma solução
que só passa em células com densidade igual nos dois eixos.

### Contorno arredondado isolado pela sonda de gradientes

`isolation/results.json.gz` e `svg-controls/results.json` usam o estado preservado
em first-pass/candidate-final.json.gz. Round-solid tem 396 pixels acima de 16 no
PNG e zero no SVG (144 PPI). svg/geometry.rs:78–99 e seu L0 vigente estabelecem
raios externos: o caminho central usa max(raio - espessura/2, 0). render.rs ainda
usa o raio sem inset, aumentando o raio externo. Não corrigir essa geometria
alterando gradientes. Em RoundedRect com stroke, construir fill fechado e stroke
aberto de volta ao início com os quatro raios descontados da meia espessura,
clampados ao semieixo menor. Sem stroke e em clips manter raios intactos. Bbox
do gradiente de contorno deriva desse mesmo caminho. Testar cobertura de canto
declarado, controles assimétricos, dash/cap e clip; resselo e nova suíte completa.

## Contorno de glifos sob matriz afim — correção medida

### Evidência anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/refuters-third/results.json.gz`
registra fontes, comandos, HEAD, diffstat/UTC e hashes dos binários. O controle
reflow-wrap-rotate coincide em SVG, mas diverge no PNG; PDF foi observado também
em refuters-pdf-first. `03_infra/src/export/render.rs:402–405` calcula ppem com sy
e usa apenas sx/sy para a origem; kx/ky são ignorados e a forma permanece uniforme.
Fonte pinada `typst-render/src/text.rs:53–91` rasteriza contornos com a matriz
completa quando a rasterização uniforme não representa a transformação.
Isso distingue posição/forma da linguagem de bytes de PNG e estratégia de cache.
A falha é interna, dentro do transporte já aprovado; não cria API nem muda fase.

### Responsabilidade exclusiva

Glifos TextShaped já suportados mantêm avanço, offsets, fonte preparada e alpha.
Quando a matriz inclui rotação, shear, escala não uniforme ou reflexo, rasterizar
o contorno da mesma face com a matriz completa: transformação do grupo, origem
do glifo, escala de unidades da fonte e inversão do eixo vertical, nessa ordem.
Conservar a máscara acumulada em coordenadas do dispositivo. Caminho uniforme
positivo pode continuar pixglyph; tamanhos grandes podem usar contorno. Fechar
contornos abertos inclusive CFF2 antes da rasterização. Glifo sem contorno não
fabrica tinta; esta correção não certifica fontes coloridas/bitmap, Glyph direto,
Text sem shaping ou suporte integral de matemática.

### Refutadores

Glifo assimétrico com rotação de 90 graus, escala não uniforme, shear e reflexão:
comparar posição e distribuição de cobertura com a transformação geométrica da
mesma figura. Máscara deve recortar também esse caminho sem afetar irmãos.
Controles uniformes preservam o comportamento anterior; corpo fonte→PNG deve
acompanhar a geometria medida em SVG/PDF e a referência pinada, sem realinhamento.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C047`: `03_infra/src/export/render.rs:349`, `render_text_shaped`, 9 argumentos contados pelo Clippy (limiar 7).
- `C048`: `03_infra/src/export/render.rs:388`, `render_glyph`, 8 argumentos contados pelo Clippy (limiar 7).
- `C049`: `03_infra/src/export/render.rs:839`, `render_shape`, 10 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C047` — `render_text_shaped`: Correção interna prescrita

O argumento _text não é lido pelo helper raster; o dispatcher já fornece glyphs/estilo/face, e PNG não emite camada textual. Remover esse transporte morto primeiro. Os oito argumentos restantes são inputs reais do leaf renderer e admitem exceção pontual residual.

Remover somente _text do helper privado raster e de seus chamadores. Após a redução de nove para oito argumentos, autorizar expect local com motivo dos oito inputs reais de pintura.

Aceitação: PNG conserva glifos/offsets/avanços/alpha/transformações/face; não adicionar texto invisível nem mudar o caminho HTML/PDF. A exceção residual só vale após remover o argumento morto.

#### `C048` — `render_glyph`: Exceção pontual justificada

Leaf raster recebe alvo, estado gráfico, face/GID, posição, tamanho e cor resolvidos. Todos são utilizados na pintura do mesmo glifo; agrupar em estado global ou refazer seleção/medição violaria a fronteira do renderer.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C048: Leaf raster recebe alvo, estado gráfico, face/GID, posição, tamanho e cor resolvidos")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C049` — `render_shape`: Exceção pontual justificada

Leaf de Shape consome geometria resolvida, fill/stroke/fill-rule, estado e fontes para textura. Os dados correspondem à operação de pintar uma Shape; não há resolução de Length/layout ou aquisição de fontes neste owner.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C049: Leaf de Shape consome geometria resolvida, fill/stroke/fill-rule, estado e fontes para textura")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C048, C049. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.


## Aridade interna — materialização de seis casos, 2026-10-16

### Medição anterior à decisão

As fontes, chamadores, cláusulas vigentes, spans e inventário de 28 avisos foram conferidos antes da decisão. `00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/before.json.gz` guarda fontes/L0 completos, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A auditoria anterior classifica estes casos como correção; não são exceções diretas. O RED deve observar as aridades atuais antes de alterações; controles funcionais existentes serão corridos antes/depois. É refatoração de representação privada, sem nova paridade ou ganho de performance medido.

### Responsabilidade exclusiva e aceitação

C047: retirar apenas o parâmetro privado _text e seu argumento no dispatcher raster. Deixar de extrair text do padrão TextShaped somente nesse dispatcher, já que deixa de ser usado; o campo autoral de FrameItem permanece. Após confirmar a função com oito inputs reais, usar #[expect(clippy::too_many_arguments, reason="C047: Leaf raster usa oito inputs efetivos de pintura; texto autoral não é transportado para PNG")]. Corpo de pintura, face, upem.max(1), offsets, alpha, transformações e caminhos omitidos ficam idênticos.

Fluxo contínuo ADR-0127: nenhuma API externamente pública, default do produto, fase, dependência ou semântica é alterada. Tipos/constructors novos ficam internos; contratos públicos existentes permanecem. Este adendo concretiza somente os seis casos nominais; substitui proibições genéricas de supressão apenas para as duas expectativas residuais condicionadas C047/C066 após remover _text. As outras cinco aridades a corrigir continuam visíveis. Não elevar limiar ou silenciar no módulo/impl/crate. L0 precede código e resselo restrito. Confirmar preservação das assinaturas externamente públicas, campos/valores transportados, controles PNG/SVG/PDF, precedência/diagnósticos/recursos/MCIDs e ausência de mutações globais. Usar os testes existentes sem relaxar fixtures/esperados; não escrever testes que espelhem os carriers privados. Revalidar suíte workspace completa, build, Clippy all-targets e unfulfilled_lint_expectations, Rustdoc, fmt e linhagem estrita/reversa, mantendo 12 GiB, swap zero e um job.
