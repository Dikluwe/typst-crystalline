# L0 — line commit
Hash do Código: 0c854e09

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:9d9d6c265a3840cc5a3b8e121f403bc9e3fe55b1e586ab9014948f0784de5c3e
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:42175932c75f37ed19205d1d82526969999e18fba8fed16f1b1dcff6e418bc0f
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:04f91fc931871165d4fef399ea9c2e21e8b75f7fd57031183a4e2aed0ed1b62a
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78


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

## Máximo efetivo da linha — correção de paridade 2026-10-08

Medição bilateral após a equivalência de fluxo: `context` tem baseline PDF
15.238 pt no Tekt e 14.580 pt no vanilla; `text-dense` desloca apenas os
dez termos da última linha em 0.658 pt. Sondas `#set text(size: 10pt)` com
texto direto e contextual repetem o delta, enquanto `#par[Page 1]` coincide.
GDB em `line_commit.rs:476` mostrou `first_text_size=10`, `max_font_size=11`
e `max_style` exterior 11 pt, `assumed_ascent=6.58`, `current_text_top=7.238`
e `line_extra_ascent=0.658`. `line_commit.rs:300-328` inicia o fold no
estilo exterior, de modo que nenhum item de 10 pt o substitui. A intenção
histórica do vanilla não é inferida; a posição do texto é observável da
linguagem. Refutar a correção se baselines de estilos mistos, páginas ou
texto mudarem indevidamente.

Para linha com itens textuais fora do modo de colunas, obter `max_font_size`
e `max_style` dos itens reais, começando pelo primeiro. Sem itens textuais,
conservar o fallback do estilo exterior. No modo de colunas, preservar o
máximo com o estilo exterior: o controle P1381 de prefixo 8 pt e fallback
exterior exige a diferença de leading entre as linhas; retirar esse máximo
altera `One` de 34.4 para 17.7 pt. O teste de mistura com o estilo exterior
e a seleção do próximo ascent/leading permanecem independentes;
`force_outer_singleton`, extensões inline, custos de região e callbacks
mantêm o contrato anterior.
Nenhum campo público, modo, fase ou default muda. Exigir RED→GREEN para
dois fragmentos menores que o exterior, equivalência bilateral de `context`
e última linha de `text-dense`, mais controles de mistura/linhas ordinárias.

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

## Direção de parágrafo distinta de direção de run — 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/direcao-alinhamento-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes, binários e treze sondas. rtl_lines põe ABC em x=20 no Tekt e x=152.949220 no vanilla pinado; rtl_inline_only e rtl_scope coincidem à esquerda. ltr_lines põe árabe em x=174.567870 no Tekt contra x=20 no vanilla. Em `cursor.rs:156–162` e `line_commit.rs:210–216`, a âncora depende do primeiro item da linha. Vanilla `flow/collect.rs:129–136` obtém trunk_from_pairs e `inline/mod.rs:189,232` resolve direção/alinhamento compartilhados antes das linhas. Semântica observável: direção dos trechos e âncora de todas as linhas do parágrafo. Estratégia de state/preflight é mecânica interna, não cópia do vanilla. Inferência: preservar direção compartilhada evita perder a âncora em linhas só com override; refutada por controles integralmente estilizados/parbreak/wrap.

Correção interna/de paridade ADR-0127, fluxo contínuo L0 primeiro e RED→GREEN. Apenas campos privados de Layouter e helpers descendentes, nenhuma entidade ou assinatura pública/fase/default novo. Esta rodada certifica conteúdo textual estrutural (Text, Empty, Space/SpaceFromNewline, Linebreak, Sequence, Styled e Strong/Emph); unidades desconhecidas conservam fallback anterior. Não prometer ordem bidi mista, align explícito, HTML ou todas as formas de parágrafo. As equações existentes de largura/tracking permanecem.

Responsabilidade deste owner: Justificação usa a mesma consulta interna paragraph que o alinhamento. Ao drenar linha, limpar somente a direção da cauda; manter escopo ativo para a próxima linha de wrap. Não alterar equações de oportunidades/tracking ou altura.

Aceitação: RED→GREEN em parágrafo RTL com linha LTR e inverso, escopo integral e reset por parbreak; checar checkpoints e subframes, texto/posições em CLI bilateral, build e lint sem novos findings. Slots e helper não transportam metadados ao L3; divergências de ordenação bidi ou align explícito ficam registradas separadamente.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

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

A migração/rejeição da linha por altura regional reduzida aplica-se somente fora de is_sub_frame. Drenar a linha local normalmente e conservar conteúdo, posições, extensões e decoração.

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

Usar o ascent instalado quando Some, inclusive zero; só ausência permite o fallback histórico. O ascent efetivo de uma linha com texto direto é max(ascent inline, aresta superior real desse texto); linha sem texto direto usa apenas ascent inline para a correção positiva da baseline. Não alterar o avanço/leading já especificado. A presença de texto vale também quando todos os itens textuais têm o mesmo tamanho: Box pode ter instalado uma baseline menor antes deles. Guardar Some(next_text_top) ao preparar a linha seguinte.

Correção interna ADR-0127, sem API, default ou fase novos. L0 primeiro,
RED→GREEN, controle de ascent zero, sondas de geometria e suíte completa.
Não substituir esperados de regressão para conservar o defeito.

### Separar ascent emitido do fallback de avanço

A primeira suíte completa deste lote refutou o candidato nos controles
p1381_prefixo_8pt_preserva_fallback_exterior_e_leading_final e
p1392_wrap_abaixo_e_acima_do_exterior_distribui_nas_duas_ordens, preservados
sem alteração. line_commit.rs:342-350 seleciona estilo exterior para
certos avanços em colunas; usar essa seleção para corrigir a baseline de
texto uniforme menor o desloca indevidamente. Snapshot integral anterior
ao reparo: box-baseline-inicial-2026-10-14/first-candidate.json.gz.

Capturar a aresta do máximo textual efetivamente emitido antes da seleção
do fallback de avanço. Em linha uniforme, combinar esse ascent efetivo
com o inline para a correção positiva; manter a regra pré-existente de
linha mista e todas as fórmulas de avanço/leading. Os dois controles devem
voltar a passar com as mesmas posições e páginas; preservar os novos
controles de Box inicial, inclusive zero e texto pequeno.

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

Após geometria/alinhamento e antes de transferir a linha, fechar segmentos de escopos ativos no buffer comitado e na linha separadamente. Após a transferência, reabrir continuações em ambos. Isso exclui o prefixo externo já presente na linha; os limites são neutros para avanço/materialidade.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`00_nucleo/diagnosticos/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Após estabilizar e drenar linha causal, registrar baseline_y + line_descent junto do cursor já preparado para a próxima linha. Flush sem linha causal não cria fronteira.

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

Antes do alinhamento/numeração visual, acumular extensão da linha emitida: limite visual e TextSeparator Literal, excluindo terminais Markup/Newline. Não usar cursor global como largura textual nem contar números na margem. Fracionários e caixas usam os carriers já expandidos.

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


O linter completo do lote (pre-exhaustive/followup-architecture.log) aponta
V16 no curinga neutro do projetor de espaços literais. Enumerar as variantes
sem esse avanço explicitamente, com retorno zero individual, para que nova
variante de FrameItem exija decisão na compilação. Preservar a mesma projeção
e a recursão dos wrappers sem criar novo observável.

## Largura na quebra automática — correção causal

Medição anterior: logical-before-complete/results.json.gz, absolute-markup e
absolute-literal; a escala usa 51.678 pt em vez de 48.928 pt porque a linha
quebrada registra o espaço literal após TWO. Fonte pinada inline/linebreak.rs:78–113
distingue quebra Normal (espaço sem avanço de layout) de Mandatory (literal
terminal conservado). line_commit.rs:105 registra ambos indistintamente.
É geometria da linguagem; o carrier textual não deve desaparecer.

Na linha continuada sem justificação, registrar a extensão material sem
ampliá-la por separadores literais terminais. Na linha final ou fechamento
explícito, conservar o avanço literal existente. A linha justificada conserva
a ocupação expandida; o limite regional do consumer transformado continua
independente. Não alterar bytes textuais, ordem, baseline, efeito de estilos,
justificação nem critérios de quebra. Regressão: escala absoluta de texto
quebrado, mais controles de espaço literal terminal e Block Auto.

### Refutador de espaçamento fracionário terminal

`logical-spacing-after/results.json.gz` registra a divergência de
terminal-fraction após a adoção da largura causal; expand_fr_spacings:249–251
apenas avança o cursor, sem publicar a ocupação já distribuída. Registrar
no acumulador causal o extremo material anterior mais o delta total alocado
quando há frações pendentes, inclusive sem item material após h(fr). Não
contar duas vezes: o acumulador usa máximo de extremos, não soma. Conservar
a distribuição, tratamento de pesos zero/negativos e infinito, suffixes,
repeat e cursor. Refutar por h(1fr)/h(2fr) terminal sob escala absoluta e
controle h(0fr); preservar espaços literais e quebra automática.


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

Somente sob geometric_text_lines, usar top/bottom reais dos carriers materiais sem floor de fonte exterior da coluna; leading e top previsto derivam da text_segment_chain. Reconciliar baseline prevista com ascent real inclusive delta negativo. Preservar toda a lógica ordinária/legacy, decoração, causalidade, overflow e commit.

## Fronteira Par/Box e região finita — 2026-10-17

### Medição anterior à decisão

`00_nucleo/diagnosticos/par-box-fronteira-2026-10-17/before.json.gz`,
`before-measurement.json.gz`, `before-analysis.json`,
`supplement-before-measurement.json.gz` e `finite-before-measurement.json.gz`
guardam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/diffstat, UTC, fontes, argv, hashes de binários e o pin
vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
Em EmptyPar→Box8×7→AFTER, Tekt yMin31.108413 contra22.400001pt;
em BEFORE→EmptyPar→Box→AFTER,54.625613 contra45.917187pt.
Espaçamento30pt inverte o erro:70.225613 contra77.117188pt.
Block8 abaixo0 seguido só de Box tem altura33.1172 contra31pt.
Em alturas finitas60–63pt, AFTER desaparece no Tekt e fica visível na página2
no pin;64–70pt deixam tinta na página1 mas página terminal extra no Tekt.
Box alto45pt na primeira região30pt cria duas páginas contra uma no pin.
boxed.rs:22–42 usa constantes normalizadas e ensure_initial_baseline;
line_commit.rs:501–539 só admite antes do commit em altura reduzida;
region_progress.rs:118–133 pode avançar após tinta já comitada fora da página.
Fonte pinada inline/box.rs:35–68 publica a baseline do frame;
flow/distribute.rs:321–373 admite frames pela geometria real.
São posições, dimensões e páginas da linguagem, não exigência de copiar
buffers/algoritmos Rust ou bytes PDF. A intenção histórica não é inferida.
A hipótese de fronteira geométrica é refutada por controles inline,
margens fortes, fontes, baseline deslocada, assentamento tardio, posições,
primeiro frame alto, subframes ou reservas/rollback alterados.

### Decisão e limites

Correção interna/de paridade ADR-0127: L0 primeiro, RED→GREEN e resselo
restrito, sem API pública, novo modo/default, dependência ou fase.
A composição fracionária de Box permanece incompleta na etapa B anterior:
a testemunha width:1fr tem largura0 contra145.849609pt. Sua rota de
calibração é conservada neste reparo e não recebe crédito de paridade.
A admissão nova certifica somente página raiz finita ordinária, sem colunas,
reservas/floats, transação/checkpoint pendente ou collector externo de
decoração. Esses mecanismos conservam os protocolos próprios, não são
silenciados nem declarados corrigidos. Aceitação inclui matriz bilateral,
controles sem fronteira, finite60–70, fonte maior posterior, primeira caixa
alta e rebase de origem/contexto; suíte completa, build, Clippy, rustdoc,
linhagem bidirecional e inventário SARIF diferencial sob12GiB e swap zero.

### Responsabilidade exclusiva deste owner

Na página raiz ordinária certificada acima, antes de numeração/commit,
considerar o fundo real de toda a linha já expandida e o ajuste de ascent
que ainda será instalado. Não usar descida tipográfica fictícia quando
não há texto. Se não cabe e existe unidade causal anterior na página
(tinta ou fundo causal de Par vazio), fechar a região anterior e transportar
junta a linha ainda não comitada e seus wrappers/posições internos.
Instalar baseline nova pelo ascent real da linha no topo útil da nova
página, uma só vez, preservando avanço horizontal e indentação relativamente
às margens resolvidas para essa página. Número/collector nascem só depois
da migração. Não migrar collectors externos já comitados nem ignorar
transações de suffix: os caminhos reservados existentes ficam intactos.
Se a primeira unidade da região é mais alta que a região, comitá-la uma
vez para garantir progresso, sem fabricar página vazia terminal.
Linhas que cabem não criam página por uma baseline seguinte hipotética.

Refinamento de evidência regional: region_progress.rs:144–151 limpa
flow_placement_end ao fechar a região, mas last_block_descent_y é evidência
de altura automática que pode atravessar páginas. O critério de unidade
anterior deve usar tinta comitada e a evidência causal da região vigente,
excluindo a própria current_line e o fundo herdado de página anterior.
Refutar com Par vazio→pagebreak explícito→primeiro Box alto: exatamente as
duas páginas causais, sem uma terceira página vazia de origem.

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

Expansão fr/repeat e justificação usam a borda final efetiva; numeração preserva margem exterior do parágrafo. Só após drenar uma linha causal, informar commit ao State, reabrindo aplicação horizontal para a próxima linha. Preservar métricas verticais, cauda fracionária e demais protocolos.

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

Responsabilidade deste owner: O segmento do collector usa a borda esquerda efetiva do parágrafo já admitido, mantendo end_x/baseline e a ordem de coleta/alinhamento existentes. Não substituir o collector nem alterar flush/paginação.
