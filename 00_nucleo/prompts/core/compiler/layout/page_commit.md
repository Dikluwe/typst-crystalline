# L0 — page commit
Hash do Código: fbc75fec

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_commit.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_commit.rs:18`, `01_core/src/compiler/layout/page_commit.rs:174`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners. Manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. The pre-repair snapshot is `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own intermediate Page assembly only, receiving dimensions already chosen by its caller. Drain emitted items; apply par.line, equation, horizontal-align and vertical-align fixups in their current order. Obtain explicit marginal layers from `00_nucleo/prompts/core/compiler/layout/page_running.md`, and background/foreground canvas layers from `00_nucleo/prompts/core/compiler/layout/page_canvas.md`; append marginal items to foreground. Preserve fill, resolved supplement and binding/parity-dependent bleed. Initialize logical_number to the physical page index; the later pipeline logical overlay is not this owner's work.

When the selected marginal is Auto, format a unary pattern using the known logical current number if available, otherwise the physical index. Defer compound patterns with the current marginal geometry to the hub's terminal total-number coordinator. Function numbering uses the already-realized visible PageStore Content, never Func execution. Append the completed Page without draining floats/footnotes, advancing a region or resetting the next cursor. Final-page composition remains in `00_nucleo/prompts/core/compiler/layout/mod.md`; its terminal lifetime is not a second consumer of this prompt.

For the ratified bidi-carrier migration, use None in uncertified constructors and `..` in read patterns, preserving existing language observables. Transformations follow the pinned origin nucleus, with no broader phase/API.

Close active segments in the committed buffer before taking it for the physical page; reopen continuations in the new buffer. Do not move already committed content into another page. Keep fixup/composition order. This link correction is internal under ADR-0127, with no new API, default or phase.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN for breaks before/within the body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Own intermediate physical-page assembly with source-owned slot and body contributions. Preserve fixup/paint order and page geometry. Register physical Background/Header before accepted body packets and Footer/Foreground after them even though paint composition occurs at page close; do not reconstruct semantic packets from frames. Final page association must reflect accepted deferred emission. Keep actual slot/environment bindings for the configuration that created this page, not a later sibling delta. Caller retains transition/float/note policies. Existing logical overlays and automatic numbering remain unchanged.

### Acceptance and preserved boundaries

Require semantic RED→GREEN for repeated physical header/footer/four-slot order, zero-paint updates, source-derived parents, overwritten inert templates, aliases and physical pages, effective20/30pt ambient plus explicit inner styles, nested PageRun restoration, Context reads, and exact duplicate-label/source diagnostics. Cover speculative suffix rollback, deferred floats/notes, subframes, measurement isolation, complete body/slot coverage, stale/mixed authority and math/par.line coexistence. Do not weaken existing tests, callback budgets, per-store sealing, source morphology, HTML isolation or old public compatibility. No generic painted-frame semantic reconstruction, source re-evaluation, second arena/Context solver, direct Func marginal mode, new default/flag or extra public API is authorized. Every changed descendant retains its own current 1:1 owner and explicit typed route; finite source inventories are not blanket traversal permission. Exhaustive new decisions need actual future-variant barriers, and the complete lineage/Nucleus, build/test/lint/Rustdoc/fmt/Clippy gates remain required without suppressions.
