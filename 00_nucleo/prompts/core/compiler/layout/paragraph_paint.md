# L0 — core/compiler/layout/paragraph_paint
Hash do Código: 10d50b42

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-line-products.toml sha256:417d55b8ae48d8b71220974310a7a31ce00566318c0cfc4837e84c955bdcdfa8


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
