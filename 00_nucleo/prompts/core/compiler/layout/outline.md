# L0 — outline
Hash do Código: 4e2e0a59

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/outline.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/outline.rs:23`, `01_core/src/compiler/layout/outline.rs:78`, `01_core/src/compiler/layout/outline.rs:192`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record sources, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` likewise saves indices before the body. Clamping the index proves neither preservation of prefix/order/fragments/destination nor geometry. The pre-repair snapshot is `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

For Figures and Tables targets retain the existing LoF/LoT paths, optional title or English default, numbered caption text and page suffix from the corresponding known-page vector by document-order index. For Headings, dispatch an available realized body rather than rebuilding entries; close a material exterior line before it and the realized terminal line after it, rearming the block boundary. The current consumer also retains a legacy fallback when no realized body exists: localized title, depth/active-indent filtering, stored heading number without supplement, cloned rich body and known label page, under the existing read-only flag. Do not remove this fallback under the guise of documentation. The private destination-link helper dispatches its body, requests fractional/repeat expansion through `00_nucleo/prompts/core/compiler/layout/line_commit.md` before closing the private capture scope; terminal geometry computes fragment bounds through `00_nucleo/prompts/core/compiler/layout/link.md`, preserving all copies and their internal destination. It does not implement repeat allocation itself. The realized-outline fail-closed proposal and the surviving fallback are a contract ambiguity requiring separate resolution.

The internal-link helper uses the shared private scope. Preserve required fraction/repeat expansion before closing that scope, and every destination, copy and outline rule. Do not move committed emission into the pending line. This internal correction under ADR-0127 introduces no API, default or phase.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN for breaks before/within the body, preserved prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content, context/rollback and no private boundaries in output. Revalidate no-break controls, the suite, build and lineage. The realized-outline fail-closed proposal versus the surviving fallback remains explicitly unresolved.


## Planned source-child transport

The approved physical-page protocol is recorded in `lab/results/set-page-header-2026-10-21/review.md`; baseline source/owner evidence and actual semantic RED are preserved in `lab/results/set-page-header-implementation-2026-10-21/`. This consumer already dispatches the source child named below; ordinary layout_content intentionally isolates generated content and therefore cannot transport its source packet.

In a planned pass, descend by OutlineRealizedBody for the existing realized body only. Preserve one actual source occurrence, its source authority and inherited deferred anchor. Known generated render output remains unplanned and cannot acquire source membership. Measurements and repeated visual copies do not execute or admit the source again. Preserve all existing geometry, wrappers, diagnostics, cache behavior, zero-render cases and unplanned entrypoints. No new public API/default or phase is authorized here.


The destination-link helper's sole productive caller is the Styled outline-destination branch. It obtains that actual StyledBody cursor and supplies it explicitly to the Link capture owner without changing its existing private signature, fraction expansion or destination. Unplanned calls preserve the same body dispatch. No source route is inferred from the link target.
