# L0 — core/compiler/layout/paragraph_paint
Hash do Código: 442cd2a9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-line-products.toml sha256:9e606cf207ae72ade678378bb4fe6c0aa4d1daa77b9c8fc2817b7c2afd4c8ea6


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_paint.rs`


## Laboratory line products — 2026-11-13

### Measurement before decision

Frozen shaping-products c2096c812761f5ab5278eb5376142e406990ec4b234615c9fbf6ce3be32feba6,
freeze 2026-11-13T06:06:24.083725Z, eliminates backend112->1/64->1 on edits,
but nominal CPU remains14.997/25.187ms versus vanilla3.683/4.234ms. Its twelve
raw profiles attribute30.45% inclusive layout to100 paragraphs and39.41% physical
coordination tooutline; overlapping groups are not a promised speedup.
paragraph_plan.rs:143-174,222-286 re-emits retained Prepared/Line products;
paragraph_emit.rs:90-125 walks fragments again; word_layout.rs:209-224 clones
styles and invokes baseline/indent for every fragment. cursor.rs:94-191 and
region_progress.rs:156-187 show that, with initial_baseline_pending=false,
block_chain_active=false and already material current_line, repeated admission
cannot advance region. paragraph_indent.rs:348-372 returns immediately once
origin_applied is true. bibliography.rs:30-33 returns when no prefix is pending.
word_layout.rs:89-159 then only appends Text/Replacement (plus Shape for
highlight), sets optical policy and records tracking. These are mechanics,
not language acceptance. No whole-Layouter snapshot is justified by these facts.
User authorizes laboratory work without further approval; no integration.

### Owned responsibility

Own private PaintSession, exact keys, bounded retention and pure-tail capture/replay. Tests use a local emitter invocation count and actual frames/state; cold foundation calls the original emitter so positive reuse fails at runtime. The same experiment completes implementation and real pipeline wiring before acceptance.

Runtime RED before implementation: demand-red.log records two passing guards and two failing reuse assertions (2 versus1 cold calls;8 versus4 for distinct exact keys). The earlier missing test-trait methods caused a compiler error, archived in before-provider-complete, and are not RED. Add real parts/direct pipeline tests before wiring; they must fail on absent paint reuse.

Real pipeline GREEN6 is recorded in wired-green. Before full validation add tests for every representable admission guard, current lexical tracking/language/anchors, horizontal offset, oversized products, eviction budgets and retained identity lifetime. The once-computed certificate must require nonempty words/fragments, so every admitted tail executes at least one text emission. Refuse empty retained outputs and post-emission cursor-y changes; no extra behavior is authorized for future emitter side effects.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.

## Reúso da emissão em outra baseline

A contagem no release de controle em `lab/results/construction/diagnostics/references-500-reused-lines-2026-11-19/probe/observed.json` registra 1699 hits em
2400 consultas durante a recompilação de comentário. `paragraph_emit.rs`
emite separadores com cursor_y + baseline_offset. `word_layout.rs` emite
o texto com a mesma soma, via `cursor.rs`; a admissão após a primeira
palavra mantém o cursor quando há material, a origem foi aplicada e
não há baseline inicial, bloco pendente ou escape bibliográfico.

Mantenha identidade e coordenadas horizontais
exatas, os guardas atuais e os limites da sessão. Para o tail imutável
só de Text, TextSeparator e TextReplacement permitido, recomponha o y
com a nova baseline mais o offset do estilo de cada item, na mesma ordem
aritmética da emissão original. Preserve o caminho de coordenada idêntica
e recuse publicação se algum y capturado não reproduzir essa expressão
bit a bit. Não retenha a primeira palavra, os efeitos de fechamento nem
qualquer estado de página. Exija RED/GREEN com baselines, offset e
substituição, e PDFs frescos do último candidato como controle.

## Width reuse at the source paragraph boundary

Store a finite horizontal extent in the bounded Paint product together with
its immutable metric epoch. The width witness owns the matching Paint and item
range. Recompute geometry on cold, unretained or invalidated paint.
