# L0 — tests
Hash do Código: b0b88fb6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c


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

## D08B2 — presença lexical proposta; não vigente antes do gate ADR-0127

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

### Contrato proposto e responsabilidade exclusiva

Fixtures TableCell especificam presence coerente com seus argumentos explícitos. Preservar todos os observáveis/assertions de render existentes; não relaxar tolerâncias nem fabricar defaults conhecidos. Testes não autorizam mudanças produtivas adicionais.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Esta proposta muda o contrato público por campo/tipos novos: só materializar depois da confirmação humana do manifesto. Não altera fase/defaults de render.
