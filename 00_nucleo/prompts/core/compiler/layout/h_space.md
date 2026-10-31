# L0 — h space
Hash do Código: 2797a759

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/horizontal-space-paragraph.toml sha256:bbcb7c79c93e3d2da4580dfbc5586174a9698f02e3d3579ea886a1ff2608ed4b

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

## Image flow and horizontal paragraph correction — 2026-10-27

Measurement precedes this correction: `lab/results/construction/diagnostics/image-flow-cause-2026-10-27/{baseline.json,results.json,refuters.json,completion.json}` binds HEAD212aed044, uncommitted inputs/diffstat/UTC, pinned64c5e8c0/tree483bd6f1/SHA22897f93 and received Tekt SHA2941a5ba. Direct PDF streams show image1 at y5 and images2–16 at y13.2; vanilla has eight30x20pt images per page at step46.4. Removing h gives vanilla step33.2 but retains Tekt overlap. Margin30 still resets later images to13.2; a60x40pt image does too. Source `image.rs:198,325–335` advances cursor without publishing the physical end consumed by `cursor.rs:128–169`. Vanilla `rules.rs:771`, realization `lib.rs:1061–1087` and flow `collect.rs:162–188,240–285` distinguish image blocks and h-only paragraph gaps. These are language/geometry observations, not a requirement to copy buffers, PDF bytes or upstream execution structure. No upstream intent is inferred.

Classification: private parity repair, continuous flow ADR-0127. No public fields/signatures/trait methods, new product mode, phase change, dependency or compatibility break. Keep all received footnote/R11–R13/JPEG work. New shared obligations are pinned in the horizontal-space-paragraph Nucleus; this owner authorizes only its exclusive consumer.

### Exclusive correction

Own the finite main-flow horizontal-paragraph producer and private closure helper. Resolve and retain the current source paragraph spacing (custom par.spacing or default1.2em) when h begins an otherwise causally empty paragraph, including zero and weak h. Repeated contributions do not add multiple paragraph gaps. Existing horizontal absolute/fractional allocation stays authoritative. A normal causal line absorbs the pending cause on flush; no fabricated space/text or change to width allocation. Closure of a cause without a causal line takes its preceding physical block end plus max(pending below, paragraph spacing), or the fresh physical top, and publishes a zero-height paragraph end with pending below spacing. Reset horizontal cursor at closure. Below remains trim-able at document/region end. Capture/reset/restore state follows the Nucleus; plain subframe h stays on its received inline route. Do not claim arbitrary mixed lexical h scopes without measuring them.

The old no-emission rule for this helper remains: no new FrameItem, public Content or entity API. Layout-only causal bookkeeping is authorized. Tests must distinguish h(0) from absence, several h from several paragraphs, actual text absorption, explicit paragraph boundaries and source spacing overrides.

Require targeted RED→GREEN, bilateral PDF/SVG<=0.001pt, source order/complete image counts, all source-margin/size/spacing refuters, prior R8–R13/notes and A–H/R1–R7, full build/tests/Clippy/fmt/Rustdoc and full lint without warning/error. Preserve known tracking/paint limits; remeasure JPEG latency/RSS against received/legacy/oracle and retain R6 resource bounds. Snapshot provenance and preserve unrelated inputs/historical receipts. Validate V15/V26 before restricted resealing; no commit/push.

### Existing margin priority refinement

The image owner's measured zero-spacing refuters and pinned flow/collect priorities also govern closure here. A preceding explicit block below overrides the h-only paragraph's weak above; Auto/paragraph neighbors collapse to max. Use existing prev_margin_is_parbreak to retain that distinction. This refines the earlier max-only wording; private pending source spacing and all lifecycle/width/no-paint obligations remain.

### Measured lexical horizontal scope refinement

The candidate2 frozen state and pilot/geometry.json in image-flow-fix-2026-10-27 show mixed-h-scopes and siblings-h-scopes differ by43.2pt at the third image, while sole-h-scope and wrapper-h-scopes match. Equal styled siblings do not establish a shared lexical paragraph wrapper. Resolve h-only paragraph spacing from the nearest actual common StyleChain ancestor of all producers. Preserve the original anchor and monotonically narrow to its ancestor; do not infer lexical scope from equal values. Existing raw_deltas supplies identity without a public contract change. The private pending state stores an Arc-backed source chain and ancestor depth; normal one-producer closure resolves directly, mixed scope may reconstruct only the ancestor suffix for resolution. Checkpoint clones, subframes save/clear/restore, region reset and real-line absorption apply to this complete state. No public API, source reevaluation, paint, dependency or phase change. Require RED→GREEN for mixed outer/sibling scope and sole/wrapped controls, then rerun full validation.

### Measured terminal horizontal paragraph region admission

terminal-image-boundary/receipt.json and next-geometry-before.json bind the prior frozen state and executables: images370/380/385/390pt without h fit one page in both; with terminal h,370pt fits one, but380/385/390pt require a second page in vanilla and were omitted in Tekt. Following380pt image+h with another20pt image places that image at18.2pt in vanilla, but5pt in Tekt. The zero-height paragraph itself, not its pending lower gap, requires admission once its preceding gap puts its physical end past the region.

Before publishing closure, compare that end with page_bottom_limit for a finite nonempty main region. An active speculative suffix must be rejected without committing a page; rollback owns restoration. Otherwise use the existing next-region route once, recover geometric top by removing any assumed text ascent, clear the prior region's last-block descent, and explicitly preserve unpainted materiality on the successor. Publish the same zero-height paragraph there, with its lower margin still pending. Preserve exact-fit images without h and the370pt terminal-h control. No synthetic glyph/line, public API, source reevaluation or phase change. New tests must fail on missing successor/incorrect y5 before the implementation; repeat all final gates and bilateral observations afterward.

Additional hidden-before.json shows the same missing successor for a hidden380pt image. Nonempty region here means physical progress, not painted-item presence: require end beyond geometric page top, so invisible material participates and a fresh zero-height paragraph cannot loop. Recover the successor top with the existing initial-baseline flag and current text-edge metrics, as image admission does, including column transitions.
