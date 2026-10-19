# Prompt L0 — `entities/elements/math_matrix` — `MathMatrixElem`
Hash do Código: 14391705

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_matrix.rs`

## Evidence and approval before specification

The original L0 described only rows plus a delimiter pair while the consumer had accumulated optional absolute gaps and integer augment; neither retained the complete public signature/presence. The public math.mat carrier gate was explicitly confirmed on 2026-09-17 in `lab/history/construction/records/00_nucleo/materialization/typst-passo-1431.md`, section Confirmação do gate. It is bounded to the complete math.mat slice and its owners, not the other 16 math routes. This reconciles the stale proposed heading.

`lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diff/sources/binaries/cases at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Raw/query mat exposed only {func:math.matrix}, omitted rows/fields and returned false func-equality. Upstream matrix.rs:91–221 declares properties/rows; local math_matrix.rs:45–52 retained rows/explicit but get_field defaulted, content_methods.rs:151–191 omitted mat and cli.rs:1154–1348 used body fallback. This is language morphology/semantics, not PDF bytes or required Rust representation. additional-before.json.gz and matrix.rs:404–417 establish compact integer augment for absent stroke/empty hline/single vline; otherwise dict. No upstream intent is inferred.

## Ratified carrier

MathMatrixElem retains rectangularized rows:Vec<Vec<Content>>, delim:(Option<char>,Option<char>), align:HAlign, augment:Option<MathMatrixAugment>, row_gap:Rel<Length>, column_gap:Rel<Length>, and explicit:MathMatrixExplicit with delim/align/augment/row_gap/column_gap bits. MathMatrixAugment retains hline:Vec<isize>, vline:Vec<isize> and optional stroke, where absence is auto. Keep signed offsets until the consumer. The augment bit distinguishes omitted from explicit none. gap is not a carrier field: the boundary expands it causally into both gaps.

Use Content::MathMatrix with Arc and preserve the ergonomic constructor through its approved adaptation. Derived Debug/Clone/PartialEq/Hash retain every field, order, value and bit; the old rows+delim-only shape is historical. plain_text joins cells with `, ` and rows with `; `; is_empty stays default false. map_content recurses row/cell order retaining all other fields, map_text is terminal. No walker resolves cascade, negative offsets, auto stroke or alignment points. element_kind/to_payload remain None: the carrier is nonlocatable.

## Field projection

Implement the existing get_field method: rows is always present; delim/align/augment/row-gap/column-gap only when its explicit bit is set. Project rows/cells as arrays, delimiters as string/none pairs, horizontal align as Value::Align, gaps as Value::Relative, and padding Empty as the language's empty Sequence without changing carrier/render.

For Some augment, emit an integer exactly when stroke is absent, hline empty and vline has one item. Otherwise retain the complete dict with hline/vline arrays and auto/actual stroke. This supersedes the older always-dict description without a raw-input field or new signature. None and all explicit bits remain unchanged.

The compact-Value integration evidence in `lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/{before.json.gz,inventory.json,sizes.json}` freezes sources/file-line occurrences and measures an external Value model 272→144 bytes, not RSS. The owner confirmed manifest `6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237` on 2026-10-08. The Value owner alone defines the carrier; this getter uses existing From<Stroke>, while MathMatrixAugment keeps its own Option<Stroke>. Preserve integer/dict/auto/arrays/presence/offsets/all fields and do not expand semantics/defaults/phase.

## Explicit guard evidence and contract

`lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz` records source/L0/SHA/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat for 28 guards in 21 owners, including T285 at math_matrix.rs:127. This is Rust structure, not proof of a functional defect or improved AI reading. Match Some(augment) once and put the integer-versus-dict condition inside its body, preserving short-circuit behavior/order/fields and existing success/fallback/error paths. No helper to hide the conjunction, suppression or linter reconfiguration is allowed.

## Acceptance and limits

Require field/has/at/func/JSON/query/padding/presence bilateral RED→GREEN, builds/lint and D05/D06/query regressions. Preserve existing hash/equality/traversal, and do not certify unmeasured mat selectors, render, general Styled or other elements. Compact-value adaptation retains tests/oracles, adds RED→GREEN only for new isolation behavior and reseals only the owner.

Guard maintenance is continuous ADR-0127 without API/default/compatibility/phase change: existing functional controls first with unchanged expectations, real V17 RED28→GREEN0, restricted resealing; full workspace tests/build/all-target Clippy with warnings and unfulfilled expectations as errors, fmt, strict Rustdoc, strict/reverse V1/V5/V15/V26 and no-new-warning SARIF. Use 12 GiB, swap 0, one job. No generic memory/performance/parity claim follows. The pinned reuse proof below preserves these owner-specific algorithms and does not enlarge any approval.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_matrix.rs:162` preserves traversal but materializes the element even when
descendants do not change. `01_core/src/entities/content/traversal.rs:111–192`
unconditionally wraps results from these owners in Some. This is mechanical
cost; language behavior and element semantic identity remain preserved.
Refutable inference: propagating explicit absence of change reduces allocations
and temporary retention. Probes and timing controls may refute that gain.

Internal refinement under ADR-0127: provide only a crate-private
map_content_if_changed helper for elements already owned by this consumer.
Return None only when no actually visited child was replaced; childless
terminals return None. Transform children once in the existing map_content
order and stop at the first error. A callback Some, even containing equal
content, always means change. Preserve every field, value, explicit presence,
omitted/none/present slot, flag, span, and metadata item. Do not use equality,
hash, or address as a certificate. Allocate vectors/rows only after the first
changed child/row; unchanged siblings retain shared content. Public signatures
and trait methods remain unchanged, with internal adaptation to this proof;
map_content_pre and map_text retain their previous contracts. This refinement
does not expand any ratification of historically proposed contracts.

Specific logic remains with this owner; Content's exhaustive match only
delegates statically. No vtable, new enum/fields, global state, I/O, or new
phase. Acceptance requires RED→GREEN for reuse, order, errors, equal-value
replacement, optional fields/slots, and metadata; core suite, builds and lint;
text/morphology/render/query qualification against the previous release and
pinned vanilla; and paired allocation/timing measurements with provenance.
Do not infer a universal improvement from isolated cases.

The shared proof of absence of change follows the pinned nucleus claims.
This prompt and its sole consumer still own the algorithm, transformed fields,
visit order, and their specific preservation obligations.



## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_matrix.md` preserves the original evidence and chronology.
