# L0 — svg/navigation
Hash do Código: 4c3e29ad

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/navigation.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:199640afc0a7243ca509288840d9c9e0c431f850f67919148fc8dac3b1cb33a0
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b

## Consumer evidence

`03_infra/src/export/svg/navigation.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Page-local destination context

Own SvgDestinationContext/SvgDestination, label normalization, lookup, destination
nodes and render_link. Consume the pinned destination-context nucleus rather than
duplicating its shared page-local registry/routing obligations in the façade.
The immutable context performs no layout/filesystem lookup and carries no filenames.

Emit one local node per present destination at its supplied position and bind the
link edge using the same collision-free identity. Missing/cross-page destinations
without an explicit route preserve visual children and remain Unknown, not dangling
fragments. External URLs retain their target. Preserve hit area, escaping, nested
children and transforms. Caller-owned bundle/cross-page filenames are not inferred.
SvgDestinationContext constructors and normalization belong here. SvgOptions and
export wrapper defaults remain in `00_nucleo/prompts/infra/export/svg.md`.

## Verification

Check same-page closure, deterministic IDs, missing/cross-page fallback, URL targets,
escaped labels, nesting and hit-area transforms.


## Prepared fonts and navigation session

### Evidence

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/probe.txt`
supports private PreparedFonts transport distinguishing original face/axes and
prepared bytes. Human approval covers TextShaped.font; pinned shaped-face-identity
owns shared invariants, not this consumer.
`too-many-arguments-2026-10-15/before.json.gz` retains full sources/prior L0/
signatures/hashes/UTC/status/diffstat at HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree captured
2026-10-15T16:39:18.873477+00:00; classification.json records individually read
contracts/reconfirmed spans. render_link at navigation.rs:83 has 12 arguments
(threshold seven), historical C069. Forwarded SVG session components are shared;
arity alone proves no functional defect. No new parity/upstream intent inferred.

`clippy-aridade-final-2026-10-16/before.json.gz` retains checked sources/L0/HEAD/
tree/UTC/status/diffstat/global hashes; callers.txt covers workspace/fixtures.
MathLayouter homonyms are another engine. Prior inventory: 22 warnings/five
arities. The follow-up materializes only this named correction, superseding its
earlier audit-only pending status.

### Contract and acceptance

Preserve public geometry/semantics/API while transporting private PreparedFonts
through wrappers, no new name resolution. Consume hub's private session with
definitions/fonts/IDs/mode; keep target/items/pos/size explicit and a static
navigation free function. Current signature:
render_link(xml,target,items,pos,size,session). Destinations come from
session.destinations; children reuse session. Preserve immutable registry/public
API, href/fallback/hit area/transforms/serialization/children/HTML mode. No filename
inference/layout.

Keep layers/phase/defaults/static dispatch/no mutable globals; migrate private
callers/their L0 before code. New public/external compatibility needs concrete
ADR-0127 approval. Internal continuous materialization adds no external/default/
phase/compatibility contract. L0 first/scoped resealing, too_many_arguments real
RED→GREEN without suppression/threshold changes, previous expectations intact.
Check bodies/value transport/existing language-integration/call controls with no
relaxed fixtures/tolerances/carrier-mirror tests; Grid/Table/outline/SVG/tiling
before/after, full workspace suite/build/all-target Clippy/Rustdoc/fmt/strict/
reverse lineage/Tekt no new warnings under 12 GiB, zero swap, one job. No global
parity/PDF-UA certification.
