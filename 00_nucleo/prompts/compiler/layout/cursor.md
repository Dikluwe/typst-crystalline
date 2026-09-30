# L0 — cursor
Hash do Código: a875a21a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

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
