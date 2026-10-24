# L0 — h space
Hash do Código: 7733ad2c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/h_space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/h_space.rs:22`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The spacing-only probe in `logical-before-complete/results.json.gz` preserves explicit space under absolute scaling. `h_space.rs:27–30` only updates the cursor, which is not the causal witness consumed by transform. Changing to that witness must preserve absolute h's contribution, including at the end of a line.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve absolute spacing against the active font size and advance cursor_x. Fractional spacing records the current line-item boundary and its weight, with matching natural width zero; `00_nucleo/prompts/core/compiler/layout/line_commit.md` owns allocation and expansion. Do not allocate vertical fractions or emit an item here.

Record the cursor's endpoint after advance in the existing causal accumulator, without a new item, repeated execution or new state. Fractional spacing remains allocated by `00_nucleo/prompts/core/compiler/layout/line_commit.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Refute the correction with text followed by explicit h under absolute scaling and controls without transformation. This is an internal correction under ADR-0127, with no new public contract.


## Certified inline width consumption

Fresh nested-budget measurements in
`lab/results/nested-fractions-repeat-2026-10-21/box-before/` precede the
private assignment specification owned by
`00_nucleo/prompts/core/compiler/layout/sequence/inline_allocation.md`.
For Fractional spacing with an actual scoped Fraction assignment, consume
its known finite width once, advance the existing cursor and record causal
flow width without appending a second pending fraction. Some(0) is assigned
zero, not absence. Absolute spacing and every no-assignment case retain
their exact original semantics/queue/bidi/diagnostic behavior. Pure h-fr-only
lines remain under the legacy path. No public field/variant/API, source
evaluation or phase changes. Require RED→GREEN nested parent/child positions
and existing zero/negative/nonfinite/h-only controls with unchanged bounds,
plus root's full parity, tests, hashes, Tekt and lineage gates.

## Source-scope refinement measurement (2026-10-21)

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/scope-refuters/measurement.json.gz`
records exact source/font bytes, UTC, HEAD, dirty state, commands, diagnostics,
PDF/SVG and source-bound prototype binary
`380224b96e7d12e15f7d8ee02ad2200d96664eb14f0bbe879fbd3eb265bd96d7`
against the same ratified oracle commit/tree/SHA above. All six sources compile
without errors. Global-flow-tail paints joined PAB and Q at134.05884 in the
prototype, versus PA and BQ at215.79541 in the oracle. Fraction-rounding-residual
paints joined AB and Q at226.88379, versus A at8, B at36.686947 and Q at225.4.
These are source-language positions; PDF token grouping alone does not certify
source deletion. Zero-width controls retain the visible source body and fixed
sibling, including coincident A/Q; an empty zero Box has its own real baseline.

`eval/markup_stream.rs:305` adds `eval.flow.scope` to a complete lexical tail;
`layout/paragraph_prepare.rs:65–99` already positively classifies bounded text
fields/channels. Rejecting every custom delta propagates Blocked into such a
complete tail. The initial arithmetic model in
`agents/box/rounding-certificate-analysis.json` is withdrawn as a causal proof:
Python's compensated builtin sum did not model the current Rust ordered sum.
`agents/box/rounding-ordered-fold-correction.json` preserves that refutation
and a separate provisional ordered-fold analysis; it is not compiler evidence.
`lab/results/nested-fractions-repeat-2026-10-21/corrected-certificate-red/`
retains the exact input/source state, UTC, full test log and receipt. Its
corrected scope test fails at actual29.8 versus expected133.2, while fourteen
controls, including the
rounding geometry control, pass. Preserve their exact test inputs/tolerances.
No arithmetic correction follows from the refuted model or its passing control;
retain the existing normalized ratios and strict cursor<=right proof unchanged.

### Refinement decision and refuters

Consume assigned Some(0) once as zero exterior advance without enqueueing
that same fraction. The inline-allocation owner alone certifies a common
representable budget and strict fit; this consumer neither rounds an extra
share nor snaps a suffix. Unknown or absent assignments retain current queues.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Record actual explicit horizontal-spacing admission as unpainted page material outside isolated subframes, including zero and fractional spacing. Preserve all existing geometry/weak limitations; do not synthesize a frame item or emit paint.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.
