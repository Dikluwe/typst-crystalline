# L0 — builder
Hash do Código: c1a5f9e0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder.rs:45` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own PdfBuilder, its private shared state, constructors and internal option setters,
the build entrypoint, and the Helvetica fallback orchestration. Preserve existing
signatures, visibility and defaults. Return PDF bytes and accumulated subsetting time.
Parse supplied font data for the CIDFont path; otherwise use Helvetica. Dispatch
CIDFont and multifont construction to the fonts child without moving their algorithms
into this façade. StreamMode, PdfSerializationMode, PdfTags and resolved PdfStandards
remain independent inputs; this module does not redefine their owning contracts.

Normalize an empty input document to one blank A4 page before dispatch, preserving
document metadata and attachments. Page filtering is supplied by the navigation
child. Keep catalog/pages allocation, shared object-ID coordination and assembly of
page dictionaries here; descendant emitters use that state without collisions.
Page geometry uses the physical canvas including bleed, with a nonzero TrimBox
describing the logical page; omit the optional TrimBox for zero bleed.

Compose fonts, resource dictionaries, page operators, physical stream envelopes,
navigation, metadata and final serialization through their owners below. The façade
must not duplicate their normative algorithms. Private descendants remain in L3;
do not introduce dynamic dispatch, public APIs or a pipeline phase change.

## Delegated owners

- `00_nucleo/prompts/infra/export/builder/fonts.md` owns `03_infra/src/export/builder/fonts.rs`.
- `00_nucleo/prompts/infra/export/builder/metadata.md` owns `03_infra/src/export/builder/metadata.rs`.
- `00_nucleo/prompts/infra/export/builder/navigation.md` owns `03_infra/src/export/builder/navigation.rs`.
- `00_nucleo/prompts/infra/export/builder/resources.md` owns `03_infra/src/export/builder/resources.rs`.
- `00_nucleo/prompts/infra/export/builder/serialization.md` owns `03_infra/src/export/builder/serialization.rs`.
- `00_nucleo/prompts/infra/export/builder/streams.md` owns `03_infra/src/export/builder/streams.rs`.

## Physical minimum and literal-space geometry

### Evidence

`lab/results/construction/diagnostics/correcao-auto-espacos-2026-09-29/{before/results.json,provenance.json,initial.json}`
retains 30 sources/60 PDFs, font-path/commands/UTC/HEAD/working tree/binaries against
historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Literal-only spaces
#text("   ") with tracking 0/±0.2 produce zero MediaBox (Poppler displays 612×792),
versus vanilla 9.36/9.76/8.96×8.568pt. Trailing literal separators lose auto-page
width; leading followed by text matches. At 18pt/+0.2pt vanilla is 14.44×12.852pt.
Empty text/inkless markup exposes separate baseline/minimum debt, final newline
its own vertical divergence, outside this slice.
space.rs:22–35 omits initial untracked Literal; page_geometry.rs:104–135 uses
line_content_right excluding all TextSeparator for correct RTL alignment and
carrier-only line has no typographic top; builder.rs:291/fonts.rs:633,1167 emits
zero MediaBox. Upstream PDF convert.rs:108–118 imposes 3×3 physical minimum. This
supports separate auto projection versus RTL alignment and one shared exporter
minimum, without inferred historical intent.

### Contract and acceptance

Centralize private builder PDF minimum at 3pt per axis, after adding bleed to
canvas in Helvetica, shared by both font paths without copied constant. Preserve
logical Page/TrimBox, above-minimum extent, page selection/version/modes. This is
pinned PDF output rule, no ad hoc typography compensation. Internal continuous
ADR-0127: no public field/signature/flag/phase; real RED→GREEN for 0/± literal
spaces/PDF minimum, bilateral causal controls/prior corpus/build/lint zero errors,
manifest-only seals. Explicit empty text/final newline require separate completion.

## Certified shaped-face boundary

`identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}`
distinguishes regular GIDs from Oblique selection. Human approval of public
TextShaped.font: Option<Font> is manifesto
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`;
implementation state/inventory: identidade-fonte-shaped-implementacao-2026-10-05/.

Migrate only this owner's references: legacy constructors None, transforms preserve
Font, shaped comparison/join distinguishes face/axes. The shaper supplies the actual
Font clone even on hits without semantic/style changes to force selection. Collectors/
subset/CMap/PDF bind the same original face and style coordinates. PNG/SVG prepare
local bindings and instance once per face/request before render, keeping original
resource index separate from instanced bytes; None follows legacy resolution.
Preserve public APIs/FontKey, no fabricated fonts/unsafe/globals/new phase; private
functions may receive Font/bindings explicitly.

Association uses original face bytes plus axes, never names/slots alone. Table
lifetime is export-local; preserve supplied resource order and append missing faces.
For multifont distribute items once before per-font collectors; no cross-face
GID/CMap reuse. References never share ownership. Require RED→GREEN/bilateral
miss/hit/variable-font controls without a promised speedup. Shared invariants remain
in pinned shaped-face-identity, no second productive owner.

## Certified single-face dispatch

first-benchmark.json records single-face regressions. If certified bindings contain
one font and require no VF instancing, use existing build_cidfont with that same
font/GID domain. VF axis requests remain multifont; without certified carrier keep
prior dispatch. Same phase/API, no fake names/style re-selection; verify single-
face and real-mix regressions.

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

Integrate private tiling PatternType 1 registration/emission in the Type1 path
before final page streams. Reserve IDs after gradients/before other resources,
share fonts/images/spot/pattern references and preserve page selection/tagging.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.
