# L0 — tests
Hash do Código: a8d5fa0f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:ecf965471eedd8599209f0575811dd5b456dce1c43baa26881251e24e8ec6c6d
- 00_nucleo/prompts/_nuclei/layout/block-spacing-resolution.toml sha256:2e2f3cfb51bd0c484e65cc9e5c3e1eee9f3dad12c77aa9330674b474e11f3342
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:c983fdb9884f90548e110fe7c1709632fc00b6e2a1387ad9e80444009c3602fd
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:2ce5a50363605eb1a122920b92773b1fc956c52817dd57830c2ce12bfe5f285e
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:d207e8a3a934b2eff2b67c4bfc7f15b9af0c0f3abc83acdfaf774326e7b242eb
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L1, test-only
**Exclusive consumer:** `01_core/src/compiler/layout/tests.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/tests.rs:31`, `01_core/src/compiler/layout/tests.rs:38`, `01_core/src/compiler/layout/tests.rs:48`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own only the existing test-only worlds/metrics, affine observation helpers and layout regression suite. Preserve assertions for rendered text/morphology, page/position/style, diagnostic behavior and architectural delegation. Geometric observers compose ancestor affine transforms, Group position/matrix and nested wrappers once into global coordinates; Semantic is transparent and Link follows its own convention. Variant-only tests need not project geometry.

Keep the absolute Block fixtures lifted into the current relative carrier without changing their values/tolerances. Preserve full-region Block followed by text, terminal-no-empty-page and relative/fractional sizing controls. Preserve the independent float clearance-anchor versus reserve/fitting controls, the atomic post-marker suffix including Place and later floats, and controls without marker/float or with explicit block spacing; do not weaken them to page totals or inspect private checkpoint flags as a substitute for observable position.

Retain mixed-size/no-wrap and certified one-wrap fractional-prefix controls, both orders across the outer size, equal/unequal size cases, and opaque compound-style/multiword negative controls. Keep cases alignment points reaching layout, without suppressing panic or deleting align markers to make a test green. Preserve the structural regression for thin static descendant dispatch arms and the exhaustive match; it checks architecture, not claimed performance.

This document neither authorizes production changes nor requires tests that are absent to be declared implemented. Existing historical probe coordinates and executions are archived in the diagnostic; future quantitative parity/performance claims need exact source/binary provenance, not aggregate test status.

## Acceptance and limits

Preserve the existing tests and their observable assertions. This documentation operation does not modify fixtures, tolerances or Rust code.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/tests.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

A projeção geométrica dos testes trata TextSeparator como folha posicionada; o carrier não cria tinta nem altera a semântica dos outros refutadores.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Empty-page and Place witness correction — 2026-09-28

### Measurement before decision

The full domain run in `00_nucleo/diagnosticos/correcao-falhas-2026-09-28/core-full.log` rejects the old zero-page assumption at `tests.rs:1013` and the former ascender-cancelled baseline formula at `tests.rs:20919`. The pinned vanilla probe `probes/empty-vanilla.pdf` has one configured 160×180pt page. The five Place integration witnesses establish that a text baseline includes its measured top edge after geometric placement. The p898 witness concerns dy preservation and must retain that observable rather than an artificial baseline-origin convention. This is a language/geometry correction, not permission to weaken failing assertions.

### Responsibility

An empty document must retain one page with no material items. The bottom/dy witness must still check the full baseline position: bottom-aligned frame top plus dy plus its text top edge. Preserve its 0.01pt tolerance and fixed metrics. Keep the pending-line Place witness unchanged; production must retain its flow anchor.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Add a structural refutator requiring thin cursor adapters, absence of inline hyphenation/word measurement/page commit in cursor, and separate private module registrations. Keep all behavioral tests and tolerances unchanged. Existing behavior and the 63 archived cases are the independent regression oracle, including currently known limitations.

## Regional demand correction — 2026-09-28

### Evidence before decision

The 30 bilateral controls in `00_nucleo/diagnosticos/estudo-correcao-falhas-texto-2026-09-28/` measure terminal pages 2/1, explicit terminal pagebreak 3/2, and continuation 3/2 (Tekt/vanilla). The isolated pre-export control emits Four/additional on page 2 with baseline 101.84pt and page height 100pt. The causal sources are line_commit.rs:657-696 (hypothetical next baseline), region_progress.rs:113-128 (post-line transition), cursor.rs:80-135 and parbreak.rs:41-80 (paragraph baseline/spacing). Current source and L0 hashes precede edits in `00_nucleo/diagnosticos/correcao-demanda-regional-2026-09-28/initial.json` and `before/`. This is internal parity correction under ADR-0127, not a public contract/default/pipeline change.

### Decision and acceptance

Add regressions for terminal fitting text, deferred ordinary admission, wrap/chunk admission, paragraph continuation within viewport, explicit repeated breaks/parity, finite/infinite and empty roots, and a fitting speculative suffix. Preserve existing tests, assertions, fixtures and tolerances. Cross-module tests may inspect private demand state but must also assert page count, text and positions; private flags alone do not replace observable guarantees. Verify real-font source-to-PDF controls, columns, inserted floats/notes and complete suites before closing the batch.

### Integration controls preceding the refinement

The first implementation's 36 real-font controls are preserved in first-after-pdf/ and first-pdf-green.log (before any later rebuild). terminal-columns still loses Three; terminal-footnote changes from 2 to 1 pages and its raster shows note/main overlap. Column start returns before clearing previous block spacing; terminal note fitting uses cursor_y and must retain the conservative following-line clearance rather than the terminal geometry cursor. These measurements require the refinements below, without changing the footnote body format, reservation model or public contract.

Add regressions for two consecutive words following a paragraph across a column transition and for terminal notes whose baseline-only budget would overlap the main line. Assert causal page/text/positions, not only internal reset flags.

## Identidade textual — controles do contrato aprovado

Medição anterior: initial.json, before/resultados.json e red.log em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/` registram
texto/caixas e a perda de SHY. O dono confirmou o contrato
`00_nucleo/prompts/entities/layout_types.md`. A compilação de fixtures exaustivas
exige tratar a variante nova. Projeções visuais descem nos filhos; extractors
textuais usam text uma vez. Expectativas de identidade discricionária passam a
SHY, mantendo U+002D literal. Não relaxar caixas/seleção. A suíte de shaper
exercita fixture Noto obrigatória e ausência de fontes, conserva envelope e
prova que text substituto não altera glyphs/avanços nem contamina o cache.

## Tracking interfragmentos — correção do controle histórico

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-2026-09-29/green-core.log` refuta o teste histórico `layout_tracking_um_char_nao_acumula_passo_137`, que exigia ausência de tracking entre palavras e ainda misturava tamanhos 11/12pt. `00_nucleo/diagnosticos/tracking-residual-2026-09-29/results.json` mede dois intervalos por um espaço e um por adjacência contra o pin vanilla, independente de quantização. O teste antigo codifica justamente o defeito agora corrigido.

### Decisão e aceitação

Superseder somente essa expectativa por controle de mesmo tamanho 12pt: `A B` deve deslocar B em exatamente 2*tracking, com tracking positivo/negativo. Não relaxar tolerância nem mudar demais regressões. Alterar o nome/comentário para declarar a semântica atual. Sem código produtivo neste owner; resselo restrito.

## Regressão da largura Auto — 2026-10-03

### Medição anterior à decisão

`00_nucleo/diagnosticos/grid-vanilla-phase-2026-10-03.md` mede desvio acumulado de 2,5 pt por coluna Auto de uma palavra, ausência desse desvio nas colunas fixas e largura de espaço de 2,5 pt nos dois motores. `measurement.rs:70-96` soma o espaço após a palavra final. A causa é hipótese verificável por regressão e ablação, não intenção atribuída ao vanilla.

### Responsabilidade deste owner

Adicionar testes de medição restrita com métricas controladas: palavra única sem espaço final, duas palavras com exatamente um separador, e quebra quando apenas a palavra adicional excede a largura. Cobrir largura e número de linhas, sem relaxar testes existentes. A aceitação externa usa posições de PDF contra o vanilla pinado; estes testes isolam a fórmula interna.


## Reuso de identidades preparadas — 2026-10-05

Adicionar sonda de origem já preparada em Realized: construir Heading em Located, indexar a entrada, limpar o probe thread-local de introspecção já existente e emitir o produto. O layout deve conservar a âncora original e não reindexar o heading para registrá-la. Essa contagem é observável mecânico de trabalho evitado, não requisito de paridade Rust. Revalidar origens com descendentes preparados e caminhos de fallback por síntese/unprepared, além da suíte de layout.


## Produto de célula medido — 2026-10-05

A evidência de duplicação em grid/tracks.rs:175 e grid/cell_body.rs:155 precede o experimento definido pelo owner `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. Adicionar contador por thread exclusivamente cfg(test), exigir dois subframes para duas células Auto de tokens e validar texto e last_sub_frame_width/bottom; confirmar RED anterior. Controles Fixed, alinhamento, placement explícito e locatable conservam as contagens e observáveis anteriores. Não alterar expectativas de linguagem para admitir cache.

A leitura de layout_align (wrapper que constrói outro subframe) explica o controle de alinhamento: duas medidas e quatro subframes de emissão, seis no total. Não exigir quatro por uma contagem incompleta do wrapper. Fixed usa dois e placement explícito quatro nesse controle.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Controle de borda de colspan — 2026-10-06

Medição: `tests.rs:13525-13583` esperava comprimento 100pt para duas
tracks de 60+40pt e stroke de 2pt. O oracle pinado em
`lab/typst-original/crates/typst-layout/src/grid/layouter.rs:779-791`
mede comprimento visual 102pt, incluindo extremos. O teste herdado
confundia fronteira das tracks com caminho pintado. Ajustar a expectativa
do controle de colspan à geometria especificada pelo owner canônico
`00_nucleo/prompts/compiler/layout/grid/borders.md`; continuar exigindo
cobertura das duas colunas e espessura de 2pt. Não enfraquecer os demais
controles nem alterar código produtivo neste consumer de testes.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

Testes primeiro para default/en/de/fr, reuse após estilo/disabled/custom,
primes/apóstrofo/nesting/mixed markup e função, palavra inserida/objeto inline,
parbreak/linebreak/wrap/narrow width e rollback speculative suffix. Examinar
glyph/texto/geometry quando relevante, não flags de alternância como gate.
Preservar output esperado de controles corretos anteriores; mudar somente
expectativas refutadas com recibo vanilla. Alguns testes migram de eval para
render sem perder cobertura. Não certificar bidi/locale/region universal.

## D05 — fronteira do produto textual de equação de bloco

### Medição antes da decisão

Equation-where-2026-10-06/delivered.json.gz, pin 9f2b6e871, binários/HEAD/diff/UTC
preservados: show-block emite 𝑎 DISPLAY na mesma linha no Tekt e DISPLAY em linha
própria no vanilla. O filtro e produto coincidem; a fronteira de bloco perde-se.
layout/mod.rs:1172–1175 emite visual de Realized sem considerar a origem;
paragraph.rs:114–130 já realiza o segmento textual em fronteira própria.

### Decisão e aceitação

Testar origem Equation block:true substituída por Text entre Before/After:
linhas distintas por posição y, sem duplicar origem, e controle block:false
permanece inline. Invariantes de Location e owners existentes permanecem.

Fluxo contínuo ADR-0127, L0 primeiro e RED→GREEN; manter fallback fora do
recorte, build/lint e nova medição bilateral. Não é novo modo/default deliberado.

## D08B2 — presença lexical ratificada em 2026-10-07

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz e
presence-refuters.json conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree, diffstat, UTC, binários e argumentos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Upstream model/table.rs:734–778 possui campos Smart/Option; o Tekt
entities/elements/table_cell.rs:40–51 conserva apenas Option de valor.
X:auto e x omitido colapsam; fill:none e stroke:none perdem presença.
Os refutadores medem auto permitido em x/y/fill/align/inset/breakable,
rejeitado em colspan/rowspan/stroke; none permitido só em fill/stroke.
São observáveis da linguagem, sem obrigação de copiar a estrutura upstream.

### Contrato ratificado e responsabilidade exclusiva

Fixtures TableCell especificam presence coerente com seus argumentos explícitos. Preservar todos os observáveis/assertions de render existentes; não relaxar tolerâncias nem fabricar defaults conhecidos. Testes não autorizam mudanças produtivas adicionais.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Fixtures/testes preservam observáveis e tolerâncias. Ajustar constructors para novos campos crus/defaults; adicionar RED→GREEN observando fields, order, efeitos/alias/ocultação/HTML/geometry. Testes não autorizam comportamento além do contrato ratificado.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.

### D08B3-A — controle de sobrevivência da tabela aninhada

geometry-ninth.json e after-ninth.json.gz medem perda de CELL a após reparo
de espaçamento aplicado somente na emissão. Testar que a tabela aninhada
com show Cell mantém dois CELL e um a, na mesma página, com a linha interior
abaixo da exterior; a coleta bilateral de glifos decide o gap exato.

## Raw defaults — medição pinada 2026-10-07 antes da decisão

raw-defaults-2026-10-07/before.json.gz e before-state.json.gz registram
HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, diffstat,
UTC, fontes, comandos e binários. Tekt 58456b9b54041983612e5946f89fef2cf4591b9f31a09d8dcce9ae5c8a9c0eea;
vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 e binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
A medição dos 16 controles mostra Serif 11pt em Raw contra Mono 8.8pt,
20pt contra16pt sob tamanho global, show-set absoluto15pt prevalecendo,
show-font explicitamente Sans, show-size1em vanilla8.8pt/Tekt0pt,
substituiçãoABC mantendo defaults de Raw, e fill vermelho herdado.
Fonte pinada text/raw.rs:658–670 aplica defaults show-set antes dos overrides;
raw.rs:103–123 conserva fonte/tamanho e descarta fill; apply.rs:207–253
embrulha overrides mas perde defaults de Raw no produto; style_scope.rs:41–44
resolve StyleChain.size por abs, perdendo em. São estilos/geometria
observáveis da linguagem, não exigência de buffers ou bytes PDF.

### Decisão interna de paridade, fluxo contínuo ADR-0127

Usar a composição Styles/StyleDelta já existente, sem campo, trait ou
assinatura pública novos, sem antecipar coordenadas ou mudar fase do pipeline.
Os defaults de render ficam no owner Raw: fonte Mono, tamanho0.8 do exterior,
bold/italic desativados e fill exterior herdado. Na emissão direta de Raw
aplicar esses defaults e restaurar o exterior. Quando show de Raw produz
conteúdo (inclusive Text), marcar um escopo privado raw.defaults no Styles
exterior aos overrides, para que o mesmo owner instale defaults no render
antes dos overrides. Marcar raw.defaults.active somente nesse escopo,
e impedir aplicação dupla ao Raw devolvido por it. Styled resolve text.size
Length contra tamanho anterior apenas dentro desse escopo Raw, preservando
composição absoluta/relativa e restauração da chain. Nenhum parser ou
algoritmo de show/matching/guard é duplicado.
Para typc true/false, usar o foreground constant.language da paleta existente
#d73948 quando não houver gramática suportada; não transformar em Text nem
remover Raw morfológico. Outras gramáticas/whitespace continuam no owner atual.

### Aceitação e limites explícitos

RED→GREEN de fonte/tamanho/fill/restauração, overrides e substituição.
Recoletar os16 controles, glifos reais de células e regressões. Mensagens,
ordem, índices e targets continuam preservados; fonte subset e bytes não
são critério. Whitespace Raw/syntaxes customizadas e reconstrução fresca
de Raw dentro de show não são certificados por estes controles. A alegação
histórica P1055 de tamanho100% é refutada para este pin e substituída aqui.


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

## D08B3-B — candidato, não vigente antes de ratificação

### Medição anterior à decisão

00_nucleo/diagnosticos/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz contém 53 sondas, coletadas em 2026-10-07T14:47:56.340144+00:00 até 14:48:12.375245+00:00. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diff HEAD --stat/status/hashes/fontes/argv/cwd/ambiente e saídas completas registrados. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt 6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f. Não misturar before/refuters/complete como rodadas adicionais independentes.

isolated-default: vanilla expõe x/y/inset/align/fill/breakable auto, spans 1 e stroke dict vazio; Tekt omite esses defaults e publica duas metadata contra uma. isolated-x rejeita no Tekt, funciona no vanilla. raw-fields continua só body nos dois e raw-x rejeita nos dois. isolated-table-style confirma que defaults de Table não viram os da Cell avulsa. raw-vs-prepared mede false no vanilla e true no Tekt. nested-effects/body-effects/late-rule refutam execução única e origem lexical. fresh-isolated/table, fresh-block, fresh-styled e multi-fresh são aceitos indevidamente pelo Tekt; vanilla rejeita maximum show rule depth exceeded com span do constructor novo. identity/body/alias e fresh-conditional/hidden continuam válidos. multi-identity e multi-return-table mostram que devolver it deve permitir receita anterior ainda não consumida. isolated-query mantém cell is not locatable nos dois; location é none, sem localização falsa. cell-set é negativo separado, explicitamente excluído deste pacote.

Fonte vanilla lab/typst-original/crates/typst-library/src/model/table.rs:734–778 declara defaults e propriedades; typst-realize/src/lib.rs:367–386 prepara/guarda o argumento, :418–429 visita produto com profundidade, :491–505 distingue guards por elemento, :571–586 materializa propriedades. Tekt entities/elements/table_cell.rs:111–143 só projeta lexical ou plano posicionado; compiler/realize/table_cells.rs:110–129 reaplica Cell crua sem preparo; :280–304 termina snapshot inteiro; eval/show_rule_termination.rs:65–70 trata mesma forma como ponto-fixo; eval/rules/apply.rs:139–166 revoga regra globalmente durante callback. Classificação: campos, efeitos, ordem, terminação e erro causal são língua. A estrutura de guards e materialização upstream é mecânica e não é requisito. Não se infere intenção do upstream; a escolha nominal abaixo é inferência refutável pelos controles de aliases, duas regras, novas gerações e conteúdo ocultado.

### Responsabilidade exclusiva proposta

Adaptar literals ao campo standalone=None sem alterar sentido dos controles antigos. Acrescentar apenas controles necessários ao comportamento owned de visual/clone/maps/representação/HTML/geometria, sem espelhar a implementação.

### Gate, aceitação e incompletude explícita

Candidato não vigente até confirmação humana do manifesto D08B3-B: há um campo público novo e um tipo público novo; o contrato A aprovado não os autoriza. Após ratificação instalar somente estes candidatos, escrever testes e confirmar RED antes de implementação; selar apenas owners mudados. Aceitação: coletas acima/revisões adicionais bilaterais, efeitos uma vez, igualdade/getters, seleção, produto HTML e texto/estilos/geometria reais; mensagens/spans sem apagar diferenças. Cargo build/debug/release e suítes core/infra/shell apropriadas; crystalline-lint sem erros e owners sem V5/V6/V15/V26. Revalidar D08A/B2/B3-A, Raw defaults/blocos, Equation/math/matrix/query. Preservar árvore anterior por snapshot/diff restrito, sem commit/reset/resselo global.

Este recorte completa defaults intrínsecos da Cell isolada e distingue alias de nova geração de show. Continua incompleto no universo Cell: D08B3-C completa set table.cell/defaults de estilo (cell-set), mediante medição e L0 próprios; nenhuma promessa de prazo/implementação é legitimada por este scope-out. Defaults/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters/span multipágina, PDF/UA e política geral não-localizável ficam abertos. O warning preexistente de eval de ficheiro vazio não é corrigido ou normalizado para afirmar paridade.


## C2-R1 — testes de estilo local e largura Auto

Medição anterior: 00_nucleo/diagnosticos/performance-c2-2026-10-07/table-local-regression.json e samples/r{1,2,3}-table-local/samples.json, HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b dirty, hashes/UTC/fontes/argv completos. Vanilla pin9f2b6e871: primeiro Local tem largura21.880pt, seguinte x60.880; atual24.068pt, seguinte x38.999, sobreposição. Tamanho, coordenadas e contribuição são língua; bytes PDF não são gate.

Acrescentar testes E2E com layout_test: Cell em Styled local com fill deve herdar set text externo de10pt, conservar tamanho local explícito11pt e não sobrepor duas colunas Auto. Contar contribuições reais, verificar estilos e coordenadas, não a estrutura Rust. Registrar RED antes da correção; suíte completa e PDF bilateral com métricas reais obrigatórios. Nenhum novo modo/API/fase.

## Static delegation accepts Rust trailing argument commas — 2026-10-13

Measurement: `01_core/src/compiler/layout/mod.rs` retains the `Content::Space`
and `Content::SpaceFromNewline` calls to `space::layout` with identical arguments,
but rustfmt expands them with a trailing argument comma. The structural test
`atomized_remaining_behavior_arms_delegate_statically` then fails because its
whitespace-only comparison treats `Markup,)` differently from `Markup)`.
Decision: canonicalize only whitespace and a trailing comma immediately before
`)` for this structural observer. Preserve the required variant, static callee,
argument expressions and full arm. No dispatch, language behavior or public
contract changes are authorized by this test repair.

## Clippy cleanup in the layout harness — 2026-10-13

Measurement: the pinned Rust 1.92 Clippy run preserved in
`00_nucleo/diagnosticos/correcoes-producao-2026-10-13/clippy-final.jsonl` reports
uses of deprecated `FrameItem::Text` and its fields in this test-only consumer.
The owner `00_nucleo/prompts/entities/layout_types.md`, P484, defines Text as
the permanent pre-shaping output of L1; shaping with font bytes belongs to L3.
The same run identifies unused test imports/bindings, repeated `#[test]` on
`p1073_ref_table_supplements_en_e_pt`, and obsolete private fixture helpers.

Decision: keep pre-shaping Text observations. Record an explicit, reasoned
`expect(deprecated)` on this test-only module; do not suppress this lint in
production or suppress other lint classes. This expectation must become a
warning itself if it is no longer exercised. Remove unused imports and unused
private helpers, keep evaluation of unused tuple outputs under underscore
bindings, and keep exactly one registration of each test. Equivalent assertion,
collection-membership, borrow and fixture-initialization spellings may be
simplified without changing inputs, tolerances, conditions, evaluation counts
or observables. No test witness may be deleted and no failing behavioral
expectation may be normalized away. Preserve text, styles, page and geometry
assertions and run the full layout test module after cleanup.

## Expectativa de obsolescência encerrada — 2026-10-13

Medição anterior à decisão: a rodada pre-shaping-ci-2026-10-13, sobre HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b e working tree não commitado,
removeu a marcação P483 do owner permanente de Text, sem mudar seus dados
ou pipeline. O Clippy subsequente (`clippy.jsonl`, conservado em evidence)
reporta unfulfilled_lint_expectations neste consumer:17. Fontes e L0 estão
em `00_nucleo/diagnosticos/pre-shaping-ci-2026-10-13/layout-tests-before.json.gz`;
a proveniência completa da árvore está em before.json.gz da mesma rodada.

A expectativa temporária de deprecated da limpeza anterior cumpriu seu
papel: tornou-se aviso quando deixou de ser exercida. Remover esse expect
e os 24 allow(deprecated) locais deste consumer, todos associados à mesma
marcação antiga de Text. Esta cláusula supersede somente a obrigação
anterior de manter expect(deprecated). Conservar integralmente todos os
helpers, cenários, dados, condições, tolerâncias e assertivas; nenhum teste
é ignorado ou retirado. Revalidar compilação, suíte core e Clippy sem criar
outra supressão. Não altera produção, API, defaults ou fase de pipeline.

## Observação geométrica de Box — 2026-10-13

Medição anterior à decisão: box-dimensoes-2026-10-13/probe.json.gz demonstra
que folhas comitadas fora do grupo, soma indevida de inset/outset e recorte
condicionado só à altura violam o contrato de caixa. Testes históricos
como :7587 e :10366 procuram folhas somente na raiz e :10730 exige ausência
de máscara se o corpo cabe. Essas condições não são observáveis de língua.
Para a composição corrigida, adaptar somente tais observadores para
descender grupos com posição/matriz, mantendo texto, dimensões, estilos,
baselines e tolerâncias. A máscara pode existir também quando não há
overflow; exigir que o conteúdo que cabe continue visível. Nenhum outro
fixture/esperado pode ser relaxado para aceitar regressão. Preservar testes
fracionários vigentes enquanto a etapa B não implementar sua composição.

core-first.log revela doze observadores de Box presos a folhas na raiz.
Atualizá-los para frame_items_recursive, conservando tolerâncias. P1028
com width=40pt e inset=3pt esperava indevidamente 46pt; corrigir para 40pt,
conforme a dimensão total comprovada em box-dimensoes. P248 height-cabe
deve exigir texto ainda contido, aceitando a máscara declarada; height-none
com clip=true deve exigir conteúdo preservado e máscara de tamanho natural
em vez do antigo let _ sem assert. O controle clip=false continua sem
máscara em toda a árvore. Não mudar o teste P1491 de baseline: ele revelou
regressão produtiva e exige transportar o resultado medido corretamente.

## Origem geométrica de Place em região local — 2026-10-14

### Medição anterior à decisão

place-origem-local-2026-10-14/{before,extra-before}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Box-place publica shape em y37.75pt contra28pt no canvas480×400 (bbox raster aproximado); no SVG fonte, box sem margem do corpus tiling tem y14.834 local contra5. O deslocamento cresce com fonte. placement.rs:463 veta origem geométrica dentro de qualquer subframe e :533 conserva o ascender local; place.rs:96–131 soma text top-edge mesmo para Top explícito. Root-text shape diverge29.5/22.25pt; Box-text32/22.25; Grid e Align são controles positivos. Fonte pinada flow/distribute.rs:703–715 separa align_y explícito de offset de fluxo. Isso é geometria de linguagem; não obrigação de buffers Rust. A hipótese é refutada se texto, VAlign, scopes, fixups ou checkpoint perderem coordenadas após origem única.

### Responsabilidade exclusiva

Adicionar testes de linguagem/geométricos com layout_test: Place de forma em Box fixo/auto/inset/aninhado/outer Place; VAlign Bottom/Horizon; variação de fonte; linha pendente shape/text e Top explícito; controles de Align/Grid/Table. Usar composição afim real para medir posição, sem aparar ou normalizar deslocamento errado. Preservar testes P1292 e de paginação existentes.

### Aceitação e etapas

Esta seção supersede a restrição histórica de body geométrico apenas na raiz de Place. L0 primeiro e RED→GREEN em fluxo interno ADR-0127: sem campo público/trait/assinatura/fase/default novos. Comparar antes/depois SVG/PDF e preservar corpos textuais, mistura, páginas automáticas, clipping, posições observáveis e floats; build/workspace/rustdoc/fmt/linhagem/inventário.
A origem do body e a âncora após fluxo comitado são defeitos distintos. Esta etapa A não fecha âncora depois de Shape/Parbreak/Linebreak nem inset de Block, já refutados pelos controles. A etapa B de place-origem-local completa o fim causal do fluxo e a região de Block; não declarar Place universal pronto enquanto B estiver aberta. O transporte público de Tiling permanece pendente de aprovação em seu pacote, sem alteração por esta correção.

### Retificação de cinco expectativas de baseline — medição antes da decisão

`layout-attempt3.log` conserva cinco falhas, todas com diferença de top-edge7.7pt em FixedMetrics. P898 Place raiz (tests.rs:20900–21010) já inclui `+top` ao medir FrameItem::Text.pos, mas P908 aninhado usa a fórmula do topo do frame sem a baseline interna. `baseline-expanded/results.json.gz` e `candidate-expanded/results.json.gz` reproduzem fontes anteriores exatas, restauram o candidato por hash e comparam o pin: Align bottom auto passa bbox de y12.75 para20pt, igual ao vanilla; candidatos com dy=-5 tornam o marcador visível para comparação. Align horizon após bloco alto passa y212.75 para220pt, mas vanilla tem233.25pt: ainda falta o espaçamento de bloco13.2pt, pendência distinta, não fechada por esta retificação. Os casos mistos também têm diferença residual registrada. Fonte upstream distribui o frame pela altura e conserva a baseline interna; não igualar baseline de texto ao topo geométrico.

Corrigir P898 Align bottom/horizon e P908 Place Align/Align duplo/Transform para medir a baseline como topo + text top-edge, preservando altura final e testes de finitude/dy. P908 mantém seu controle interno de scope Parent não flutuante criado diretamente em Rust; isso não autoriza tal construção pela linguagem, que continua rejeitada. Não declarar paridade universal de Rotate/Align. Adicionar teste de restauração de origem entre subframes e checkpoint; os testes de geometria existentes seguem sem relaxamento.

### Origem X do subframe — testemunho adicional

Os registros baseline-expanded/candidate-expanded mostram Place dentro de Align/Align duplo em x0.25pt contra20.25pt no vanilla, tanto antes como depois do reparo vertical. `layout_align` inicia o body em line_start_x absoluto e recompõe só delta_x; o ramo subframe de layout_place usa zero e perde essa origem. Usar `regions.current.line_start_x.0` como origin_x nesse ramo (Column/Parent), preservando o caminho de célula e o body local zero. Box/Transform já iniciam essa referência em zero. Adicionar RED de Align simples/duplo com Place e conferir o corpus de altura auto. Não alterar resolução de alinhamento horizontal ou largura do envelope.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Adicionar regressões root/Box após Linebreak/Parbreak, Shape, Block vazio/textual/fixo/auto, Grid, Align e VSpace; preservar Top explícito, overlays sucessivos, página nova e aninhamento. Usar coordenadas de linguagem e métricas injetadas, nunca redefinir esperados pelo resultado.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.

O controle já existente de checkpoint/LIFO deve também registrar uma fronteira física, verificar sua restauração após subframe, invalidá-la ao mover o cursor sem evento certificado e recuperá-la pelo checkpoint. Isso refuta tanto vazamento do filho quanto uso de âncora antiga em cursor novo.

## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

Regressões root/Box para inset simétrico/assimétrico, Top/Bottom/Horizon, aninhamento, texto, auto e clip. Verificar dimensões da máscara e coordenadas dos filhos, não apenas existência. Preservar controles anteriores, checkpoint e região após o Block.

### Aceitação e incompletude explícita

Correção interna/de paridade ADR-0127, sem campo público, modo ou fase. Este adendo substitui somente as fórmulas incompatíveis de região/width/clip anteriores; não legitima outros reparos sem medição. L0 primeiro, RED→GREEN, validação bilateral, suíte e resselo restrito. B2a cobre região local e clipping na região sobrevivente. B2b completa fragmentação de Block, dimensões/outset e integração dos metadados/decoradores que controles adicionais refutarem; B3 continua responsável por baseline tardia de inline Box/Transform e produtores ainda não certificados. Não declarar objetivo completo com essas frentes abertas.

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
consumer as construções/desestruturações antes da edição: [173, 181, 23206].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Adicionar testes da fronteira ratificada: body de Box não fracionário e Block em região única ganha raiz incluindo inset; backing usa raiz externa; nested e Place/Transform preservam o escopo. Testar transportes em tradução/offset/slicing com true e false. Observações de geometria compõem matrizes e não exigem número arbitrário de wrappers; não mudar valores esperados de coordenadas para acomodar regressões.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.

### Observadores das regressões históricas

layout-first.log registra folhas ausentes em buscas planas após o novo
envelope, embora o body permaneça no frame. Os observadores dos testes
afetados devem usar a vista recursiva com projeção afim, preservando todos
os valores de geometria/texto/tolerâncias. Disponibilizar o helper somente
a árvore de módulos de teste. Não alterar esperados para tornar uma
regressão visual verde. A fixture de tiling reconhece células como raízes
locais; máscaras técnicas externas continuam soft.


## Observador exaustivo de raiz — 2026-10-14

Medição anterior: paint-root-2026-10-14/observer-cleanup/before-followup-architecture.log
registra V16 em tests.rs:24369 e notas V19/V20 no mesmo observador novo.
O curinga vazio ignora silenciosamente futuras variantes do domínio.
Neste helper cfg(test), enumerar todas as variantes de FrameItem, mantendo
recursão individual por Semantic/TextReplacement/Link e coleta apenas de
Shape sólido. Separar o pattern aninhado de fill sem mudar os observáveis
ou expectativas. Novas variantes devem exigir decisão explícita na compilação.
A suíte completa anterior não é atribuída à edição posterior do helper;
reexecutar os quatro testes afetados e checks de compilação/linhagem.


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

Adicionar regressões de largura Auto em Shape, Empty, Place isolado e misto, Block aninhado/explícito, inline Box, Grid e Stack; backing e paint_root devem concordar. Cobrir espaço literal vs markup e isolamento do estado/rollback após a implementação privada. Não exigir paridade de altura/alinhamento ainda refutados nem relaxar valores existentes.

### Aceitação e limites

Correção interna/de paridade ADR-0127: nenhum contrato público, default ou
fase novo; L0 primeiro, RED→GREEN, resselo restrito. A largura é uma parte
da correção B2b/T1–T3 já em andamento: altura Auto/backing, alinhamento tardio,
fracionários, fragmentação, outset e PDF Parent continuam com critérios próprios.
Não declarar render aprovado por testar apenas o carrier. Preservar controles
bilaterais completos e refutadores, suíte/build/doc/fmt/linhagem/inventários.


A execução green-first do lote de largura revelou gutter escalar ignorado
em grid.rs:171. Conservar o esperado68 do controle de tracks20+30/gap4,
implementando primeiro o contrato horizontal no owner de Grid. Não reduzir
o esperado para64 nem apresentar esse controle como certificado vertical.


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

Adicionar refutadores de posição do filho em Auto com clip, left/center/right, irmã maior, nested/fixed Block, Link e isolamento de estado; não trocar dimensão esperada para acomodar o defeito. Manter controles de texto e decoração como medição separada.

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

Migrar observadores/fixtures de Shape para FramePaint/FrameStroke explicitamente. Nos testes de cor sólida, exigir a variante Solid (sem to_color/fallback); conservar valores, coordenadas, tolerâncias, strokes e assertions. Observadores genéricos da página continuam sem entrar na célula de paint. Testes específicos de tiling devem acessar a célula preparada e sua raiz implícita em vez de assumir Groups de repetição.

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

## Transformações — integração aprovada em 2026-10-14

Medição antes da decisão: corpus transformacoes-2026-10-14 e style-contract/results.json
registram o pin e os comandos/UTC. Fora de context há erro; aliases preservam
identidade; with não admite set/getter/show; dois set origin dobram por eixo.
A fixture de layout/tests.rs:21648 constrói matrix diretamente. Manifesto
aprovado 960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d.

Migrar a construção Rust direta de TransformElem na fixture de Place para operation: Matrix sem mudar expectativas. As demais fixtures Rust genéricas continuam exercitando a compatibilidade Matrix.

Validar corpus e testes causais; não confundir construção com layout ou release.

## Regressões de região transformada

Medição: `transformacoes-2026-10-14/refuters-first/results.json.gz`, casos
reflow-wrap e reflow-wrap-rotate, e fonte pinada transforms.rs:205–218.
Verificar que a região ajustada altera as quebras observáveis de palavras
antes da transformação; reflow false conserva as quebras originais.
Exercitar estilos herdados e aliases. Observar coordenadas globais com projeção
afím; não substituir aceitação por estrutura de buffers nem alargar tolerâncias.

A sonda margin-before confirma overflow falso por margem física em coordenadas
locais. Conservar regressão de caixa com margem grande e palavra maior que a
caixa: primeira palavra não cria linha vazia; palavra seguinte inicia outra linha.

O inventário `transformacoes-2026-10-14/png-afim/validation/followup-clippy-inventory.json`
identifica f64::sin/cos proibidos nos novos observadores. Usar libm pinado para
a projeção afim determinística. O filtro das duas palavras do teste de limiar
é pertencimento ao conjunto explícito, sem guard lógico composto; valores e
tolerâncias dos testes permanecem.

## Regressões da largura lógica transformada

Medição: logical-before-complete/results.json.gz do diagnóstico de
transformações, com pin, árvore, fontes e UTC. Reflexos de palavra/forma/Box
maiores que a região usam a caixa lógica limitada, sem recortar a tinta.
Adicionar testes de coordenadas globais e controles de espaço terminal fraco
versus literal na escala absoluta. Não comparar representação de matrizes.

A fonte pinada inline/linebreak.rs:78–113 e as sondas absolute-markup/literal
distinguem espaço na quebra automática de espaço literal no fim do parágrafo.
Testar escala absoluta na linha quebrada, preservando o controle literal
terminal. O controle spacing-only exige também preservar h explícito; testar
a distância entre palavras quando h vem após elas.

O refutador terminal-fraction em logical-spacing-after/results.json.gz
exige regressão de ocupação total após h(1fr)/h(2fr) terminal sob escala
absoluta; h(0fr) continua sem expansão. Verificar distâncias renderizadas,
sem inspecionar o acumulador nem usar a implementação como oráculo.

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

Testar documentos com Ratio/Relative/em, resolução no estilo do uso, Box
explícito/auto/aninhado, Block com inset e altura explícita/auto, Transform
dentro de Block, restauração da região no irmão e página ilimitada. Observar
dimensões das formas emitidas e conferir a revalidação bilateral; não fazer
da estrutura privada do acumulador o oráculo. Defaults e casts continuam
refutadores abertos do próximo passo, não expectativas reescritas.

Correção interna/de paridade ADR-0127, sem API/trait/campo público nem fase
nova. L0 primeiro, RED→GREEN, resselo restrito, corpus, suíte e verificadores.
Preservar origem, pintura, contornos, recursos e efeitos; a medição especulativa
existente não ganha execução nova.


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

Adicionar regressões de linguagem para forma terminal, duas formas,
forma→texto, texto→forma e as duas ordens Shape/Rotate, fronteira explícita
de página e VSpace forte/fraco final. Conferir dimensões/posições e conteúdo
preservado, sem afirmar paridade a partir dos campos privados de estado.

Correção interna ADR-0127: sem assinatura/campo público, fase ou modo novo.
L0 primeiro, testes RED→GREEN, resselo restrito, corpus bilateral e suíte.
Defaults de formas, alinhamento RTL, Block Auto e colapso geral de espaços
continuam abertos; este contrato não autoriza encerrar esses eixos.


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

Testar a alocação vertical observável de Block e seu backing, restauração
do fluxo exterior e continuação por texto/shape, altura explícita/Auto,
corpos vazios/textuais, inset e aninhamento. Outset amplia só pintura/clip,
sem antecipar paginação. Usar conteúdo e geometria globais; não substituir
asserts por inspeção de flags internas. A medição de fronteiras abaixo
corrige a hipótese anterior sobre P248: altura80 cabe, altura90 não cabe;
outset não deve alterar essa decisão. Cobrir ambos os lados do limite.

Sem API/fase nova. RED→GREEN e corpus bilateral; não relaxar esperados
legítimos nem certificar fragmentação por aprovar a região única.


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

Adicionar regressão de origem após pagebreak, tanto corpo textual quanto
forma com backing/inset; conservar os casos de colunas existentes que medem
One−Two e caudas. Linha fechada e espaço forte devem preservar a fronteira
causal sem inserir top-edge extra. Atualizar P248 para testar alturas80/90
com outset0/30 e conteúdo preservado, sem consolidar expectativas antigas
refutadas pela medição. Fonte maior isolada e Align vertical continuam
pendências explícitas. Nenhuma API/fase nova; RED→GREEN e resselo restrito.


## Altura causal de Align — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/align-flow-before/results.json.gz`
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

Cobrir Top/Horizon/Bottom após Block e após texto, fim da páginaAuto e
continuação textual; controlar alturas declaradas sem tinta e Align vazio.
Verificar origem após pagebreak e posições em página fixa. Atualizar P898
para incluir a margem de parágrafo medida, mantendo os controles de altura
finita, presença e baseline. Nenhuma expectativa refutada é preservada só
para obter teste verde; nenhum teste horizontal é relaxado. Registrar RED
antes da implementação e executar o conjunto de layout após a correção.


Refutador de continuação: os documentos tail21/25/29 de align-flow-before
mantêm a mesma baseline de X e POST no pin. Os novos testes devem comparar
as posições entre Top/Horizon/Bottom sob páginaAuto, pois a altura isolada
não detecta recomposição que sobrepõe irmãos. Nenhuma folga vertical surge
de incluir a altura dos irmãos na página final.


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

Adicionar regressões fonte→frame para par.spacing0/5/2em antes/depois e entre
Blocks, prioridade de above/below zero, e dois Blocks1fr com baseline de B
60/62.5/70 na página100 margem10. Verificar texto, altura e posições com
métricas top10/bottom0, preservando os esperados anteriores sem mascarar
as pendências documentadas. Observar envelopes recursivamente.


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

Adicionar REDs fonte→frame: wrapper18 único versus dois wrappers18;
Strong dentro do wrapper; Block/Align após texto; caudas TAIL, Styled18 e
Linebreak após1fr com spacing0/5/2em e bordas explícitas. Medir baselines,
páginas, conteúdo e alturas, com observador recursivo. P1381 deixa de ser
um teste que legitima fallback de duas páginas: exigir uma página e posições
medidas de Lead/One/After/Two/Tail, sem trocar somente a contagem para verde.
Preservar regressões de prefixos mistos/wrap/colunas e scoping após irmãos.


### Refutadores de extensão e primeira região

scope-tail-after-first/results.json.gz confirma26/28 documentos. Os dois
restantes são caudas com cinco linhas que excedem a primeira região; reservar
altura integral deixa o início32/17pt acima. O primeiro fragmento deve ser
medido sob a capacidade disponível, usando a mesma admissão de viúvas/órfãs,
e somente sua altura causal entra na reserva anterior ao1fr. Próximas regiões
preservam sua capacidade normal; não somar margem terminal de parágrafo.

scope-tail-refutators/results.json.gz mede os três controles antigos que
exigiam caudas desalinhadas. Três fragmentos numa linha agora coincidem
integralmente. Multiword21 e par40/8 com fill ainda divergem em entrelinha:
a medição isolada reproduz essa geometria defeituosa, não a certifica.
O novo fallback de prefixo geométrico exige uma única baseline textual;
prefixos multiline conservam os certificados escalares anteriores ou a rota
opaca. Caudas multiline continuam sob medição integral/primeiro fragmento.
Não alterar as asserções opacas desses dois prefixos para declará-los verdes.
O aninhamento parcial OUTER18 + Styled8 contendo Block também continua
incorreto no pin; um segmento local que entrou com linha exterior aberta
não prova o escopo do parágrafo inteiro e não recebe novo fechamento.


scope-tail-third.log refuta a largura nominal da fixture P1383: FixedMetrics
quebra End, enquanto as métricas reais do PDF mantêm três trechos numa linha.
Para o novo controle de prefixo geométrico sem wrap, usar página240 (coluna105)
e manter as posições/fontes/cauda esperadas. Os controles multiline opacos
preservam a página200 original. Isso distingue métricas injetadas de paridade
real, já medida separadamente; não redefine uma quebra errada como acerto.


## Prefixo multiline e aninhamento parcial — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz`
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

Substituir os dois negativos nomeados p1392_wrap_com_delta_composto_permanece_opaco e p1389_multiword_permanece_opaco por regressões positivas: uma página, After/Tail230pt, entrelinhas do escopo comum, cor e texto preservados. Acrescentar nested OUTER18/INNER8 com BODY49.8pt sob FixedMetrics e controles de restauração. Não alterar expectativas dos certificados legacy para esconder regressão.

Verificar também o transporte privado causal: snapshot restaura cadeia/mode depois de alteração; subframe isolado não herda o modo e restaura ambos para o pai sem mudar sua geometria.

A medição prefix-nesting-controls refuta restauração completa: AFTER18 permanece12pt acima depois de BODY8. Acrescentar regressão de margem automática versus below0/5 explícitos: baselines98.4/76.8/81.8 sob FixedMetrics, sem alterar a expectativa do corpo ou transformar margens fortes em fracas.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F013: 01_core/src/compiler/layout/tests.rs:703 (walk); F014: 01_core/src/compiler/layout/tests.rs:1610 (has_shape); F015: 01_core/src/compiler/layout/tests.rs:1635 (has_text); F016: 01_core/src/compiler/layout/tests.rs:1662 (collect_text); F017: 01_core/src/compiler/layout/tests.rs:3017 (sum_block_limites_empilhados_verticalmente); F018: 01_core/src/compiler/layout/tests.rs:3063 (sum_inline_usa_right_scripts); F019: 01_core/src/compiler/layout/tests.rs:3162 (align_duas_linhas_tem_ys_distintos); F020: 01_core/src/compiler/layout/tests.rs:8322 (layout_stack_ltr_empilha_horizontalmente); F021: 01_core/src/compiler/layout/tests.rs:8369 (layout_stack_spacing_avanca_cursor_entre_children); F022: 01_core/src/compiler/layout/tests.rs:8392 (layout_stack_spacing_avanca_cursor_entre_children); F023: 01_core/src/compiler/layout/tests.rs:8437 (layout_block_height_forca_minimo_vertical); F024: 01_core/src/compiler/layout/tests.rs:8466 (layout_block_height_forca_minimo_vertical); F025: 01_core/src/compiler/layout/tests.rs:10290 (extract_texts); F026: 01_core/src/compiler/layout/tests.rs:11588 (p248_table_cell_overflow_preserva_fill_stroke_externos); F027: 01_core/src/compiler/layout/tests.rs:11760 (p248_boxed_height_overflow_cross_radius_clip); F028: 01_core/src/compiler/layout/tests.rs:12638 (extract_texts_rec); F029: 01_core/src/compiler/layout/tests.rs:13520 (text_fragments); F030: 01_core/src/compiler/layout/tests.rs:17880 (walk); F031: 01_core/src/compiler/layout/tests.rs:18061 (walk); F032: 01_core/src/compiler/layout/tests.rs:22250 (find_abs_y); F033: 01_core/src/compiler/layout/tests.rs:23150 (collect_texts); F034: 01_core/src/compiler/layout/tests.rs:23304 (p994_box_em_math_produz_caixa); F035: 01_core/src/compiler/layout/tests.rs:24370 (smartquote_rendered_text). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `00_nucleo/diagnosticos/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F013 — `walk`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F014 — `has_shape`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: Shape: kind: ShapeKind::Line { .. }.

- **F015 — `has_text`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F016 — `collect_text`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F017 — `sum_block_limites_empilhados_verticalmente`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F018 — `sum_inline_usa_right_scripts`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F019 — `align_duas_linhas_tem_ys_distintos`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`crate::entities::layout_types::FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F020 — `layout_stack_ltr_empilha_horizontalmente`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F021 — `layout_stack_spacing_avanca_cursor_entre_children`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard.

- **F022 — `layout_stack_spacing_avanca_cursor_entre_children`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard.

- **F023 — `layout_block_height_forca_minimo_vertical`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard.

- **F024 — `layout_block_height_forca_minimo_vertical`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard.

- **F025 — `extract_texts`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F026 — `p248_table_cell_overflow_preserva_fill_stroke_externos`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; Group: clip_mask: Some(ShapeKind::Rect).

- **F027 — `p248_boxed_height_overflow_cross_radius_clip`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard.

- **F028 — `extract_texts_rec`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F029 — `text_fragments`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F030 — `walk`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F031 — `walk`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F032 — `find_abs_y`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard.

- **F033 — `collect_texts`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F034 — `p994_box_em_math_produz_caixa`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: Shape: stroke: Some(_).

- **F035 — `smartquote_rendered_text`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## TS1 — tamanho completo de texto (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação obrigatória

Este candidato altera dois contratos públicos, descritos em api.md. Não é
L0 vigente até aprovação explícita. Hash do Código antigo é apenas contexto,
não resselo de implementação inexistente. Materializar somente após ratificar
TS1; sem release/merge/push. Testes S1–S16 e gates sob12GiB, swap zero.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.
