# L0 — context block
Hash do Código: faae0241

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/context_block.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/context_block.rs:11`, `01_core/src/compiler/layout/context_block.rs:42`, `01_core/src/compiler/layout/context_block.rs:47`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`box-contexto-2026-10-14/probe.json.gz` preserves sources/binaries/HEAD/working tree/UTC: Box, inset2pt, prefix and nesting publish (0,0) despite displaced paint. The direct-context baseline difference is separate; do not copy vanilla merely because it differs. `mod.rs:1131-1148/1180-1200` writes local points to runtime.positions; Box groups its body in another coordinate system; `finish.rs:295-302` seals the map without recomposing those points. Realized writes through or_insert; labels have a parallel map. HTML already carries private markers through the frame at `html_frame.rs:30-199`. Preserve single-pass registration and per-sub-store sealing under ADR-0073/0074 position/hybrid-model sections; do not adopt vanilla global introspection. Inspection of coordinates and its owners found absolute uses in Grid and explicitly local uses in Place/Box; nucleus qualification preserves both and requires page transformation without doubled origin. Earlier snapshot: `posicoes-compostas-2026-10-14/before.json.gz`. Geometry/identity are observables; carrier/index are mechanics.

`place-origem-local-2026-10-14/html-failures.log` reproduces four existing historical P1493 failures: aqui() publishes y11.2 rather than 20 inside Place(Box HTML), an ascender8.8 difference with FixedMetrics. `html_frame.rs:219` composes at the legacy baseline; `context_block.rs:16–22` subtracts the active ascender even when Place/Box use geometric origin 0. Pipeline tests explicitly preserve x30/y20, no page and same-target fixpoint. Earlier state: `html-before.json.gz`; `workspace-interrupted` preserves the first occurrence and full state. Capture/drawing must share geometry under coordinates; this does not justify a paginated HTML pipeline or changing ADR-0128.

The pre-revision refuter `html-green-attempt1.log` keeps two transform/link tests RED: translation 5 becomes-3.8. Transform recomposes the legacy frame by removing origin 8.8; normalizing the carrier beforehand removes it twice. The HTML island already enters at geometric origin.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Ordinary paginated layout is a no-op and never executes the closure. When detailed HTML context capture is enabled, append the private marker supplied by `00_nucleo/prompts/core/compiler/layout/html_frame.md`, identified by the element id, at `(cursor_x, cursor_y - ascender)` using the active injected metrics. Do not advance the cursor or extract positions here.

Paginated operation remains a no-op; only the HTML marker is opt-in, without closure execution. Tests distinguish the HTML carrier from the paged registration emitted by the hub: neutrality, cursor and nonexecution remain required, while total Vec length is not a language contract.

Within a subframe, register the physical cursor without subtracting ascender: the caller already has the recomposition translation, including removal of legacy origin. Outside a subframe, preserve the legacy active-metrics formula. A subframe marker carries the physical cursor coordinate like other items, never a prenormalized coordinate. This supersedes unconditional ascender subtraction only when the body uses geometric origin. Do not execute closures, advance cursors or change paged capture.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

This is an internal correction under ADR-0127 preserving the existing HTML contract: no new public field/API, flag/default or phase. L0 precedes RED→GREEN for coordinates, no fictitious paint/height, identity, order, visual clones and rollback. Revalidate HTML, geometry, context/queries and complete suite. Keep all historical P1493 and the two transform/link expectations unchanged. Validate all p1493 tests, context/HTML, full suite, PDF/SVG controls and lineage gates.
