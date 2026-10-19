# L0 — compiler/layout/columns/flow
Hash do Código: 2f314381

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57

## Evidence before specification

At HEAD `f7195cdf6` (clean tree, 2026-09-30), the block is at `01_core/src/compiler/layout/columns.rs:125-191,640-704`. This observation establishes extraction boundaries, not performance or new bilateral parity.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. The pre-repair snapshot is `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

## Ownership and contract

Layer L1. Sole productive consumer: `01_core/src/compiler/layout/columns/flow.rs`. Own continuous flow and block boundaries between columns, preserving collapse, cursor and pagination coordination. Preserve lexical capture through column transitions performed by `00_nucleo/prompts/core/compiler/layout/columns/transitions.md`; no scope belongs to indices from the previous column.

Preserve public contract, default behavior, pipeline phase, compatibility, purity and static dispatch. Link capture is an internal correction under ADR-0127 with no new API, default or phase.

## Acceptance and limits

The L1 suite, build and crystalline-lint must pass. Separation does not prove a performance gain.

Require RED→GREEN for breaks before/within the body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.
