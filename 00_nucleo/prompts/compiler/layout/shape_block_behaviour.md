# L0 — shape block behaviour
Hash do Código: 19f0ed0c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:04f91fc931871165d4fef399ea9c2e21e8b75f7fd57031183a4e2aed0ed1b62a
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/shape.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/shape.rs:25`, `01_core/src/compiler/layout/shape.rs:35`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

The equivalent-input pinned SVG probes in
`00_nucleo/diagnosticos/font-demand-geometry-study/results.json` show that an
initial standalone non-text block moves when only text top-edge changes in
Tekt, while the pinned vanilla position remains the region's physical top.
This is observable geometry, not a requirement to copy upstream algorithms.
Those earlier controls do not cover the root Place regressions measured below; legacy subframe conventions remain unchanged outside the bounded correction.

Inspection of the subframe path in the owned consumer shows that cap-height is
queried before a branch whose ordinary positioning uses cursor_y directly.
`00_nucleo/diagnosticos/font-demand-geometry-study/catalog-counts/results.json`
records remaining actual Place catalog access; it does not prove every accessor
call is a catalog scan. The no-typography descendant controls discriminate the
unused query from the shared subframe's actual origin acquisition.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Shape rendering in `shape.rs`. Resolve the current Rect/RoundedRect/Ellipse/Path dimensions or Line dx/dy, establish the initial typography reference, close pending text and retain the main-flow block-spacing/collapse protocol. Shape position follows the source's distinct first/post-block cap-height versus post-text baseline-plus-above branches; subframes use direct positioning without main-flow above/below. Request ordinary progress only from the candidate shape's actual bottom, so a fitting shape is not rejected by unrelated typographic leading. Resolve radius against effective size, preserve Paint, stroke, fill rule and parent bbox. Delegate supported content-backed tiling fills to `00_nucleo/prompts/compiler/layout/tiling.md`; when materialized, preserve the separate unfilled stroke shape and paint order; otherwise retain the declarative fill. Advance using the current shape extent/below protocol and leave parent state coherent. Do not rasterize, sample all paints into a single solid color, shift Place through a global scan or move this render logic into ShapeElem.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Initial non-text block correction

When main-flow layout still has its initial baseline pending and no pending line,
place the first block at the physical region origin without acquiring typography
merely to add and subtract an artificial baseline. Mark that initial state handled;
retain the existing post-block cursor/margin protocol so later text can request its
own metrics. Explicit text top-edge must not move this block. Preserve pending-text
and subframe paths, dimensions, clipping/paint and existing overflow behavior except
that first-block fitting uses its actual geometric extent. Tests in this consumer
must discriminate explicit top edges and prove this path needs no FontMetrics query.
This internal parity repair does not change public API or pipeline phases.

## Subframe metric demand

Ordinary subframe block positioning must not query cap-height when the result is
unused. Preserve the current subframe origin and cursor/extent protocol. For a
shape whose actual extent triggers the existing new-page branch, resolve the
needed cap-height before transitioning pages, preserving its original style and
fallback coordinates. Image subframe positioning/extent does not use cap-height.
Keep the main-flow typography-dependent paths unchanged. This removes unnecessary
queries, not the subframe owner's initial catalog demand; no speedup is asserted.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/p1431_line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `00_nucleo/diagnosticos/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

A first shape with an initial baseline pending and an empty line consumes a geometric origin in the dedicated root Place sublayout as well as in main flow. It must not request typography for that origin. Existing baseline-origin sublayouts still clear the pending flag before dispatch and retain their positions.

## Isolamento de overflow local — 2026-10-13

### Medição anterior à decisão

As sondas em containers-clip-2026-10-13/before-renders.json registram
block(shape maior que altura declarada) criando duas páginas vazias no
Tekt contra uma região recortada no controle. Fonte e proveniência completas
estão naquele lote. Para corrigir a composição é necessário que o helper
isolado retenha o corpo, mas shape.rs:119, cursor.rs:125,
line_commit.rs:438 e region_progress.rs:115,131 ainda permitem avanço
externo por overflow com is_sub_frame ativo. sub_frame.rs:109-121,317-340
salva buffers/cursor, mas não uma página já comitada por esses descendentes.
Essa hipótese será testada diretamente antes de modificar as condições.
Fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC estão
em `00_nucleo/diagnosticos/subframe-isolamento-2026-10-13/before.json.gz`.

### Responsabilidade exclusiva e aceitação

Antes de pedir nova região pelo fundo da forma, distinguir is_sub_frame. A forma local excedente conserva a posição corrente e seu tamanho; não executar a compensação de cap-height reservada à transição externa.

Os invariantes compartilhados pertencem ao núcleo subframe-overflow pinado.
Correção interna ADR-0127, sem campo/assinatura pública, fase ou novo default.
Exigir RED→GREEN antes/depois das condições, preservação da paginação
ordinária externa, suíte core, build, formato e linhagem. Não elevar a
altura de teste para esconder overflow nem descartar itens do frame.
A correção do helper não equivale a terminar a composição de block/box;
a coleta, dimensões e recorte desses containers continuam nos seus owners.

## Posições no referencial composto — 2026-10-14

### Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz, fontes/binários/HEAD/working tree/UTC
preservados: Box, inset2pt, prefixo e aninhamento publicam (0,0), mesmo
quando a tinta se desloca. A diferença da baseline do context direto é
uma questão separada: não copiar o vanilla apenas por diferir. mod.rs:
1131-1148/1180-1200 escreve pontos locais em runtime.positions; Box agrupa
o corpo em outro referencial; finish.rs:295-302 sela o mapa sem recompor
esses pontos. Realized escreve com or_insert; labels têm mapa paralelo.
HTML já transporta markers privados pelo frame em html_frame.rs:30-199.
O registro single-pass e sealing por sub-store são preservados, ADR-0073/
0074 §§posição/modelo híbrido; não se adota introspecção global vanilla.
A inspeção do núcleo coordinates e seus owners mostrou usos absolutos
em grid e explicitamente locais em Place/Box. A qualificação do núcleo
conserva ambos e exige transformação até a página, sem dupla origem.
Snapshot anterior: posicoes-compostas-2026-10-14/before.json.gz.
Geometria/identidade são observáveis; o carrier e seu índice são mecânica.

### Contrato deste owner

No teste de início de bloco fresco, uma linha contendo apenas carriers posicionais diretos equivale à linha vazia. Conservar o significado de todos os outros itens, inclusive separadores. Não introduzir ascent de fonte por causa do carrier.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Ao concluir a forma, registrar shape_base + resolved_h, depois de atualizar o cursor; excluir below e cap-height preparatório. Preservar desenho e margens existentes.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.


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

Ao emitir Shape, publicar a dimensão horizontal resolvida na posição lógica, independentemente de fill/stroke/tiling. Não usar stroke/outset como avanço de fluxo.

### Aceitação e limites

Correção interna/de paridade ADR-0127: nenhum contrato público, default ou
fase novo; L0 primeiro, RED→GREEN, resselo restrito. A largura é uma parte
da correção B2b/T1–T3 já em andamento: altura Auto/backing, alinhamento tardio,
fracionários, fragmentação, outset e PDF Parent continuam com critérios próprios.
Não declarar render aprovado por testar apenas o carrier. Preservar controles
bilaterais completos e refutadores, suíte/build/doc/fmt/linhagem/inventários.


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

Preparar fill e stroke antes do emit por tiling::prepare_paint/prepare_stroke, registrando erros de layout. Emitir uma única Shape com as duas pinturas preparadas; nenhum Group de repetição. Preservar pos, kind, dimensões, fill_rule, parent_bbox_at_emit, protocolo de bloco e testemunhas de fluxo. Não alterar o ShapeElem declarativo. Testar fill e stroke com body Image contado uma vez por ocorrência, ausência de grade e custo independente da área alvo.

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

Resolver Length com tamanho efetivo da fonte, Ratio pelo eixo regional e
Relative pela soma dessas parcelas, no layout existente. Produto relativo
não finito contribui zero, conservando a parcela absoluta. Width usa a
largura local disponível; height usa altura interna explícita de Block,
senão a região relativa real salva, senão a altura disponível da raiz.
Não usar a sentinela de subframe como base percentual. Conservar o helper
legado para Values fora deste subconjunto e para None/Auto, e Line dx/dy.
Este recorte é explicitamente incompleto: o passo `shape-dimensoes-automaticas` de identidade e
dimensões automáticas de formas completa None/Auto, círculo/quadrado e casts.
Não certificar esses controles como corretos nem mudar API pública nesta etapa.

Correção interna/de paridade ADR-0127, sem API/trait/campo público nem fase
nova. L0 primeiro, RED→GREEN, resselo restrito, corpus, suíte e verificadores.
Preservar origem, pintura, contornos, recursos e efeitos; a medição especulativa
existente não ganha execução nova.

O primeiro GREEN candidato (implementation/shape-dim-green.log) ainda mede
37.5 em vez35 no Block com inset5: available_width na raiz desconta as
margens físicas, não line_start_x local. Para a base percentual horizontal,
a largura interna de block_place_region prevalece sobre available_width;
fora de Block mantém-se o cálculo local existente. Não alterar o fallback
legado None/Auto nesta etapa.


## Fronteira vertical de Shape — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/shape-terminal-complete/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/diffstat/UTC/fontes/binários e pin vanilla 9f2b6e871.
Página width100/heightAuto/margem10 e rect20×10: Tekt mede altura43.2,
pin30. Duas formas: segunda em y25.962 contra33.2. Texto depois da forma:
baseline20.438 contra40.438. Shape→Rotate usa top13.2 contra33.2;
Rotate→Shape mantém altura30 contra53.2. shape.rs:196–266 subtrai cap-height
de um topo geométrico e shape.rs:289–304 não publica prev_line_baseline
nem last_block_descent_y. cursor.rs:90–110 e transform.rs:352–356 usam
esses campos para a próxima unidade. v_space.rs:22–31 publica apenas a
âncora de Place, sem manter o fundo terminal confirmado. A fonte pinada
flow/collect.rs:253–285 e flow/distribute.rs:302–318 distingue frame e
margem fraca terminal. São observáveis geométricos; a organização Rust
e a estratégia de buffers não são requisitos de paridade.

### Responsabilidade exclusiva

No fluxo principal, posicionar a forma no fim físico anterior mais o gap
colapsado, sem subtrair cap-height do topo geométrico. Texto pendente fecha
pelo hook existente e fornece seu fim causal; primeira forma conserva a
origem física. Ao concluir, publicar o fim da forma em prev_line_baseline
e last_block_descent_y, limpar descida de equação/marcador de parbreak e
manter abaixo pendente separado do fundo. Transição de página usa o topo
geométrico da nova região. Isso supersede a fórmula P767 que preservava
cap-height e a proibição antiga de mudar esse protocolo. Não consultar
fontes por uma sequência só de formas. Preservar a rota local de subframe,
dimensões, pinturas e transporte. VSpace explícito posterior deve continuar
a contribuir para a altura; não aceitar uma correção que corte esse avanço.

Correção interna ADR-0127: sem assinatura/campo público, fase ou modo novo.
L0 primeiro, testes RED→GREEN, resselo restrito, corpus bilateral e suíte.
Defaults de formas, alinhamento RTL, Block Auto e colapso geral de espaços
continuam abertos; este contrato não autoriza encerrar esses eixos.
