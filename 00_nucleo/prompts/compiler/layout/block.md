# L0 — block
Hash do Código: cf8b5eb9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:ecf965471eedd8599209f0575811dd5b456dce1c43baa26881251e24e8ec6c6d
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:8d6cb7ee2458f368648c0ea6cfa84386dc506cf7273a16aa42bfbcbfeff05592
- 00_nucleo/prompts/_nuclei/layout/block-spacing-resolution.toml sha256:2e2f3cfb51bd0c484e65cc9e5c3e1eee9f3dad12c77aa9330674b474e11f3342
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e
- 00_nucleo/prompts/_nuclei/layout/intrinsic-literal-width.toml sha256:2f3643db87d4237fa5eb9ea2c2c264c60397812126f9b9a81593cbc58b166944
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/block.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/block.rs:21`, `01_core/src/compiler/layout/block.rs:32`, `01_core/src/compiler/layout/block.rs:43`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve optional relative width/height once against the original outer available width/height and active font, accepting only finite nonnegative results. Height percentages refer to the outer region, not remaining cursor space. Fractional height consumes only a private allocation supplied by `00_nucleo/prompts/compiler/layout/sequence.md`; absence is not zero and does not permit an intrinsic measurement fallback to masquerade as a resolved fraction.

Close prior pending text as currently implemented, resolve above/spacing/below against the effective font and collapse against the existing block/paragraph boundary protocol. Default spacing comes from `00_nucleo/prompts/compiler/layout/vanilla_defaults.md`, not an absolute fixture value. For an unbreakable block, request ordinary progress only when its measured complete size can fit a region but not the remaining one.

Install the body's constrained dimensions/origin and parent bbox temporarily, preserving RTL positioning, inset/outset, radius, clip, fill, stroke and overhang. Dispatch/close body content once and restore the outer dimensions/bbox/line start. In the surviving region, use the allocated height (explicit or causal Auto with inset) for the frame, backing and outgoing flow; visual outset affects paint/clip only. Keep the fragment path explicitly uncertified until its dedicated correction. Block owns its own frame end/gap; Cursor must not compensate it through generic tail translation. The measured vertical-allocation contract below supersedes the old minimum/cursor and below-minus-leading rules in the single-region path.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Block/Align — correção interna 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-align-2026-10-06/baseline-verification.json.gz` registra 14 controles completos, fontes, comandos, caixas e binários no HEAD/working tree/UTC de before.json. RTL Block automático põe ABC em x20 no Tekt contra x152.949220 no vanilla; Block(width:100pt) coincide em x100. #set dir:ltr no body não altera a âncora externa do Block automático no vanilla. Align(left/right) mistura bordas por direção do run e desloca Y por 8.35742pt. `placement.rs:35-46,176-177` renderiza no subframe com baseline pronta e subtrai ascender na recomposição; `cursor.rs:154-162` só aplica direção RTL; `block.rs:175-185` posiciona container externo apenas quando largura é explícita. Fonte vanilla pinada `flow/collect.rs:193` resolve Align por linhas e `flow/block.rs:145-166,285` distingue regiões expandidas de auto-size. Paridade de posições/forma, não cópia da mecânica (ADR-0107).

### Responsabilidade exclusiva

Preservar direção e layout do corpo. Largura explícita resolve a posição externa por explicit_line_alignment, se existente, senão direção da chain, sem passar RTL para o trecho LTR. Para largura automática, body textual certificado, região finita, sem clip/paint/outset e sem mudança de página durante emissão, mover o frame emitido como unidade para a borda RTL externa quando não há override explícito. Usar items emitidos e métricas existentes, sem medir/renderizar o corpo novamente. Mover também novos segmentos de decoração. Preservar Y, margens, body order e resto do algoritmo. Frames desconhecidos/paginados/geométricos ficam fora desta correção; não inferir que todos os Blocks foram certificados.

Correção interna/de paridade ADR-0127 em fluxo contínuo. Nenhum contrato público, modo, fase, compatibilidade ou unsafe novo. O certificado é limite explícito desta rodada e deve ser refutado por nested Align, Block de largura explícita, scoping após Align, auto dimensions, texto regular e corpus anterior. RED→GREEN, revalidação bilateral, builds/lint e resselo somente do manifesto. Controles negativos não são redefinidos como aceitos. Sem promessa de ganho de tempo.

## Sequência em coluna Auto e frame literal — 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06/before.json` conserva
estado/HEAD/stat/status/fontes L0 e código/binários. O lote predecessor em
espaco-terminal-2026-10-06/verification.json.gz mede seis tabelas: ABC/123 e
árabe ficam em duas linhas no Tekt, uma no vanilla pinado 9f2b6e871.
measurement.rs:104-113 toma max das larguras de filhos e soma alturas de
Sequence, mesmo quando Styled/Text/Space formam um fluxo inline. O vanilla
inline/grid mede o frame completo (grid/layouter.rs:967-1100). Block RTL com
LTR literal terminal desloca ABC em 3.49658pt por espaço: block.rs:315 usa
line_content_right que exclui todo TextSeparator. page_geometry.rs:143-154
já distingue extensão auto de espaço literal da extensão de tinta da linha.
tracks.rs:108-113 ancora toda coluna em margin.left, inclusive uma coluna
Auto RTL; no oracle a coluna única termina na margem direita. Observáveis
de geometria/texto, não obrigação de copiar algoritmo ou bytes. Hipótese
refutável por quebra/âncora ainda divergente ou regressão de espaço fraco,
coluna fixa, estilos/linhas/parágrafos após medição de frame/âncora corretas.

### Classificação

Correção interna de paridade ADR-0127, sem API pública/fase/flag/dependência
ou unsafe novo. Não inferir intenção histórica de P80. Núcleo de extensão
intrínseca distingue dimensões do container de tinta/alinhamento de linha.

### Responsabilidade exclusiva

No certificado existente de Block auto textual RTL, usar a extensão intrínseca
pinada para mover o frame inteiro: max da extensão visual vigente e dos
carriers Literal emitidos (pos.x+advance). Markup/Newline seguem excluídos.
Não modificar cursor, direção do body, Y, decoração ou certificados anteriores.


## Restore dimensions only within the same region — 2026-10-13

### Measurement before decision

`block.rs:178` snapshots the outer height and `:334` restores it unconditionally
after body dispatch, even if the body advanced a region. The paired source in
`diagnosticos/correcoes-producao-2026-10-13/math-bilateral.json` puts FLOAT_AFTER on
a fourth page in Tekt instead of beside AFTER_MARKER on page three in the pinned
vanilla. The hypothesis is stale reduced-region geometry leaking after a Block's
body advances; a pure unit fixture must refute or confirm it before repair.

### Internal correction and acceptance

Restore a temporary body's outer height only if its original physical page and
column are still active. After an ordinary region transition, keep the height
installed by that transition; do not copy a prior page's float-reduced limit into
the new region. Preserve the existing local constraints while the body runs,
width/bbox restoration, minimum height, styles and spacing. No new API, default
or phase. A RED unit exercises a region transition from a float-reduced height
and verifies the next region's available height; the P1292 flush transcript and
Block/float controls must retain content, position and no duplicate emission.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Isolar a fronteira anterior ao entrar no body; conservar o fim físico produzido antes dos avanços de inset/below/leading. Ao sair, registrar fim da região explicitamente alocada quando há altura (start_y + outset_top + altura + outset_bottom), ou fim causal do body mais inset inferior/outset; sem alterar a política de desenho ou cursor legado nesta etapa. O ramo multipágina deve usar a fronteira da região sobrevivente, nunca o start_y da página anterior.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.

## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

Instalar região interna com as duas bordas de inset; largura declarada é total com inset, não largura do body acrescida de inset. Conservar certificado do body; resolver alinhamentos Y auto contra sua altura causal antes de agrupar. Clip usa origem externa e altura alocada/causal, rebaseia filhos e caminhos uma vez. Restaurar região pai somente se página/coluna sobrevivem. Preservar fluxo de margem existente; tamanho/desenho/outset e fragmentação ainda precisam controles adicionais B2b.

### Aceitação e incompletude explícita

Correção interna/de paridade ADR-0127, sem campo público, modo ou fase. Este adendo substitui somente as fórmulas incompatíveis de região/width/clip anteriores; não legitima outros reparos sem medição. L0 primeiro, RED→GREEN, validação bilateral, suíte e resselo restrito. B2a cobre região local e clipping na região sobrevivente. B2b completa fragmentação de Block, dimensões/outset e integração dos metadados/decoradores que controles adicionais refutarem; B3 continua responsável por baseline tardia de inline Box/Transform e produtores ainda não certificados. Não declarar objetivo completo com essas frentes abertas.

B2a conserva também uma lista privada de faixas de decoração associadas ao índice do fixup Y de Place auto. O hub armazena; checkpoint clona/restaura; subframe e Block isolam/restauram; transição limpa; placement registra somente os segmentos emitidos pela ocorrência; Block aplica a mesma delta da geometria antes de encerrar o escopo.

### Refutação por percentuais aninhados

extra-first/results.json.gz em block-regiao-2026-10-14 registra root-percent-nested Tekt(44,33), pin(42,34); dentro de Box Tekt(42,38), pin(42,34). block.rs:80-85 resolve porcentagens pela dimensão global mutada, apesar de existir região local do pai. Ao resolver Block descendente, usar width/height da região interna privada quando existente (altura somente finita); na ausência manter região exterior. Isso completa a mesma correção de região local, sem mudar API/fase. Teste RED→GREEN distingue raiz/Box e controla restauração.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`00_nucleo/diagnosticos/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [332].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Na mesma região física, envolver o body já composto numa raiz de pintura com dimensões alocadas totais e origem externa do Block. O recorte continua grupo técnico dentro dela; rebasear filhos e caminhos X pendentes uma única vez por envelope. O backing permanece irmão anterior fora da raiz. Aplicar o envelope depois do ajuste RTL existente para preservar sua medição de folhas. Preservar cursor, fluxo causal, posições, decoração e bbox legados. Não corrigir outset, fragmentação ou height mínimo implicitamente. Blocos que avançam página/coluna ainda precisam de raízes por fragmento no lote de fragmentação; esta etapa não os certifica.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.

### Refutação adicional: limite local descontava a margem da página

block-pages-before/results.json.gz deste lote usa o mesmo binário SHA-256 de
before/results.json.gz (fontes T1a antes da migração). Block(width:60pt,
height:40pt,inset:3pt) com forma 8×6pt e margem de página padrão produz duas
páginas no Tekt, a primeira vazia, contra uma no vanilla pinado, com clip false
e true. O RED de paint_root confirma a transição mesmo antes de criar a raiz.
block.rs instala height como start_y+outset_top+h; shape.rs e
page_bottom_limit em layout/mod.rs subtraem margin.bottom desse limite.
Portanto o limite do body perde a margem física da página uma segunda vez.
É erro de fórmula/coordenada interno, não comportamento por defeito novo.

Ao instalar o limite temporário de altura, somar margin.bottom ao extremo
local antes de limitar pelo height externo salvo. Assim os consumidores que
subtraem margin.bottom observam exatamente o fim local alocado, sem exceder
o limite físico externo. Não mudar o tamanho declarado, outset, avanço
mínimo ou política de fragmentação. Revalidar os dois casos bilaterais e
controles de Block/Place, além da suíte; não mudar a expectativa de uma página
nem reduzir a margem da fixture para ocultar a falha.

### Observadores das regressões históricas

layout-first.log registra folhas ausentes em buscas planas após o novo
envelope, embora o body permaneça no frame. Os observadores dos testes
afetados devem usar a vista recursiva com projeção afim, preservando todos
os valores de geometria/texto/tolerâncias. Disponibilizar o helper somente
a árvore de módulos de teste. Não alterar esperados para tornar uma
regressão visual verde. A fixture de tiling reconhece células como raízes
locais; máscaras técnicas externas continuam soft.

### Integração PDF ainda refutada nesta etapa

first-render-delta.json e after-first/results.json.gz em paint-root-2026-10-14
registram uma regressão no controle block-auto-gradient-parent/PDF: 3840
pixels mudam mais de 16 níveis contra o baseline e a distância ao pin cresce.
Os demais 35 renders Tekt do corpus mantêm pixels. O collector legado de
gradientes troca parent_bbox_override a cada Group; o novo envelope de raiz
expõe que esse override não é um referencial afim. Os campos de Shape ainda
são declarativos. Essa diferença é falha aberta, não nova referência aceita,
e não autoriza mudar tolerância ou o esperado. T1c/T2 devem fechar o contexto
resolvido de pintura e os recursos para corrigir a causa; T1b isoladamente
não certifica render nem produção. Preservar a amostra refutadora e também
os controles que coincidem, inclusive a divergência vanilla PDF de quarter.


## Largura causal de Block Auto — 2026-10-14

### Medição anterior à decisão

block-auto-width-2026-10-14/before/results.json.gz conserva HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diffstat/status/fontes/binários/argv e pin 9f2b6e871. O extrator inicial
procurou blue como #0000ff; blue-geometries-corrected.json lê a cor real
#0074d9 dos mesmos SVGs, sem nova execução. Em shape, Empty, Place-only,
Grid e Stack a largura do backing Tekt é160; pin54,14,14,68,68 respectivamente.
block.rs:222,416,453 usa largura disponível; line_commit.rs:753 perde cursor
no fechamento e sub_frame.rs:354 guarda apenas cursor terminal. Fonte pinada
flow/block.rs:145–166,281–290,339–363 distingue Auto de expansão.
É geometria da linguagem; estrutura da testemunha é mecânica interna.
A hipótese é refutada se Place contaminar largura, escopos vazarem, texto
literal perder avanço ou containers explícitos mudarem de dimensão.

### Responsabilidade exclusiva

Isolar origem/largura ao entrar no corpo, obter largura Auto do fluxo emitido + inset; explícita prevalece. Usar a mesma largura final no clip, paint_root e backing. Restaurar extensão exterior e publicar a alocação do Block; helper privado record_flow_width_at acumula origem+largura relativa ao escopo. No caso RTL textual já certificado, mover a origem da raiz com a translação existente do conteúdo. Não usar tinta de Place ou segunda medição.

### Aceitação e limites

Correção interna/de paridade ADR-0127: nenhum contrato público, default ou
fase novo; L0 primeiro, RED→GREEN, resselo restrito. A largura é uma parte
da correção B2b/T1–T3 já em andamento: altura Auto/backing, alinhamento tardio,
fracionários, fragmentação, outset e PDF Parent continuam com critérios próprios.
Não declarar render aprovado por testar apenas o carrier. Preservar controles
bilaterais completos e refutadores, suíte/build/doc/fmt/linhagem/inventários.


## Alinhamento geométrico em Block Auto — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-auto-alignment-2026-10-14/before/results.json.gz`
conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/diffstat/status/fontes e SHA dos binários. right/center/link com clip
perdem todo o retângulo vermelho no Tekt (zero pixels), contra 3840 no pin
9f2b6e8715237cb086899a42873660fe744622e8, bbox [48,40,128,88] em 2px/pt.
Com irmã de 70pt, right precisa bbox [108,40,188,88]. placement.rs:241 usa
avail_w finito como dimensão final; block.rs:287–303 só conhece a largura
causal depois do corpo. Fonte pinada flow/block.rs:278–287 distingue
constraint de expansão. Posição e clipping são observáveis de linguagem;
filas/flags são mecânica interna. Refutadores: inset, irmãos maiores,
Link, Block fixo/aninhado e escopo após subframe/rollback.

### Responsabilidade exclusiva

Instalar block_auto_width segundo resolved_width e restaurar após o corpo. Na mesma região, resolver fixups X produzidos pelo corpo contra o limite interno final, antes de clip e paint_root. Deslocar somente os paths registrados por helpers existentes; não alterar largura causal nem executar corpo novamente. A fragmentação mantém pendência explícita.

### Classificação e limites

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem contrato público,
modo/default ou fase novos. L0 primeiro, RED→GREEN, resselo restrito e suíte.
Esta etapa completa o refutador geométrico da largura causal; não completa
alinhamento textual/matemático, decoração externa diferida, Place horizontal,
altura Auto ou fragmentação. São pendências de B2b/T3, sem crédito de paridade.


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

Preparar o stroke do próprio backing no referencial externo já restaurado, antes de emprestar o buffer para inserção; fill Color torna-se FramePaint::Solid. Preservar overhang, dimensões, posição e ordem do backing. Não ampliar o tipo declarativo de Block.fill.

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

## Dimensões relativas de formas — correção medida

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/shape-dim-before-complete/results.json.gz`
registra HEAD, working tree não commitado, diffstat, UTC, fontes e binários.
Root Ratio50%/25% em região160 mede80×40 no pin; o Tekt descarta Ratio e
em. Block80 com inset5 exige largura35 e altura75 se auto,25 se altura60.
Box80×60 exige40×30; Box de altura auto exige40×80. A forma unbounded com
50%+10pt em ambos os eixos mede10×10 no pin; o candidato produz SVG inválido.
helpers.rs:427–438 só resolve parte absoluta; shape.rs:150–151 usa esse
fallback para todos os Values. sub_frame.rs:130–141 herda infinito do Block
auto e perde a região real. Fontes pinadas: flow/block.rs:255–290,
layout/ratio.rs:108–110 e layout/rel.rs:112–113. Geometria de linguagem,
não igualdade de bytes ou exigência de organização Rust.

### Responsabilidade exclusiva

Ao entrar no corpo, instalar relative_region_size com largura interna e
altura interna de referência: dimensão resolvida, ou base externa real, menos
insets. A base externa consulta primeiro Block finito, depois região relativa
herdada, depois a região física. Salvar/restaurar o campo existente ao redor
do corpo em LIFO. Altura ilimitada genuína permanece ilimitada. A região de
Place com altura Auto continua separada da base percentual: não substituir
seu marcador infinito nem mudar clip, paginação ou altura causal final.

Correção interna/de paridade ADR-0127, sem API/trait/campo público nem fase
nova. L0 primeiro, RED→GREEN, resselo restrito, corpus, suíte e verificadores.
Preservar origem, pintura, contornos, recursos e efeitos; a medição especulativa
existente não ganha execução nova.


## Alocação vertical de Block — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/block-height-before/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
status/diffstat/UTC, fontes e binários contra pin 9f2b6e871. Em página
width100/heightAuto/margem10, Block(height40) com Shape20×10 termina em30pt
contra60pt; inset5 termina em35pt contra40pt. Block textual e vazio conservam
leading/abaixo artificial. Continuações Shape/Text/Place e Block aninhado
refutam o uso do fim do filho como fim exterior. Outset5 altera a posição do
body e página no Tekt, sem fazê-lo no controle.

`block.rs:298–308` já calcula root_height com região explícita ou fim causal
e insets, mas `:475–498` usa cursor+insets/minimum/outset; `:506` usa esse
cursor para backing; `:568–604` mantém duas regras de saída e não publica
last_block_descent_y do próprio container. A entrada preserva estado de
cadeia do pai enquanto despacha o filho. Pin flow/block.rs:66–89,220–240
força a alocação antes de padding, e shapes.rs:617–684 aplica outset só ao
recorte/pintura. Não inferir intenção da implementação antiga nem copiar
seus buffers: tamanho, posições, ordem e clipping são observáveis da língua.

### Contrato exclusivo

Na região física sobrevivente, uma única alocação vertical deve alimentar
paint_root, backing, cursor/fim causal e fundo terminal. Altura explícita
prevalece; Auto usa fim causal do corpo mais insets, sem leading/abaixo
preparatório. Publicar fim próprio em prev_line_baseline e last_block_descent_y,
limpar descida de equação, deixar abaixo pendente separado e registrar a
mesma fronteira para Place. Isso substitui o ramo legado below-minus-leading
e a regra de mínimo baseada no cursor apenas nesta composição. Não inferir
altura por tinta nem executar corpo novamente.

A entrada converte uma linha anterior já fechada em fim causal+above,
preserva colapso da cadeia externa e inicia a cadeia/baseline do corpo na
origem interna; o filho não herda margem nem fundo terminal do pai. Ao
retornar a um cursor externo, a primeira linha seguinte deve adquirir seu
próprio top-edge uma vez, usando a fronteira do Block. Preservar efeitos,
ordem e coordenadas dos metadados nas translações existentes.

Outset é expansão visual, não padding nem avanço de fluxo. Não deslocar
body/paint_root/limite da região por outset. Backing e máscara expandem
a alocação pelas bordas e deslocam a origem por -left/-top; o body mantém
sua posição global ao ser rebaseado dentro do grupo técnico de clip.
Preflight unbreakable usa dimensão declarada ou medida+insets, sem outset;
medir sob largura interna descontando insets e sem nova execução de efeitos.
Não alterar assinatura pública, campo, fase ou estratégia estática.

Blocos que atravessam região conservam a rota de fragmentação até uma
medição própria a completar: não misturar start_y antigo com fundo novo,
nem declarar sua pintura/altura/raiz por fragmento certificadas. Esta
limitação não permite restaurar dimensões de uma região que já terminou.

Classificação ADR-0127: correção interna/de paridade em fluxo contínuo.
L0 primeiro, RED→GREEN, verificação bilateral SVG/PNG/PDF, regressões de
continuação, inset, nested, explícito/Auto, texto/vazio, outset, clipping e
paginação; resselo restrito e suíte/build/doc/fmt/check/linhagem.


## Fronteiras preparadas de Block — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/block-boundaries-before/results.json.gz`
conserva HEAD, árvore não commitada, UTC, diffstat, hashes das fontes/binários
contra pin 9f2b6e871. Em PDF, prefixo Lead em columns desloca One por
7.238pt adicionais; prefixo8 e prefixo18+More repetem esse erro. Após
pagebreak, X no Block inset5 também desce7.238pt a mais. block.rs:201
reinicia a baseline do corpo sem converter o cursor já preparado do pai;
sequence.rs:242–257 prepara a fronteira do prefixo e region_progress.rs:81–87
prepara a baseline da nova região. São posições de conteúdo observáveis,
não equivalência mecânica. A sonda inclui linha fechada, espaço explícito,
colunas, página nova e controle com fonte maior; esse último ainda expõe
espaçamento de parágrafo por estilo, a corrigir separadamente.

A mesma sonda refuta a explicação anterior de P248: em A4, v620 deixa
cerca de80.157pt físicos. Block80 cabe numa página, Block90 exige duas,
independentemente de outset0/30. Outset continua exclusivamente visual.
A saída final de Align(horizon) após Block200 também permanece incorreta:
a página Tekt230pt perde o corpo seguinte; controle250.438pt. Não declarar
estes alinhamentos nem os prefixos de fonte maior certificados por esta correção.

### Contrato exclusivo

Ao entrar sem linha pendente nem cadeia de blocos, distinguir origem
física ainda pendente de cursor de baseline já preparado. Se a baseline
está preparada e a testemunha causal ainda corresponde ao cursor, usar
fim causal+above (incluindo espaço forte já registrado). Se a testemunha
foi invalidada pela fronteira do prefixo ou pela troca de região, retirar
uma vez o top-edge preparado antes de iniciar a baseline interna. Não
retirar top-edge da origem física inicial nem da cadeia externa já colapsada.
Preservar regiões, translações e efeitos; nenhuma nova execução do corpo.
Correção interna ADR-0127: L0 primeiro, RED→GREEN, resselo e revalidação.


## Fronteira fracionária geométrica — 2026-10-14

Medição anterior: implementation/block-boundary-layout-{pre,result}.json.gz
registra HEAD/árvore/UTC/diffstat/fontes; o log mostra caudas After222.3 contra
Tail230 sob FixedMetrics após retirar top7.7 da origem. A sonda bilateral
block-boundaries-before/results.json.gz já demonstrou que a baseline de One
precisa subir o top7.238 das métricas reais, enquanto After/Tail já coincidem.
sequence/fractional_preflight.rs:140–141 reserva prefix.fixed_height, que em
inline_prefix.rs:484 inclui também o top da próxima baseline; a nova origem
geométrica de Block já o retira. Portanto a parcela precisa devolver esse
espaço reservado, não deixar um buraco no fundo. Critério é a geometria dos
filhos e da cauda, não a forma de calcular a distribuição Rust.

Contrato exclusivo complementar: quando fractional_block_height está
alocado e a entrada já está sem linha/cadeia, a fronteira é preparada pelo
prefixo fracionário; retirar o top preparado mesmo se block_boundary zero
preservar numericamente a testemunha de flush. O espaço livre pertence ao
owner 00_nucleo/prompts/compiler/layout/sequence/fractional_preflight.md;
Block não altera pesos nem infere a distribuição a partir do filho.


### Refutador de origem local — 2026-10-14

block-boundaries-after/results.json.gz preserva proveniência completa e PDF:
checkpoint põe AFTER_FLOW na página2 e perde FLOAT_AFTER; pin exige ambos na
página3. implementation/block-boundary-layout-v2.log confirma P1292 RED.
sub_frame.rs:211–217 fornece uma origem local de baseline e :371 mede altura
por cursor−start_y. Retirar top nessa origem sem mudar o start do caller
encurta o float em7.238pt; sua reserva fica incorreta. É perda de conteúdo e
paginação, sem justificativa de divergência intencional.

Restringir a conversão do cursor sem testemunha à região exterior ou à
fronteira fracionária certificada. No subframe local sem testemunha, conservar
a origem estabelecida pelo caller; o corpo e o retorno mantêm a mesma base.
Testemunha válida de linha fechada continua usando o fim causal. Não mudar
API, fase, política de floats nem subtrair tinta. Revalidar P1292, as páginas
com todos os marcadores e a suíte. Correção interna ADR-0127.


## Margens automáticas de Block — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/block-par-spacing-before/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC, diffstat, hashes de fontes/binários e pin vanilla 9f2b6e871. Com fonte10,
top10 e bottom0, PRE seguido de Block20 produz página62 no Tekt contra50/55/70
com par.spacing0/5/2em. Block20 below0 seguido de texto produz página40 contra50.
block.rs:127–130,585–588 e sequence/fractional_preflight.rs:216–227 substituem
par.spacing por 1.2em; cursor.rs:103–110 omite top quando below zero.
Fonte pinada typst-layout/src/flow/collect.rs:253–258 resolve Auto por ParElem::spacing.
`block-par-extra-before/results.json.gz` mede dois Blocks1fr numa região80:
a baseline de B deve ser60/62.5/70 para spacing0/5/2em, contra66 em todos.
São posições/paginação observáveis da língua. Não é obrigação de copiar o Rust.
A inferência é refutada por overrides explícitos, troca de região, margens
terminais, estilos e regressões de Raw/Place/Align. before-edit.json.gz no
diretório block-par-spacing conserva os owners antes da edição.

### Classificação e limites

Correção interna ADR-0127, L0 primeiro, RED→GREEN e resselo restrito; sem
API pública, fase, default novo ou compatibilidade alterada. A amostra
scope19 refuta usar consenso de tamanhos dos runs como escopo de parágrafo:
um wrapper18 e dois wrappers18 adjacentes têm margens distintas no pin.
O escopo lexical continua pendente, assim como a cauda textual após Block1fr,
valores negativos rejeitados pelo constructor e colapso entre margens de
forças diferentes. Não marcar esses casos como corrigidos por esta fórmula.

### Responsabilidade exclusiva

Resolver above e below pelo núcleo block-spacing-resolution pinado, capturando
par.spacing na chain exterior antes de entrar no corpo. Preservar precedência
dos campos explícitos, terminal sem margem extra e protocolo de fronteira.
A referência anterior a vanilla_defaults é apenas o último fallback.


## Saída de escopo aninhado — 2026-10-14

Medição anterior à decisão: prefix-nesting-controls/results.json.gz preserva
HEAD/working tree/UTC/hashes e vanilla pinado9f2b6e871. Nos controles nested22–24,
deep e box, BODY coincide mas AFTER18 fica12pt acima e a página auto12pt menor.
block.rs:621 apaga a força fraca ao publicar below automático8=9.6pt;
cursor.rs:90–107 já colapsa margem fraca com spacing do parágrafo seguinte.
Vanilla flow/collect.rs:253–260 atribui Auto a weak spacing, enquanto margem
explicitada tem prioridade. Não é uma decisão por coincidência de valores.

Correção interna ADR-0127, teste RED→GREEN. No fechamento do Block, publicar
prev_margin_is_parbreak=true somente quando below e spacing estiverem ausentes,
reutilizando o protocolo privado de margem fraca já usado por Raw/parágrafo.
Below/spacing explícitos, inclusive zero, mantêm false e precedência. O Cursor
existente resolve o colapso na admissão do parágrafo exterior. Não alterar altura,
fonte do corpo, valores, API, fases nem flags. Terminal sem próximo conteúdo
não materializa margem. Revalidar controles de zero/explícito, consecutivos,
fracionários, nested e suíte inteira; isto não certifica toda força de margem
em todos os containers.

## A2 — prefixo admitido e região inline de Block (2026-10-18)

### Medição anterior à decisão — admissão e continuação

pagination-block.json.gz e block-freeze.json.gz em
00_nucleo/diagnosticos/bibliography-structure-2026-10-18 preservam estado completo,
UTC/HEAD/fontes/binários/argv e caixas do candidato Block/Place c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea.
Em IEEE/200×90pt ele recupera duas páginas, mas a continuação começa em x8
contra29.924 no oracle bd5ebec285505889e8ff1d4f68804ab147a699f3 (binário
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9).
region_progress.rs:75–76 reinstala a margem física ao transitar, descartando o
inset ativo. O teste corrigido para70pt também mostrou prefixo na primeira
página e corpo admitido na segunda: paragraph_pagination.rs:87–105 pode adiar
o corpo para respeitar widow/orphan depois que o Place já foi emitido.

A exigência inicial de FIRSTWORD sempre na primeira página da fixture FixedMetrics
não é o contrato correto: group_size em paragraph_pagination.rs:69 pode manter
três linhas juntas. Substituir esse critério pelo prefixo acompanhar a página
real da primeira linha; preservar o RED PDF90/duas páginas e a distinção entre
coluna estável e estratégia de quebra. Não forçar greedy nem desabilitar
widow/orphan para satisfazer um assert. Posições geométricas, conteúdo e agrupamento
são observáveis da língua; slots/representação são mecânica privada.

Correção interna ADR-0127 dentro do layout, sem assinatura/campo/trait público,
flag/default novo, compatibilidade ou execução eval/show. A primeira tinta do
prefixo continua antes da primeira tinta do corpo, após a admissão física do
parágrafo. Não é nova ordem de realização de show nem transferência de fase.
A hipótese é refutável por prefixo órfão, recuo perdido, emissão dupla, efeitos
novos, estratégia alterada ou regressão dos controles de Block/columns/subframe.

### Responsabilidade exclusiva

Definir contexto Copy privado InlineRegion com offset horizontal relativo ao início da página/coluna exterior e largura interna. Instalar ao entrar no body depois de resolver inset e dimensões; restaurar em LIFO depois do body. Expor apenas helper descendente que reinstala a geometria horizontal na região atual a partir do contexto, ou da região externa natural quando ausente. A mesma região conserva saved_line_start/saved_width. Após mudança de região, restaurar o contexto do pai e a geometria atual desse pai; não copiar origem/limites da região encerrada nem alterar altura. O contexto não transfere inset vertical, pintura ou dimensões de outra fase.

Aceitação: RED→GREEN de prefixo/corpo na mesma página e inset nas continuações, inclusive saída para OUTSIDE, simple/optimized e controles existentes. Medir PDF90/150/250, RTL, prefix-only/formatado, cores e regressões; build, suites/doctests/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial Tekt. MemoryMax12GiB/High8GiB/swap0/jobs1; sem publicação. Hanging genérico/overrides e múltiplas bibliografias continuam frentes abertas.

## A2 — preparação do parágrafo próprio de Block (2026-10-18)

### Medição anterior à decisão

A auditoria estrita complete-audit.log da primeira tentativa em
00_nucleo/diagnosticos/bibliography-structure-2026-10-18 parou em ieee-none-size.
attempt1-artifacts.tar.gz e admission-sources.json.gz conservam recibos e estado
exato do HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/working tree não
commitado, fontes/L0s, binários, UTC/argv/textos/caixas. No PDF18pt, Tekt põe
2020 em x155.654/y173.576 e o ponto sozinho em x40.886/y197.12; oracle pinado
bd5ebec285505889e8ff1d4f68804ab147a699f3 põe 2020. unido em x40.886/y197.120014.
O cache-shape.txt extraído pela API pública mostra Text(2020) adjacente a Text(.),
sem espaço/quebra causal; o body inteiro é elegível à preparação latina.
sequence.rs:46–58 suprime tentativas dos filhos depois de recusa do fluxo
exterior (que contém Bibliography). Block não reinicia essa supressão ao abrir
seu corpo independente. Isso deixa as folhas incrementais quebrarem antes do
ponto, embora paragraph_prepare.rs:158–183 já preserve sua adjacência.

### Responsabilidade exclusiva

Salvar paragraph_planning_suppressed e reinstalá-lo false somente durante o
corpo do Block, restaurando em LIFO após sua emissão/fechamento. Block constitui
um novo fluxo próprio; a recusa de preparar a sequência exterior não recusa o
parágrafo interno. Preservar direção/estilos/tracking e backend/configuração
existentes; não habilitar preparação parcial de Styled/filhos no mesmo fluxo
recusado, não forçar greedy/optimized e não modificar caracteres CSL. Sem API,
fase ou flag nova, correção interna ADR-0127. RED de tokens adjacentes em Block
sob sequência exterior, corpo com estilos/mais de uma linha, restauração de
estado e demais controles. Repetir PDF18, corpus e validação completa sob12GiB.
