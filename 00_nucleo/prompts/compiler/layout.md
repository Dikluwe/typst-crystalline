# L0 — Layout engine coordination
Hash do Código: b85dda8e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml sha256:ecf965471eedd8599209f0575811dd5b456dce1c43baa26881251e24e8ec6c6d
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:dd8c7661f2246385e2aed1737e8494d778b9f563b7ac709fc3c750a23fe50596
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:42175932c75f37ed19205d1d82526969999e18fba8fed16f1b1dcff6e418bc0f
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/layout/subframe-inline-extents.toml sha256:d9b9c00602c2c68bbe82eb37a84cc01a4e0575bf84f5df8950b86b7f4dab7dd3
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8



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


## Realização com avaliação única

Layout usa Location explícita de Located, sem realocar seu corpo; Realized registra âncoras das origens consumidas no início do produto e emite somente visual. Ocorrências produzidas registram suas próprias posições. Carrier pendente projeta corpo; a realização deve resolvê-lo antes do layout normal.


## Realização com avaliação única

Separar despacho de corpo de alocação de Location para não contar duas vezes o elemento dentro de Located. Lógica dos elementos continua nas unidades de layout.


## Realização com avaliação única

Layouter conserva flag privada de reserva inicial por instância. Reservar as Locations explícitas do body inteiro uma vez antes de emitir ocorrências novas; não transformar reserva em índice temporal.


## Realização com avaliação única

Inicializar reserva por Layouter. Sub-layouts possuem sua própria instância e conservam os seeds existentes.


## Realização com avaliação única

Primeira ocorrência reserva a faixa explícita antes de gerar IDs ordinários; layout e introspecção usam o mesmo serviço puro de reserva.


## Reuso de identidades preparadas — 2026-10-05

O backtrace produtivo e a sonda RED apontaram a reconstrução do índice de cada origem Realized no hub. Declarar o módulo descendente realized e delegar registro de origens a sua free function estática, emitindo somente visual depois. A lógica e a prova de completude pertencem exclusivamente a `00_nucleo/prompts/compiler/layout/realized.md`; estado continua no Layouter. Atomização forma B, sem nova fachada, fase, trait ou despacho dinâmico.

## Direção de parágrafo distinta de direção de run — 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/direcao-alinhamento-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes, binários e treze sondas. rtl_lines põe ABC em x=20 no Tekt e x=152.949220 no vanilla pinado; rtl_inline_only e rtl_scope coincidem à esquerda. ltr_lines põe árabe em x=174.567870 no Tekt contra x=20 no vanilla. Em `cursor.rs:156–162` e `line_commit.rs:210–216`, a âncora depende do primeiro item da linha. Vanilla `flow/collect.rs:129–136` obtém trunk_from_pairs e `inline/mod.rs:189,232` resolve direção/alinhamento compartilhados antes das linhas. Semântica observável: direção dos trechos e âncora de todas as linhas do parágrafo. Estratégia de state/preflight é mecânica interna, não cópia do vanilla. Inferência: preservar direção compartilhada evita perder a âncora em linhas só com override; refutada por controles integralmente estilizados/parbreak/wrap.

Correção interna/de paridade ADR-0127, fluxo contínuo L0 primeiro e RED→GREEN. Apenas campos privados de Layouter e helpers descendentes, nenhuma entidade ou assinatura pública/fase/default novo. Esta rodada certifica conteúdo textual estrutural (Text, Empty, Space/SpaceFromNewline, Linebreak, Sequence, Styled e Strong/Emph); unidades desconhecidas conservam fallback anterior. Não prometer ordem bidi mista, align explícito, HTML ou todas as formas de parágrafo. As equações existentes de largura/tracking permanecem.

Responsabilidade deste owner: Armazenar dois slots privados Option<Dir>: direção ativa do escopo de parágrafo e direção da cauda textual pendente após saída do escopo. Inicializar None. O owner paragraph governa resolução/consulta; o hub não implementa preflight. Nenhum campo público/FrameItem muda.

Aceitação: RED→GREEN em parágrafo RTL com linha LTR e inverso, escopo integral e reset por parbreak; checar checkpoints e subframes, texto/posições em CLI bilateral, build e lint sem novos findings. Slots e helper não transportam metadados ao L3; divergências de ordenação bidi ou align explícito ficam registradas separadamente.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.

Guard privado bidi_scope_active distingue ausência de cursor certificado de ausência de escopo; após recusa integral, filhos não certificam um subconjunto parcial. Checkpoint e subframe conservam/suspendem/restauram este guard.

## Block/Align — correção interna 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-align-2026-10-06/baseline-verification.json.gz` registra 14 controles completos, fontes, comandos, caixas e binários no HEAD/working tree/UTC de before.json. RTL Block automático põe ABC em x20 no Tekt contra x152.949220 no vanilla; Block(width:100pt) coincide em x100. #set dir:ltr no body não altera a âncora externa do Block automático no vanilla. Align(left/right) mistura bordas por direção do run e desloca Y por 8.35742pt. `placement.rs:35-46,176-177` renderiza no subframe com baseline pronta e subtrai ascender na recomposição; `cursor.rs:154-162` só aplica direção RTL; `block.rs:175-185` posiciona container externo apenas quando largura é explícita. Fonte vanilla pinada `flow/collect.rs:193` resolve Align por linhas e `flow/block.rs:145-166,285` distingue regiões expandidas de auto-size. Paridade de posições/forma, não cópia da mecânica (ADR-0107).

### Responsabilidade exclusiva

Armazenar explicit_line_alignment: Option<HAlign> como campo privado, inicial None. O cálculo pertence ao cursor e a instalação lexical pertence a placement. Não alterar API pública nem enum/entidade.

Correção interna/de paridade ADR-0127 em fluxo contínuo. Nenhum contrato público, modo, fase, compatibilidade ou unsafe novo. O certificado é limite explícito desta rodada e deve ser refutado por nested Align, Block de largura explícita, scoping após Align, auto dimensions, texto regular e corpus anterior. RED→GREEN, revalidação bilateral, builds/lint e resselo somente do manifesto. Controles negativos não são redefinidos como aceitos. Sem promessa de ganho de tempo.

## Célula textual certificada — refutação 2026-10-06

### Medição anterior à decisão

focused-candidate-verification.json.gz e candidate-source.json.gz em
`00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06/` preservam candidato
com largura/âncora corretas mas ordem ABC/123/árabe incorreta em RTL.
bidi_origin.rs:111-117 recusa todas as células/subframes; o input integral
não chega à reordenação L3. cell_body.rs:223-235 classifica Text e TextShaped
como texto para rebase de baseline, mas não TextSeparator, introduzindo
um delta de top-edge nos separadores da mesma linha. São duas causas
internas refutáveis por ordem/baseline ainda divergentes após certificação
limitada e recomposição consistente. Correção de paridade interna, sem
contrato público, fase, flags de produto ou unsafe novo.

### Responsabilidade exclusiva

Armazenar somente cell_bidi_allowed privado, false no constructor. Capacidade lexical de emissão, não estado mutado pelo fluxo especulativo; não adicionar contrato público ou nova fase.


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

Armazenar SmartQuoteContext privado no Layouter, fresco por fluxo isolado,
substituindo a independência dos dois flags antigos. Compartilhar entre nós
de markup/programáticos e Styled/Strong/Emph no mesmo fluxo. Observar texto
lógico emitido, separadores e objetos inline no dispatch causal, delegando
seleção de quote ao owner lang/quotes e emissão a layout/smartquote.
Match fechado permanece magro; helpers privados de observação podem envolver
a chamada de unidade, sem duplicar pair resolver. Captura/restore delega a
checkpoint. Medição/rollback não consome o contexto confirmado duas vezes.
Sublayout independente possui estado próprio; integração em parent observa
objeto, sem importar o texto interno como prefixo. Preservar bidi/locator,
footnotes/cols/floats, styles, budgets e recursos; não ampliar guards de reuso
por aparente igualdade dos glyphs. Não fazer contexto em layout de entidades.

## D05 — fronteira do produto textual de equação de bloco

### Medição antes da decisão

Equation-where-2026-10-06/delivered.json.gz, pin 9f2b6e871, binários/HEAD/diff/UTC
preservados: show-block emite 𝑎 DISPLAY na mesma linha no Tekt e DISPLAY em linha
própria no vanilla. O filtro e produto coincidem; a fronteira de bloco perde-se.
layout/mod.rs:1172–1175 emite visual de Realized sem considerar a origem;
paragraph.rs:114–130 já realiza o segmento textual em fronteira própria.

### Decisão e aceitação

O arm Realized delega emissão/registro à free function realized::layout,
sem corpo de lógica no hub. Prova de completude, âncoras e preservação do
bloco textual transformado pertencem exclusivamente ao owner realized.md.

Fluxo contínuo ADR-0127, L0 primeiro e RED→GREEN; manter fallback fora do
recorte, build/lint e nova medição bilateral. Não é novo modo/default deliberado.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Layout Source é transparente; delegar ao body sem nova sentinela, locator, geometria ou replay de origem. Preservar estilos existentes.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/layout/mod.rs` nas linhas 556, 698, 875 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `00_nucleo/diagnosticos/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Conservar o argumento f64 posicional legado do construtor, marcando o binding _font_size: o estilo inicial continua derivado exclusivamente da StyleChain default, como no código medido. Não restaurar tamanho 12 contra o default da chain. Restringir os campos internos bidi_cursor e pending_repeats à visibilidade privada de layout, onde seus tipos e todos os consumers residem (incluindo descendentes/checkpoint). Não alterar a API externa nem mover os tipos.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.

## Limites de uma região isolada — 2026-10-13

Medição antes da decisão: mod.rs:1040-1078 subtrai margens da página
mesmo em subframe. sub_frame.rs instala width=origin+largura+margin.right
e altura já local; portanto available_width resulta origin+largura-margin.left
e available_height perde margens novamente. Os snapshots em
box-composicao-2026-10-13/before.json.gz preservam a fonte/HEAD/estado/UTC.
O teste de região local com margens externas deve refutar essa subtração.

Quando is_sub_frame, available_width deve ser width-line_start_x-margin.right,
limitada inferiormente a zero; available_height e page_bottom_limit usam
a altura da região, sem margens externas. A região finita local prevalece
mesmo se a dimensão da página externa é auto. Fora de subframe, preservar
as fórmulas vigentes. Isso corrige fórmulas privadas ADR-0127, sem novos
campos, APIs ou fase. Testes de limites locais e controles de página comum,
seguido de build, suites e linhagem; não importar política de Box ao hub.

## Resultado de extensões inline — 2026-10-13

Medição anterior à decisão: core-first.log em box-composicao-2026-10-13
registra falha de measure_preserva_baseline_causal_de_texto_box_vazio_e_composto.
measurement.rs:509-512 lia line_inline_ascent/descent vazados pelo subframe;
a restauração correta do pai removeu esse canal implícito. O esperado
17pt para caixa 20pt com shift 3pt permanece correto e não será relaxado.
A fonte/owner e o estado anterior estão no snapshot before.json.gz desse
lote. Correção privada ADR-0127, sem campo/assinatura pública ou fase novos.

Armazenar/inicializar last_sub_frame_inline_extents: (f64, f64) como metadado privado, inicialmente (0,0), análogo a last_sub_frame_width. Nenhum campo de entidade ou API pública muda.

O núcleo pinado fixa a distinção resultado/estado externo. Exigir o teste
P1491 verde com os mesmos valores e a suíte de checkpoints/layout.

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

Representar line_assumed_ascent como Option<f64> privado, inicializado em None. Some(0) é baseline instalada válida; manter demais estados e API pública. Não criar strut tipográfico por inferência do zero.

Correção interna ADR-0127, sem API, default ou fase novos. L0 primeiro,
RED→GREEN, controle de ascent zero, sondas de geometria e suíte completa.
Não substituir esperados de regressão para conservar o defeito.

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

Declarar o módulo privado position_marker e guardar uma lista append-only privada de destinos capturados. Nas duas entradas de registro canônico (locatable e Located), manter observe/next/página/ponto e delegar a escrita e captura ao helper. A lista começa vazia; nenhuma API pública ou ordem de Locator muda.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

00_nucleo/diagnosticos/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
00_nucleo/diagnosticos/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

Declarar o módulo privado link_capture e os vetores privados de destinos e escopos ativos, inicialmente vazios. Não alterar APIs nem a alocação de Location.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.

## Origem geométrica de Place em região local — 2026-10-14

### Medição anterior à decisão

place-origem-local-2026-10-14/{before,extra-before}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Box-place publica shape em y37.75pt contra28pt no canvas480×400 (bbox raster aproximado); no SVG fonte, box sem margem do corpus tiling tem y14.834 local contra5. O deslocamento cresce com fonte. placement.rs:463 veta origem geométrica dentro de qualquer subframe e :533 conserva o ascender local; place.rs:96–131 soma text top-edge mesmo para Top explícito. Root-text shape diverge29.5/22.25pt; Box-text32/22.25; Grid e Align são controles positivos. Fonte pinada flow/distribute.rs:703–715 separa align_y explícito de offset de fluxo. Isso é geometria de linguagem; não obrigação de buffers Rust. A hipótese é refutada se texto, VAlign, scopes, fixups ou checkpoint perderem coordenadas após origem única.

### Responsabilidade exclusiva

Adicionar estado pub(super) sub_frame_origin_y:f64, inicial zero, representando somente a origem de recomposição vertical do subframe. Não alterar assinatura pública, entidade nem política de layout no hub.

### Aceitação e etapas

Esta seção supersede a restrição histórica de body geométrico apenas na raiz de Place. L0 primeiro e RED→GREEN em fluxo interno ADR-0127: sem campo público/trait/assinatura/fase/default novos. Comparar antes/depois SVG/PDF e preservar corpos textuais, mistura, páginas automáticas, clipping, posições observáveis e floats; build/workspace/rustdoc/fmt/linhagem/inventário.
A origem do body e a âncora após fluxo comitado são defeitos distintos. Esta etapa A não fecha âncora depois de Shape/Parbreak/Linebreak nem inset de Block, já refutados pelos controles. A etapa B de place-origem-local completa o fim causal do fluxo e a região de Block; não declarar Place universal pronto enquanto B estiver aberta. O transporte público de Tiling permanece pendente de aprovação em seu pacote, sem alteração por esta correção.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Armazenar pub(super) flow_placement_end:Option<(f64,f64)> (fim físico,cursor preparado), inicial None. install_page_config limpa a evidência.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.

## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

Armazenar block_place_region:Option<Rect> privado, inicial None; configuração nova limpa. Nenhuma entidade pública nova.

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

Armazenar flow_frame_extent:(f64,f64) privado (origem,largura), inicial (0,0), e last_sub_frame_flow_width:f64 inicial zero. Não alterar last_sub_frame_width legado nem API pública.

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

Armazenar block_auto_width:bool privado, inicial false; configuração nova limpa. Não modificar API pública.

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

Armazenar relative_region_size privado opcional; None no root. Representa tamanho real do contexto de porcentagens durante subframes, separado do sentinel de altura usado para evitar paginação. Não altera entidade ou assinatura pública.


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

Guardar apenas Option<StyleChain> text_segment_chain e bool geometric_text_lines privados no Layouter, com None/false iniciais. Nenhuma interface pública muda. A lógica pertence aos owners descendentes.


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

- `C017`: `01_core/src/compiler/layout/mod.rs:1911`, `layout_with_introspector_and_metrics_and_callbacks_from`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C017` — `layout_with_introspector_and_metrics_and_callbacks_from`: Exceção pontual justificada

Fachada pública seeded do layout; delega ao coordenador de convergência os oito inputs completos. O L0 manda preservar caminhos e assinaturas de callbacks e LocatorSeed.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C017: Fachada pública seeded do layout; delega ao coordenador de convergência os oito inputs completos")], somente nesta função.

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

Materializar somente as expectativas diretas nominais já justificadas neste owner: C017. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.
