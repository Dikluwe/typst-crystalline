# L0 — line commit
Hash do Código: 852ad07f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/line_commit.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/line_commit.rs:20`, `01_core/src/compiler/layout/line_commit.rs:66`, `01_core/src/compiler/layout/line_commit.rs:189`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

The initial-block no-typography regressions reach the empty-line flush in
`01_core/src/compiler/layout/line_commit.rs:358`: text edges are requested even
when material-line classification is false. The intermediate failure is preserved
in `00_nucleo/diagnosticos/font-demand-geometry-study/green.stdout`.
The empty flush still owns draining/resetting state; eliminating an unused font
query does not authorize skipping that protocol.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own expansion of horizontal fractions/repeats, justification and physical line closure, including baseline/leading, causal decorations and line numbering. The hub retains buffers and checkpoints. Delegate region progress through the cursor facade to `00_nucleo/prompts/compiler/layout/region_progress.md`, repeat multiplication to `00_nucleo/prompts/compiler/layout/repeat.md`, and callback matching/final marginal-width adjustment to `00_nucleo/prompts/compiler/layout/par_line_callbacks.md`. Do not move page assembly or Sequence preflight here.

For a finite line, sum weights without requiring positive individual weights or a positive sum. Add provisional natural fractional-box widths back to distributable remaining space. A share is `max(weight / total * remaining, 0)` only when the ratio and remaining are finite; otherwise zero. Replace provisional occupancy by share-minus-natural-width; h(fr) has natural width zero. Associate a repeat with its actual boundary, shift its suffix once, insert copies before that suffix and repair later indices by the actual insertion. Infinite width/orphan state clears pending work defensively without looping.

Justification expands fractions first, then distributes only over opportunities before material content ends, preserving the present RTL behavior and link/semantic bounds. No opportunity is invented after the last material item.

Consume the validated-prefix one-shot arm at the first ordinary flush; explicit certified closure clears the arm and uses the same closing path. Empty/direct-marker-only lines drain carriers but add no typography or page. Material lines combine actual text and inline extents. Distinct text sizes, or multiple text items differing from the active outer style, select outer next-ascent/leading while retaining the current maximum/ascent/descent. Correct a positive difference from the assumed ascent by translating the entire line and collectors together.

A certified larger singleton uses the outer next-ascent/leading; a certified smaller singleton keeps its own baseline/ascent/next-ascent and only substitutes outer leading. Ordinary singleton behavior is unchanged. When float reservation makes ink exceed the effective region, reject the hub transaction or request ordinary Cursor transition; never migrate only current_line. Any material line whose real ink fits does not create a page merely because a hypothetical next baseline does not fit; reserved/fractional protocols remain otherwise unchanged.

Number material non-subframe lines only after stabilizing baseline and causal page. Preserve document/page scope, none/absent neutrality, pattern formatting and Func request/realized-Content consumption without Func execution. Missing resolution records a request rather than inserting a provisional number. The last physical column uses the end margin; earlier columns retain the configured margin. Keep region-local greatest-width fixups, clearance, horizontal alignment and the existing overlay suppression/edge compensation. Under a segmented pattern context, buffer baseline/page/pattern/chain candidates without immediately incrementing or formatting; callbacks stay on their existing request path. Columns owns consolidation and shared-style capture.

Drain the line in order into emitted items, clear justification opportunities, record causal baseline/advance/descent, reset cursor_x to line start and request any remaining necessary transition. Preserve typed style, visual position, content order and diagnostic behavior; Rust buffer shape and exported PDF bytes are not language-parity gates.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

When the expanded pending line has no material items, use neutral local text-edge
values without consulting FontMetrics. Continue normal drain/reset and marker
transport. Material lines retain current and next style-edge measurements and
baseline/leading/overflow behavior. Initial-block tests in the image/shape owners
exercise this shared path with an injected metrics provider that rejects font queries.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/line_commit.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Justificação conserva a mesma fórmula para posições visíveis. Atualizar pos e advance do carrier pela diferença de deslocamento entre suas duas extremidades, sem participar nas métricas visuais; excluir trailing carriers das oportunidades pelo limite de tinta existente.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Keep horizontal allocation, physical baseline/leading, item/metadata transport, causal numbering and line commit here. Delegate only the existing final post-line overflow/progress request to region_progress, passing the existing had_items and effective_line_committed evidence. The reserved-ink rejection/transition before line commit and its flow-suffix transaction behavior remain unchanged. Do not change the terminal line protocol or its known empty-page limitation in this separation.

## Regional demand correction — 2026-09-28

### Evidence before decision

The 30 bilateral controls in `00_nucleo/diagnosticos/estudo-correcao-falhas-texto-2026-09-28/` measure terminal pages 2/1, explicit terminal pagebreak 3/2, and continuation 3/2 (Tekt/vanilla). The isolated pre-export control emits Four/additional on page 2 with baseline 101.84pt and page height 100pt. The causal sources are line_commit.rs:657-696 (hypothetical next baseline), region_progress.rs:113-128 (post-line transition), cursor.rs:80-135 and parbreak.rs:41-80 (paragraph baseline/spacing). Current source and L0 hashes precede edits in `00_nucleo/diagnosticos/correcao-demanda-regional-2026-09-28/initial.json` and `before/`. This is internal parity correction under ADR-0127, not a public contract/default/pipeline change.

### Decision and acceptance

Certify real ink of every material line, not only a reserved/fractional region: after baseline stabilization, use current baseline plus max(actual text descent, inline descent) against the effective bottom before advancing the cursor. Retain existing reserved-region migration/rejection and fractional-tail rules. Drain/reset, decorations and line numbers keep their order. A fitting line's next baseline is a prediction and cannot itself create a new page or reject a fitting suffix. Do not suppress real overflow rejection.

## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

Fornecer flush_continued_line privado: se par.justify Bool(true) chamar justify_current_line existente, então flush normal. Manter fórmula por oportunidades, expansão de fr, sem esticar linha final nem alterar explicit false/true. Todo caminho comum de fechamento limpa terminal_paragraph_line antes de fechar; o owner Paragraph arma a evidência só após seu fechamento final. Preservar ink certification, rollback e wrapped singleton. Adicionar controles de posições/baselines e final natural.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Justificação reconstrói TextReplacement com os filhos justificados e text intacto, sem deslocamento adicional do envelope.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Alinhamento da linha RTL continuada — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/after-pdf/results.json` e `summary.json` medem 66 fontes/198 PDFs com fonte, pin e estado em `provenance.json`; `wrap-initial.json` registra o estado exato antes desta segunda correção. Em latin-wrap ±0.2/0.333/1.2pt, a primeira/segunda linha do Tekt deslocam-se relativamente ao vanilla por exatamente um intervalo ±t, enquanto a última coincide. O texto e as linhas escolhidas coincidem. `line_commit.rs:241-248` conhece se há continuação, mas `:477` chama `cursor.rs:150` sem essa informação; `cursor.rs:164-177` ancora pela direita real. A inferência de um intervalo terminal pendente é refutada se controles de wrap/hifenização mostrarem outro deslocamento. Esta medição não prova uma regra universal para bidi misto ou espaços iniciais.

### Contrato interno

flush_continued_line sinaliza continuação somente ao fechamento que ele provoca quando não há justificação ativa; a linha justificada conserva o caminho anterior, preservando prefix arm, baseline e página. flush_line ordinário e flush_wrapped_inline_prefix_line permanecem terminal/não continuados. Usar parâmetro privado na chamada de fechamento, sem novo estado em Layouter nem checkpoint; encaminhar sinal ao alinhamento RTL. Nenhum valor numérico fixo no fechamento.

Mudança interna ADR-0127 em fluxo contínuo, sem assinatura pública, default ou fase. Teste RED→GREEN e controles bilaterais, preservação dos terminais e LTR, build/lint sem erros e resselo restrito.

## Linhas causais vazias em página auto — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/before/results.json` mede fontes→PDF com binários pinados e entradas sem newline artificial; `initial.json` guarda HEAD, working tree, hashes e snapshots. `#text("")` e a variante 18pt têm Tekt 3×3pt versus vanilla 3×8.568/12.852pt. `abc\n\n` termina em Tekt 19.872×31.536pt versus vanilla 19.872×8.568pt, enquanto `abc\n\nxyz` coincide a 19.872×31.536pt. `#text("  ")\n\nabc` fica numa linha Tekt (26.112×8.568pt), mas em duas linhas vanilla (19.872×31.536pt). Fonte: `text.rs:37-86` não emite item para Text vazio; `html_frame.rs:72` exclui todo TextSeparator de materialidade; `line_commit.rs:280-286`, `parbreak.rs:43-48` e `mod.rs:1965` usam essa materialidade para linha; `parbreak.rs:40-85` avança cursor após fechar linha e `page_geometry.rs:180-201` usa cursor se não há fundo confirmado. Vanilla `inline/linebreak.rs:695-699` oferece breakpoint obrigatório até para texto vazio. Isto mede o observável; não infere a intenção histórica nem exige a mecânica do vanilla.

### Contrato interno segregado

Fechar linha apenas de TextSeparator Literal como linha textual causal, incluindo métricas do estilo de seus carriers no máximo da linha, sem tratá-los como tinta para alinhamento, justificação ou HTML. Uma linha literal fechada reinicia x e atualiza a baseline; nada é duplicado no PDF.

Aceitação: testes RED→GREEN e matriz bilateral, regressões anteriores, build/lint sem erros; resselo somente dos owners deste manifesto.

### Refinamento medido antes da decisão

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/after-legacy33/results.json` mostrou regressão de altura em `#text("  ")`: Tekt 9.898pt versus vanilla 8.568pt após reconhecer a linha literal, porque `text_baseline::admit` só era chamado por Word/Chunk e o fechamento usava cursor sem baseline admitida. Ao fechar uma linha formada apenas por TextSeparator Literal, chamar o helper existente de admissão antes das métricas; ele reposiciona os carriers na baseline e a linha conserva o mesmo contrato de altura de uma linha textual.
