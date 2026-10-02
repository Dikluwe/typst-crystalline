# L0 — Layout engine coordination
Hash do Código: 6057cf99

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/mod.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/mod.rs:240`, `01_core/src/compiler/layout/mod.rs:711`, `01_core/src/compiler/layout/mod.rs:1129`, `01_core/src/compiler/layout/mod.rs:1219`, `01_core/src/compiler/layout/mod.rs:1418`, `01_core/src/compiler/layout/mod.rs:1435`, `01_core/src/compiler/layout/mod.rs:1455`, `01_core/src/compiler/layout/mod.rs:1923`, `01_core/src/compiler/layout/mod.rs:2218`, `01_core/src/compiler/layout/mod.rs:2446`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

Native dispatch and state storage are specified here. Checkpoint capture/restore and terminal document composition belong to `00_nucleo/prompts/compiler/layout/checkpoint.md` and `finish.md`.

This is the exclusive owner of `01_core/src/compiler/layout/mod.rs`: Layouter construction and state, causal dispatch, public façade entrypoints, attempt coordination, and compatibility helpers actually defined here. Checkpoint and terminal composition behavior is delegated to the child owners named above. Child owners define their own layout behavior; importing or calling them does not transfer ownership to this prompt.

### State and injected dependencies

Keep the existing Layouter API, generic FontMetrics/ImageSizer injection, tracked introspector and runtime. No filesystem, network, clock, environment access or Func execution is introduced. The active region holds cursors, line start, emitted items, pending line, dimensions and horizontal fractions. PageConfig and region dimensions stay synchronized when a configuration is installed on an empty page.

Preserve the chain/cached TextStyle, locator seed/current Location, document language, page/column and cell contexts, parent bbox, read-only runtime, figure progress, enum/list grouping, smartquote and smallcaps state. Preserve existing deferred buffers for floats, cell tails, footnote bodies, attachments, page/line/equation numbering, alignment paths, repeat and fractional natural widths, decoration collectors, warnings and errors. The hub owns this storage and its lifetime; it does not re-specify child formulas.

Fractional Block allocation remains private optional state: absent allocation is not a zero allocation. Preserve the preflight re-entry guard, terminal fractional-tail flag, one-shot validated-prefix line arm, assumed ascent and inline ascent/descent accumulators. Child policies may set/consume these through the existing descendant interface; no public entity or alternative state machine is created.

### Causal dispatcher and transaction boundary

`layout_content_once` performs the single canonical `is_locatable` gate before its exhaustive Content match. An eligible occurrence advances Locator exactly once, makes current_location available to descendants and records its current physical page and point. Structural recursion and visual children do not allocate extra Locations by heuristic. Measurement does not add emission events.

Native dispatch remains static and exhaustive with thin descendant delegation in the render layer, as required by ADR-0109 form B. Keep the closed Content enum and the current explicit transparent/no-op branches for already-consumed metadata/state/counter updates, structural entry carriers, alignment points and HTML-only elements. Retain the existing Dynamic extension adapter without turning native dispatch into dyn/vtable/string/PropMap dispatch or moving render into entities.

When a stabilized float marker leaves a checkpoint pending, `layout_content` opens the transaction before the next occurrence and routes top-level dispatch to `00_nucleo/prompts/compiler/layout/flow_suffix.md`; nested dispatch remains part of the same atomic unit. The hub provides capture/restore, while `00_nucleo/prompts/compiler/layout/flow_suffix.md` owns acceptance/rejection/replay decisions and `00_nucleo/prompts/compiler/layout/cursor.md` owns regional progress.

### Checkpoint integrity

A FlowSuffixCheckpoint records the causal state and append-only tail boundaries of both active/cell regions, completed pages, collectors, floats, attachments, numbering/alignment/repeat buffers, warnings/errors, columns, and all associated geometry/style/runtime/locator state actually captured by this consumer. RegionTailCheckpoint restores cursor/line-start/dimensions and truncates emitted items, pending line and fraction boundaries together.

Rollback restores the complete speculative suffix, including already-emitted Place items and causal counters/positions, not merely current_line. It reconstructs Locator from the captured seed, restores callback-related numbering state and one-shot arms, truncates buffered candidates/fixups, and clears transaction/replay flags. Committed prefix floats and pre-boundary items remain outside rollback. Missing structural storage where a captured region/collector should still exist is a debug invariant violation, not invented replacement state. Do not copy the checkpoint into children, inspect future Content variants or apply generic translation outside rejected replay.

### Attempt coordination and public entrypoints

Preserve the existing public layout, introspector/metrics, callback-aware, locator-seeded, intrinsic HTML-frame and real-measurement façade paths and signatures. The metrics entrypoint returns SourceResult and diagnoses unresolved math callbacks rather than returning a placeholder document; the legacy FixedMetrics convenience entrypoint retains its current expect behavior.

The convergence child installs fresh math and par.line pass state for each candidate Layouter. Each attempt restores the same supplied LocatorSeed and uses the tracked introspector, injected metrics/sizer and current known label/figure/table/page stores. Carry forward extracted page maps until they stabilize within the existing retry limit of five attempts. The source currently runs this convergence comparison regardless of the computed has_outline flag: do not restore an obsolete “no outline means exactly one attempt” promise.

Return Complete only with a stable candidate and successfully consumed math/par.line transcripts; otherwise return Pending requests without exposing the tentative document. A rejected candidate's transcript is discarded, not merged into its successor. Exhausting the retry bound returns the existing empty Pending request sets, not a completed candidate. The mathematical callback protocol remains owned by its math prompt; the par.line protocol belongs to `00_nucleo/prompts/compiler/layout/par_line_callbacks.md`. No eval ↔ layout phase change or layout-time Func execution is introduced.

The convergence child discovers the first bibliography's supplied style/locale and constructs a pure CSL cache only for an explicitly named or already-resolved style. Retain cache-conversion diagnostics in each candidate and the local fallback when no style is requested. `00_nucleo/prompts/compiler/layout/bib_csl.md` owns cache interpretation; `00_nucleo/prompts/compiler/layout/bibliography.md`/`00_nucleo/prompts/compiler/layout/cite.md` own rendering. The BibEntry compatibility body formatter belongs to `00_nucleo/prompts/compiler/layout/bibliography.md`, preserving conditional field order and organization-as-publisher fallback.

### Terminal composition

`finish` closes the final line through `00_nucleo/prompts/compiler/layout/line_commit.md`, preserving its causal last baseline/bottom. Request finite auto dimensions from `00_nucleo/prompts/compiler/layout/page_geometry.md` before terminal float/footnote emission. Drain pending floats and footnote bodies through Cursor hooks, retaining the bounded extra-region loop for remaining note bodies.

Create the final Page under the existing material/empty-page-run conditions. Apply descendant line-number, equation and horizontal/vertical alignment fixups in their current order, delegate explicit running layers and canvas construction, preserve fill/bleed/supplement, and initialize Page.logical_number to the physical index. Numbering in an automatic margin uses the injected known logical number when available, otherwise the physical index. Function numbering consumes already-realized visible Content; it is never executed here.

Preserve the terminal-only coordination for compound page-number patterns: record the page index, chosen current number and marginal geometry, then format with the final known logical total or physical page total fallback after all pages exist. Intermediate assembly belongs to `00_nucleo/prompts/compiler/layout/page_commit.md`; the hub does not absorb its consumer or duplicate a general page-transition policy.

Transfer pages, document language, attachments, label pages/positions, sealed occurrence positions, figure/table page vectors, warnings and errors into PagedDocument. Do not mutate the injected introspector or fabricate logical overlays belonging to the pipeline.

### Delegation boundaries

Line closure/allocation/physical line numbering: `00_nucleo/prompts/compiler/layout/line_commit.md`. Baseline/wrap/regional transition and marker hooks: `00_nucleo/prompts/compiler/layout/cursor.md`. Sequence preflight/grouping/spacing: `00_nucleo/prompts/compiler/layout/sequence.md`. Isolated sublayout: `00_nucleo/prompts/compiler/layout/sub_frame.md`; constrained/intrinsic measurement: `00_nucleo/prompts/compiler/layout/measurement.md`. Geometry/fixups: `00_nucleo/prompts/compiler/layout/placement.md`, `00_nucleo/prompts/compiler/layout/equation.md`, `00_nucleo/prompts/compiler/layout/page_geometry.md`; page layers: `00_nucleo/prompts/compiler/layout/page_running.md`, `00_nucleo/prompts/compiler/layout/page_canvas.md`. Child element behavior is specified by its own exclusive prompt, including `00_nucleo/prompts/compiler/layout/style_scope.md`, `00_nucleo/prompts/compiler/layout/space.md`, `00_nucleo/prompts/compiler/layout/parbreak.md`, `00_nucleo/prompts/compiler/layout/align.md`, `00_nucleo/prompts/compiler/layout/context_block.md`, `00_nucleo/prompts/compiler/layout/state_display.md`, `00_nucleo/prompts/compiler/layout/counter_display.md`, `00_nucleo/prompts/compiler/layout/counter_display_callback.md`, `00_nucleo/prompts/compiler/layout/outline_entry.md`. Column behavior remains in `00_nucleo/prompts/compiler/columns.md`, which is outside this sanitation scope.

The coordinate and math-callback nuclei pinned above remain the shared invariant authority; child calls are boundary references, not additional consumers or duplicated normative ownership.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/mod.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Manter match exaustivo magro; Space e SpaceFromNewline delegam à mesma unidade Space com origem Markup e Newline respectivamente, sem mudança de fase.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/p1431_line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `00_nucleo/diagnosticos/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

The empty initial document commits one page using the resolved PageConfig, including its dimensions, margins and layers. Do not delegate an empty page to an exporter default. Preserve the existing suppression of a trailing empty page-run boundary after committed pages.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Register text_measure, text_tracking, text_baseline, break_selection, word_layout and region_progress as private sibling descendants. Keep the native exhaustive dispatcher, shared state, public facade, checkpoints and terminal assembly unchanged. Each new module has its own exclusive prompt; no new public contract, default, pipeline phase or dynamic dispatch is introduced.

## Regional demand correction — 2026-09-28

### Evidence before decision

The 30 bilateral controls in `00_nucleo/diagnosticos/estudo-correcao-falhas-texto-2026-09-28/` measure terminal pages 2/1, explicit terminal pagebreak 3/2, and continuation 3/2 (Tekt/vanilla). The isolated pre-export control emits Four/additional on page 2 with baseline 101.84pt and page height 100pt. The causal sources are line_commit.rs:657-696 (hypothetical next baseline), region_progress.rs:113-128 (post-line transition), cursor.rs:80-135 and parbreak.rs:41-80 (paragraph baseline/spacing). Current source and L0 hashes precede edits in `00_nucleo/diagnosticos/correcao-demanda-regional-2026-09-28/initial.json` and `before/`. This is internal parity correction under ADR-0127, not a public contract/default/pipeline change.

### Decision and acceptance

The existing cursor prediction represents next-line demand without new state. finish drains the last fitting line on its causal page and does not manufacture a trailing empty page from that prediction. Keep existing final Page conditions, intentional empty pages and explicit pagebreak handling. Preserve checkpoint fields, region/column and pending insertion protocols.

### Integration controls preceding the refinement

The first implementation's 36 real-font controls are preserved in first-after-pdf/ and first-pdf-green.log (before any later rebuild). terminal-columns still loses Three; terminal-footnote changes from 2 to 1 pages and its raster shows note/main overlap. Column start returns before clearing previous block spacing; terminal note fitting uses cursor_y and must retain the conservative following-line clearance rather than the terminal geometry cursor. These measurements require the refinements below, without changing the footnote body format, reservation model or public contract.

Preserve a local copy of the predicted terminal baseline for footnote clearance when a material final line closed. Compute auto geometry and flush floats with the causal terminal cursor as before; then, only while pending footnote bodies require fitting, use max(causal cursor, predicted clearance). Retain that clearance across a deferred body and its next_region attempt. Restore the causal cursor if no physical-page/column transition occurred; after a transition preserve the new region cursor. This keeps the existing conservative note fitting budget without manufacturing a page when no note demands it. No new stored field, note reservation model or formatting change.

### Bounded note-drain refinement

The refined regression retains two pages for a deferred note, but the pre-existing terminal loop calls next_region twice: the first opens a region and the second drains the note then opens an empty successor. After each required next_region in finish, immediately drain notes in that newly opened region using the existing bottom override and bounded iteration count. A note that fits there must not need another transition to become visible. Preserve the loop's defensive body count bound, existing forced-too-large body behavior, and the causal-cursor restoration rule above.

## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

Registrar o descendant privado paragraph e delegar Content::Par ao seu layout, sem lógica de negócio no match. Adicionar apenas o boolean privado terminal_paragraph_line, inicial false, capturado/restaurado por FlowSuffixCheckpoint. finish captura essa evidência antes do flush vazio; trata a baseline já fechada como geometria terminal e conserva o clearance previsto para notas, junto com had_last_line existente. Os outros estados, fachada pública, drenagens e paginação permanecem.

### Medição adicional: parágrafos vazios e altura automática

`00_nucleo/diagnosticos/correcao-composicao-paragrafos-2026-09-28/empty-details/results.json` registra pin, binários, HEAD, working tree e hora. O vanilla distingue corpo vazio de Text(""): sozinho mede altura 16pt contra 24.568pt, com margens 8pt; antes de AFTER, Text vazio acrescenta o top-edge 8.568pt à posição de par[]; após BEFORE a altura automática é 38.968pt para vazio e 47.536pt para Text vazio. O teste composition_empty_styled_terminal_auto_height_keeps_explicit_spacing falhou no estado após a primeira implementação (styled-empty-red.log): fechar antes do estilo perdeu a evidência causal anterior.

Decisão interna: o fechamento anterior a Styled/Par preserva terminal_paragraph_line quando houve material. Body vazio mantém spacing entre fluxos; Text vazio, atravessando Styled/Sequence, também reserva seu top-edge, sem tinta. Acumular em campo privado empty_paragraph_weak_spacing os espaçamentos iniciais enquanto initial_baseline_pending; no término inteiramente vazio, sem bloco material, remover apenas esse spacing fraco, preservando o top-edge virtual. Incluir o campo no checkpoint/rollback. Não reduzir spacing depois de material nem de vazios consecutivos entre fluxos. Nenhuma interface pública muda.

## Preparação e seletores — 2026-09-28

### Medição anterior à decisão

O lote bilateral `00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem tracking/math/smallcaps, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Declarar quatro módulos privados de planejamento. Armazenar guard boolean paragraph_planning_suppressed inicial false; capturar/restaurar em FlowSuffixCheckpoint. Não colocar algoritmos no hub.

## Hifenização global — 2026-09-28

### Medição anterior à decisão

O lote before-pdf de `00_nucleo/diagnosticos/correcao-hifenizacao-global-2026-09-28/` mede 67 entradas contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8; estado inicial, binários, fontes e comandos estão em initial.json, before/ e proveniencia.json. paragraph_prepare.rs:80-94 recusa a política true, paragraph_select.rs:66 considera só finais de palavras, paragraph_plan.rs:18 escolhe estratégias já transportadas, break_selection.rs:41-66 hifeniza apenas por overflow local. Vanilla inline/linebreak.rs:170-209 usa os mesmos candidatos no greedy, :626-684 aplica 135*(1+0.15*(max(5-left,0)+max(5-right,0))) por hífen, mais 135 em duas linhas consecutivas; inline/shaping.rs:583-635 desenha o glyph de '-' mas conserva SHY no texto copiado. Isto mede comportamento e contratos, não intenção histórica.

### Decisão

Correção interna ADR-0127 em fluxo contínuo: sem campo/trait/assinatura pública, modo novo, fase ou quebra de compatibilidade. A restrição histórica de hifenização desligada é supersedida nesta rodada para palavras latinas de estilo/política uniformes, inclusive pontuação terminal simples. Mistura dentro da palavra, apostrofes internas, outras quebras UAX14, largura variável e objetos mantêm fallback. Texto copiado '-' versus SHY continua limitação de export independente; a geometria usa o glyph de '-' separado, como fonte medida. Aceitação mede palavras/posições/estilos e política lexical, normaliza somente o hífen de quebra para a comparação geométrica e declara a divergência textual separadamente.

Declarar o módulo privado paragraph_hyphen; nenhuma fórmula, despacho dinâmico ou contrato público no hub.

## Proteção regional de viúvas/órfãs — 2026-09-28

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28/before-pdf/` registra 78 entradas e `before-extra/` registra 24 controles, com estado completo em initial.json e before/, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. page-65-100%-0%-False deixa três linhas e uma no Tekt, mas duas e duas no vanilla; custos zero coincidem. Tiny/large-positive têm a mesma proteção vanilla. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` mede habilitação estritamente positiva, primeiros dois/últimos dois e proteção integral de três linhas; `flow/distribute.rs:343-364` só avança por necessidade agrupada quando cabe na próxima região. paragraph_emit.rs emite linha por linha sem essa necessidade. A semântica observável é a distribuição de linhas/palavras em regiões, não os frames/Child do vanilla. Não inferir intenção do scope-out antigo.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127: nenhum campo/trait/assinatura pública, flag ou fase nova. Supersede a exclusão anterior de widow/orphan exclusivamente para parágrafos preparados com métricas verticais uniformes, sem baseline offset, geometria paginada constante, sem colunas/células/subframes/notas/floats ou transação regional ativa. Os demais contextos conservam fallback e limites explícitos. Não promete algoritmo completo nem desempenho.

O módulo privado paragraph_pagination possui exclusivamente certificação vertical, agrupamento e admissão regional. Recebe ranges já escolhidos, não refaz seleção, não emite glyphs nem duplica a lógica de new_page. A certificação inteira precede emissão; todos os fragmentos devem ter size/arestas/leading iguais ao estilo exterior (negrito/itálico admissíveis se as métricas coincidirem), e finite nonnegative top/descent/leading. Bounds auto não precisam de proteção. Respeitar a baseline inicial e o colapso existente de block-chain antes de comparar necessidade real com page_bottom_limit. Novo espaço disponível é calculado com margens resolvidas para a página seguinte. Avançar só se há material na região atual e o grupo cabe na próxima. Grupos muito altos conservam progresso ordinário e conteúdo íntegro; nunca repetir new_page em laço.

Helpers privados de text_costs resolvem widow/orphan independentemente dos pesos hyphenation/runt, reutilizando FIELDS, com default Ratio 1 e teste estrito > 0. O default de estratégia do planejamento passa a Simple para parágrafo comum sem linebreaks, permitindo a preparação necessária à proteção padrão; a escolha automática por justify e todos os contextos recusados permanecem. Emitir sem alterar justificação, carriers, última linha aberta, fronteiras e restauração lexical. Preservar notas e colunas por não aplicar proteção incompleta nesses caminhos.

Aceitação: RED→GREEN com mesmas palavras distribuídas 2+2 versus 3+1, proteção integral de três, orphan desligado independentemente, grupo excessivo, restauração e default sem flag. Sondas bilaterais com caixas/palavras/páginas, tipos/spans anteriores preservados e controles de fallback. Não usar grupos XML como número físico de linhas. Resselo restrito aos owners, build e crystalline-lint sem erros.

## Separação interna de tracking e baseline — contrato vigente

### Evidência anterior à decisão

`01_core/src/compiler/layout/text_measure.rs:85` e `word_layout.rs:89` acumulam fronteiras e alinhamento de carriers; fontes e hashes anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A movimentação conserva a correção previamente medida.

### Responsabilidade do hub

Registrar dois descendentes privados: text_tracking, owner `00_nucleo/prompts/compiler/layout/text_tracking.md`, e text_baseline, owner `00_nucleo/prompts/compiler/layout/text_baseline.md`. O hub somente declara os módulos; regras de fronteira e baseline ficam nos owners. Preservar dispatcher exaustivo, contratos públicos e checkpoints.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Armazenar text_tracking::State em campo privado ao módulo pai, inicializado por Default. Capturar/clonar e restaurar integralmente no FlowSuffixCheckpoint. O hub não possui fórmulas de tracking, alocação de identidade ou decisões de continuidade: delegar ao descendente. Nenhum campo público/trait/assinatura/flag/fase muda.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.

## Linhas causais vazias em página auto — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/before/results.json` mede fontes→PDF com binários pinados e entradas sem newline artificial; `initial.json` guarda HEAD, working tree, hashes e snapshots. `#text("")` e a variante 18pt têm Tekt 3×3pt versus vanilla 3×8.568/12.852pt. `abc\n\n` termina em Tekt 19.872×31.536pt versus vanilla 19.872×8.568pt, enquanto `abc\n\nxyz` coincide a 19.872×31.536pt. `#text("  ")\n\nabc` fica numa linha Tekt (26.112×8.568pt), mas em duas linhas vanilla (19.872×31.536pt). Fonte: `text.rs:37-86` não emite item para Text vazio; `html_frame.rs:72` exclui todo TextSeparator de materialidade; `line_commit.rs:280-286`, `parbreak.rs:43-48` e `mod.rs:1965` usam essa materialidade para linha; `parbreak.rs:40-85` avança cursor após fechar linha e `page_geometry.rs:180-201` usa cursor se não há fundo confirmado. Vanilla `inline/linebreak.rs:695-699` oferece breakpoint obrigatório até para texto vazio. Isto mede o observável; não infere a intenção histórica nem exige a mecânica do vanilla.

### Contrato interno segregado

Na finalização, reconhecer uma última linha apenas literal como linha causal para usar a sua baseline final; manter o match exaustivo e todos os outros predicados de materialidade. Nenhum estado global, campo público, contrato externo, default ou fase muda.

Aceitação: testes RED→GREEN e matriz bilateral, regressões anteriores, build/lint sem erros; resselo somente dos owners deste manifesto.

## Atomização da convergência — 2026-09-30

A fachada mantém as entradas públicas, o estado Layouter e o dispatcher estático. A preparação de candidatos, transporte de páginas conhecidas e decisão Complete/Pending pertencem exclusivamente a `00_nucleo/prompts/compiler/layout/convergence.md`. A descoberta do primeiro style bibliográfico permanece auxiliar dessa coordenação. As obrigações históricas de convergência acima são contrato preservado, não ownership duplicado. O formatter bibliográfico de fallback pertence exclusivamente a `00_nucleo/prompts/compiler/layout/bibliography.md`.

## Protótipo de seleção limitada — 2026-09-30

Medição anterior à decisão: `paragraph_plan.rs` liga a seleção exata de `paragraph_select.rs`; `mod.rs` declara apenas o módulo existente. O hub declara `paragraph_select_bounded` apenas sob `cfg(test)`, com owner exclusivo em `00_nucleo/prompts/compiler/layout/paragraph_select_bounded.md`. Não altera a fachada, o despacho ou o caminho produtivo.

## Protótipo de seleção com início fixo — 2026-10-02

Medição anterior à decisão: `paragraph_select.rs` percorre finais com inícios variáveis; `paragraph_select_bounded.rs` é test-only. Declarar o novo descendente experimental `paragraph_select_forward` somente em `cfg(test)`, com owner exclusivo `00_nucleo/prompts/compiler/layout/paragraph_select_forward.md`. Sem alteração do planejamento, façade ou default de produção.
