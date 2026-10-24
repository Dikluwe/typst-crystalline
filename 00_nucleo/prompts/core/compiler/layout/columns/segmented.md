# L0 — compiler/layout/columns/segmented
Hash do Código: 450c961e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

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


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Continuous planned flow uses the original ColumnsBody source child, while unplanned legacy callers retain their current constructed segment. Explicit colbreak segmentation records original child indices in the same construction pass that clones/pads/merges segments. Generated padding has no source packet. Preserve the original Source/at-most-one-Styled/Sequence spine and emit its source Root/PostChildren phases once around the sibling-region composition. Each actual segment child routes through its original source packet address, not through the index of a generated Sequence. Generated style wrappers keep existing geometry and do not duplicate source facts. The sequence owner supplies an optional exact per-part cursor route without changing grouping, fraction, direction, spacing or strategy.

Retain parallel footnote cursor queues together with the existing body queues at sibling-region entry/exit. Existing packet geometry carriers carry deferred child events to their accepted physical page in the same final projection pass. Do not infer source identity from copies, values, hashes, pointers or Locations, reconstruct global semantics from paint, activate new callback phases or alter any existing column/footnote geometry policy.


Content::sequence may unwrap a singleton source part. Preserve the constructor-selected outer-style bit and normalize that one retained cursor through exactly the Source, optional single Styled, Source and Sequence spines consumed by the existing segment helper. Generated outer Styled already occupies that optional style position, so an actual inner Styled remains an emitted child. Admit removed source carrier phases once around the segment; never rediscover children by equality, pointer or transformed local index.


## Private deferred Note frame transport

### Measurement before correction

`lab/results/set-page-deferred-corrections-2026-10-21/red-2/tests.log` records
the actual missing Note-subtree admissions. The later root-controlled fitting
measurement and pinned oracle page-text evidence distinguish a complete Note frame
from its accepted prefix and continuing suffix. The oracle keeps NOTE TEXT/START
in the first page and emits the later Figure in the second; whole-body deferral
or moving the complete call does not establish that language behavior. Exact
before bytes/hashes/HEAD/dirty state for this owner are archived in
`lab/results/set-page-deferred-corrections-2026-10-21/agents/index/note-transport-before.json.gz`.
The root authorizes an internal layout parity correction, with no eval/layout
phase change, public contract, new identity source or attempt budget.

### Exclusive transport obligation

At sibling-region entry/exit move the optional captured Note frame queue
alongside its numbered-body/source-cursor queues. Suspend exterior reserved
height and optional capture while local sibling columns run; restore both after
composition. Column flow must not consume an exterior Note's reservation/cut
collector, and an admitted frame must never become associated with another
numbered Note by index drift. Geometry/certification/remainder policy belongs to
footnote_fragments/footnote_flush; this owner only transports their actual state.

Validate actual split Note text/page/counter/query/identity and no-fit progress,
plus existing no-break notes, columns, subframes, rollback, floats and running
content controls. Preserve strict trace coverage/foreign authority, warnings,
canonical owner paths and forward/reverse lineage; no suppression or fixture
weakening. These transport declarations alone do not certify general pagination.

For each page-column sibling, reset its predecessor's reserve after the new
column geometry is installed and request feature-owned refresh from the retained
actual frame remainder. Container-column accumulation keeps its existing policy;
this coordinator contains no cut or fitting formula.

## Keep productive items before the test module

Measurement before correction: the exact source/L0 bytes, SHA-256, UTC, HEAD
and dirty-file list are retained in
`lab/results/set-page-report-blockers-2026-10-21/agents/domain/clippy-followup/before.json.gz`.
The retained source places the existing smoke test module at lines 415–419,
before the productive `normalize_singleton_route` helper starting at line 422.
Root-controlled all-target Clippy reports `items_after_test_module` at that
module. This is source organization mechanics, not a language behavior change.

Place the existing test module after all productive items, preserving its exact
contents and the exact helper implementation. No suppression, new test, public
contract, phase, traversal, source-route or geometry change is authorized. This
internal correction follows ADR-0127 continuous flow. Revalidate all-target
warnings-denied Clippy, existing segmented-column controls and lineage.


## R8 — measured column note geometry

Evidence before decision: `lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/before/receipt.json`
measures Beta atx156 instead of155.2 and container notes immediately under the
columns rather than in the page note area. Pinned library/layout/columns.rs:73
uses4% of the available region; current columns.rs:170 applies4% to full page
width. At300pt with20pt side margins the correct gutter is10.4pt, not12pt.
These are physical language coordinates, not traversal mechanics. ADR-0127
continuous parity correction applies, with unchanged public signatures and modes.
For explicit container columns, transport accumulated original note bodies,
planned cursors and normalized captured frames out to the enclosing page queues
in causal order; restore exterior queues first and append the local queues. Do
not flush them into a shallow artificial region below container content. Preserve
page-column flush/translation and source routing. Captured container note frames
already use the full outer available width. Refresh the feature-owned reservation
after restoring the outer geometry; preserve nested-column/exterior notes.

Tests first: default and explicit gutter, container/page columns, note text/position and exterior queue preservation; bilateral0.001pt and full validation.
