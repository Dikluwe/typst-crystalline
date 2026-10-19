# L0 — placement
Hash do Código: eca8fdf5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:ecf965471eedd8599209f0575811dd5b456dce1c43baa26881251e24e8ec6c6d
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/placement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/placement.rs:21`, `01_core/src/compiler/layout/placement.rs:277`, `01_core/src/compiler/layout/placement.rs:539`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own layout_align, non-float layout_place, resolve_alignment and deferred horizontal/vertical fixups. `00_nucleo/prompts/core/compiler/layout/align.md` and `00_nucleo/prompts/core/compiler/layout/place.md` own their element adapters. Use the existing absolute sublayout origin and add only the incremental alignment delta on recomposition, never the full origin twice. Width comes from already-laid-out items and FontMetrics; vertical alignment resolves cell-region first, then unconstrained sublayout fallback, then remaining page region.

For Place, preserve Page/Parent/Column scope, inherited per-axis cell alignment, resolved dx/dy and out-of-flow emission. The Place adapter supplies the current-flow correction when a pending line requires it; this geometry helper does not apply a global post-layout scan.

When auto dimensions are still infinite, use the current finite-content fallback and record path-based deferred fixups in hub storage. Nested Group/Link/Semantic paths and orphan rebase remain in the same physical/logical conventions. Items, decoration segments and alignment metadata receive their respective matching deltas, with baseline compensation applied only where the item convention requires it. Apply deferred fixups once against final dimensions and clear them. No page policy, cloned checkpoint or entity-to-render import is created.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `lab/results/construction/diagnostics/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

Root Place without a cell anchor uses the isolated geometric-origin path. Recompose its local positions at the resolved page anchor without subtracting an artificial ascender or acquiring that ascender. Text keeps its own measured top edge; non-text placement must not acquire a font catalog merely to create an origin. Preserve existing nested and cell coordinate conventions, scope/alignment, deferred fixups and outer cursor restoration.

The unchanged p1292 pending-line witness fails when geometric origin also applies after pending text (`tests.rs:12886`); its adapter already supplies the text top edge. Restrict the dedicated geometric-origin path to root placements with no material pending line and no cell anchor. Pending-line placements retain the existing baseline-origin path to avoid applying that edge twice.

## Block/Align — correção interna 2026-10-06

### Medição anterior à decisão

`lab/results/construction/diagnostics/block-align-2026-10-06/baseline-verification.json.gz` registra 14 controles completos, fontes, comandos, caixas e binários no HEAD/working tree/UTC de before.json. RTL Block automático põe ABC em x20 no Tekt contra x152.949220 no vanilla; Block(width:100pt) coincide em x100. #set dir:ltr no body não altera a âncora externa do Block automático no vanilla. Align(left/right) mistura bordas por direção do run e desloca Y por 8.35742pt. `placement.rs:35-46,176-177` renderiza no subframe com baseline pronta e subtrai ascender na recomposição; `cursor.rs:154-162` só aplica direção RTL; `block.rs:175-185` posiciona container externo apenas quando largura é explícita. Fonte vanilla pinada `flow/collect.rs:193` resolve Align por linhas e `flow/block.rs:145-166,285` distingue regiões expandidas de auto-size. Paridade de posições/forma, não cópia da mecânica (ADR-0107).

### Responsabilidade exclusiva

Definir certificado estrutural interno para bodies textuais (Empty/Text/separadores/Linebreak/Sequence/Styled/Strong/Emph/Par/Align e Block simples sem paint/clip/outset). Para esses bodies, instalar override horizontal lexical (Start/End resolvidos pela direção do container), herdando o pai no alinhamento só vertical; renderizar uma vez no subframe geométrico existente; rebase vertical em origem zero. Cada linha é alinhada pelo cursor, sem uma segunda translação horizontal do envelope; diferimento de dimensões auto e VAlign continuam pelas rotas existentes, sem coordenadas infinitas. Restaurar override antes de recompor no pai. Bodies não certificados mantêm caminho legado. Testar duas linhas com direções distintas, baseline de Align vs texto simples, scoping/rollback e origem Block; nenhum teste é mera cópia de branches.

Correção interna/de paridade ADR-0127 em fluxo contínuo. Nenhum contrato público, modo, fase, compatibilidade ou unsafe novo. O certificado é limite explícito desta rodada e deve ser refutado por nested Align, Block de largura explícita, scoping após Align, auto dimensions, texto regular e corpus anterior. RED→GREEN, revalidação bilateral, builds/lint e resselo somente do manifesto. Controles negativos não são redefinidos como aceitos. Sem promessa de ganho de tempo.

Medição adicional anterior à correção do gap: after-inline/newline/spaced/leading no baseline dão primeiro texto y11.642578 e seguinte y28.357418; vanilla y19.999978/41.557438. par.leading=2em não muda o gap após o container, enquanto par.spacing=3em muda o segundo texto para61.357418. `vanilla_defaults.rs:14` e fonte `model/par.rs` fixam par.spacing em 1.2em. Para Align textual material, conservar fundo causal do frame e instalar o protocolo de margem de bloco existente (prev_line_baseline=frame_end, below=par.spacing resolvido, initial_baseline_pending=false), consumido apenas quando entrar texto seguinte. Não somar leading nem margem fraca à altura física da página final. O caso vazio não inventa frame material. Teste de Align(A) seguido de Parbreak/B versus parágrafo A/B refuta a proposta.

Refutadores adicionais: a suíte L1 acusa dois controles P898 (Bottom/Horizon sob altura automática). O caminho textual novo fica restrito a VAlign ausente/Top; alinhamento vertical diferente preserva a recomposição/deferimento legado. linebreak-baseline.json.gz também mostra ABC/Align(DEF)/GHI em y20/35.507418/43.864848 contra20/41.557438/63.114878. Ao entrar Align textual depois de linha material, usar fundo/prev baseline causal + par.spacing, em vez do cursor já preparado para a próxima linha; depois de outro frame de bloco, consumir sua margem pendente pelo protocolo existente. Sem duplicar a margem no término.


## Align com corpo Grid/Table — 2026-10-06

### Medição anterior à decisão

A matriz exploratória `lab/results/construction/diagnostics/bordas-cantos-2026-10-06/verification.json.gz`
conserva os 12 controles, comandos, fontes, HEAD/working tree/UTC e binários:
Align(left) põe o texto inicial em y16,642578pt no Tekt contra y24,999978pt
no vanilla pinado 9f2b6e871. Uma coluna RTL ainda começa em x140pt contra
x20pt, embora Left seja físico. `placement.rs:81-106,270-275` instala override
e origem geométrica apenas para o certificado textual; Grid/Table seguem
origem de baseline e subtração de ascender. `grid/tracks.rs:213-223` já calcula
topo do grid a partir do estado inicial. No oracle `flow/collect.rs:193` e
`flow/block.rs:58-70`, alinhamento do fluxo e tamanho geométrico do bloco são
observáveis separados de baseline. Não inferir intenção histórica.

### Responsabilidade exclusiva

Para largura finita, VAlign ausente/Top e corpo que seja exatamente uma Grid
ou Table, opcionalmente em wrappers transparentes Styled/Located/Realized ou
Sequence de um único filho, usar a mesma origem geométrica e override lexical
do caminho textual. Restaurar override antes da recomposição. A resolução
das colunas pertence a `00_nucleo/prompts/core/compiler/layout/grid/tracks.md`.
O envelope ocupa a largura disponível para evitar uma segunda translação
horizontal do conteúdo já ancorado; Y local é geométrico, sem descontar font
ascender. Conservar o protocolo existente de margens causais de fluxo.
Bodies mistos, regiões infinitas e VAlign Bottom/Horizon conservam o caminho
legado; não alargar o certificado por ausência de prova negativa.

Correção interna de paridade ADR-0127, sem API/default/fase/unsafe novo.
Testar equivalência de topo de Grid direto vs Align(Left,Grid), Left físico
sob RTL, Right/Center de uma e duas colunas, scoping e posição após Align.
Os casos de gap/nesting são refutadores da hipótese de que o protocolo textual
existente também governa esse frame, não sucesso presumido.

## Iteração e parâmetros internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/placement.rs` nas linhas 722, 766 contém os avisos selecionados do Clippy. O snapshot anterior à decisão, spans conferidos contra a fonte, owners, referências, binário, UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, status e diffstat estão em `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`. A inspeção mede mecânica interna e usos; não é nova prova bilateral de paridade ou ganho de desempenho.

### Decisão e aceitação

Os dois helpers pub(super) de fixup horizontal/vertical recebem uma fatia mutável de FrameItem em vez de exigir Vec. A visibilidade continua limitada ao módulo layout; nenhum entrypoint público muda. Os helpers não alteram comprimento/capacidade. Preservar mem::take da fila, resolução de path/count, ordem das entradas, delta incremental, eixos e fórmulas. Chamadores internos com Vec continuam coercidos à fatia; não mudar armazenamento ou recomposição.

Refatoração interna equivalente em fluxo contínuo ADR-0127, sem contrato público, default ou fase novos. Conservar as expectativas dos testes existentes; não criar testes que apenas espelhem estes loops. Validar suíte core (incluindo conflitos, spans, fixups, marcadores e seleção diferencial), todos os targets, build, documentação, Clippy e linhagem com resselo restrito. Nenhuma supressão adicional de lint é autorizada por este lote.

## Origem geométrica de Place em região local — 2026-10-14

### Medição anterior à decisão

place-origem-local-2026-10-14/{before,extra-before}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Box-place publica shape em y37.75pt contra28pt no canvas480×400 (bbox raster aproximado); no SVG fonte, box sem margem do corpus tiling tem y14.834 local contra5. O deslocamento cresce com fonte. placement.rs:463 veta origem geométrica dentro de qualquer subframe e :533 conserva o ascender local; place.rs:96–131 soma text top-edge mesmo para Top explícito. Root-text shape diverge29.5/22.25pt; Box-text32/22.25; Grid e Align são controles positivos. Fonte pinada flow/distribute.rs:703–715 separa align_y explícito de offset de fluxo. Isso é geometria de linguagem; não obrigação de buffers Rust. A hipótese é refutada se texto, VAlign, scopes, fixups ou checkpoint perderem coordenadas após origem única.

### Responsabilidade exclusiva

Compor todo body de Place não flutuante por layout_sub_frame_at_geometric_origin. Não subtrair ascender nem cell_origin_y do body local zero. No pai subframe, origin_y é sub_frame_origin_y; raiz e célula conservam regiões existentes. Cálculo de size/align/diferimento e translações de metadados permanecem; nenhum teste de presença de texto decide origem do body.

### Aceitação e etapas

Esta seção supersede a restrição histórica de body geométrico apenas na raiz de Place. L0 primeiro e RED→GREEN em fluxo interno ADR-0127: sem campo público/trait/assinatura/fase/default novos. Comparar antes/depois SVG/PDF e preservar corpos textuais, mistura, páginas automáticas, clipping, posições observáveis e floats; build/workspace/rustdoc/fmt/linhagem/inventário.
A origem do body e a âncora após fluxo comitado são defeitos distintos. Esta etapa A não fecha âncora depois de Shape/Parbreak/Linebreak nem inset de Block, já refutados pelos controles. A etapa B de place-origem-local completa o fim causal do fluxo e a região de Block; não declarar Place universal pronto enquanto B estiver aberta. O transporte público de Tiling permanece pendente de aprovação em seu pacote, sem alteração por esta correção.

### Controle Align: compensação acidental identificada antes da correção

`green-attempt1.log` registra a primeira implementação: os três testes vermelhos passam, mas Align(Box(Place(shape))) desloca y15 para6.2 em FixedMetrics (origem de ascender8.8). `placement.rs` compõe Align legado retirando sub_origin_y; `boxed.rs:467–477` e `sub_frame.rs:300–320` já colocam a caixa geométrica na origem zero após acomodar o ascent da linha. Antes, o strut indevido de Place compensava essa subtração. O controle bilateral `before/results.json.gz` estabelece y15 para este caso no pin; não autoriza preservar erros que se cancelam.

Compor também o body de Align em origem geométrica, sem subtrair ascender. Manter decisões flow_aligned (largura preenchida, gaps de parágrafo), alinhamentos, recomposição e fixups. Texto deve instalar a própria baseline. Aceitação inclui controles de Align com texto, formas, caixas, células, altura automática e suíte completa; qualquer regressão exige medição, não reescrever expectativas para o resultado novo. Isso remove convenção interna de origem para esse caller, sem mudar API pública ou fase.

Controle de célula: attempt2 conserva root/Box mas Grid retorna y16.2 em vez de15. cell_body.rs:218 compõe em subframe legado e :299 recompõe body_y + (local_y - local_start_y). Assim, o Y de ancoragem dentro da célula também é sub_frame_origin_y, não body_y absoluto; dimensões e origem X já absolutas permanecem. Fora de subframe, conservar cy. A forma geométrica local não subtrai cy. Esta medição esclarece o referencial previsto no núcleo antes da correção.

### Origem X do subframe — testemunho adicional

Os registros baseline-expanded/candidate-expanded mostram Place dentro de Align/Align duplo em x0.25pt contra20.25pt no vanilla, tanto antes como depois do reparo vertical. `layout_align` inicia o body em line_start_x absoluto e recompõe só delta_x; o ramo subframe de layout_place usa zero e perde essa origem. Usar `regions.current.line_start_x.0` como origin_x nesse ramo (Column/Parent), preservando o caminho de célula e o body local zero. Box/Transform já iniciam essa referência em zero. Adicionar RED de Align simples/duplo com Place e conferir o corpus de altura auto. Não alterar resolução de alinhamento horizontal ou largura do envelope.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Ao concluir Align, registrar target_y+sub_h como fim físico, com cursor preparado atual; não capturar a tinta de Place nem usar limite de página como fim do body.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.

## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

Resolver região local de Block antes de compor o body, sem herdar a região no sublayout do body. Aplicar flow_y somente quando v ausente. Preservar seleção cell/subframe/page fora de Block. Altura auto mantém fixup para resolução pelo owner Block; translação de itens/decorações/metadados é única.

### Aceitação e incompletude explícita

Correção interna/de paridade ADR-0127, sem campo público, modo ou fase. Este adendo substitui somente as fórmulas incompatíveis de região/width/clip anteriores; não legitima outros reparos sem medição. L0 primeiro, RED→GREEN, validação bilateral, suíte e resselo restrito. B2a cobre região local e clipping na região sobrevivente. B2b completa fragmentação de Block, dimensões/outset e integração dos metadados/decoradores que controles adicionais refutarem; B3 continua responsável por baseline tardia de inline Box/Transform e produtores ainda não certificados. Não declarar objetivo completo com essas frentes abertas.

B2a conserva também uma lista privada de faixas de decoração associadas ao índice do fixup Y de Place auto. O hub armazena; checkpoint clona/restaura; subframe e Block isolam/restauram; transição limpa; placement registra somente os segmentos emitidos pela ocorrência; Block aplica a mesma delta da geometria antes de encerrar o escopo.

## Observação dos grupos de pintura — T1b — 2026-10-14

Medição anterior à decisão: layout-first.log em paint-root-2026-10-14
registra testes que procuram Text somente em page.items e não encontram
as folhas dentro da nova raiz de Block. before-extra-test-owners conserva
fonte/L0, HEAD/UTC/diffstat antes deste ajuste. O contrato de Group.paint_root
foi aprovado no manifesto d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d.

Neste consumer alterar somente o observador de teste afetado para percorrer
a árvore e projetar as coordenadas por Group.pos/matrix, usando o helper
de teste de layout. Conservar conteúdo, valores esperados e tolerâncias.
Nenhum algoritmo produtivo deste owner muda. O owner do helper é
00_nucleo/prompts/core/compiler/layout/tests.md; sua visibilidade interna para
testes irmãos não é API do produto. Revalidar os testes e os renders do lote.


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

Depois de layout_align, publicar a largura causal devolvida pelo subframe na origem lógica anterior à translação visual. layout_place permanece fora do fluxo: apenas a restauração do subframe conserva a extensão do pai.

### Aceitação e limites

Correção interna/de paridade ADR-0127: nenhum contrato público, default ou
fase novo; L0 primeiro, RED→GREEN, resselo restrito. A largura é uma parte
da correção B2b/T1–T3 já em andamento: altura Auto/backing, alinhamento tardio,
fracionários, fragmentação, outset e PDF Parent continuam com critérios próprios.
Não declarar render aprovado por testar apenas o carrier. Preservar controles
bilaterais completos e refutadores, suíte/build/doc/fmt/linhagem/inventários.


## Alinhamento geométrico em Block Auto — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/block-auto-alignment-2026-10-14/before/results.json.gz`
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

Para Align geométrico (fora da rota textual/grid flow_aligned), tratar largura de Block Auto como ainda não resolvida: posicionar inicialmente usando content_w e registrar fixup X existente. Preservar transferência de subframe, metadados e largura causal. Alinhamentos de linhas textuais/Grid mantêm a rota anterior nesta etapa.

### Classificação e limites

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem contrato público,
modo/default ou fase novos. L0 primeiro, RED→GREEN, resselo restrito e suíte.
Esta etapa completa o refutador geométrico da largura causal; não completa
alinhamento textual/matemático, decoração externa diferida, Place horizontal,
altura Auto ou fragmentação. São pendências de B2b/T3, sem crédito de paridade.


## Altura causal de Align — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/align-flow-before/results.json.gz`
conserva HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, UTC/diffstat/status/hashes/fontes/binários e PDF de40 documentos
contra pin9f2b6e871. Block200 seguido de Align(short) gera página230pt
contra250.438pt em Top/Horizon/Bottom; antes de Align, texto simples deixa
1.188pt extra. Com fonte10/top10 e Block20, páginaAuto termina40pt contra62pt.
Página fixa100 conserva baseline correta em Bottom, mas Top fica12pt acima
e Horizon6pt acima. Align de Block20 vazio precisa consumir sua alocação;
Align vazio preserva apenas a margem entre irmãos. Fonte pinada flow/collect.rs:
178–185 separa espaçamento de parágrafo de linhas; :245–285 aplica alinhamento
e margens sem apagar o consumo físico do frame. Geometria, presença e ordem
são observáveis da linguagem, não equivalência de estruturas Rust.

placement.rs:127–146 restringe gap a flow_aligned (um certificado horizontal);
:407–422 restringe publicação do fim a material textual/Top. page_geometry.rs:
112 prioriza o fundo do bloco anterior, por isso o fixup vertical reposiciona
contra uma página que não contém o consumo de Align. O controle P898 antigo
supunha ausência de margem depois de Block; a sonda bilateral refuta isso.
A descoberta de Source não reconhecido pelo certificado horizontal é um
eixo separado: esta correção não alarga esse certificado nem muda RTL.

### Contrato exclusivo

O consumo vertical de Align independe do certificado horizontal e do eixo
Top/Horizon/Bottom. Depois de obter o subframe uma única vez, se sua altura
causal for positiva, estabelecer a entrada pelo fim causal da linha anterior
mais par.spacing, ou pela margem pendente do bloco anterior, respeitando
mudança de página. Uma região exterior recém-aberta com baseline preparada
precisa recuperar a origem física uma vez; conservar a base local dos
subframes. O fitting reabre na origem física da nova página.

Publicar o fim do frame positivo em last_block_descent_y e no protocolo de
bloco existente, separado de below pendente. Assim uma página Auto contém
a altura causal antes do fixup e o irmão seguinte não reutiliza o bloco
anterior. Em página finita, Horizon/Bottom continuam consumindo o restante:
a fronteira publicada deve conservar esse consumo. Altura zero não inventa
consumo ou margem. Place fora de fluxo conserva sua rota; a sonda de Align
contendo somente Place ainda acusa deslocamento próprio, sem certificação.

Manter medição de largura, estratégia horizontal, escopo e remapeamento de
items/decorações/fixups. Sem campo público, trait, modo ou fase novos;
correção interna ADR-0127. L0 primeiro, RED→GREEN, corpus com continuação,
corpos vazios/de altura declarada, páginas Auto/fixas e resselo restrito.


Refutador de continuação no mesmo corpus: tail21/25/29 colocam X e POST nas
mesmas posições no pin, qualquer que seja Top/Horizon/Bottom. Em páginaAuto
não sobra altura livre para alinhar um frame de fluxo contra os irmãos que
vêm depois. Para frame positivo, usar seu próprio consumo no fallback Y e
não registrar recomposição contra a altura final da página inteira; essa
altura inclui irmãos, não folga desse frame. Conservar os fixups de Place
e os de subframes transferidos com seus escopos. A página finita e a célula
continuam usando suas regiões conhecidas. Verificar posição de X e POST,
não somente altura final. Esta cláusula substitui a expectativa de deferir
Align positivo sob alturaAuto descrita acima e em P898.


### Refutadores de origem e fundo por página

align-flow-after/results.json.gz mantém a proveniência completa. edge40/44
mostram que V20 inicial adquire par.spacing13.2 indevidamente; edge41/45
mostram páginaAuto seguinte conservando60pt do Block da página anterior,
embora X só precise27.238pt. A linha fechada e o V5 após texto já coincidem.
placement.rs guarda max(last_block_descent_y, frame_end), permitindo vazamento
de uma região encerrada. A testemunha inicial de VSpace não implica um
parágrafo anterior quando initial_baseline_pending ainda é verdadeiro.

Para o frame positivo, publicar seu fim corrente diretamente. Usar testemunha
de linha anterior somente quando a primeira baseline já foi adquirida;
um espaço forte na origem física não inventa par.spacing. Preservar V forte
após texto e os demais casos positivos; testar os dois refutadores RED→GREEN.
