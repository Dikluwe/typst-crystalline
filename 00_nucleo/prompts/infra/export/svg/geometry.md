# L0 — svg/geometry
Hash do Código: 32089e90

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/geometry.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/geometry.rs:14` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Shapes and stroke geometry

Own line/shape rendering, generic path and rounded-rectangle path helpers, fill/
stroke/cap/join/dash/fill-rule attribute writers. Consume PaintReference from paints;
do not reconstruct paint, layout or Length. RoundedRect receives four resolved Pt
radii. With stroke, use max(radius - thickness/2, 0) per corner on the stroke-center
path, separating fill and stroke so the external radius remains the declared radius.
Without stroke use resolved radii directly. Fill closes its path; rounded stroke
returns to its start without Z, preserving cap/join morphology.

For Shape stroke attributes, preserve cap/join/miter limit, dash array order and
phase including negative values;
resolve DashLength::LineWidth using effective thickness. Omit attributes only when
semantically identical to SVG defaults and supplied carriers; do not bake transforms
into stroke lengths. Keep alpha and operation order. Use supplied evenodd/nonzero
fill rule without inferring winding. The separate FrameItem::Line writer currently
emits butt regardless of the line-cap carrier; do not claim cap parity for that path.

An exactly open MoveTo+LineTo path uses translate(first point) and a relative line
delta, without changing geometry/paint. Closed, cubic and other paths retain generic
serialization. Rounded-path helpers also serve groups' geometric clips without
taking ownership of clip transforms or support validation.

## Verification

Check four radii, stroke-center inset, open paths, complex dash/cap/join, alpha,
fill-rule and resolved relative-radius inputs.


## Resolved-paint transport and nominal arity

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

Receive FramePaint/FrameStroke while preserving all fill/stroke/fill-rule geometry.
Tiling references use Shape geometric origin for stroke too; half-thickness expansion
continues only for existing gradients. Never synthesize a cell/reduce tiling to color.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

### Arity evidence and exact exception

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`
and classification.json retain full source/prior L0/signatures/hashes/UTC/status/
diffstat at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree captured
2026-10-15T16:39:18.873477+00:00; spans/contracts checked before classification.
render_shape at geometry.rs:31 has nine arguments, threshold seven, historical
C065. Resolved geometry/fill/stroke/fill-rule/PaintDefs are one leaf operation,
not a retransmitted recursive session; paint/layout decisions remain separately
owned. Arity alone proves no functional/parity/cognitive benefit.
Nominal materialization evidence clippy-locais-2026-10-16/clippy-inventory.json
(79 warnings) and clippy-excecoes-2026-10-16/before.json.gz retains reconfirmed
sources/reasons/spans/signatures/hashes/HEAD/tree/UTC/diffstat.

Only render_shape receives
`#[expect(clippy::too_many_arguments, reason="C065: Leaf SVG de Shape usa a geometria resolvida, fill/stroke/fill-rule e PaintDefs para referências locais")]`.
Preserve signature/order/body/calls/cfg/other attributes. This materializes only
C065, superseding its audit-only pending status and generic suppression ban for
this item/lint. No module/impl/crate/global scope/threshold/other lint; remove or
review if arity drops, never suppress unfulfilled expectations. Pending corrections
including C047/C066 gain no direct annotation. Private signature correction
requires caller/L0 migration; new public/external need requires concrete ADR-0127
approval. Preserve existing owner language/integration/call controls and tolerances.

### Acceptance

Continuous ADR-0127 changes only lint metadata, no public contract/executable body/
cfg/default/phase/dependency/architecture. L0 first/scoped resealing. Removing added
attributes and lineage header must recover prior rustfmt-normalized source.
Measure real RED, then all-target inventory: historical batch exactly 46 arities
plus five test warnings satisfied by 49 expectations, no new warnings and
unfulfilled_lint_expectations as error; its 11 correction arities remain visible.
Keep owner functional/call controls without fixture/tolerance relaxation; private
signature corrections migrate callers/L0 before code and new public/compatibility
needs another ADR-0127 gate. Validate build/fmt/strict/reverse lineage/Tekt no new
errors/warnings, existing Counter/coverage tests under 12 GiB, zero swap, one job.
Prior full suite is evidence for identical executable code, not a fresh run;
no lint-attribute mirror tests or new performance comparison for metadata.
