# L0 — page running
Hash do Código: 3fb4df03

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/page-number-baseline.toml sha256:0c492a09d2660de765df65c22e703cc8c3e63cc663ee4d9ac2dd612afc80dfa3
- 00_nucleo/prompts/_nuclei/layout/page-running.toml sha256:9f5dc1dba49ac843612415dfe6018b40308686a4fef442d153bdbd06834466d2
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_running.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/page_running.rs:15`, `01_core/src/compiler/layout/page_running.rs:19`, `01_core/src/compiler/layout/page_running.rs:40`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [67, 146]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own marginal eligibility, positions, offsets and isolated layout of explicit/realized running Content. Automatic numbering is enabled only for PageMarginal::Auto; None and explicit Content suppress automatic text in that marginal. Resolve offsets as relative margin plus absolute/em length and preserve the current typed top/bottom and left/center/right placement.

Lay out explicit header/footer Content in the corresponding finite marginal rectangle, recompose at the left margin and its proper y, and retain rich frame children. For a realized numbering Content, empty means no layer; the current helper estimates width from plain_text using active metrics, then lays out the rich Content in an isolated subframe and places the resulting group at the calculated position. Do not claim rich intrinsic-width parity or replace the width projection here. Shared defaults/types belong to the existing entity/nucleus owners; no callback is executed and no main-flow cursor policy is added.

Suspend main-flow lexical link inheritance while composing explicit marginals and realized numbering; restore after subframe. Links belonging to the marginal remain active within it. This internal correction adds no API/default/phase.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN for breaks before/within body, preserving prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Own finite Header/Footer slot geometry and isolated composition for the planned channel. Resolve the selected physical slot through the retained source/template binding; ask the pure protocol for matching prepared content at the exact physical key, page-effective environment and finite region. Do not lay out a raw operational template as an implicit empty success. Missing/mismatched resolution contributes a request and prevents Complete. Actual marginal descendant traversal uses its own prepared source plan; save/restore caller plan/current-location/style/link scopes. Keep the existing offset, rich-frame, number-position and legacy unplanned helpers. Automatic numbering is distinct from explicit running Content. Language query/counter membership is not erased by accessibility decoration.

### Acceptance and preserved boundaries

Require semantic RED→GREEN for repeated physical header/footer/four-slot order, zero-paint updates, source-derived parents, overwritten inert templates, aliases and physical pages, effective20/30pt ambient plus explicit inner styles, nested PageRun restoration, Context reads, and exact duplicate-label/source diagnostics. Cover speculative suffix rollback, deferred floats/notes, subframes, measurement isolation, complete body/slot coverage, stale/mixed authority and math/par.line coexistence. Do not weaken existing tests, callback budgets, per-store sealing, source morphology, HTML isolation or old public compatibility. No generic painted-frame semantic reconstruction, source re-evaluation, second arena/Context solver, direct Func marginal mode, new default/flag or extra public API is authorized. Every changed descendant retains its own current 1:1 owner and explicit typed route; finite source inventories are not blanket traversal permission. Exhaustive new decisions need actual future-variant barriers, and the complete lineage/Nucleus, build/test/lint/Rustdoc/fmt/Clippy gates remain required without suppressions.


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

Own a compiler-private pending-number record and capture factory containing the existing page index/logical number/pattern/align/margins/resolved offsets plus the OWNED effective TextStyle. Read style from the existing retained page-running chain when planned, otherwise the existing current chain; never create a new PageConfig field, solver or source snapshot. Resolve em offsets using that same captured size. Own the pure baseline conversion: Bottom adds injected metrics.text_edges(style.size,style).top to the existing anchor; Top preserves the existing anchor. Keep number_position and all public signatures intact. Existing rich/function-numbering composition and explicit Header/Footer paths retain their route and remain separately unqualified.

ADR-0127 continuous internal parity correction. L0/pins first, actual RED before
code, V15/V26 before resealing, exact source words/pages/0.001pt and independent
PDF/SVG paint. Preserve all previous running/source/callback/rollback/notes,
Unicode, layout, technical/lineage and historical/JPEG resource gates without
new tolerances, ignored tests or suppression. No overall readiness claim.
