# L0 — infra/export/font_binding
Hash do Código: 2dd39803

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/font_binding.rs`

## Evidence and approval

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/probe.txt`
shows regular GIDs bound to Oblique by later style resolution: shaping.rs:254–255,
358–362 loses the face and stream/text.rs:22–43 uses only style key. Confirmed
TextShaped.font manifesto:
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`.
Mechanical identity integrity is necessary for correct ink, not identical PDF/
vanilla algorithm. Already identical faces at both stages would refute the cause.

first-benchmark.json records +25.36ms grid320/+14ms rtl160/mixed160. index compares
whole bytes per lookup and verbose_run_compatible compares Font bytes per adjacency;
this causal inference requires A/B confirmation.
`font-binding-memoria1-2026-10-08/before.json.gz` freezes font_binding.rs:117–127,
which supplied.to_vec() copies all resource byte Vecs before traversal even if
nothing is appended, then discards the copy and returns original slice. This is
internal temporary duplication; a repeatable paired-RSS reduction must establish
product benefit, not inference alone.

## Private binding and partition contract

Own association by original face and complete request (derived variant plus explicit
axes), independent of World/slot/family. Use unchanged public FontKey/signatures.
Preserve supplied resource order, append missing actual faces at first occurrence
across background/body/foreground/containers. Family remains actual, no fake name/
variant to simulate identity.

For Some, binding index must match original bytes/request. Missing binding never
uses slot zero or style selection. None retains legacy selection. PDF binds original
bytes with instancing/subset at the same index. SVG/PNG preparation uses parallel
original/prepared arrays, instances once per resource, looks up original and
returns prepared bytes; failures retain existing instancing fallback. No renewed
World lookup. Cache scope is one export.

Distribute textual/Glyph leaves by resource in one traversal. Private collection
views for glyphs/clusters/chars/CMap carry no product geometry and never mutate
original document. GID range membership cannot move one face's GIDs into another.
Per-occurrence author ToUnicode repair remains outside scope. Direct mathematical
Glyph font identity is explicitly uncompleted: views retain legacy face participation
and emitter uses its existing reverse map.

## Local memoization and borrowed prefix

Keep positive validated cache per resource-table/export. Retain Font Arc clone to
keep memory alive and index after first full bytes/request equality. Hits require
slice address+length identity and same complete request; addresses never serialize
or become public identity. Distinct allocations require first complete equality.
Appending preserves positive indices; do not cache misses while the binder can
append. Merge comparison uses slice identity only as shortcut, full bytes for
different allocations. No globals/session reuse/World content memoization or
invalidation policy.

Begin borrowing supplied prefix. Copy it only at the first missing appendable
face, then append subsequent faces in encounter order to that same table. Preserve
single traversal and append-valid positives, no negative memoization. Missing
explicit family/valid metadata does not trigger copy. Empty documents/None font/
complete resource tables retain borrowing without byte-buffer copies. Preserve
face/request/duplicates/order/input immutability; SVG/PNG parallel arrays remain
unchanged. No public/default/phase change or enlarged scope.

## Acceptance

Require RED→GREEN fallback/duplicate-metadata/None/variable-axis/reuse controls,
bilateral ink/text/geometry/PDF/SVG/PNG, no unsafe/globals/new phase/API. Private
copy witness requires zero copies for complete prefix and one for multiple missing
faces, equivalent resources/exports and paired time/RSS before claiming gain.
Pinned shaped-face-identity is shared invariants only; this owner remains exclusive.

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

Traverse visual resources within fill/stroke TilingFrame for font binding and
partitioning. Use the private resource walker; public semantic L1 visitor still
does not descend into paint.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.
