# L0 — sub frame
Hash do Código: 41c1486b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:64d61513bab0293f918958404cf4fe0387dea84cc53f5442faad2b467fa4e077
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:ecf965471eedd8599209f0575811dd5b456dce1c43baa26881251e24e8ec6c6d
- 00_nucleo/prompts/_nuclei/layout/block-inline-region.toml sha256:8d6cb7ee2458f368648c0ea6cfa84386dc506cf7273a16aa42bfbcbfeff05592
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:dd8c7661f2246385e2aed1737e8494d778b9f563b7ac709fc3c750a23fe50596
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:42175932c75f37ed19205d1d82526969999e18fba8fed16f1b1dcff6e418bc0f
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:c983fdb9884f90548e110fe7c1709632fc00b6e2a1387ad9e80444009c3602fd
- 00_nucleo/prompts/_nuclei/layout/subframe-inline-extents.toml sha256:d9b9c00602c2c68bbe82eb37a84cc01a4e0575bf84f5df8950b86b7f4dab7dd3
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:04f91fc931871165d4fef399ea9c2e21e8b75f7fd57031183a4e2aed0ed1b62a
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/sub_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/sub_frame.rs:29`, `01_core/src/compiler/layout/sub_frame.rs:65`, `01_core/src/compiler/layout/sub_frame.rs:329`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

The isolated path temporarily installs a SubLayoutRegion and preserves/restores the existing outer cursor, dimensions, line/emitted buffers, page margins, pending baseline/collapse metrics, subframe/unconstrained flags, collectors and validated-prefix one-shot arm in LIFO order. Clear the outer arm locally so a body flush cannot consume it. Return the local items/height/decoration segments plus orphaned horizontal/vertical alignment paths without applying a second parent origin.

The inline path dispatches the body and drains only its pending-line tail; optionally align that tail while preserving the parent's prefix. It is not an all-state isolated path: callers such as Box own additional fraction/repeat isolation. Preserve the existing inline assertion that origin_x and width are zero and the present height fallback for material non-text items.

Direct private HTML markers are retained and returned in order but do not contribute visual had-items, fallback height or decoration metrics. Existing Semantic/Link/Group wrappers retain their old material classification. This module neither extracts ids nor removes carriers; `00_nucleo/prompts/compiler/layout/html_frame.md` owns both. No independent page dispatcher or checkpoint is introduced.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/p1431_line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `lab/results/construction/diagnostics/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

Provide a private geometric-origin path for root Place without a cell anchor or material pending line. It shares the isolated save/restore protocol, starts at y=0 with the initial text baseline pending, and defers typography to actual text. A first non-text block consumes this pending origin geometrically without font metrics. For a pending text tail, measure its height from the geometric top to the final text bottom, without subtracting the first top edge twice. Keep the existing baseline-origin path for all other callers and nested/cell placements.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`lab/results/construction/diagnostics/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

No percurso isolado, salvar/clonar State de tracking junto ao buffer exterior e restaurá-lo quando restaurar a linha. O alinhamento da cauda inline conserva a proveniência do body já emitido; não a substituir pela do prefixo durante a troca temporária usada para alinhar.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.


## Sonda de testes — 2026-10-05

O experimento do owner `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md` exige contar invocações do subframe. Permitir chamada de sonda cfg(test) da suíte layout/tests na entrada isolada; nada é emitido no build produtivo. Não alterar o comportamento do subframe neste owner.

## Direção de parágrafo distinta de direção de run — 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/direcao-alinhamento-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes, binários e treze sondas. rtl_lines põe ABC em x=20 no Tekt e x=152.949220 no vanilla pinado; rtl_inline_only e rtl_scope coincidem à esquerda. ltr_lines põe árabe em x=174.567870 no Tekt contra x=20 no vanilla. Em `cursor.rs:156–162` e `line_commit.rs:210–216`, a âncora depende do primeiro item da linha. Vanilla `flow/collect.rs:129–136` obtém trunk_from_pairs e `inline/mod.rs:189,232` resolve direção/alinhamento compartilhados antes das linhas. Semântica observável: direção dos trechos e âncora de todas as linhas do parágrafo. Estratégia de state/preflight é mecânica interna, não cópia do vanilla. Inferência: preservar direção compartilhada evita perder a âncora em linhas só com override; refutada por controles integralmente estilizados/parbreak/wrap.

Correção interna/de paridade ADR-0127, fluxo contínuo L0 primeiro e RED→GREEN. Apenas campos privados de Layouter e helpers descendentes, nenhuma entidade ou assinatura pública/fase/default novo. Esta rodada certifica conteúdo textual estrutural (Text, Empty, Space/SpaceFromNewline, Linebreak, Sequence, Styled e Strong/Emph); unidades desconhecidas conservam fallback anterior. Não prometer ordem bidi mista, align explícito, HTML ou todas as formas de parágrafo. As equações existentes de largura/tracking permanecem.

Responsabilidade deste owner: Salvar e limpar os dois slots de direção na entrada de região independente e restaurar LIFO com a linha pai, evitando que a direção de parágrafo pai comande o texto da célula/box/align. Preservar coordenadas e demais scopes.

Aceitação: RED→GREEN em parágrafo RTL com linha LTR e inverso, escopo integral e reset por parbreak; checar checkpoints e subframes, texto/posições em CLI bilateral, build e lint sem novos findings. Slots e helper não transportam metadados ao L3; divergências de ordenação bidi ou align explícito ficam registradas separadamente.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.

Guard privado bidi_scope_active distingue ausência de cursor certificado de ausência de escopo; após recusa integral, filhos não certificam um subconjunto parcial. Checkpoint e subframe conservam/suspendem/restauram este guard.

## Consumidor textual Align — 2026-10-06

Medição anterior à decisão: baseline-verification.json.gz em `lab/results/construction/diagnostics/block-align-2026-10-06/` reproduz Y deslocado por um ascender em Align. `placement.rs:35-46,176-177` compõe duas origens tipográficas. Para o body textual finito certificado pelo owner placement, permitir também o uso do helper geométrico existente e origem zero na recomposição. Outros consumers preservam baseline-origin. Nenhuma assinatura pública ou nova fase. O slot privado explicit_line_alignment pertence ao hub e é herdado, não zerado, por esse subframe; quem instala/restaura o override lexical é placement. Layout isolado continua zerando/restaurando direção de parágrafo e bidi, pois alinhamento não é direção. Testes RED→GREEN/escopo e corpus bilateral refutam a correção; não se afirma suporte geral a Block/Align geométrico ou dimensions auto.

Medição adicional: green.log da rodada tem Center BBB ainda na origem. `sub_frame.rs:220-222` só alinha a cauda manual quando region.align_rtl, embora as linhas anteriores passem pelo cursor. Ao haver explicit_line_alignment, chamar o cursor na cauda isolada também, exatamente uma vez antes de drenar itens. O caminho inline conserva política antiga.

## Célula textual certificada — refutação 2026-10-06

### Medição anterior à decisão

focused-candidate-verification.json.gz e candidate-source.json.gz em
`lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06/` preservam candidato
com largura/âncora corretas mas ordem ABC/123/árabe incorreta em RTL.
bidi_origin.rs:111-117 recusa todas as células/subframes; o input integral
não chega à reordenação L3. cell_body.rs:223-235 classifica Text e TextShaped
como texto para rebase de baseline, mas não TextSeparator, introduzindo
um delta de top-edge nos separadores da mesma linha. São duas causas
internas refutáveis por ordem/baseline ainda divergentes após certificação
limitada e recomposição consistente. Correção de paridade interna, sem
contrato público, fase, flags de produto ou unsafe novo.

### Responsabilidade exclusiva

O subframe isolado suspende cell_bidi_allowed quando já entrou de outro subframe; restaura ao retornar. O caminho inline sempre suspende/restaura. A célula direta conserva a capacidade instalada por seu owner. Guard/cursor do pai seguem o protocolo existente.

## Raw bloco — medição 2026-10-07 antes da decisão

raw-block-spacing-2026-10-07/{before,before-additional}.json.gz e before-state
registram HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/diffstat/status/fontes/comandos/binários. Tekt43207349fba51afbdbcac3847c1a7d884e9ff845b6401022be22272679c00575;
vanilla pin9f2b6e8715237cb086899a42873660fe744622e8 e binário7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Raw prefix/suffix: yMin85.806243/95.064120 contra91.304203/108.594100.
Cell usa x70.86614 contra75.86614; Cell isolada acrescenta0.55208pt vertical.
Unknown-language colapsa abc/def emabcdef. SubstituiçãoABC perde a fronteira.
Show-size15 usa margem18pt no vanilla; show-em1 mantém13.2pt.
block-margins e show-margins above2/below7 são efetivos; par.spacing20
produzRaw98.104163/After122.194100. EmptyRaw temAfter101.90816.
Fonte pinada typst-layout/src/rules.rs:655–679 realiza Raw como Block;
container.rs:324–360 define margem/colapso. Raw consumer fecha linhas e
usamargem global; cursor.rs:88–132 já consome below pendente;
sub_frame.rs:315–325 usa avanço hipotético quando a linha terminal já fechou.
São geometria/texto observáveis, não algoritmo interno ou bytes.

### Decisão — correção interna de paridade ADR-0127

Seguir o núcleo raw-block-boundary pinado abaixo. Nenhum campo/trait/API
público ou fase nova. Raw possui begin/end físicos privados, local origin,
resolução de margens e classificação terminal do produto Raw. Styled orquestra
esses helpers para produto transformado, com markers do apply no Styles
existente. Apply conserva flag block e Length de tamanho/margens dos
overrides sem resolvê-los. Sub-frame terminal Raw mede a borda final real
menos seu topo, não leading/margem pendentes. Testes cobrem região local,
produto substituído e linhas distintas em fallback.

Ao iniciar, fechar prefixo material; usar baseline/descent anteriores e
margem acima colapsada, com baseline inicial geométrica. Primeiro Raw de
sub-frame vazio começa no topo local. Ao terminar, registrar borda inferior
real e deixar below no protocolo existente do Cursor. Nenhuma soma de
margem final à altura terminal. Unknown syntax mantém quebras explícitas
entre linhas de Raw bloco, sem normalizar espaços/tabs nem alterar inline.

### Aceitação e limites

RED→GREEN e recoleta dos14 controles +5 refutadores, tipografia anterior,
Cell/morfologia/math/query, suites/build/lint e preservação restrita.
Geometria observada decide; não introduzir tolerância posterior para apagar
resíduos. Whitespace/tabs/linhas vazias internas, syntax customizada,
reconstrução fresca e rich block styles/HTML continuam não certificados.

### Refutação e revisão da primeira fórmula de margem

after.json.gz/after-additional.json.gz (binário/snapshot/UTC completos)
refutam o primeiro reparo: consecutivos têm gap extra2.64pt; Cell prefix
continua5.49796pt acima; Cell isolada fica2.59598pt acima; margens explícitas
não chegam à chain, pois set.rs:976–1006 não reconhece Native block.
refuters-red.json.gz mostra Strong-cell perdendo altura (After83.27018
contra89.95610); plain-cellAfter87.36012. Também identifica negativos
preexistentes de parágrafo sizes-up/down, cada um comdelta10.8pt.

A margem Raw default é fraca e usa Raw80%:10.56pt, não13.2pt. Um parágrafo
exterior11pt contribui13.2pt, enquanto Raw consecutivo contribui10.56pt.
Usar prev_margin_is_parbreak como protocolo já existente de margem fraca
de parágrafo, incluindo Raw que herda essa margem: below explícito continua
forte e não muda esse ramo. Cursor colapsa margem fraca com par.spacing
actual, sem alterar o ramo de margens fortes, regiões/fracionárias/checkpoints.
Parágrafo anterior já fechado em sub-frame ainda participa da margem acima.
Primeiro Raw de sub-frame usa cursor inicial menos top-edge exterior, porque
cell_body.rs:253–260 recompõe sub-frame legado com ascender-top-edge.
Classificação terminal inclui Strong/Emph transparentes, sem wildcard
descartando dados: é predicado de fronteira, não projeção de enum.

Set proprietário aceita por identidade Native block e aliases somente
o subconjunto spacing/above/below com Length ou Auto; avalia uma vez,
publica delta apenas depois de validar o conjunto. Campos/tipos fora
desse subconjunto conservam fallback/advertência anterior e não são
certificados. Apply transporta essas propriedades e o tamanho como Length.
A margem de render usa o tamanho Raw efetivo (default ou show-set); o
parágrafo anterior contribui o exterior separado. Styles não mede glyphs.

A hipótese é refutável por qualquer negativo que permaneça nas coletas.
O parágrafo sizes-down já era negativo e não é declarado corrigido aqui;
não ampliar para regra de tamanho de parágrafo anterior sem medição própria.

### Região inicial versus estilo Strong do conteúdo

refuters-second.json.gz mantém Strong-cell0.14302pt acima do vanilla,
mas plain-cell coincide até0.00002pt. raw.begin usa o top-edge do estilo
Strong interior para converter a origem legada; cell_body recompõe com o
ascender/top-edge do exterior da região. Sub-frame deve inicializar seu
line_assumed_ascent pelo estilo próprio na entrada, depois de guardar o
ascent do pai. Raw primeiro usa esse ascent regional para converter o topo,
não o estilo Strong/Emph interior. O helper continua restaurando o pai.
É correção de coordenada/contexto privado na fase existente, sem novo slot,
API ou tipo. Aceitação: o refutador Strong deve conservar a mesma origem e
altura do plain, com recompilação e suites/regressões.

### Refutadores finais de fronteira zero/auto e acesso a fontes

edges-red.json.gz prova: below0 sobrepõe After (74.95612 contra82.19410);
negativoAfter72.95612 contra77.19410; auto +spacing20 não sobrepõe auto
corretamente (Raw98.10418 contra91.30420); primeiro below0 deve vencer
above automático do Raw seguinte (def97.99011, não108.55011).
A margem explícita tem prioridade nos dois sentidos; ambas explícitas ou
ambas fracas colapsam por máximo. Auto de edge interrompe o shorthand
spacing e retoma par.spacing/default fraco. Below explícito<=0 deixa cursor
no fim real +below com baseline geométrica pendente; below fraco, inclusive
zero, passa ao ramo fraco do Cursor, que adiciona top-edge do texto seguinte.

suites-third.log refuta inicialização incondicional de FontMetrics: dois
controles place nontext (image/shape) passaram a adquirir font book.
Inicializar ascent regional somente no sub-frame legado cujo produto
terminal é Raw block. Geometric/nontext conserva a rota sem consulta nova.
Dados sem texto não dependem de fontes por esse reparo.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/sub_frame.rs` nas linhas 265 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar somente a varredura que calcula line_leading_pt e nunca é consumida. Não substituir por ela as fórmulas vigentes de altura, baseline ou leading. Preservar isolamento e restauração do estado externo, inclusive last_block_descent_y (Option::take equivale à substituição por None).

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.

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

Manter o protocolo produtivo de save/restore e a interface privada existentes. Acrescentar regressões puras que executem subframes finitos com forma excedente e texto multilinha, preservem prefixo/cursor/dimensões do chamador, retornem todos os itens e não criem páginas externas. Testar origem geométrica e de baseline e conservar o controle de paginação normal fora de subframes.

Os invariantes compartilhados pertencem ao núcleo subframe-overflow pinado.
Correção interna ADR-0127, sem campo/assinatura pública, fase ou novo default.
Exigir RED→GREEN antes/depois das condições, preservação da paginação
ordinária externa, suíte core, build, formato e linhagem. Não elevar a
altura de teste para esconder overflow nem descartar itens do frame.
A correção do helper não equivale a terminar a composição de block/box;
a coleta, dimensões e recorte desses containers continuam nos seus owners.

## Pendentes de linha em região isolada — 2026-10-13

### Medição anterior à decisão

A investigação de composição de Box, registrada em containers-clip-2026-10-13,
exige coletar o corpo completo pelo caminho isolado. A inspeção de
sub_frame.rs:109-121,218-277,317-340 revela que current_line é trocada,
mas pending_fr, pending_fr_natural_widths, pending_repeats e
justify_opportunities não acompanham sua linha. line_commit.rs:86-199
consome seus índices durante o flush; no fim manual do subframe não há
expand_fr_spacings. Hipóteses refutáveis: um flush local consome os
pendentes externos e uma cauda local conserva frações/repeats sem resolver,
vazando-os para a linha restaurada. Prova causal RED→GREEN e snapshot
HEAD/working tree/UTC/status/diffstat/fontes em
`lab/results/construction/diagnostics/subframe-pendentes-2026-10-13/before.json.gz`.

### Responsabilidade exclusiva e aceitação

No caminho isolado, trocar junto com current_line os quatro buffers de
pendentes acima por buffers locais vazios e restaurá-los em LIFO. Resolver
frações e repeats locais da última linha exatamente uma vez antes do
alinhamento e drain finais; linhas anteriormente fechadas conservam seu
fluxo de fechamento. A largura é a da região local. Pendentes externos
não participam do filho e pendentes do filho não se referem a itens do pai.
O caminho inline e seus callers permanecem com sua política existente.

Testar cauda fracionária, repeat, flush interno e preservação do prefixo,
seus índices e sua expansão final, nas duas origens disponíveis. Usar
posições e conteúdo como observáveis, não só ausência de panic. Correção
interna ADR-0127: sem entidade, API, default ou fase novos; L0 primeiro e
resselo. Isso é pré-requisito de isolamento para a composição de Box,
não fecha a geometria dos containers. Build, suites e linhagem obrigatórios.

### Controle adicional e limite da prova

first-candidate.json.gz: o isolamento passou, mas o teste de contagem
externa esperava 14 repetições e obteve 13. O cálculo de repeat no
line_commit.rs:189-195 usa apenas share da fração após descontar a peça
inicial da largura restante. Essa fórmula não pertence ao reparo de
isolamento. O teste deste owner compara também a linha externa finalizada
sem entrar no subframe, mantendo as coordenadas absolutas da borda direita
e os asserts de não interferência. Não alterar o esperado para 13; registrar
a diferença de preenchimento como questão separada, cuja classificação
exige medir o contrato de fluxo de Repeat antes de mudar a fórmula.
A expansão do repeat local sem fração explícita continua exigindo oito
peças completas e posições calculadas pela largura da região.

### Refutação do enquadramento simples de Repeat

repeat-controls.json no mesmo diagnóstico compara três fontes sem
subframe. No vanilla pinado, Repeat ocupa a região e o texto seguinte
passa para outra linha; no Tekt ele permanece inline. O L0 de Repeat ainda
especifica uma faixa inline pendente. Logo a expectativa de 14 peças,
isoladamente, não prova que somar a peça ao share corrigiria a linguagem.
Não legitimar esse reparo por este owner. A investigação de Repeat deve
classificar o fluxo e a largura nos seus owners antes de decidir; os
asserts deste lote provam apenas isolamento e expansão local já definida.
Esta qualificação documental não altera o código produtivo ou seus testes.

## Extensões da cauda isolada — composição de Box, 2026-10-13

Medição anterior à decisão: sub_frame.rs:120-165 guarda line_assumed_ascent
mas herda extensões inline externas; :270-310 drena a cauda sem acomodar
seu ascent e calcula o fundo usando fonte fallback mesmo quando a cauda
só contém grupos. line_commit.rs:402-438 já distingue ascent assumido de
ascent real. Hipótese para testes: uma caixa alta no subframe final pode
ficar acima da origem e a altura automática adquirir descender tipográfico
sem texto direto; a extensão do pai também pode contaminar a medição.
Snapshot em box-composicao-2026-10-13/before.json.gz.

Guardar/zerar/restaurar line_inline_ascent/descent e reiniciar ascent
assumido local (conservando a regra Raw legada). Antes de coletar decoração
e drenar a cauda, acomodar a extensão inline excedente deslocando somente
a cauda e sua baseline. Medir fundo com descent inline; aplicar bottom-edge
tipográfico somente se houver Text/TextShaped/Glyph diretamente na linha.
Não fabricar strut tipográfico para Group sem texto direto. Preservar o
percurso de linhas já fechadas e o protocolo de regiões/pendentes, os
controles Raw e o caminho inline. Testes de caixa aninhada medem dimensões
e origem; sem API/entidade/fase nova, correção interna ADR-0127.

## Resultado de extensões inline — 2026-10-13

Medição anterior à decisão: core-first.log em box-composicao-2026-10-13
registra falha de measure_preserva_baseline_causal_de_texto_box_vazio_e_composto.
measurement.rs:509-512 lia line_inline_ascent/descent vazados pelo subframe;
a restauração correta do pai removeu esse canal implícito. O esperado
17pt para caixa 20pt com shift 3pt permanece correto e não será relaxado.
A fonte/owner e o estado anterior estão no snapshot before.json.gz desse
lote. Correção privada ADR-0127, sem campo/assinatura pública ou fase novos.

Antes de restaurar ascent/descent do pai, publicar as extensões da cauda local em last_sub_frame_inline_extents. O resultado externo substitui o resultado de seus filhos depois que a composição externa termina.

O núcleo pinado fixa a distinção resultado/estado externo. Exigir o teste
P1491 verde com os mesmos valores e a suíte de checkpoints/layout.

## Ascent instalado na origem legada — correção interna

Medição anterior: workspace.log desta composição falha nos quatro testes
P1493 de posição em html.frame: y=21.1pt em vez de 20pt. O caminho legado
em sub_frame.rs:190-194 instala vertical_metrics como baseline, mas a cauda
usa text_edges como ascent presumido; no MockFontMetrics a diferença é
1.1pt. Preservar os testes causais existentes. Snapshot com fontes e logs:
box-composicao-2026-10-13/radius-legacy-before.json.gz.

Para origem legada sem a exceção Raw já especificada, guardar como ascent
assumido exatamente a baseline inicial instalada por vertical_metrics.
Origem geométrica continua diferida; Raw mantém sua calibração própria.
Não alterar contratos HTML, posições esperadas ou fixpoint para mascarar
a diferença. Revalidar P1493, Raw, medição causal e suíte completa.

## Baseline inicial de caixa sem strut fictício — 2026-10-14

### Medição anterior à decisão

box-baseline-inicial-2026-10-14/before/results.json.gz registra doze fontes
contra o vanilla pinado, binários, estado sujo, HEAD e UTC. Caixa 8×6pt
sozinha começa em y pixel 22 contra 20 a 144dpi; com Q adjacente ambos
começam em 22. Shift baseline 10pt sem texto revela a mesma contribuição
fictícia. boxed.rs:21-38 instala fonte antes de conhecer a caixa;
line_commit.rs:421-437 e sub_frame.rs:292-300 confundem zero com ausência
e só consideram texto em condições parciais. Fonte pinada inline/line.rs:
559-568 acumula top/bottom dos frames realmente presentes. A posição é
observável; Option e buffers são mecânica. Snapshot fonte/L0: before.json.gz.
Hipótese: diferir a baseline da primeira Box e combinar somente conteúdo
real elimina o deslocamento. Refutar se controles de texto, transformações,
Raw, HTML causal ou caixas aninhadas passarem a se deslocar.

### Contrato deste owner

Isolar/restaurar a presença e o valor de line_assumed_ascent. Origem geométrica começa em None; origem legada e exceção Raw guardam Some do ascent que realmente instalam. No fechamento da cauda, comparar ascent inline e aresta superior do texto direto real; Some(0) não aciona fallback. Aplicar a diferença positiva à cauda inclusive quando o único ascent não zero vem do texto. Não acrescentar descent tipográfico a grupos sem texto direto; preservar resultado privado de extensões e todos os buffers.

Correção interna ADR-0127, sem API, default ou fase novos. L0 primeiro,
RED→GREEN, controle de ascent zero, sondas de geometria e suíte completa.
Não substituir esperados de regressão para conservar o defeito.

### Aresta de fonte efetivamente presente

Inspeção adicional anterior ao reparo: sub_frame.rs:266-275 inicia o fold
no tamanho exterior, mesmo depois de um escopo de texto menor ter emitido
a cauda. Ao passar a acomodar texto real, esse fallback viraria um ascent
fictício. O máximo deve partir dos itens textuais presentes, usando o
estilo exterior somente quando não existir item textual; testar texto
menor dentro e fora de Box com a mesma baseline, sem relaxar posições.

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

lab/results/construction/diagnostics/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

O frame isolado inicia segmentos locais dos escopos lexicais herdados; fechar segmentos locais antes de retornar seus itens, preservando os limites abertos dos buffers do pai. A pertença acompanha também frames diferidos. O caminho inline preserva limites com os itens do corpo. Não resolver links antes dos fixups geométricos.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.

## Origem geométrica de Place em região local — 2026-10-14

### Medição anterior à decisão

place-origem-local-2026-10-14/{before,extra-before}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Box-place publica shape em y37.75pt contra28pt no canvas480×400 (bbox raster aproximado); no SVG fonte, box sem margem do corpus tiling tem y14.834 local contra5. O deslocamento cresce com fonte. placement.rs:463 veta origem geométrica dentro de qualquer subframe e :533 conserva o ascender local; place.rs:96–131 soma text top-edge mesmo para Top explícito. Root-text shape diverge29.5/22.25pt; Box-text32/22.25; Grid e Align são controles positivos. Fonte pinada flow/distribute.rs:703–715 separa align_y explícito de offset de fluxo. Isso é geometria de linguagem; não obrigação de buffers Rust. A hipótese é refutada se texto, VAlign, scopes, fixups ou checkpoint perderem coordenadas após origem única.

### Responsabilidade exclusiva

Salvar sub_frame_origin_y no protocolo isolado; após calcular start_y instalar start_y; restaurar o anterior antes de retornar. Geometric origin usa zero; legado usa a baseline instalada. Não mudar composição dos demais callers.

### Aceitação e etapas

Esta seção supersede a restrição histórica de body geométrico apenas na raiz de Place. L0 primeiro e RED→GREEN em fluxo interno ADR-0127: sem campo público/trait/assinatura/fase/default novos. Comparar antes/depois SVG/PDF e preservar corpos textuais, mistura, páginas automáticas, clipping, posições observáveis e floats; build/workspace/rustdoc/fmt/linhagem/inventário.
A origem do body e a âncora após fluxo comitado são defeitos distintos. Esta etapa A não fecha âncora depois de Shape/Parbreak/Linebreak nem inset de Block, já refutados pelos controles. A etapa B de place-origem-local completa o fim causal do fluxo e a região de Block; não declarar Place universal pronto enquanto B estiver aberta. O transporte público de Tiling permanece pendente de aprovação em seu pacote, sem alteração por esta correção.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Salvar/take flow_placement_end antes do body e restaurar no retorno, para que body de Place não consuma o fluxo do pai.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.

## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

Retirar a região de Block durante sublayout e restaurá-la em LIFO; filhos constroem a própria região.

### Aceitação e incompletude explícita

Correção interna/de paridade ADR-0127, sem campo público, modo ou fase. Este adendo substitui somente as fórmulas incompatíveis de região/width/clip anteriores; não legitima outros reparos sem medição. L0 primeiro, RED→GREEN, validação bilateral, suíte e resselo restrito. B2a cobre região local e clipping na região sobrevivente. B2b completa fragmentação de Block, dimensões/outset e integração dos metadados/decoradores que controles adicionais refutarem; B3 continua responsável por baseline tardia de inline Box/Transform e produtores ainda não certificados. Não declarar objetivo completo com essas frentes abertas.

B2a conserva também uma lista privada de faixas de decoração associadas ao índice do fixup Y de Place auto. O hub armazena; checkpoint clona/restaura; subframe e Block isolam/restauram; transição limpa; placement registra somente os segmentos emitidos pela ocorrência; Block aplica a mesma delta da geometria antes de encerrar o escopo.


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

Salvar/restaurar flow_frame_extent; iniciar origem do subframe e largura zero. Devolver largura do fluxo em last_sub_frame_flow_width privado antes do restauro. Preservar last_sub_frame_width legado e todos os retornos públicos.

### Aceitação e limites

Correção interna/de paridade ADR-0127: nenhum contrato público, default ou
fase novo; L0 primeiro, RED→GREEN, resselo restrito. A largura é uma parte
da correção B2b/T1–T3 já em andamento: altura Auto/backing, alinhamento tardio,
fracionários, fragmentação, outset e PDF Parent continuam com critérios próprios.
Não declarar render aprovado por testar apenas o carrier. Preservar controles
bilaterais completos e refutadores, suíte/build/doc/fmt/linhagem/inventários.


O RED green-second do lote registra Stack horizontal com largura18 em vez68:
sub_frame.rs:246 drena a linha terminal sem flush_line. Capturar também essa
linha após expandir fr/repeat e antes de alinhamento RTL, usando o mesmo
helper privado do owner line_commit. Não acrescentar flush vertical nem
medir o body outra vez. O helper verifica linha causal e registra sua
extensão intrínseca; sub_frame apenas o chama no ponto de fechamento local.


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

No caminho isolado, salvar/limpar block_auto_width antes do corpo e restaurar no retorno. Não mudar a semântica do caminho inline.

### Classificação e limites

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem contrato público,
modo/default ou fase novos. L0 primeiro, RED→GREEN, resselo restrito e suíte.
Esta etapa completa o refutador geométrico da largura causal; não completa
alinhamento textual/matemático, decoração externa diferida, Place horizontal,
altura Auto ou fragmentação. São pendências de B2b/T3, sem crédito de paridade.

## Região relativa e fluxo das transformações — medição causal

`transformacoes-2026-10-14/after-first/results.json.gz` registra o candidato e
seus hashes: Move percentual fica fora da página porque a altura é o sentinel;
textos A/TEST/B ficam numa linha no Tekt e em blocos no pin. As imagens foram
inspecionadas, sem realinhamento. Fonte pinada `typst-layout/src/rules.rs:725–739`
usa BlockElem::single_layouter para as quatro funções, confirmando a semântica
de bloco; não é uma convenção mecânica de enum. Transform Rust genérico não
é uma função da linguagem. Correção dentro de X4 aprovado, sem novo contrato
público. A hipótese é refutada por deslocamento em região explícita/aninhada,
espaçamento, perda de body ou vazamento após rollback.

Salvar/restaurar relative_region_size em LIFO. No subframe, largura é a região fornecida; altura explícita vence, ausente herda região real do pai em vez do sentinel. Um transform nativo terminal retorna seu fim físico sem margem abaixo; manter o caminho Matrix legado. Não mudar o algoritmo dos demais tipos.

## Dimensões relativas de formas — correção medida

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/shape-dim-before-complete/results.json.gz`
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

Na herança de relative_region_size, altura finita de Block prevalece;
se Block tem altura Auto infinita, herdar a região relativa real existente
ou a região física anterior, não apagar a base com esse marcador. Altura
explícita fornecida pelo caller continua prevalecendo; infinito genuíno da
região herdada permanece. Conservar a restauração LIFO e o sentinel separado.

Correção interna/de paridade ADR-0127, sem API/trait/campo público nem fase
nova. L0 primeiro, RED→GREEN, resselo restrito, corpus, suíte e verificadores.
Preservar origem, pintura, contornos, recursos e efeitos; a medição especulativa
existente não ganha execução nova.


## Prefixo multiline e aninhamento parcial — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/stat/UTC, fontes e binários contra o pin vanilla
9f2b6e8715237cb086899a42873660fe744622e8. Multiword21 gera duas páginas
contra uma; Long/More estão 6.5/13pt abaixo. O prefixo40/8 com fill gera duas
páginas; More está39.906pt abaixo. Nested OUTER18/INNER8 tem BODY12pt acima
e página auto12pt menor. sequence/fractional_preflight.rs:151–171 recusa
multiline; word_layout.rs:49 fecha dentro do estilo local; line_commit.rs:
398–480 usa esse estilo para floor/leading/top previsto. sequence.rs:239
recusa fechar segmento parcial; Block sem cadeia anterior usa spacing8pt.
Vanilla flow/collect.rs:129–145,179–214 resolve trunk dos filhos, spacing e
leading comuns; cada frame contribui sua geometria. Esses são observáveis
textuais, geométricos e de paginação da linguagem (ADR-0107), não obrigação
de copiar buffers, algoritmos ou bytes. A intenção histórica não é inferida.

### Decisão e limite

Correção interna de paridade ADR-0127 em fluxo contínuo: sem entidade,
trait/assinatura pública, nova flag/default ou fase. O núcleo text-segment-scope
pinado define as invariantes compartilhadas. Aceitação RED→GREEN dos três
casos, controles de fontes, cores, margens, colunas, restauração, checkpoint
e subframes, mais suíte completa e linhagem. A nova rota só amplia prefixos
multiline textuais positivamente certificados; não declara paginação universal,
bidi/floats nem conteúdo opaco. Preservar os certificados escalares anteriores.

### Responsabilidade exclusiva

Subframe isolado suspende text_segment_chain e geometric_text_lines e restaura em LIFO. Subframe inline conserva herança lexical e não ganha isolamento adicional.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F011: 01_core/src/compiler/layout/sub_frame.rs:293 (layout_sub_frame_with_origin); F012: 01_core/src/compiler/layout/sub_frame.rs:553 (text_of). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F011 — `layout_sub_frame_with_origin`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`crate::entities::layout_types::FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F012 — `text_of`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.

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

Suspender ambos os contextos ao entrar no subframe e restaurar em LIFO ao sair, juntamente com block_place_region e geometria já existentes. Medidas/Place não podem consumir o prefixo do corpo exterior nem herdar os limites de uma página exterior; Blocks internos podem instalar contexto próprio. Não alterar retornos, offsets, callbacks, dimensões ou a pureza da medição.

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

Isolar State e fila pendente da região exterior. Contexto local segue fragmento inline versus Par explícito/fluxo com fronteira do body; não consumir a compensação CSL do pai dentro do frame auxiliar. Restaurar em LIFO junto das coordenadas e contexto existente, inclusive inline e isolamento de callbacks.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.

### Decorações: medição complementar de origem (2026-10-18)

Os12 pares SVG em bibliography-hanging-2026-10-18/deco-before.json.gz,
com a leitura corrigida de primitivas line/path em deco-inspection.json,
usam o mesmo binário/freeze da suíte inicial9026 e o oracle pinado bd5ebec.
Em overline h10, a segunda linha tem texto em x18 mas traço em x8;
vanilla começa o traço em x18. h0 já tem diferenças de y/espessura/cauda,
que não serão encobertas nem fechadas por este reparo horizontal.
Fonte local: decorations.rs:43–45,76–78 usa cursor/line_start_x globais;
line_commit.rs:615–619 e sub_frame.rs:350–354 capturam início global.
Isso é geometria da linguagem, não obrigação de copiar buffers vanilla.
Refutação: origem duplicada, consumo precoce da compensação, mudança de
baseline, fim, espessura, texto ou região fora do delta horizontal prescrito.
Correção privada de paridade ADR0127 em fluxo contínuo, sem fase/API nova.

Responsabilidade deste owner: O collector do fechamento manual final usa a borda esquerda efetiva local do parágrafo antes de restaurar o estado exterior. Manter isolamento, end_x e baseline existentes.
