# L0 — svg/groups
Hash do Código: 27cad24e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/groups.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/groups.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Groups and local clips

Own render_group, geometric clip validation/writing and matrix-identity checks.
Recurse through façade dispatch. Position/transform apply once; local clips and
children share the same spatial basis. Nested clips intersect with ancestors.
None adds no clip. Finite representable Rect/RoundedRect/Ellipse/Path creates a
local clipPath and resolved reference: dimensions use inner_width/height, corners
remain distinct and paths preserve subpaths/curves. Share collision-free deterministic
clip IDs with images. Allocate a new definition for each accepted clip; this
consumer does not deduplicate equivalent clip definitions.

clip_mask carries no fill-rule. Paths requiring evenodd, open lines without area,
nonfinite/unrepresentable or absent information remain Unknown/CONTRACT-GAP;
do not claim missing clipping is equivalent or silently drop the content. Alpha masks,
filters, opacity and rasterization are not substitutes for geometric clipping.

## Verification

Check supported shapes, transformed/nested clips, unique clip IDs, reference
closure and opaque/nonfinite/fill-rule gaps.


## Prepared font context and private rendering session

### Evidence

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/probe.txt`
measures original-face/axis identity. Human approval covers TextShaped.font;
pinned shaped-face-identity supplies shared invariants, this owner only its unit.
`too-many-arguments-2026-10-15/before.json.gz` retains full sources/prior L0/
signatures/hashes/UTC/status/diffstat at HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree captured
2026-10-15T16:39:18.873477+00:00; classification.json records per-unit decisions
and reconfirmed Clippy spans. render_group at groups.rs:16 has 14 arguments
(threshold seven), historical C067. Group forwards seven shared SVG components;
local inner dimensions already form Size. Arity alone proves no functional defect;
this is private Rust maintenance, no new parity proof.

`clippy-aridade-final-2026-10-16/before.json.gz` retains checked sources/L0/HEAD/
uncommitted tree/UTC/status/diffstat/global hashes; callers.txt audits workspace/
fixtures. Homonymous MathLayouter calls are a separate engine. The prior inventory
has 22 warnings/five arities. This follow-up materializes only the named correction,
superseding its earlier audit-only pending status, no other pending contract.

### Current contract and acceptance

Transport private PreparedFonts through SVG wrappers without public API/geometry/
semantics change, distinguishing original face/axes and prepared bytes, no renewed
name lookup. Consume the SVG hub's private borrowed definitions/fonts/IDs/mode
session through static descent. render_group(xml,pos,matrix,clip_mask,size,items,session)
uses existing Size/internal Pt without recalculation, reading width/height and
sharing the clip counter. Keep explicit geometry/children/free-function delegation,
serialization/matrices/fallback/child order/geometric helpers and local paint_root
restoration, destinations/fonts/HTML. Clips/transforms apply once and image-shared
clip IDs remain unique.

Preserve layer/phase/default/static dispatch/no mutable globals. Private signature
changes migrate callers/their L0 first. Any new public/external compatibility need
requires concrete ADR-0127 confirmation; no suppression or threshold change is
implied. Continuous internal materialization adds no external/default/phase/
compatibility change. L0 first/scoped resealing; real too_many_arguments RED→GREEN
without new suppressions/threshold, old expectations intact. Check bodies/value
transport and existing language/integration/call controls, Grid/Table/outline/
SVG/tiling before/after, no fixture/tolerance relaxation/carrier-mirror tests.
Full workspace suite/build/all-target Clippy/Rustdoc/fmt/strict/reverse lineage/
Tekt no new warnings under 12 GiB, zero swap, one job; no global-parity/PDF-UA claim.
