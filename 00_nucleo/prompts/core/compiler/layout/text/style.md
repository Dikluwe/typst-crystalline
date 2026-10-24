# L0 — Effective inline text style
Hash do Código: 12660f32

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/raw-defaults.toml sha256:24f0b69fa1db115ff13a786121c80cb7be3abfa2028a9a473075374a6815cfd4
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f

Layer: L1. Exclusive consumer: `01_core/src/compiler/layout/text/style.rs`.

## Evidence before specification

`01_core/src/compiler/layout/text.rs:29-222` resolves TextStyle through typed/custom chain precedence, including script scaling. Space previously used only cached style at `01_core/src/compiler/layout/space.rs:14`, which does not prove effective font at a lexical boundary. The approved carrier requires actual font identity. No new parity measurement is claimed.

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`, before-measurement.json.gz, morph-before-measurement.json.gz and audit.json preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, status/diffstat, sources/L0/binaries/argv and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. named-em A measures 0pt versus 24pt; affine 3pt versus 27pt; nested over 18pt measures 0pt versus 36pt. Pin accepts all six Size/Color/body permutations but Tekt rejects them. Positional set 18pt is ignored (12 versus 18pt). Pinned `text/mod.rs:290,303,924–930,1136–1142` and `args.rs:178–247` establish typed search, named preference, remaining body and composition. `style_chain.rs:513–517` and `constructor.rs:123` discard em; collapse at `style_chain.rs:394–395,419–422` uses top-wins where size needs folding. Size/scope are language observables; Rust representation is internal. Infer no historical intent. Restoration, Raw, heading, math, capture/show, duplicates/spans, zero/negative or metrics divergence can refute the correction. Equality probe refuted comparing declared values: in pin, text(size:2em)[A] == text(size:24pt)[A] is true; pinned StyledElem::eq at `content/mod.rs:779–782` compares child. That equality defect needs a separate correction; full type alone does not close it.

## Ownership and contract

Own pure effective-style resolution extracted unchanged from Text. Preserve field-specific precedence, typed size, custom font override, folded variations, semantic strong/emph provenance, math fields, script scaling 0.65, explicit script size and baseline offsets. Do not mutate Layouter, perform I/O, choose installed fonts or emit text. Text invokes once before its temporary style scope. Markup/Newline separators invoke once at current chain; Literal separators already within the resolved scope reuse that style to avoid double scaling. This internal descendant function introduces no public data/trait contract.

Use the already-resolved Pt size from active style; remove dead abs-only custom text.size projection without replacing deliberate heading/math/script sizes with exterior style. Preserve all other folds, metrics and emission.

## Approval, acceptance and remaining scope

The human ratified full text size (TS1) and both public contracts in proposal manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba` on 2026-10-17. Implementation/tests S1–S16 operate under 12GiB/zero swap; no release/merge/push. Reseal only after implementation and valid graph. RG1 remains independent. The approved cross-owner public changes are boundary criteria, not extra ownership of those consumers; language equality remains separately unresolved.


## Batch D V16 measurement (2026-10-19)

Evidence precedes the decisions below. The exact HEAD, uncommitted file list,
UTC, source bytes, argv, source/output/binary SHA-256 and pinned oracle recipe
are recorded in `lab/results/v16-corrections-2026-10-19/batch-d/agents/measurement/probes/png-before.json`.
The oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
SVG geometry is evidence of occupied layout space, not a byte-equality contract.
These observations establish behavior; they do not infer upstream intent.
`text/style.rs:71-137` decodes literal family entries and dictionaries with
name Str/Regex plus required variants array; malformed entries are skipped.
`eval/rules/set/text.rs:220-315,423` and `eval/font_dict.rs:30-240` normalize valid
source schemas to that transport. `FontList::new` returns None for an empty
result and effective style then retains the ambient font. Direct custom chains
can contain invalid Values; source validation is not a proof they are impossible.

### Decision and refuters

At the two selected matches, explicitly distinguish Str/Regex dictionary names
from all invalid name variants, and Str/Dict family entries from all invalid
entries. Keep order, required variants and optional variant/weight/style fields,
custom-font precedence, and every unrelated TextStyle field. Do not select fonts,
reparse source or diagnose malformed raw transport here. Missing/invalid entries
retain existing filtering; an empty decoded list retains ambient font. The
misleading comments describing these schemas as nontext Content projections must
be removed. A valid writer emitting another name/entry kind refutes this schema;
the writer's validation remains outside this owner. Each new Value variant must
force a decision at both exact matches. ADR-0127 continuous internal flow applies.

## Source-scope refinement measurement (2026-10-21)

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/scope-refuters/measurement.json.gz`
records exact source/font bytes, UTC, HEAD, dirty state, commands, diagnostics,
PDF/SVG and source-bound prototype binary
`380224b96e7d12e15f7d8ee02ad2200d96664eb14f0bbe879fbd3eb265bd96d7`
against oracle commit bd5ebec285505889e8ff1d4f68804ab147a699f3, tree
cac836c07e4dec2a89219370428292393a3216b5, executable SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9. All six sources compile
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

Expose a private from_parts(&StyleChain, &TextStyle) for the exact existing
pure effective-style policy. The existing Layouter wrapper delegates to it.
Extract the current expression unchanged: field-specific typed/custom winners,
custom-font override/filtering, folded variations, semantic provenance, math
fields, script scaling and offsets, and every exhaustive Value alternative
remain identical. Do not add another policy or consult Layouter state beyond
those two explicit immutable inputs. Existing style tests remain unchanged.
This descendant-only helper adds no public API, default or pipeline phase.

## Candidate HTML and Raw replacement refuters

### Measurement before correction

`lab/results/remaining-parity-2026-10-22/candidate-2/qualification/html-length-18/receipt.json.gz`
SHA-256 `8c7f679fd7633958fa7402be97c67b3f7b0c2ef846357b023076309601175c05`
retains UTC `2026-10-22T13:43:20.929159+00:00`–`2026-10-22T13:43:43.693372+00:00`,
exact uncommitted HEAD/dirty stat, full source archive, fonts, argv/artifacts,
and source-bound candidate SHA-256
`feb025296531c92350e29bff032f655cc2fa47542cf8ab4aca0e239108bc4e8f`.
The build receipt is `candidate-2/candidate-build/receipt.json` in that result root,
SHA-256 `df1ee28f39d889a05e9aba69b306736c7a500abfc8264781d8f2a9e7853ff5b2`.
Oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
All provenance guards pass. The ignored Page-only source has correct exact
warning/source but produces `<body><p></p></body>` instead of empty `<body>`.
Raw show replacement preserves actual text/paint/root extents, but glyph origins
differ by up to `5.005859375pt`: replacement glyph advances use exterior Sans
rather than the scoped Mono font; direct Raw passes. These are separate real
language failures, not a reason to relax diagnostics or geometry criteria.

### Exclusive correction

The current from_parts selects any decoded custom text.font before the active
typed font, ignoring their relative lexical positions. This contradicts Raw's
inner typed Mono scope and the existing typed/custom nearest-declaration policy.
Use the existing pure raw_deltas traversal to distinguish the nearest explicit
typed font from the nearest custom text.font declaration. A nearer typed font
retains the already-active font; a nearer custom declaration uses the unchanged
rich family decoder and existing invalid/empty ambient fallback. Within one
delta, typed declaration keeps its existing precedence. Do not change family
schema, font selection, geometry, engine capture, evaluator style, other fields,
public contracts or pipeline phase. Require RED with outer custom Sans / inner
typed Mono; inverse precedence, rich inner families and invalid entries remain
preservation controls. The full original bilateral show-raw glyph criterion
must pass before closing this debt.


## R8 — approved font-backed superscript integration

Evidence before decision: the source-bound `footnote-layout-r8-2026-10-24/before/receipt.json`,
`font-tables.json` and `red-geometry.json` under `lab/results/construction/diagnostics/`
record wrong 0.65 scaling/positive baseline and font-dependent pinned geometry at
bd5ebec285505889e8ff1d4f68804ab147a699f3. `approval.json` binds the human-approved
FontMetrics::text_superscript_metrics addition. OS/2/whole-run GSUB are text metrics;
MATH constants and fixture/font-name coefficients are not substitutes. This private
integration follows ADR-0127 continuous parity flow, superseding only superscript
scaling/feature assumptions below. Keep subscript behavior and unrelated folds.
Own a private actual-text style resolver. Reuse the current field merge without
pre-applying the legacy superscript scale. For superscript (and not subscript), ask
the approved injected method at outer size, resolve explicit size when synthetic,
store downward shift as absolute Pt and return horizontal compensation separately.
Keep the existing inert from_parts policy for callers without actual text. No new
public field or trait change. Preserve synthetic/native marker identity.

Require actual RED→GREEN tests, fixed-pin PDF/SVG controls, unchanged geometry tolerance and complete workspace/lineage validation.


## R8 approved resolved native-script choice

Measurement before decision: R8 numbering-candidate5/receipt.json records
Nimbus11 numbering1 Z with1.4718pt error; the full marker chose synthesis but its
shortened digit was shaped with sups again. The root is
lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/.
The exact public TextStyle::superscript_native: Option<bool>, default None, was
approved from owner draft SHA4b14829d1eb38475d1985a9707a6e6ce40166196dfe524afe512d99b9cabf4d0
and is recorded in native-carrier-approval.json. Its data/default contract belongs
only to core/entities/layout_types.md; this owner retains its existing consumer.
No MATH flag, baseline sign, font-name constant, private string or source identity
may substitute for the explicit choice. No extra public field/signature/phase.
Populate Some(native) from the approved injected metric result only when actual
superscript text is resolved; preserve semantic superscript=true, actual chosen
size/offset and explicit synthetic-size override. Inert merges preserve the
already-resolved choice; source styles/normal text remain unresolved None.

Require new field/default/forced-false RED→GREEN, all twelve numbering refuters at0.001pt PDF/SVG, original ten notes, six font controls, warm/cold cache and full preservation/workspace/lineage gates.


## R10 — source superscript runs and mandatory breaks

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,baseline-audit.json,approved-metrics.md,approval.json}` binds received R9 source/UTC/dirty stat and pinned bd5ebec2 binaries. Six of fifteen controls fail the unchanged complete physical words/pages and0.001pt gate: three literal-newline cases, Nimbus hard break, and Nimbus/Liberation mixed fallback. Pinned inline/collect.rs:191-196 includes mandatory breaks in source script text; inline/shaping.rs:970-1004,1090-1107,1167-1217 retains outer size, actual face/run and full causal coverage. This is language geometry and source preservation, not a byte/algorithm obligation. Font-specific synthetic metrics cannot be guessed from the primary face after fallback. The exact additive FontMetrics::text_superscript_runs contract was approved at SHA65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9; every other public method/field/default/phase stays unchanged.

Classification: approved public method plus private parity integration under ADR-0127. No font I/O, callbacks, dynamic evaluation or font-name coefficients in L1. Require real module RED→GREEN before runtime code, default/override/borrowed contract tests, source partition and restoration, native/synthetic/mixed-face/warm-cold/explicit-size refuters; all original controls, R8/R9 and A–H/R1–R7 replays, PDF/SVG0.001pt and full workspace/lineage/resources validation. Preserve all historical observations and unrelated received input bytes.

### Exclusive owning obligation

Expose a private unscaled actual-script request resolver using current field/chain precedence and no legacy script scaling. Keep existing actual-run resolver as fallback. A positively authenticated mandatory-break script scope supplies Some(false) to the new run method, so covered prefixes cannot regain native sups. Ordinary/subscript/empty effective-style paths remain unchanged.
