# L0 — region progress
Hash do Código: fc48c17c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/region_progress.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Own the implementation of next_region and the existing post_line progress predicate. next_region preserves flow-suffix acceptance before any mutation; column close/next/merge ordering; auto dimensions before draining admissible floats and footnotes; page_commit; physical-page margin/binding resolution; next cursor/baseline/reservation/block-state reset; pending cell tails; and first-column startup. Delegate fitting to Place and draining/geometry/assembly to their current owners. The post_line fallback remains for uncertified overflow; fitting-line and next-line admission semantics are superseded by the measured regional demand correction below. Preserve drain/reset ordering. No future-content scan, IO, new checkpoint or dynamic dispatch is authorized.

## Acceptance

Preserve the existing observable text, positions, styles, pages and errors in the controlled corpus and current suites. Structural tests refute reintroduction of algorithms in adapters. Pure-selection tests discriminate no-wrap, wrap and Unicode-safe prefix decisions; measurement tests retain overrides and shaped/fallback paths. Each consumer is owned exclusively by this prompt. This is internal atomization, not completion of the audit findings.

## Regional demand correction — 2026-09-28

### Evidence before decision

The 30 bilateral controls in `00_nucleo/diagnosticos/estudo-correcao-falhas-texto-2026-09-28/` measure terminal pages 2/1, explicit terminal pagebreak 3/2, and continuation 3/2 (Tekt/vanilla). The isolated pre-export control emits Four/additional on page 2 with baseline 101.84pt and page height 100pt. The causal sources are line_commit.rs:657-696 (hypothetical next baseline), region_progress.rs:113-128 (post-line transition), cursor.rs:80-135 and parbreak.rs:41-80 (paragraph baseline/spacing). Current source and L0 hashes precede edits in `00_nucleo/diagnosticos/correcao-demanda-regional-2026-09-28/initial.json` and `before/`. This is internal parity correction under ADR-0127, not a public contract/default/pipeline change.

### Decision and acceptance

Own admission of the next text line through a private static descendant function. For finite geometry, advance before emitting a new material line when the predicted baseline exceeds the effective bottom, the pending line has no material, and the current region has material. Do not advance an empty region repeatedly or acquire metrics to decide that predicate. Cursor invokes this only after resolving initial/block-chain baselines; paragraph and fractional-tail collapse keep their current owner. post_line retains its rejection/transition fallback only for a line not certified to fit by LineCommit; a fitting committed line never advances just because its predicted next baseline overflows. Preserve suffix acceptance, column transition, floats/notes/tails and auto dimensions. No new checkpoint/state field, future-content scan or dynamic dispatch.

### Integration controls preceding the refinement

The first implementation's 36 real-font controls are preserved in first-after-pdf/ and first-pdf-green.log (before any later rebuild). terminal-columns still loses Three; terminal-footnote changes from 2 to 1 pages and its raster shows note/main overlap. Column start returns before clearing previous block spacing; terminal note fitting uses cursor_y and must retain the conservative following-line clearance rather than the terminal geometry cursor. These measurements require the refinements below, without changing the footnote body format, reservation model or public contract.

After start_next_column, clear last_flush_advance, prev_block_equation_descent, prev_block_below_pending and block_chain_active exactly as a physical-region transition does. Do not let another word rebuild a baseline from the closed column. The private reset belongs to this owner and is shared by the two transition branches.
