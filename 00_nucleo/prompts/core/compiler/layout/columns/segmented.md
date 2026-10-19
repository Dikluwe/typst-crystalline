# L0 — compiler/layout/columns/segmented
Hash do Código: fb3ce2d6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57

## Evidence before specification

At HEAD `f7195cdf6` (clean tree, 2026-09-30), the block is at `01_core/src/compiler/layout/columns.rs:192-309,407-639`. This observation establishes extraction boundaries, not performance or new bilateral parity.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. The pre-repair snapshot is `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

The source-origin baseline at `lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv and binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. The source has the corresponding transparent operation in existing ShowRecipe/Located/Realized branches; Source must not leak as an observable element. Human approval was `confirmo`, 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`. `workspace-fifth` refutes hiding colbreak under Source.

## Ownership and contract

Layer L1. Sole productive consumer: `01_core/src/compiler/layout/columns/segmented.rs`. Own colbreak segmentation, direction discovery and sibling-region layout, preserving order, logical state and empty-segment semantics. Initialize/close boundaries in isolated segment buffers, preserving parent boundaries when restoring. Do not include exterior prefixes/suffixes in the link; retain sibling-region geometry and order.

Segment transparent body before Styled/Sequence and discover direction/visibility through the same body, preserving sibling regions and empty segments. Do not manufacture colbreak or alter position/order/fractional share. Retain exclusive ownership/static dispatch, without I/O or a new signature/trait/flag or eval↔layout phase.

Preserve public contract, default behavior, pipeline phase, compatibility, purity and static dispatch. Link capture is an internal correction under ADR-0127 with no new API, default or phase.

## Acceptance and limits

The L1 suite, build and crystalline-lint must pass. Separation does not prove a performance gain.

Require RED→GREEN for breaks before/within the body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

Source-origin tests and C2/C/macro collections, builds and lineage are required before certification.
