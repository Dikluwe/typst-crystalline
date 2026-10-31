# L0 — entities/content/construction/layout
Hash do Código: ee3e18f4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/construction/layout.rs`

## Extraction evidence and contract

Clean HEAD `f7195cdf6` on 2026-09-30 places the extracted block in construction.rs:837–1102,1193–1350. This establishes ownership, not performance/new bilateral parity. Own block/geometry/spacing/break/position/column constructors, preserving original variants/values/signatures/defaults/pipeline phase/compatibility/purity/static dispatch. L1 suite/build/linter must pass; separation alone proves no speed improvement.

## Semantic SmartQuote evidence and approval

The historical smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz collection records pin `9f2b6e8715237cb086899a42873660fe744622e8`, binaries/HEAD/tree/diffstat/UTC/sources/argv/cwd/environment/output. PDF uses extracted text, HTML DOM/text/diagnostics, not PDF bytes. Eval converted markup quotes into Text, losing double/enabled and retaining English glyphs when reused under German/French/disabled styles. Primes6'/2-inch were misread as quotes. Native-function show/public-name corrections already passed separately; empty constructor has no fields upstream, explicit double:false is known while omission is Unset, and enabled:false remains SmartQuote. New set-double/show-filter positives distinguish presence. HTML literal worked through Text while programmatic SmartQuote rejected; literal titles render quotes but script title expects string and rejects Content before export in both binaries.

Historical markup_stream.rs:79–128 chooses glyph during eval; layout smartquote toggles two flags without preceding text; stdlib smartquote.rs:112–117 erases disabled elements; observation/elements.rs:242–250 compares current fields. Upstream eval/markup.rs:141–146 creates explicit-double elements, inline/collect.rs:196–211 resolves style/context and text/smartquote.rs:119–158 handles prime/apostrophe/nesting.

Human approval on 2026-10-06 covers double bool→Option<bool>, enabled:Option<bool>, pure shared helper/context and glyph selection moving eval→target composition. Keep identity/fields during eval. Paged and semantic HTML use their own target composition (ADR-0128), sharing one pure L1 contextual selector with per-stream/compilation state. No I/O/global/vtable/property registry/upstream memoization copying, new mode/flag/unsafe/watch API/math phase. HTML escaping remains separately owned. Stack representation is mechanics; fields/known-versus-omitted/glyph/style are acceptance. Other proposals/ADRs remain unchanged.

This owner's Content::smartquote(double:bool) signature stays unchanged and builds double:Some(double), enabled/alternative/quotes:None. Change only SmartQuote initialization; never choose a glyph or modify other constructors here.

## Typed transform evidence and approval

`lab/results/construction/diagnostics/transformacoes-2026-10-14/{before,results}.json.gz` plus controls/results.json, binding-controls/results.json and context-controls/results.json preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, tree/UTC/diffstat/source/argv/binary hashes at the same historical pin. TransformElem stored matrix/body; stdlib transforms.rs:19,51,91,191 reduced arguments to matrices and layout chose center pivot/has_orphaned. Upstream retains four elements and resolves styles/region/body/origin/reflow.

Probes show empty local fields, false func equality, rotate(0deg)[X]==scale(100%)[X] locally true/upstream false, collapsed explicit defaults, ignored/rejected origin/reflow, scale(x:150%) wrongly scaling y and lost move percentages/em. Missing fields must remain lexically unknown. set rotate(23deg) is valid but named angle is not; contextual getters reflect style upstream. Initial wrapper-labeled contextual controls were invalid for value, corrected in context-controls. Upstream HTML warns and drops body; local strict HTML rejects per its separate contract. Fields/presence collapse, baked style, wrong origin/lost errors, reflow:false changing advance or dropped body refute the proposed language behavior. Upstream structs and measurement steps are mechanics.

Approval receipt `lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/approval-receipt.json`, manifest `960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`, ratifies the typed family only. Keep `Content::transform(matrix:TransformMatrix,body:Content)->Content`, constructing TransformElem{operation:TransformOperation::Matrix(matrix),body}. Never infer native arguments from a matrix. The stdlib owner constructs NativeMove/Rotate/Scale/Skew with independent typed presence. Other constructors remain unchanged.

## Nominal constructor lint boundary

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` freezes full source/L0/signatures/hash/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/status/diffstat at 2026-10-15T16:39:18.873477+00:00. C023 at construction/layout.rs:177 is page_run_with_geometry with 9 Clippy arguments above 7. Arity is Rust structure, not a functional defect or language/readability measurement. Its pure public PageRun parameters, especially absence and Option<Option<_>> numbering, are meaningful; resolve no geometry here.

Preserve signature/order/semantics/body/calls/cfg/other attributes. Permit only `#[expect(clippy::too_many_arguments, reason="C023: Construtor público puro com argumentos correspondentes ao PageRun: ausência e numbering Option<Option<_>> têm significado próprio")]` on that function. No module/impl/crate/global suppression, threshold increase or other lint. Remove/review it if arity falls below threshold; never silence unfulfilled_lint_expectations. This bounded exception supersedes generic suppression bans only for its named item.

The later `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` records 79 warnings; `clippy-excecoes-2026-10-16/before.json.gz` reconfirms sources/spans/reasons/signatures/hashes/HEAD/tree/UTC/diffstat. It materializes C023, superseding the initial audit-only pending status. Correction-class entries, including conditional C047/C066 elsewhere, remain pending and unannotated.

## Acceptance and limits

Require existing constructor/language/integration controls without relaxed fixtures/tolerances. Keep layers/phase/defaults/static dispatch/no mutable global state. Any newly needed public contract or external compatibility change needs a concrete ADR-0127 gate; private signature repairs must update callers and their owners first.

For the lint-only slice, L0 precedes metadata/restricted reseal; removing added attributes and lineage header must recover prior executable source after rustfmt normalization. Measure RED first, then all-target inventory: exactly 46 arity and five test warnings satisfied by 49 expectations, no new warnings and unfulfilled expectations as errors; the 11 arities requiring correction remain visible. Validate build/fmt/strict+reverse lineage/no-new-error-or-warning Tekt, existing Counter/coverage tests under 12 GiB, swap 0, one job. The previous full suite is evidence only for identical executable code. Do not create attribute-mirroring tests or claim performance from lint metadata.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content/construction/layout.md` preserves the original evidence and chronology.


## Ratified Circle constructor origin

### Evidence and approval before implementation

At HEAD `954d13185e26a1ed90f6d7c60f6ba27760ca9889`, the source/owner audit in
`lab/results/v16-corrections-2026-10-19/circle-origin/l0-before.json` freezes
UTC, exact diffstat and hashes. The approved proposal
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md`
(SHA-256 `5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`)
binds the baseline equality and rejected full-Length candidate receipts.
`stdlib/shapes/ellipse.rs:92-104` erased the em component;
`entities/elements/shape.rs:21-29` used one geometry carrier for both constructors.
Those observations concern language identity and geometry, not mandatory upstream
Rust structure. The explicit user approval is recorded in
`lab/results/v16-corrections-2026-10-19/circle-origin/approval-receipt.json`;
it satisfies ADR-0127 section 2 for exactly this public type/field and migration.

### Exclusive approved responsibility

Keep Content::shape and shape_with_radius public signatures and all existing
arguments/defaults, including radius-to-RoundedRect behavior. Initialize
`ShapeOrigin::Geometry` in their ShapeElem, including an Ellipse kind with equal
axes. Never infer Circle from kind, dimensions or paint. Origin is no new render
parameter. All other constructors retain their current contracts. Require generic
Ellipse/Rect/RoundedRect controls and full dependent migration validation.


## R13 — approved ShapeElem literal migration

The source inventory in lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/migration-owners.json identifies this consumer. Owner approved the public ShapeElem.relative_line addition (proposal SHA-256 d8c9a01413232bac7113d1148af3c1b3bcd671bb37eeca68a94d5749d671db00). Initialize that field to None in existing constructors and fixture literals, preserving all geometry, paint and behavior. No other implementation change is authorized here. Full existing tests, build and lineage must pass.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Migrate only this consumer's ShapeElem literal/destructure: legacy rect_radius
and rect_stroke are None (or explicitly bound/observed for observation). Preserve
all existing values, expectations and other APIs; public addition is approved.
Do not materialize unrelated square/container features.
