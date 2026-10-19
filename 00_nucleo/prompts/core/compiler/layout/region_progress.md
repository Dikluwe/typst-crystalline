# L0 — region progress
Hash do Código: 8fecc2b1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:8d6cb7ee2458f368648c0ea6cfa84386dc506cf7273a16aa42bfbcbfeff05592
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:42175932c75f37ed19205d1d82526969999e18fba8fed16f1b1dcff6e418bc0f
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:04f91fc931871165d4fef399ea9c2e21e8b75f7fd57031183a4e2aed0ed1b62a
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/region_progress.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `lab/results/construction/diagnostics/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `lab/results/construction/diagnostics/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Own the implementation of next_region and the existing post_line progress predicate. next_region preserves flow-suffix acceptance before any mutation; column close/next/merge ordering; auto dimensions before draining admissible floats and footnotes; page_commit; physical-page margin/binding resolution; next cursor/baseline/reservation/block-state reset; pending cell tails; and first-column startup. Delegate fitting to Place and draining/geometry/assembly to their current owners. The post_line fallback remains for uncertified overflow; fitting-line and next-line admission semantics are superseded by the measured regional demand correction below. Preserve drain/reset ordering. No future-content scan, IO, new checkpoint or dynamic dispatch is authorized.

## Acceptance

Preserve the existing observable text, positions, styles, pages and errors in the controlled corpus and current suites. Structural tests refute reintroduction of algorithms in adapters. Pure-selection tests discriminate no-wrap, wrap and Unicode-safe prefix decisions; measurement tests retain overrides and shaped/fallback paths. Each consumer is owned exclusively by this prompt. This is internal atomization, not completion of the audit findings.

## Regional demand correction — 2026-09-28

### Evidence before decision

The 30 bilateral controls in `lab/results/construction/diagnostics/estudo-correcao-falhas-texto-2026-09-28/` measure terminal pages 2/1, explicit terminal pagebreak 3/2, and continuation 3/2 (Tekt/vanilla). The isolated pre-export control emits Four/additional on page 2 with baseline 101.84pt and page height 100pt. The causal sources are line_commit.rs:657-696 (hypothetical next baseline), region_progress.rs:113-128 (post-line transition), cursor.rs:80-135 and parbreak.rs:41-80 (paragraph baseline/spacing). Current source and L0 hashes precede edits in `lab/results/construction/diagnostics/correcao-demanda-regional-2026-09-28/initial.json` and `before/`. This is internal parity correction under ADR-0127, not a public contract/default/pipeline change.

### Decision and acceptance

Own admission of the next text line through a private static descendant function. For finite geometry, advance before emitting a new material line when the predicted baseline exceeds the effective bottom, the pending line has no material, and the current region has material. Do not advance an empty region repeatedly or acquire metrics to decide that predicate. Cursor invokes this only after resolving initial/block-chain baselines; paragraph and fractional-tail collapse keep their current owner. post_line retains its rejection/transition fallback only for a line not certified to fit by LineCommit; a fitting committed line never advances just because its predicted next baseline overflows. Preserve suffix acceptance, column transition, floats/notes/tails and auto dimensions. No new checkpoint/state field, future-content scan or dynamic dispatch.

### Integration controls preceding the refinement

The first implementation's 36 real-font controls are preserved in first-after-pdf/ and first-pdf-green.log (before any later rebuild). terminal-columns still loses Three; terminal-footnote changes from 2 to 1 pages and its raster shows note/main overlap. Column start returns before clearing previous block spacing; terminal note fitting uses cursor_y and must retain the conservative following-line clearance rather than the terminal geometry cursor. These measurements require the refinements below, without changing the footnote body format, reservation model or public contract.

After start_next_column, clear last_flush_advance, prev_block_equation_descent, prev_block_below_pending and block_chain_active exactly as a physical-region transition does. Do not let another word rebuild a baseline from the closed column. The private reset belongs to this owner and is shared by the two transition branches.

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
em `lab/results/construction/diagnostics/subframe-isolamento-2026-10-13/before.json.gz`.

### Responsabilidade exclusiva e aceitação

post_line e admit_next_text_line não pedem transição nem rejeitam a transação externa durante is_sub_frame. next_region e o protocolo explícito de páginas/colunas permanecem intactos.

Os invariantes compartilhados pertencem ao núcleo subframe-overflow pinado.
Correção interna ADR-0127, sem campo/assinatura pública, fase ou novo default.
Exigir RED→GREEN antes/depois das condições, preservação da paginação
ordinária externa, suíte core, build, formato e linhagem. Não elevar a
altura de teste para esconder overflow nem descartar itens do frame.
A correção do helper não equivale a terminar a composição de block/box;
a coleta, dimensões e recorte desses containers continuam nos seus owners.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Limpar flow_placement_end em reset_closed_region_flow, tanto página como coluna, sem mover ownership de transição.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.

## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

Invalidar block_place_region ao fechar página/coluna, sem restaurar limites do fragmento anterior.

### Aceitação e incompletude explícita

Correção interna/de paridade ADR-0127, sem campo público, modo ou fase. Este adendo substitui somente as fórmulas incompatíveis de região/width/clip anteriores; não legitima outros reparos sem medição. L0 primeiro, RED→GREEN, validação bilateral, suíte e resselo restrito. B2a cobre região local e clipping na região sobrevivente. B2b completa fragmentação de Block, dimensões/outset e integração dos metadados/decoradores que controles adicionais refutarem; B3 continua responsável por baseline tardia de inline Box/Transform e produtores ainda não certificados. Não declarar objetivo completo com essas frentes abertas.

B2a conserva também uma lista privada de faixas de decoração associadas ao índice do fixup Y de Place auto. O hub armazena; checkpoint clona/restaura; subframe e Block isolam/restauram; transição limpa; placement registra somente os segmentos emitidos pela ocorrência; Block aplica a mesma delta da geometria antes de encerrar o escopo.

## A2 — prefixo admitido e região inline de Block (2026-10-18)

### Medição anterior à decisão — admissão e continuação

pagination-block.json.gz e block-freeze.json.gz em
lab/results/construction/diagnostics/bibliography-structure-2026-10-18 preservam estado completo,
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

Após criar a próxima coluna na mesma página, ou concluir o início da primeira coluna/página física, reinstalar o contexto inline de Block se Some, pelo helper do owner Block. Sem contexto, conservar o fluxo anterior. Fazer isso depois de estabelecer a geometria natural da região e antes de emitir a próxima linha. Não alterar ordem de drains, reset, floats ou aceitação do suffix.

Aceitação: RED→GREEN de prefixo/corpo na mesma página e inset nas continuações, inclusive saída para OUTSIDE, simple/optimized e controles existentes. Medir PDF90/150/250, RTL, prefix-only/formatado, cores e regressões; build, suites/doctests/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial Tekt. MemoryMax12GiB/High8GiB/swap0/jobs1; sem publicação. Hanging genérico/overrides e múltiplas bibliografias continuam frentes abertas.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada,
status/diffstat/UTC e2099 fontes/L0s. before.json.gz contém92 pares;
extra-before.json.gz contém26; boundary-before.json.gz contém10 e
repeat-before.json.gz contém4. Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3,
binário9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt inicial2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095.
Em160pt/margem8/font12, hanging10 mantém a primeira linha x8 mas move
continuações x8→18; negativo-5 exige x3. RTL mantém primeira borda direita152
mas continuações vão para142. O Tekt ignora o comprimento. MLA tem default
18pt (1.5em), set ambiente3em não o troca, show-set3em exige36pt; IEEE/Plain
não adquirem recuo de hanging por esse show-set. bibliography.where(style:"mla")
não casa na sonda vanilla: conservar a decisão do seletor, não forçar override.

Block/Box com body apenas inline ignoram hanging10; Par explícito dentro
deles e Block com Parbreak aplicam. Heading/Raw/List/Quote simples não o
aplicam. Declaração parcial, zero, repetida e escopo vazio fecham fronteiras:
FIRST/MIDDLE/LAST em5.168/27.464/49.76, contra todos5.168 no Tekt. A primeira
sonda lexical usou colchetes literais: foi refutada, corrigida para #[...] e
preservada como extra-before-initial; não é evidência de escopo Styled.
Valores extremos geram tinta recortada pelo extrator: não equiparar ausência
em pdftotext a apagamento de Content. Fonte parcial24pt não muda o em do
comprimento compartilhado12pt, como partial-font registra.

Fontes pinadas: typst-realize/src/lib.rs:1061–1085,1190–1219 distinguem
fragmento/Par e interrupção; typst-layout/src/flow/collect.rs:72–134 escolhe
fluxo, inline/mod.rs:174,216–221 e collect.rs:141–147 resolvem forma;
inline/line.rs:496–503 posiciona; rules.rs:510–523 faz inset/compensação CSL.
typst-library/src/model/bibliography.rs:112–127,367 determina o default e
precedência. Isso mede língua/geometria; não exige a estrutura Rust vanilla
nem atribui intenção histórica. Refutação por fronteira perdida, em de run,
recuo duplo, reset em página, fallback greedy imposto, cópia perdida, callbacks
reexecutados ou regressões dos controles impede fechamento.

### Classificação e limites

Correção interna/de paridade ADR-0127, fluxo contínuo: sem campo ou assinatura
pública, flag/default externo novo, fase ou ordem de realização diferente.
A nova forma inicial/continuação supersede somente a limitação correspondente
de largura constante; RTL latino certificado segue as condições do núcleo,
sem BidiCursor ativo/tracking e sem inventar BidiSpan. Os demais limites
anteriores continuam. Os valores/cópia fora da página requerem inspeção dos
frames ou SVG, não igualdade de extração recortada. Não antecipar R3-T1.

### Responsabilidade exclusiva deste owner

Depois de instalar a geometria real da nova região/coluna, reabrir e reaplicar a origem horizontal da linha conforme State; conservar contador inicial/continuado. Não criar página por recuo, duplicar inset ou copiar largura da região anterior.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.

## Faces mistas, fronteiras e regiões — refinamento medido 2026-10-18

### Medição antes da decisão

Os registros first-after/first-bib-after/first-regions-after e
candidate1-sources em mixed-style-metrics-2026-10-18 preservam o candidato
com apenas line_commit alterado e os sete testes verdes. audit-candidate1
mede y SVG igual no título dos seis casos bibliográficos e na mistura
itálico→romano, mas forced ainda difere0.156. linebreak.rs:46–58 acrescenta
ExplicitLinebreakBoundary com Text vazio de estilo exterior; esse filho é
causal para bidi, não uma face material.

Os20 controles regionais PDF preservam os mesmos binários/pin e fontes do
registro before-sources. Na altura39.44pt, o Tekt põe ITALIC na página2;
o vanilla o conserva na página1. Em55.14pt forced usa2 páginas vs1. Em70pt
wrap distribui3+1 linhas vs2+2. O primeiro candidato corrige y das linhas
wrapped, mas não essas decisões regionais. A tentativa SVG sem template
de páginas foi preservada em regions-before: o oracle recusa saída múltipla
sem padrão; não é prova SVG de paridade. Para regiões, usar os PDFs válidos,
e o script regional futuro solicita somente PDF.

region_progress.rs:83–88 instala baseline com uma face mas não atualiza o
ascent assumido. :143–152 admite só pela previsão, antes do texto real.
paragraph_pagination.rs:43–59 recusa arestas diferentes, desligando a
proteção de custos positivos, embora a seleção de ranges já seja conhecida.
Fonte pinada flow/collect.rs:196–239 protege primeiros/últimos frames com
suas alturas reais; distribute.rs:343–364 avança o grupo apenas quando cabe
na próxima região. A geometria/palavras/páginas são linguagem; formato do
Profile e IDs SVG são mecânica. Refutar por salto precoce, recontagem,
callbacks novos, métrica por Group, alteração de largura/seleção/leading
ou regressão de colunas/notas/float/NoTypography.

### Classificação

Refinamento privado de paridade ADR0127 contínuo: sem contrato público,
flag/default do produto ou fase/ordem de callbacks nova. Preservar caminhos
recusados/escalares anteriores. A medição amplia apenas a certificação
regional de texto preparado com mesmo size/leading, baseline_offset zero,
arestas finitas não negativas e faces com tops/descents diferentes. Tamanhos
ou leading variáveis continuam fora dessa certificação, sem alegar paridade
completa nesses casos.

### Responsabilidade exclusiva deste owner
Possuir enum privado Copy de admissão Generic/Text/KnownTop e a fórmula de candidato regional. A condição original externa continua protegendo subframe, infinito, linha já material e região vazia. Só se a previsão excede o bottom calcular o candidato: Generic usa a previsão existente sem consultar fonte; Text mede a face corrente já resolvida, KnownTop usa o top certificado fornecido; nestes dois, candidato=cursor_y+top-real-ascent-assumido (None neutraliza a diferença; Some(0) é válido). Avançar apenas se esse candidato também excede. Não alterar cursor/itens só para testar fitting. Ao instalar baseline de página física, guardar Some(top) da face usada; se baseline continua pendente, guardar None e não consultar fonte. Colunas mantêm seu protocolo próprio e nenhum novo estado persistente é criado. Testes por APIs atuais: próxima linha de menor top que ainda cabe e baseline da primeira linha após transição com face diferente.

Aceitação: RED→GREEN das quatro regressões adicionais e dos controles existentes, matriz30+20 regional PDF, corpus132+94+21, suíte ampla/Clippy/Rustdoc/linhagem e12GiB/High8GiB/swap0/jobs1. Registrar as tentativas sem ocultar falhas.
