# L0 — measurement
Hash do Código: ff383ee1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/intrinsic-literal-width.toml sha256:49b2cc837799c63414a2f68dfd25375d2238543b9bd1d502adadfc78a8df6ab1
- 00_nucleo/prompts/_nuclei/layout/subframe-inline-extents.toml sha256:ec315f38498aa37ead497fdcb72cdd7e401cd00752baf5a535c2b5fae69374ed


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/measurement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/measurement.rs:27`, `01_core/src/compiler/layout/measurement.rs:64`, `01_core/src/compiler/layout/measurement.rs:354`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Pinned bilateral `lab/results/construction/diagnostics/grid-vanilla-phase-2026-10-03.md` measured four Auto-column starts with accumulated 0/2.5/5.0/7.5pt drift. With fixed columns, all 320 starts matched x within 0.001pt. Language probe measured 2.5pt space at Libertinus Serif 10pt in both engines. `measurement.rs:70-96` adds a space after every word, including last; `grid/tracks.rs:79` uses it for Auto. Pinned vanilla `lab/typst-original/crates/typst-layout/src/grid/layouter.rs:967-1100` measures cell frame. Causality still requires ablation/RED→GREEN; complex-cell equivalence is not established.

Paired sources/binaries are in `lab/results/construction/diagnostics/dupla-avaliacao-auditoria-2026-10-05/bench-paired-measurement-red.json.gz`. show_grid320 has width/raster divergence in geometry-measurement-red.json.gz: emission keeps text but constrained dispatcher at `measurement.rs:69` falls back to no extent for realization transports; no-show control matches. This is an internal trial regression, not vanilla intent.

`lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06/before.json` preserves state/HEAD/stat/status/L0/source/binaries. Predecessor espaco-terminal-2026-10-06/verification.json.gz measures six tables: ABC/123/Arabic take two Tekt lines versus one at historical pin 9f2b6e871. `measurement.rs:104-113` takes max child widths/summed heights for inline Styled/Text/Space Sequence; pinned grid/layouter.rs:967-1100 measures full frame. RTL Block with terminal LTR Literal shifts ABC3.49658pt per space: `block.rs:315` uses line_content_right excluding every separator; `page_geometry.rs:143-154` already separates Literal-auto extent from line ink. `tracks.rs:108-113` anchors every column at left margin, even Auto RTL; oracle single column ends at right margin. These are geometry/text observables, not algorithm/byte requirements. Remaining wrong breaks/anchors or weak-space/fixed-column/style/line/paragraph regressions refute full-frame/anchor hypothesis. Do not infer historical intent from P80; intrinsic-extent nucleus distinguishes container dimensions from ink/line alignment.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv/binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized branches contain the corresponding transparent operation; Source must not leak as observable content. Human approval `confirmo` was received 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.

box-composicao-2026-10-13/core-first.log records measure_preserva_baseline_causal_de_texto_box_vazio_e_composto failure: `measurement.rs:509-512` reads line_inline_ascent/descent leaked by subframe; correct parent restoration removed that implicit channel.20pt Box shifted 3pt still requires 17pt and expectation must not relax. Source/owner/prior state are in before.json.gz. Inspection of `measurement.rs:272-294` finds explicit width only constrains body then returns bw+inset, and explicit height only minima body, contrary to nonfractional box-dimensoes-2026-10-13 measurements. Direct prechange tests use empty/larger bodies with inset/outset and require 20×14pt for explicit 20×14pt even when max_width is smaller; initial owner snapshot is in before.json.gz.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own constrained Content/Stack measurement and isolated intrinsic measurement with causal width, height and baseline. Constrained dispatch handles the current text, sequences, shapes/curves, spaces, containers, columns, alignments and lexical-style cases using injected metrics; unsupported cases retain their current fallback. Relative Block dimensions reuse `00_nucleo/prompts/core/compiler/layout/block.md` resolution against the outer available region. Do not perform an emission walk merely to allocate fractional siblings.

Preserve the public `measure_content_real` pair and internal baseline-bearing companion: build an isolated zero-margin Layouter/subframe with the supplied chain, metrics and sizer, lay out once, and derive dimensions from the produced items/cursor/inline extents. Baseline is causal composition, including boxes/equations and inset, not a plain-text-classification shortcut or unconditional height baseline. A private extent marker, where the source uses one, preserves that measured extent without a new public Content variant. Restore lexical measurement style. Do not mutate the emitting Layouter's locator, commit its pages, iterate document fixpoint or execute callbacks.

For constrained Text, count spaces between consecutive words, never an artificial trailing space per line. Break selection compares next-word width plus separator when a word already exists. Preserve width constraint, line-count height and other dispatcher arms.

Constrained measurement projects Realized to visual and Located/ShowRecipe to body using existing branches. Consumed original adds no width/height. No recipes/callbacks/identity allocation. Existing transparent ShowRecipe.body delegation also accepts Source.body without independent origin execution/public carrier/Location; preserve this unit's classification/preparation/serialization.

For structurally inline Sequence (Empty/Text/Space/Newline/Sequence, Styled, Strong, Emph and certified transparent Realized/Located), measure whole flow once in isolated ephemeral Layouter with injected metrics, current chain/style, supplied finite positive width and Left measurement alignment. This removes exterior translation, never linguistic direction. No callback, constrained-measure recursion from helper, emitter locator/page effects or content reexpansion. Derive pinned intrinsic extent from emitted items and produced height, clamping width to supplied budget. Noninline/Linebreak/Parbreak/containers/unknown retain prior dispatcher; simple Text retains its current path. Inline Sequence is not vertical Stack; preserve lexical segmentation/styles/wrap.

Read returned last_sub_frame_inline_extents rather than restored parent-line state. Preserve width/height/baseline formulas and causal P1491 expectations.

For nonfractional Boxed, resolve explicit total dimensions and measure body within width minus inset. Return explicit size if present, otherwise natural+inset; outset occupies no space. Do not emit/replay callbacks/mutate emitting instance. Fractional arm retains prior behavior until its separate composition stage.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN for one/multiple-word/wrap cases, bilateral Auto geometry before/after and fixed-column control. Transport width/height tests and paired textual-show-table geometry rerun are required. These are internal ADR-0127 corrections with no new public contract/default/phase/flag/dependency/unsafe.

Preserve exclusive ownership/static dispatch without I/O or a new signature/trait/flag or eval↔layout phase. Source-origin tests/C2/C/macro collections/builds/lineage are required before certification.

Inline-result nucleus fixes result versus restored-state distinction. P1491 must pass unchanged with checkpoint/layout suite. Nonfractional Box dimension correction requires RED→GREEN and unchanged grid/preflight expected outputs; fractional composition remains excluded.


## Batch D V16 measurement (2026-10-19)

Evidence precedes the decisions below. The exact HEAD, uncommitted file list,
UTC, source bytes, argv, source/output/binary SHA-256 and pinned oracle recipe
are recorded in `lab/results/v16-corrections-2026-10-19/batch-d/agents/measurement/probes/png-before.json`.
The oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
SVG geometry is evidence of occupied layout space, not a byte-equality contract.
These observations establish behavior; they do not infer upstream intent.
`measurement.rs:138-426` omits Image, Equation and Figure. Caller
`grid/tracks.rs:94-125` already uses real subframes for nested Table/Raw and
composed visuals; single Image/Equation/Figure reach this dispatcher. The pinned
Auto-grid probes measure next-column x = 0 versus 40pt for Image and captionless
Figure, and 0 versus 23.124444444pt for `$x + y$`. Shape and nested Table are
40pt in both products; Empty/Metadata are zero. Fixed 40pt controls preserve
40pt in both. Align/Styled image wrappers repeat the zero-width loss. Therefore
these three demonstrated omissions cannot be ratified as neutral fallbacks.
`equation.rs:105-138` derives layout through the math engine and carries callbacks;
`figure.rs:31-94` updates figure progress/prefix before drawing caption.

### Decision, bounded correction and refuters

Add Image measurement using the existing image dimension calculator and injected
sizer, returning the requested target rectangle when present (cover ink is not
the occupied width), otherwise calculated dimensions. Do not clamp a declared
fixed image size merely because its grid budget is smaller.

Measure inline Equation through the existing isolated subframe measurement path only
when its body has a positive certificate for callback-free math content. The
certificate explicitly accepts Empty, Text, MathIdent, MathText, Space,
SpaceFromNewline, Sequence/MathSequence, MathFrac, MathRoot, MathStyled, Styled,
Strong, Emph and transparent Source/Located/Realized transport, recursively audits
children, and rejects MathCancel (its angle may contain a function), Metadata
(including Content payload), Ref, all delayed/context/state/counter/dynamic
families, and every other unapproved family. It must not execute a callback or
attach the emitter runtime/locator/request stores. Preserve current width budget,
chain, TextStyle and injected metrics; keep the existing math style/provenance
route, without an independent math formula or shared mutable cache.

For Figure with no caption, delegate to constrained measurement of its body.
Captioned and numbered-caption measurement remains open: body-plus-caption
arithmetic would omit the existing prefix/index/spacing semantics. No arbitrary
emission or figure-progress replay is authorized. Reject unsupported contextual
Equation measurement before emission; retain the prior approximation and report
it as debt, not semantic zero. A callback-bearing descendant admitted by the
certificate, a changed emitter locator/state/request store, or source next-column
geometry still collapsed to zero refutes the repair.

Replace the fallback with exhaustive nominal current Content coverage, separating
true nonvisual absence from unsupported visual approximations. Future variants
must require a decision at both dispatcher and certificate. Existing Text,
Sequence, Box/Block, styles, relative and fractional paths remain unchanged.
ADR-0127 continuous parity correction applies with tests before implementation.


The certificate uses direct recursive matching at each node, not a traversal
visitor with implicit wrapper-skipping rules. Its one exhaustive Content match
must expose ShowRecipe before projecting anything. Source and Located recurse
into body; Realized recurses into visual only, never its consumed original;
Sequence/MathSequence inspect every child; MathFrac inspects num and den;
MathRoot inspects optional index and radicand; lexical/math styles inspect body.
Every other current variant, including other mathematical composites, is outside
this deliberately narrow certificate. None is thereby declared semantically zero.
The source `$x + y$` motivates this accepted subset; expanding it requires a new
callback/geometry audit rather than generic full-layout permission.

Block Equation remains an unsupported approximation in this dispatcher: its
numbering/prefix and region alignment depend on the ambient introspector and
cannot be reconstructed with an empty isolated runtime. This restriction is
explicit debt, not a promise of zero language extent. Nested callback transport,
Ref supplements, ShowRecipe, and block Equation are negative acceptance controls.
