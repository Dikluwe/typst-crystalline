# L0 — compiler/layout/finish
Hash do Código: 5287e062

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/page-number-baseline.toml sha256:0c492a09d2660de765df65c22e703cc8c3e63cc663ee4d9ac2dd612afc80dfa3
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
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


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Own terminal assembly with the same planned physical channel as intermediate commits. Preserve final link-boundary resolution and per-store position projection/sealing; transport those sealed geometry facts into the opaque trace without rebuilding a semantic index from painted frames. A terminal candidate with missing physical slots, packet coverage or consumed inputs cannot be returned as Complete by coordination. This approved qualification adds the explicit protocol result alongside the existing document through the new facade only; old public finish/document/store contracts remain unchanged.

### Acceptance and preserved boundaries

Require semantic RED→GREEN for repeated physical header/footer/four-slot order, zero-paint updates, source-derived parents, overwritten inert templates, aliases and physical pages, effective20/30pt ambient plus explicit inner styles, nested PageRun restoration, Context reads, and exact duplicate-label/source diagnostics. Cover speculative suffix rollback, deferred floats/notes, subframes, measurement isolation, complete body/slot coverage, stale/mixed authority and math/par.line coexistence. Do not weaken existing tests, callback budgets, per-store sealing, source morphology, HTML isolation or old public compatibility. No generic painted-frame semantic reconstruction, source re-evaluation, second arena/Context solver, direct Func marginal mode, new default/flag or extra public API is authorized. Every changed descendant retains its own current 1:1 owner and explicit typed route; finite source inventories are not blanket traversal permission. Exhaustive new decisions need actual future-variant barriers, and the complete lineage/Nucleus, build/test/lint/Rustdoc/fmt/Clippy gates remain required without suppressions.


## Opaque physical trace storage

The source-bound Clippy RED in
`lab/results/set-page-deferred-corrections-2026-10-21/candidate-2/validation-remainder/clippy.log:83–113`
identifies oversized private pass variants and the public result's opaque trace
storage. Exact before texts, SHA-256, UTC and working-tree state are in
`lab/results/set-page-report-blockers-2026-10-21/agents/layout/trace-callers-before.json.gz`
and `agents/layout/before.json.gz` under that evidence root.

The trace owner retains every existing source, admission, request, sealed geometry
and completeness value behind one compiler-private owning payload Box. Access
those same fields through `trace.data`; preserve exact mutation/read order,
cloning, authority, geometry, verification and acceptance. This is private storage
indirection only: the public opaque PageRunningTrace and all public fields and
function signatures stay unchanged. No new semantic fact, source traversal, phase,
callback, diagnostic suppression or relaxed gate is authorized.


## Weak pagebreak parity correction — 2026-10-23

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/baseline.json` binds the uncommitted source/L0/config state to HEAD, dirty stat and UTC; `before/receipt.json` binds both verified executables, inputs, commands and observations. `pagebreak.rs:21-58` ignores weak and always requests transitions; `finish.rs:103-106` requires the newly opened page unless it is a PageRun boundary. `page_run.rs:126-131` unconditionally closes its body. The pinned source `lab/typst-original/crates/typst-library/src/layout/page.rs:554-557` explicitly defines skipping weak breaks on empty pages; its tests `tests/suite/layout/pagebreak.typ:15-24,61-76` document weak/material distinctions. Initial, repeated, terminal and strong/weak controls plus parity and explicit PageRun probes distinguish language intent from mechanics. Page membership, content and geometry are observables; the private successor flag is a chosen mechanism. The inference that an optional successor suffices is refuted by lost notes/placed content, changed strong/parity empties, or leaked rollback state.

### Decision and exclusive responsibility

After final line, float and note admission, omit only an optional weak successor that still has no material. Preserve at least one page for an otherwise empty document, strong-break empties and committed parity pages. Never delete a completed page by inspecting its appearance. Running decorations alone do not require an otherwise optional successor.

This is an ADR-0127 continuous-flow language-parity correction: L0 first, reseal, RED→GREEN and complete revalidation. No public contract, default mode, pipeline phase or compatibility change is authorized. Existing nucleus obligations and unrelated limits remain binding.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Commit admitted unpainted page material even when no frame items paint. The optional successor rule must not erase hidden text, explicit zero/absolute/fractional horizontal space or explicit vertical space.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.


## Terminal nonmaterial source positions

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/positions-final/receipt.json` binds actual CLI query observations to the unpainted candidate (build/source snapshot retained in `unpainted-candidate.json` and `unpainted-candidate-source.json.gz`). `terminal-metadata-query` emits TAILPAGE 2 on Tekt's only page versus TAILPAGE 1 in vanilla; a following real page correctly retains page2 in both. `finish.rs:297-310` projects only committed pages before sealing; `position_marker.rs:182-200` never sees pending carriers from an omitted successor, leaving provisional runtime page2. Pinned `lab/typst-original/crates/typst-layout/src/pages/collect.rs:11-14` explicitly describes terminal tags and `pages/mod.rs:236-243` places them at (0,last frame height). Page/position queries are language observations; carrier storage is mechanics. This refutes closing the correction by page counts alone.

### Exclusive correction

Preserve pending private position captures when omitting an optional weak/PageRun terminal successor. The existing position-marker owner projects those actual captures to the final effective page's end before sub-store sealing. Do not silently retain the nonexistent provisional page or allocate a page solely for a marker.

Require RED→GREEN for terminal Location/Label positions and actual source-derived metadata query, repeated weak breaks with following material, parity/strong/PageRun controls, marker-free output, latest writes, transformed positions, rollback and complete revalidation. This is continuous-flow ADR-0127; no default, public contract or phase change.


## R8 terminal carrier clearance

Evidence before decision: source-bound candidate3/receipt.json under
lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24 passes nine
original controls but puts split START at59.245421pt versus44.857422pt in the
fixed oracle. END now has correct page/geometry. The14.388pt displacement equals
the old following-baseline clearance reintroduced by finish.rs:71–78, after the
real carrier baseline was restored. Notes now own explicit separator1em/gap0.5em
clearance, so that old conservative baseline adds a second unrelated leading.
Use the restored causal carrier baseline/bottom for terminal note fitting;
remove only the obsolete following-baseline bump and its now-unused temporary.
Keep float draining, page requiredness, columns, terminal positions, source
sealing and remaining drain guards unchanged. Private parity formula correction
under ADR-0127; RED→GREEN actual captured-frame clearance plus complete paired
PDF/SVG output at0.001pt and all workspace/lineage gates are required.


## R9 — long notes with explicit line breaks (2026-10-24)

Measurement precedes this correction: long-footnotes-r9-2026-10-24/baseline.json and before/receipt.json under lab/results/construction/diagnostics/ bind HEAD, exact dirty stat, UTC,2162 inputs and current R8/pinned oracle binaries. h80-n5 places1/2/2 body lines versus0/2/3; both-zero instead requires1/3/1; orphan-zero requires1/2/2. h80-n6 through n9 lose late words because finish.rs:75 bounds drain by queued note count. following-flow incorrectly reserves a non-admitted first fragment and sends TAIL to another page. Original bytes and failed controls remain immutable.
Pinned flow/collect.rs:196–239 resolves strictly positive widow/orphan costs, protects first two/last two and whole-three, ignoring empty neighboring lines. flow/compose.rs:650–730 budgets the first entry against current flow+separator+gap, while757–777 inserts already laid out future-region spill. These source structures are not mandated; physical words/pages, source effects and geometry are language observables.
Classification: private vanilla-parity/correctness correction, ADR-0127 continuous flow. No public field, trait, signature, mode/default, eval/layout phase or compatibility break. Preserve prior gates, callbacks once, static exhaustive dispatch, source anchors and pure injected metrics; no arbitrary painted-frame semantic inference or hard-coded font/page fixtures.

### Exclusive owning obligation

Replace the fixed note-count iteration cap with actual pending-work progress: drain until no bodies remain. Compare the feature's private rank before/after each genuine region transition/flush. Allow a no-progress transition for initial deferral; on repeated non-progress request the feature's existing-warning force path. Actual splits, completed entries and initial-to-continuation phase transitions decrease work; no tail can disappear because a note spans more pages than note count. Preserve terminal causal cursor restoration, floats, weak/required pages, margins/columns, positions, trace sealing and source effects exactly. No unbounded silent spin or truncation, arbitrary page limit or semantic arena.

Acceptance: source-backed module RED→GREEN before implementation, independent positive/zero/negative cost refuters, complete long-note words/markers, no-progress warning escape, actual PDF/SVG physical geometry at unchanged0.001pt, R8 and A–H/R1–R7 preservation, resource recipe, complete workspace/Clippy/fmt/Rustdoc/lint and both lineage gates. Preserve failed attempts and all received unrelated changes.


## R9 — source page-column admission and drain

Evidence before decision: candidate2 and candidate3 receipts under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 preserve exact inputs, dirty stat, UTC and binaries. The first passes40/41; the wider68-case matrix refutes retaining context only at segmented exit: page-columns follows columns/flow.rs:119–126, which closes active columns before terminal draining. cols2-h80-n2 also measures Alpha atx20 versus155.2pt; cols2-h100-n3 unnecessarily defers a fitting whole-three entry. Pinned flow/compose.rs:650–777 measures admission at the source carrier and retains future column regions. These physical positions/pages are language, private state/dispatch is mechanics. This supersedes the refuted column follow-up saved in l0-column-followup.json, whose source changes were withdrawn without changing received code.

Classification: private parity/correctness, ADR-0127 continuous flow; no public field/signature/default/phase/compatibility change.

Exclusive owning obligation: Delegate the terminal pending-work loop to the feature shared private drain with no finite-certificate restriction. Retain the causal cursor restoration and final page sealing. The refuted queued column-context proposal is withdrawn; certified source page-column spill is drained by its still-active flow owner.

Require actual carrier-column RED→GREEN before implementation, the wider68-case PDF/SVG contract at unchanged0.001pt, original R8 and A–H/R1–R7 controls, unchanged effects, resources, full workspace validation, both lineage gates and preservation audits.


## Readiness: automatic number baseline and per-page style (2026-10-28)

### Measurement before decision

readiness-fixes-2026-10-28/{article-fields-before,article-fields-extra-before,
page-number-total-before}/receipt.json preserve exact source/font/compiler/oracle
hashes, current source-bound wj2 build/archive, HEAD212aed0449150dd296ceea13fc2be665cc5db01a,
dirty stat, UTC, commands and frozen productive/L0/fixture inputs. Oracle commit
64c5e8c02fe0394d913ead36e42ef8c1334d2107, tree
483bd6f1d8ac8810db47b65336d402647955fd17, executable
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The default bottom number is7.238pt too high at11pt; at17pt its first-page
offset is11.186pt. Bold/DejaVu produce different measured top edges, refuting
a universal constant. The top unary/compound control already matches. A final
page loses the17pt/bold/DejaVu style, and compound patterns borrow11pt even for
the first17pt page. Explicit CUSTOM footer is a DIFFERENT measured negative;
it is not fixed or waived here. Body text and None-number controls are retained.

Source page_commit.rs:62-111 and finish.rs:123-174 emit Text directly at the
number-position anchor. finish.rs:247-275 later builds compound text using the
then-current chain. The retained page-effective chain already exists and is
used for running content; this correction consumes it rather than introducing
another source/style authority. Positions, styles, words and pages are language
observables; tuple/record representation and queue mechanics are not parity.

### Exclusive correction and acceptance

Apply the same feature-owned effective page style and baseline conversion to the terminal unary number. Queue a compound terminal number through the same record factory. When total is known, consume each record's original owned style, geometry and logical number; do not read the final chain for an earlier page. Preserve formatter calls/counts/order, page membership/indices, link/position sealing, note draining, eligibility and body/paint composition. No rich/function/explicit-marginal path or source/template solver is changed.

ADR-0127 continuous internal parity correction. L0/pins first, actual RED before
code, V15/V26 before resealing, exact source words/pages/0.001pt and independent
PDF/SVG paint. Preserve all previous running/source/callback/rollback/notes,
Unicode, layout, technical/lineage and historical/JPEG resource gates without
new tolerances, ignored tests or suppression. No overall readiness claim.


## Automatic page width and divider — issue 8525 (2026-10-31)

### Measurement before decision

The independent agent-layout and agent-svg-math reports/receipts under
`lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/` reproduce
infinite SVG viewport/divider geometry and PDF Letter fallback/lost second text.
`01_core/src/compiler/layout/divider.rs:35` multiplies the unbounded-region
mechanism into visible geometry; `page_geometry.rs:62-69` then includes that
shape in intrinsic page extent. `finish.rs:44-48` and intermediate page assembly
consume the contaminated result. Exact received HEAD, dirty stat, UTC and
source/owner bytes are frozen in
`lab/results/construction/diagnostics/auto-page-width-8525-2026-10-31/before.json.gz`.
This is observed geometry/content semantics; it does not establish upstream
intent. The pinned vanilla zero-width divider is a separate defect and is not
an acceptance requirement. A later wider sibling, stale index, lost content,
changed fixed-page line or queue leakage on replay refutes the repair.

### Exclusive correction

After final body/float/note admission and before taking final page items, delegate pending divider resolution to the divider owner using the finite page width. Preserve existing equation/alignment/running layer ordering and all body content. No exporter normalization.

Internal correctness correction under ADR-0127 continuous flow: L0 first,
restricted reseal, functional RED then GREEN, build and unfiltered lineage.
Keep public contracts, modes/defaults, compatibility and pipeline phases.
Acceptance requires finite content-derived page/SVG/PDF geometry, a nonzero
divider when body width is positive, and complete text before/after the rule.
Do not copy vanilla zero width or force a paper size. This does not certify
other relative-size shapes, asymmetric margins or general auto-page parity.
