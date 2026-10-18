# L0 — cursor
Hash do Código: b1880a31

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:42175932c75f37ed19205d1d82526969999e18fba8fed16f1b1dcff6e418bc0f
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:c983fdb9884f90548e110fe7c1709632fc00b6e2a1387ad9e80444009c3602fd
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:04f91fc931871165d4fef399ea9c2e21e8b75f7fd57031183a4e2aed0ed1b62a
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/cursor.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/cursor.rs:27`, `01_core/src/compiler/layout/cursor.rs:33`, `01_core/src/compiler/layout/cursor.rs:41`, `01_core/src/compiler/layout/cursor.rs:94`, `01_core/src/compiler/layout/cursor.rs:195`, `01_core/src/compiler/layout/cursor.rs:314`, `01_core/src/compiler/layout/cursor.rs:429`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the thin word/chunk and regional transition adapters, initial baseline, RTL line placement, marker-prefix coordination, pending-float drain and footnote-body emission. Word emission/selection and regional transition implementation belong to the separate owners specified below. The hub retains all state/checkpoints. Line closure and horizontal allocation belong to `00_nucleo/prompts/compiler/layout/line_commit.md`; transactional suffix decisions to `00_nucleo/prompts/compiler/layout/flow_suffix.md`; float fitting/reservation/emission to `00_nucleo/prompts/compiler/layout/place.md`; cell-tail emission to `00_nucleo/prompts/compiler/layout/cell_tail_commit.md`; auto dimensions to `00_nucleo/prompts/compiler/layout/page_geometry.md`; intermediate Page construction to `00_nucleo/prompts/compiler/layout/page_commit.md`; column close/merge/start hooks to `00_nucleo/prompts/compiler/columns.md`.

Use FontMetrics.text_width as the word-width authority, including shaping and tracking; use the active space advance and baseline offset for emission. Establish the first baseline from the active top edge. At a finite full region with committed content, progress before emitting the next first line; do not drain new text beyond the old region then move it. Empty/unconstrained regions do not create no-progress loops. For an already-certified fractional tail only, preserve the existing bottom snap bounded by the outer below-minus-leading boundary and fitting tolerance; ordinary overflow is unchanged.

Maintain inline ascent/descent maxima and translate all current-line items, link/semantic children and matching metadata consistently when a baseline changes. Do not replace child geometry by recognizing Block, Place or future Content.

Delegated word handling retains ordinary measured wrapping, hyphenation fallback and overlong-word progress; `00_nucleo/prompts/compiler/layout/word_segment.md` owns script segmentation and quote tailoring, `00_nucleo/prompts/compiler/layout/hyphenation.md` owns break-point generation. Ordinary singleton closing must not acquire Sequence's validated-prefix exception.

The float marker drains only its captured prefix through the active regional distributor, preserving order and excluding later occurrences. Ask Place for admissible fitting and reservation; finish/advance only by the existing transition rules and require progress. After the prefix stabilizes, mark the hub's suffix checkpoint boundary without flushing pending text or unconditionally breaking. Accept a valid suffix before ordinary transition through `00_nucleo/prompts/compiler/layout/flow_suffix.md`; rejection is handled there, not by migrating only the line.

Regional transition preserves existing float/footnote order, auto-dimension timing, column delegation, page commit and next-region reset. Invoke cell-tail emission after the next top is prepared. Footnote bodies retain the existing separator, usable bottom, split/overflow and bounded first-body fallback; no I/O or callback execution is introduced. Treat only direct private HTML markers as non-material via `00_nucleo/prompts/compiler/layout/html_frame.md`; transport them without inventing line height or an occupied region.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/cursor.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

O cálculo interno baseline_y é compartilhado por descendentes de layout para posicionar carriers com o mesmo baseline_offset de Text; a visibilidade é apenas pub(super), sem novo contrato público.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Retain cursor origin/baseline and inline bookkeeping, RTL positioning, float-marker prefix coordination and float/footnote drain adapters. layout_word/layout_chunk become thin delegates to word_layout; space_width delegates to text_measure; new_page delegates to region_progress. Remove the local word-width helper, text emission body, break algorithm and regional transition body. Preserve every existing descendant method signature and call order.

## Regional demand correction — 2026-09-28

### Evidence before decision

The 30 bilateral controls in `00_nucleo/diagnosticos/estudo-correcao-falhas-texto-2026-09-28/` measure terminal pages 2/1, explicit terminal pagebreak 3/2, and continuation 3/2 (Tekt/vanilla). The isolated pre-export control emits Four/additional on page 2 with baseline 101.84pt and page height 100pt. The causal sources are line_commit.rs:657-696 (hypothetical next baseline), region_progress.rs:113-128 (post-line transition), cursor.rs:80-135 and parbreak.rs:41-80 (paragraph baseline/spacing). Current source and L0 hashes precede edits in `00_nucleo/diagnosticos/correcao-demanda-regional-2026-09-28/initial.json` and `before/`. This is internal parity correction under ADR-0127, not a public contract/default/pipeline change.

### Decision and acceptance

After the existing initial/block-chain baseline paths, admit an ordinary next text line through the private region_progress owner. Preserve the paragraph/fractional snap rules and do not reuse old-page paragraph baseline by advancing during its preceding fitting flush. No extra typography for empty/nontext-only initial flows. Keep the public and descendant signatures and state/checkpoints unchanged.

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

## Alinhamento da linha RTL continuada — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/after-pdf/results.json` e `summary.json` medem 66 fontes/198 PDFs com fonte, pin e estado em `provenance.json`; `wrap-initial.json` registra o estado exato antes desta segunda correção. Em latin-wrap ±0.2/0.333/1.2pt, a primeira/segunda linha do Tekt deslocam-se relativamente ao vanilla por exatamente um intervalo ±t, enquanto a última coincide. O texto e as linhas escolhidas coincidem. `line_commit.rs:241-248` conhece se há continuação, mas `:477` chama `cursor.rs:150` sem essa informação; `cursor.rs:164-177` ancora pela direita real. A inferência de um intervalo terminal pendente é refutada se controles de wrap/hifenização mostrarem outro deslocamento. Esta medição não prova uma regra universal para bidi misto ou espaços iniciais.

### Contrato interno

Manter align_current_line_rtl() para fechamento ordinário e subframe. Em alinhamento de linha continuada não justificada, receber o sinal privado de continuação e deslocar a âncora RTL pelo tracking resolvido da última unidade textual material, inclusive sinal negativo; não tocar na posição dos glyphs dentro da unidade, em textos LTR ou em linhas finais. Consultar helper privado do owner text_tracking para valor/elegibilidade. Se não houver unidade textual final elegível, nenhum ajuste. Não trocar a referência por cursor que inclua espaço terminal nominal.

Mudança interna ADR-0127 em fluxo contínuo, sem assinatura pública, default ou fase. Teste RED→GREEN e controles bilaterais, preservação dos terminais e LTR, build/lint sem erros e resselo restrito.

## Prefixo literal RTL — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/final-rtl/results.json` mede 66 fontes/198 PDFs contra vanilla pinado, com fonte, binário e árvore em `final-rtl/provenance.json`. Nas seis variantes de `#text("  abc") def ghi` com `dir:rtl`, a tinta do Tekt fica à direita do vanilla por 6.24pt no tracking zero, 6.44pt em +0.2 e 5.04pt em −1.2, a largura do par de separadores incluindo o intervalo interno. As origens dos dois espaços ficam antes de `a` no Tekt e após `i` no vanilla; glyphs restantes preservam ordem e texto extraído. `cursor.rs:154-195` alinha pelo limite direito da tinta e translada todos os items igualmente; `helpers.rs:95-107` exclui TextSeparator desse limite. A hipótese refutável é que a linha visual RTL deve reservar à direita o prefixo Literal transportado, sem alterar o tracking interno ou a posição de outros espaços. Isto é linguagem/PDF, não igualdade de bytes nem inferência da intenção histórica.

### Contrato interno

No alinhamento RTL de linha com prefixo de TextSeparator Literal direto antes do primeiro item textual, derivar a extensão causal do prefixo por posição e advance dos carriers, incluindo os intervalos já aplicados. Reservar essa extensão na âncora direita da tinta e mover o prefixo à faixa direita correspondente; manter a ordem textual do Vec/PDF e não contar novamente tracking. Sem prefixo, o deslocamento permanece idêntico; linhas LTR, espaços finais, contexto marker, justificação e continuação preservam o contrato anterior. Nenhum offset numérico fixo. Correção interna ADR-0127, RED→GREEN e replay bilateral/401 regressões.

## Direção de parágrafo distinta de direção de run — 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/direcao-alinhamento-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes, binários e treze sondas. rtl_lines põe ABC em x=20 no Tekt e x=152.949220 no vanilla pinado; rtl_inline_only e rtl_scope coincidem à esquerda. ltr_lines põe árabe em x=174.567870 no Tekt contra x=20 no vanilla. Em `cursor.rs:156–162` e `line_commit.rs:210–216`, a âncora depende do primeiro item da linha. Vanilla `flow/collect.rs:129–136` obtém trunk_from_pairs e `inline/mod.rs:189,232` resolve direção/alinhamento compartilhados antes das linhas. Semântica observável: direção dos trechos e âncora de todas as linhas do parágrafo. Estratégia de state/preflight é mecânica interna, não cópia do vanilla. Inferência: preservar direção compartilhada evita perder a âncora em linhas só com override; refutada por controles integralmente estilizados/parbreak/wrap.

Correção interna/de paridade ADR-0127, fluxo contínuo L0 primeiro e RED→GREEN. Apenas campos privados de Layouter e helpers descendentes, nenhuma entidade ou assinatura pública/fase/default novo. Esta rodada certifica conteúdo textual estrutural (Text, Empty, Space/SpaceFromNewline, Linebreak, Sequence, Styled e Strong/Emph); unidades desconhecidas conservam fallback anterior. Não prometer ordem bidi mista, align explícito, HTML ou todas as formas de parágrafo. As equações existentes de largura/tracking permanecem.

Responsabilidade deste owner: A âncora RTL usa a consulta interna do owner paragraph, em vez de primeiro Text diretamente. Demais fórmulas, literal prefix, terminal tracking e regiões auto permanecem. Não alterar run.dir.

Aceitação: RED→GREEN em parágrafo RTL com linha LTR e inverso, escopo integral e reset por parbreak; checar checkpoints e subframes, texto/posições em CLI bilateral, build e lint sem novos findings. Slots e helper não transportam metadados ao L3; divergências de ordenação bidi ou align explícito ficam registradas separadamente.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Block/Align — correção interna 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-align-2026-10-06/baseline-verification.json.gz` registra 14 controles completos, fontes, comandos, caixas e binários no HEAD/working tree/UTC de before.json. RTL Block automático põe ABC em x20 no Tekt contra x152.949220 no vanilla; Block(width:100pt) coincide em x100. #set dir:ltr no body não altera a âncora externa do Block automático no vanilla. Align(left/right) mistura bordas por direção do run e desloca Y por 8.35742pt. `placement.rs:35-46,176-177` renderiza no subframe com baseline pronta e subtrai ascender na recomposição; `cursor.rs:154-162` só aplica direção RTL; `block.rs:175-185` posiciona container externo apenas quando largura é explícita. Fonte vanilla pinada `flow/collect.rs:193` resolve Align por linhas e `flow/block.rs:145-166,285` distingue regiões expandidas de auto-size. Paridade de posições/forma, não cópia da mecânica (ADR-0107).

### Responsabilidade exclusiva

O slot explícito tem precedência sobre o alinhamento automático por direção. Left conserva origem; Center usa metade do espaço livre; Right usa a borda regional existente, conservando ajuste RTL/tracking existente quando a direção real for RTL. Texto e direção bidi não são reescritos. Ausência do slot conserva exatamente a rota anterior.

Correção interna/de paridade ADR-0127 em fluxo contínuo. Nenhum contrato público, modo, fase, compatibilidade ou unsafe novo. O certificado é limite explícito desta rodada e deve ser refutado por nested Align, Block de largura explícita, scoping após Align, auto dimensions, texto regular e corpus anterior. RED→GREEN, revalidação bilateral, builds/lint e resselo somente do manifesto. Controles negativos não são redefinidos como aceitos. Sem promessa de ganho de tempo.

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

Na admissão da baseline inicial/de bloco, não pedir nova região externa quando is_sub_frame. Manter avanço local de baseline e o restante da fórmula; new_page explícito e drains de floats não são alterados.

Os invariantes compartilhados pertencem ao núcleo subframe-overflow pinado.
Correção interna ADR-0127, sem campo/assinatura pública, fase ou novo default.
Exigir RED→GREEN antes/depois das condições, preservação da paginação
ordinária externa, suíte core, build, formato e linhagem. Não elevar a
altura de teste para esconder overflow nem descartar itens do frame.
A correção do helper não equivale a terminar a composição de block/box;
a coleta, dimensões e recorte desses containers continuam nos seus owners.

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

Ao instalar baseline de texto ou de transição já definida, guardar Some(top_edge). Não alterar as fórmulas de baseline, traduções, wrapping ou emissão; este owner não decide a baseline natural de Box.

Correção interna ADR-0127, sem API, default ou fase novos. L0 primeiro,
RED→GREEN, controle de ascent zero, sondas de geometria e suíte completa.
Não substituir esperados de regressão para conservar o defeito.


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

Ao admitir texto depois de uma cadeia com margem zero, consultar a testemunha
causal de fluxo válida. Se existir, usar fim causal + gap + top-edge para a
baseline; zero de margem não significa baseline já instalada. Preservar a
rota positiva/fraca existente, snap fracionário, progressão regional e fallback
quando a testemunha é inválida. Não reconhecer tipos de Content nem inferir
fim por tinta. A origem inicial geométrica já pendente continua no primeiro ramo.


## Escopo textual e cauda fracionária — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/scope-tail-before/results.json.gz`
registra HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/status/diffstat e hashes de fontes/binários contra vanilla pinado9f2b6e871.
Os lotes block-par-spacing-after e block-par-extra-after anteriores já
refutam escopo por consenso de runs: um wrapper18 em Lead More exige margem
21.6pt, enquanto dois wrappers18 adjacentes mantêm o escopo exterior.
sequence.rs:196–245 fecha prefixos na chain exterior; paragraph.rs:12–90 já
resolve direção pelo ancestral lexical comum. Fonte vanilla flow/collect.rs:
129–145 resolve estilos comuns antes de compor linhas e espaçamento.
A cauda TAIL top10/bottom0 após Block1fr ocupa página2 contra página1 no pin:
cursor.rs:127–135 rejeita igualdade com o limite embora a cauda esteja
certificada. Cauda Styled18 fica10.52pt acima; fractional_preflight.rs:170–211
mede nós individualmente e só corrige a altura de Text direto. Linebreak
explícito também excede a região. A nova sonda inclui wraps, fonte8/18,
Strong, wrappers irmãos, Align e colunas. Isso é geometria e paginação de
linguagem; não exige estrutura ou bytes iguais. before-edit.json.gz em
scope-tail conserva os consumers/owners anteriores.

### Classificação

Correção interna de paridade ADR-0127: L0 primeiro, RED→GREEN e revalidação.
Sem API pública, nova entidade, fase eval/layout ou default novo. Refutar
por wrappers irmãos, preservação da ordem, última linha, corpos opacos,
colunas e ausência de efeitos/estado vazando da medição. Não copiar defeito
upstream nem declarar paginação universal para cauda maior que a região.

### Responsabilidade exclusiva

Uma cauda fracionária já certificada cujo próximo baseline é exatamente
o fundo admissível não provoca nova página. Incluir igualdade no gate de
snap existente, conservando a tolerância superior e progressão ordinária.
Não autorizar texto além da região por essa igualdade nem afrouxar outros
casos sem certificado. Revalidar top-edge/bottom-edge e região seguinte.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-hanging-2026-10-18/before-sources.json.gz
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

Alinhamento horizontal da linha usa a borda final efetiva, conservando direção comum, prefixos literais e tracking. Não mudar regiões/line_start_x para simular hanging ou alterar admissão vertical.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.
