# L0 — Visual bidirectional line ordering
Hash do Código: c402e136

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3
**Exclusive consumer:** `03_infra/src/layout_bidi.rs`

## Current evidence

- `03_infra/src/layout_bidi.rs:23`, `:39`, `:49`, `:128` — metrics, document/page/line passes.
- `03_infra/src/layout_bidi.rs:183`, `:305`, `:334`, `:409`, `:485` — visual ordering, space coalescing and suffix separation.
- `03_infra/src/layout_bidi.rs:646`, `:782` — controls preserving L1 wrapping.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Preserve `reorder_bidi_document(doc: PagedDocument, metrics: &dyn FontMetrics) -> PagedDocument`. Own horizontal visual ordering of top-level page items before shaping. The paged pipeline owner places this pass; paragraph direction and line breaking remain owned by L1 layout.

Cluster page items by baseline with `Y_TOLERANCE_PT=0.01`. Semantic envelopes retain their existing first-child coordinate convention. Detect an RTL base paragraph from concatenated plain Text with Unicode bidi auto direction; an LTR-base line remains unchanged. Reordering operates on plain Text only. Nontext items retain their geometry even when vector ordering changes.

For an RTL line, use `FontMetrics::text_width` for word widths and `line_content_right` for the occupied right edge. Compute nonnegative inter-item gap from occupied width minus total word widths, divided by the number of intervals. Resolve visual runs over item text ranges; explicit RTL text style supplies the RTL default level inside this ordering helper. Preserve text/style pairing, unmapped-item handling, empty-paragraph fallback and left-to-right positioning from the existing `x_min`.

Sort the line's items by increasing x. A standalone whitespace item can be coalesced as one trailing space into the preceding nonspace Text in visual order; retain the existing empty-item placeholder behavior rather than remove entries while indices are in use. Separate a terminal LTR suffix from an RTL base using L/EN/AN classes and existing ASCII punctuation handling. Preserve original trailing whitespace on the base, keep its style/y, and keep the original occupied slot using the measured suffix-placement rule below. Pure-suffix text is not split.

L1 baselines are authoritative: never merge lines, recompute wrapping, infer paragraph boundaries from distance, estimate width from byte length or infer the region width from emitted ink. `ExplicitLinebreakBoundary` and `ParbreakBoundary` remain transparent carriers; no cross-baseline fusion consults them. Preserve page dimensions, pagination, leading and nontext positions. This pass neither supplies vertical writing nor guarantees an extractor-specific logical text order.

`00_nucleo/prompts/infra/shaper/bidi.md` owns character-level shaping runs. This owner does not duplicate its run data or shaping policies.

## Verification boundary

Existing tests cover Latin identity, Arabic/mixed visual order, real widths, whitespace and suffixes, empty text, nontext geometry and preserved wrapping/explicit breaks. Sequential extraction must not justify changing glyph order inside the shaper.

## Approved text carriers

Source→PDF RED probes/handler audit in fatia-funcional-tekt-2026-09-28 precede the
layout_types public-carrier approval on 2026-09-28, recorded in materialization
provenance. Move TextSeparator coordinates without coalescing its origin into Text;
after RTL ordering place separator in preceding slot's visual interval, never
fabricate carriers from gaps. Existing observable text/geometry/adjacency and
pinned invariants remain.

Discretionary-identity initial/before probes in identidade-hifen-discricionario-
2026-09-28 measure eight equal-bbox U+002D/SHY cases, with exact state/audit in
approved-before and owner-confirmed layout_types. Keep TextReplacement opaque to
Text split/coalescing, coordinates from children, no substitute text in shaper/
reflow/new-break inference or full-RTL certification. Require text/bboxes/causal/
recursive/literal-hyphen controls, not PDF bytes/new selection.

## Multiline suffix references and occupied slot

### Evidence

rtl-numeros-2026-10-05/initial.json.gz retains HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b/full dirty tree/UTC/binaries/12 CLI controls.
Eight attached numeric groups match; twelve extract 10 before 8/9 in Tekt versus
vanilla sequence, separated-number control matches. layout_bidi.rs:411–422,467–486
inserts into first line without updating later line indices, potentially separating
the wrong text; causal text/baseline association refutes the inference. Historical
suffix intent grants no lost-line permission; vector/index strategy is mechanics,
text/position observable.
index-fix-verification.json.gz shows residual geometry after index repair:
word+10 starts x=143.524530 versus suffix x=157.521600 upstream, preceding word
ends x=154.025016; explicit-break word+2 x=405.343870 versus 412.342400. The old
x−suffix_width was applied to an already suffix-inclusive slot, not bare base.

### Current contract and acceptance

Freeze every eligible RTL line's original indices, process insertions in decreasing
original index, never reuse shifted other-line indices. Private indices are
discarded after separation; preserve original base/suffix/style/baseline/trailing
space, wrapping/phase/public API/shaping. Protect unprocessed references regardless
of x/y/cluster order, no universal-RTL promise. Causal tests need ≥3 baselines plus
intermediate LTR, cluster-order differing from index order and nonmonotonic x;
assert text/style by baseline, not vector algorithm as language. Source→PDF measures
text/baseline/positions against pin and records residuals explicitly.

The occupied-slot refinement supersedes only old x_base−suffix_width: put suffix at
x_original, base at x_original+suffix_width, preserving slot's left edge/baseline/
style/trailing space without double width reservation/character-bidi/reflow.
RED→GREEN reserved-span and measured text/position controls. Sum widths must remain
in original slot where base/suffix have no shaping interaction; tracking/kerning
complexities need their own controls, no universal assumption.

Ratified bidi carrier inventory/1ae7513e uses None for uncertified constructors/
read `..`, no phase/API expansion.

## Full-paragraph bidi context

### Evidence

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{initial,probe}.json.gz`
retains bilateral cases/Unicode refutation and sources/state/binaries.
`layout_bidi.rs:73–82,227–240` redetects base/invents separators;
`shaper/bidi.rs:34` redetects a fragment, losing the human-ratified complete input.
This internal parity correction adds no phase or external public signature.

### Boundary contract and acceptance

For certified spans, use the compilation-local registry of auxiliary strings/maps
by input Arc, then normally borrowed BidiInfo views; no unsafe/self-reference.
Virtual LRE/RLE/PDF is inserted only when segment direction differs from base.
Retain real spaces, nonjustified Linebreak LF and author controls; justify=true
is outside certification. Resolve once per input per registry; paged pipeline
shares registry/views between reorder/shape. Existing public wrappers may create
a local equivalent registry.

Reorder queries the actual line interval, exact segment/range intersections and
end-of-line levels, never midpoints or invented spaces. Split mixed Text at original
boundaries preserving style/subspan. Nontext items and unknown sources gain no
certificate. Preserve the L1-accepted physical anchor and actual width/advance;
certified context cannot use suffix/coalescing fallback.

Shaping queries full-input runs for each original span without fragment direction
redetection, preserving sequential subrun offsets and exact TextShaped subspans.
The existing cache key already distinguishes direction; add no context to
TextStyle. Reconciliation uses input base when present, not any RTL run; retain
prior None fallback. Require RED→GREEN for `456 789` refutation, overrides/neutrals,
no auxiliary controls in extracted text and bilateral measurements. Registry
implementation is exclusively owned by `00_nucleo/prompts/infra/bidi_context.md`;
reordering and shaping remain with their own canonical owners.

Empty-range Text keeps position/style/span/causal effects but takes no ink permutation
and is not removed when the Unicode interval is empty. RED→GREEN Empty Text with
mixed text preserves original input identity.

## Test-only suffix adapter

ci-helpers-testes-2026-10-14/before.json.gz/references-before.json.gz retains
source/owner/HEAD/tree/UTC/diffstat/calls/imports/modules/references/test list and
hash-checked previous suite/inventory at :572. Audited private calls/compilation,
not dead_code alone, establish split_ltr_suffixes_page is test-only. Apply cfg(test)
only there, preserving test signature/body, productive split_ltr_suffixes_line and
its index-safe caller. Public reorder/fallback/origin/coordinates/text unchanged.
Continuous ADR-0127 no public/default/phase/dependency/suppression; L0 first/scoped
seals, selected RED→GREEN/unchanged test inventory/expectations/full suite/lib-all-
target build/Rustdoc/fmt/lineage/full inventory, no cfg/binding mirror tests.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Migrate FrameItem::Shape fixtures to FramePaint/FrameStroke with unchanged values/
semantic assertions. Tiling explicitly constructs a prepared cell; eval fixtures
remain declarative. No total productive conversion or weakened assertion.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.
