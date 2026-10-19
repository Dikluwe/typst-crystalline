# L0 — compiler/layout/finish
Hash do Código: bfd8e315

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

## Evidence before specification

At HEAD `f7195cdf6` (clean tree,2026-09-30), the block is at `01_core/src/compiler/layout/mod.rs:1957-2281`. This establishes extraction boundaries without performance or new bilateral parity measurement.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`box-contexto-2026-10-14/probe.json.gz`preserves sources/binaries/HEAD/working tree/UTC: Box, inset2pt, prefix and nesting publish (0,0) despite displaced paint. The direct-context baseline difference is separate; do not copy vanilla merely because it differs. `mod.rs:1131-1148/1180-1200` writes local points to runtime.positions; Box groups its body in another coordinate system; `finish.rs:295-302` seals the map without recomposing those points. Realized writes through or_insert; labels have a parallel map. HTML already carries private markers through the frame at `html_frame.rs:30-199`. Preserve single-pass registration and per-sub-store sealing under ADR-0073/0074 position/hybrid-model sections; do not adopt vanilla global introspection. Inspection of coordinates and its owners found absolute uses in Grid and explicitly local uses in Place/Box; nucleus qualification preserves both and requires page transformation without doubled origin. Earlier snapshot: `posicoes-compostas-2026-10-14/before.json.gz`. Geometry/identity are observables; carrier/index are mechanics.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

## Ownership and contract

Layer L1. Sole productive consumer: `01_core/src/compiler/layout/finish.rs`. Own terminal paginated-document composition, preserving flush, fixups, layers and result transport. Keep public contract, default behavior, pipeline phase, compatibility, purity and static dispatch.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

After all composition/fixups/layers, resolve private boundaries into per-fragment links with bounds in final coordinates. Do this before position projection, which traverses resulting links. No private boundary may escape into PagedDocument.

After final transformations/fixups/layers and before moving pages into PagedDocument or sealing sub-stores, project/remove position carriers through the private helper. Continue consuming runtime.positions/label maps through existing APIs. Do not reconstruct global introspection, change phase, repeat layout or create a public store. These are internal ADR-0127 corrections without a new public field/API or flag/default/phase.

## Acceptance and limits

L1 suite, build and crystalline-lint must pass; separation proves no performance gain. L0 precedes RED→GREEN for coordinates, no fictitious paint/height, identity/order/visual clones/rollback. Revalidate HTML, geometry, context/queries and the complete suite.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.
