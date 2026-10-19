# L0 — infra/shaper/traversal
Hash do Código: 83be52a8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/traversal.rs`

## Current evidence

- `03_infra/src/shaper/traversal.rs:17` — `shape_document`.
- `03_infra/src/shaper/traversal.rs:34` — `shape_page`.
- `03_infra/src/shaper/traversal.rs:40` — `shape_layer`.
- `03_infra/src/shaper/traversal.rs:59` — `shape_item`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/traversal.rs`.

## Owned contract

- Own shape_document/page/item traversal. Create run and face caches per document and visit background, items and foreground in that order.
- Descend into Group, Link and Semantic without changing wrapper metadata, positions or child order. Preserve the closed static dispatch and nontext items.
- Delegate Text with a requested font to shaping::try_shape_context; preserve original text on absent font, failed resolution or whitespace. Do not reshape existing TextShaped or implement selection in this walker.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Approved textual carriers and tracking

Source→PDF RED probes in
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and this
consumer's handler inspection precede separator changes; public layout_types
approval was received on 2026-09-28 with materialization provenance. The
`identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` probes preserve
state and eight U+002D/SHY controls at equal geometry; FrameItem traversal audit
is in approved-before/. Owner confirmed layout_types. Text/bbox/causal/recursive/
literal-hyphen controls, not whole PDF bytes/new break selection, define acceptance.
Pinned nuclei provide shared invariants without ownership sharing.

## Carrier traversal and authored identity

Preserve TextSeparator carrier, no ink conversion or fusion with neighboring text.
TextReplacement is visually transparent: descend children preserving author text,
order/coordinates/current calculation without new advance/algorithm.



### Evidence

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/{initial.json,before/resultados.json}`
retains state/binaries/commands and 30 bilateral probes. At 300pt,
repre<U+00AD>sentation extracts two SHYs in Tekt versus one in pinned vanilla;
spaces control extracts six versus one. shaping.rs:313–325 carries rustybuzz SHY;
export/fonts.rs:318–328 maps its substring to a space GID; builder/fonts.rs:523–535
keeps one identity per GID. Prior stream associates CID 0001 with 00AD then reuses
it for later space. This is observable text, not byte equality. Missing that
association or pre-shaper duplication would refute the causal hypothesis; no
historical intent inferred.

After successful try_shape delegate SHY subrun protection to text_identity,
retaining glyphs/visual text/positions. Within existing TextReplacement traverse
children with protection suppressed, preserving outer author identity without
nested envelope. Private traversal helper carries that state; public shape_document
unchanged. Missing-font failure preserves Text. shape_item pub(super) is only the
test adapter under its separate cfg rule below.

Ratified bidi inventory `bidi-contexto-2026-10-05/inventory.json`, manifesto 1ae7513e,
requires None in uncertified constructors/read `..`, no origin-control or phase/API
change.

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

## Certified per-occurrence bidi clusters

`lab/results/construction/diagnostics/bidi-identidade-ocorrencia-2026-10-06/before.json`
retains HEAD/tree/code/L0/binaries/rustybuzz probe. SHA-versioned DejaVu with
REMOVE_DEFAULT_IGNORABLES gives LRM+ABC/PDI+ABC A cluster 0, B 4, C 5; 0..4 contains
control+A. fonts.rs:320–347 removes visual controls, text_identity.rs:15 certifies
only SHY; earlier controles-bidi-pdf verification.json.gz observes absent U+200E/
U+2069 in three occurrences with preserved ink. Historical vanilla 9f2b6e871
inline/shaping.rs:1052–1099 supplies full cluster ranges to export. Unicode/text
semantics/morphology, not GID/bytes/algorithm, determines acceptance. Missing
identity/duplication on another same letter/changed ink refutes the envelope fix;
no inferred intent.

Use existing approved TextReplacement, no public field/method/signature/phase/mode/
flag/dependency/unsafe. Certification comes only from successful shaping UTF-8
ranges, never GID/advance/position/appearance/font/control presence alone. Delegate
protect_authored_identity only after successful shaping and outside existing
TextReplacement. Walker never inspects clusters or implements certification;
keep cache/face/static descent, original envelope and no nested replacement.
Integration tests retain scope/cache-hit proof under pinned text-replacement.

## Test-only single-item adapter

`ci-helpers-testes-2026-10-14/before.json.gz` and references-before.json.gz retain
source/owner/HEAD/dirty tree/UTC/diffstat/calls/imports/modules/references/test list,
hash-checked previous suite/inventory at :77. Private-call compilation was audited,
not dead_code-alone inference. Only shape_item is cfg(test): it is pub(super)
inside private traversal, not public L3 API. shape_item_inner/shape_document remain
productive with their contexts/invariants; production never reenters a contextless
wrapper. Preserve all tests/delegation. Internal continuous ADR-0127 changes no
public/default/phase/dependency/suppression; selected RED→GREEN and unchanged
inventory/expectations/full suite/lib-all-target build/Rustdoc/fmt/lineage/full
inventory/scoped seals, no cfg/binding-removal mirror tests.

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

Shape prepared cells visually once per occurrence using Arc::make_mut and
existing local caches, preserving size/spacing/placement. Each cell has its own
bidi context; do not insert its identities into the page.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.
