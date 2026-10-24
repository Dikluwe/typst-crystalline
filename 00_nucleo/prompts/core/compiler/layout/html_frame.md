# L0 — html frame
Hash do Código: 86a89098

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/html_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/html_frame.rs:22`, `01_core/src/compiler/layout/html_frame.rs:25`, `01_core/src/compiler/layout/html_frame.rs:30`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

`lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/before/results.json` measures source→PDF with pinned binaries and no artificial terminal newline; initial.json preserves HEAD/tree/hashes/snapshots. Empty Text at 12/18pt yields Tekt 3×3 versus vanilla 3×8.568/12.852pt. abc\n\n yields Tekt 19.872×31.536 versus vanilla 19.872×8.568; interior abc\n\nxyz matches 19.872×31.536. Literal-spaces then blank line/abc stays one Tekt line versus two vanilla. `text.rs:37-86` emits no empty Text; html_frame.rs:72 excludes every separator; line_commit.rs:280-286/parbreak.rs:43-48/mod.rs:1965 reuse visual materiality for causality; parbreak.rs:40-85 advances after closure and page_geometry.rs:180-201 uses cursor absent confirmed bottom. Pinned inline/linebreak.rs:695-699 allows mandatory empty-text breakpoint. These measure observables without inferred intent or required vanilla mechanics.

Selected Clippy html_frame.rs:167, source/spans/owners/references/binary/UTC/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted tree/status/diffstat are preserved in `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`; internal mechanics/uses, not new bilateral parity or speed.

`box-contexto-2026-10-14/probe.json.gz`preserves sources/binaries/HEAD/working tree/UTC: Box, inset2pt, prefix and nesting publish (0,0) despite displaced paint. The direct-context baseline difference is separate; do not copy vanilla merely because it differs. `mod.rs:1131-1148/1180-1200` writes local points to runtime.positions; Box groups its body in another coordinate system; `finish.rs:295-302` seals the map without recomposing those points. Realized writes through or_insert; labels have a parallel map. HTML already carries private markers through the frame at `html_frame.rs:30-199`. Preserve single-pass registration and per-sub-store sealing under ADR-0073/0074 position/hybrid-model sections; do not adopt vanilla global introspection. Inspection of coordinates and its owners found absolute uses in Grid and explicitly local uses in Place/Box; nucleus qualification preserves both and requires page transformation without doubled origin. Earlier snapshot: `posicoes-compostas-2026-10-14/before.json.gz`. Geometry/identity are observables; carrier/index are mechanics.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

`place-origem-local-2026-10-14/html-failures.log` reproduces four existing historical P1493 failures: aqui() publishes y11.2 rather than 20 inside Place(Box HTML), an ascender8.8 difference with FixedMetrics. `html_frame.rs:219` composes at the legacy baseline; `context_block.rs:16–22` subtracts the active ascender even when Place/Box use geometric origin 0. Pipeline tests explicitly preserve x30/y20, no page and same-target fixpoint. Earlier state: `html-before.json.gz`; `workspace-interrupted` preserves the first occurrence and full state. Capture/drawing must share geometry under coordinates; this does not justify a paginated HTML pipeline or changing ADR-0128.

The pre-revision refuter `html-green-attempt1.log` keeps two transform/link tests RED: translation 5 becomes-3.8. Transform recomposes the legacy frame by removing origin 8.8; normalizing the carrier beforehand removes it twice. The HTML island already enters at geometric origin.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [184, 193]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

FrameItem F007 at html_frame.rs:163, collect_context_positions.

AST spans, patterns/guards/bodies and complements were checked before code. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, sources/L0/configuration, hashes, status and git diff HEAD --stat. classification.json records 70 cases/complements across 26 owners using pinned CI-parser tree. Relevant L0 clauses were read with source; this proposes neither new architecture nor inferred upstream intent. Rust patterns/exhaustiveness are mechanics; product semantics/morphology/geometry/effects/diagnostics remain unchanged.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

In ordinary paginated dispatch, delegate once to the body without a visual wrapper or default page margin. Intrinsic entrypoints create an isolated Layouter with supplied chain/metrics/sizer, unbounded region and transparent Page cropped to supported visual dimensions, logical_number one, without joining the global paginated sequence. Nesting remains transparent inside the same island.

The detailed variant captures ContextBlock ids using private zero-size carriers in the existing FrameItem tree, not a public variant or HtmlElem sentinel. Transport carriers through box/place/grid/link/group/transform geometry; after all composition, extract deterministic id/HtmlPosition pairs in the island's causal reference frame before private vertical visual cropping. Then normalize only the visual Page, remove all carriers, and derive final visual dimensions without them. Missing id/Location correspondence does not fabricate a position.

Own the shared marker identity/material predicates: only a direct private marker receives private-marker neutrality. Existing Semantic, Link and Group wrappers retain ordinary legacy material classification even if they contain only a carrier; do not flatten them. Extent helpers ignore a direct marker but transformations still reach its point. No SVG encoder, World, PageStore or logical PDF-label behavior is introduced. The hub owns public reexports, this unit owns intrinsic implementation.

TextSeparator is neutral to items_have_material/visible_top, never a removable context marker. Keep items_have_material as visual-paint/material predicate. A distinct private causal-line predicate accepts visual material or Literal TextSeparator; initial Markup/Newline/context markers do not create a line. The historical P1493 visible-Page refuter compares projection without separator leaves while preserving dimensions/matrix/pivot/marker position, separately checking textual-carrier retention; Vec structure is not visible geometry.

Preserve TextReplacement while removing context markers, recursively processing children; materiality/position consult visual children. The shared nucleus does not transfer ownership.

Remove only private strip_context_marker metrics parameter/M generic: it uses no metrics, merely passed them recursively. filter_map calls same helper directly. Preserve TextReplacement/Semantic/Link/Group wrappers, fields/order/ordinary children; position collection stays before removal and entrypoints retain metric acquisition/layout. Do not convert empty wrappers into Empty or remove TextSeparator.

Extend shared direct-carrier predicate to recognize strict paged position carriers plus private link boundaries as neutral direct markers, alongside HTML markers. This does not flatten ordinary wrappers or make all carrier-containing wrappers empty. Keep HTML id/HtmlPosition protocol and extraction before visual crop. Resolve links before metadata removal/crop/publication, and remove both HTML/paged position carriers before intrinsic visual output even without detailed capture.

Compose intrinsic island through geometric-origin helper. Text installs its own baseline; capture stays before visual crop and carrier removal after projection. No target/feature/fixpoint/PageStore/signature/position-policy change.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. Preserve paint_root unchanged on Group reconstruction, including projection/marker removal/rebase; never true→false.

F007 collect_context_positions replaces only wildcard with nominal FrameItem TextSeparator/Text/TextShaped/Line/Glyph/Image/Shape. Retain entire fallback body, previous arms/guards/transformations/calls/returns/order.

Preserve callbacks/effects/metadata/clipping/fonts/positions/paint/width under their current owners. This work does not expand traversals, turn wrappers into leaves or substitute text/ActualText for geometry. Test helpers preserve fixtures/expected outputs/tolerances/counts/Unknown failure; cleanup cannot accept missing observation. Nominal alternatives retain the prior shared body and visible informative V19 notes; one arm's execution does not prove every alternative.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

Causal-empty-line correction needs RED→GREEN/bilateral matrix/prior regressions/error-free build/lint and manifest-only resealing. Internal equivalent maintenance uses ADR-0127 continuous flow, unchanged expectations and no loop-mirror tests or new suppression; validate core conflicts/spans/fixups/markers/differential selection/all targets/build/docs/Clippy/lineage.

Position/origin correction requires RED→GREEN coordinates/no fictitious paint-height/identity/order/visual clones/rollback and full HTML/geometry/context-query suite. Existing p1493 expectations, including transform/link cases, remain unchanged; revalidate PDF/SVG/lineage without new public contract/phase.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.

Internal continuous-flow ADR-0127 maintenance introduces no enum/field, trait method/public signature, default, dependency, compatibility or phase. Do not materialize historical pending proposals. L0 precedes restricted bilateral resealing. Add no suppression, threshold change or match→if-let rewrite to hide V16.

Before code, run new FrameVisitor control and workspace suite with unchanged expected outputs, and establish structural RED for 70 family warnings through AST reading. Afterwards require zero covered-family V16 and no new warning, retaining external V16 as pending. For productive core matches reached by cargo check -p typst-core, use a disposable copy with a sentinel FrameItem variant to measure absent E0004 barriers before/present after. The probe covers only compiled sites, not cfg(test) helpers or L3 blocked by dependency. Across the whole package replay patterns/guards/bodies, verify unchanged enum/signatures, real build/tests and no stub bodies.

Revalidate full workspace suite, cargo build, Clippy workspace/all-targets -D warnings and -D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/configuration during commands. Heavy commands run sequentially with MemoryMax 12GiB/MemorySwapMax 0/CARGO_BUILD_JOBS1. Structural cleanup does not establish new global parity, memory gain or measured AI benefit.


## Planned source-child transport

The approved physical-page protocol is recorded in `lab/results/set-page-header-2026-10-21/review.md`; baseline source/owner evidence and actual semantic RED are preserved in `lab/results/set-page-header-implementation-2026-10-21/`. This consumer already dispatches the source child named below; ordinary layout_content intentionally isolates generated content and therefore cannot transport its source packet.

In a planned pass, descend by HtmlFrameBody. Preserve one actual source occurrence, its source authority and inherited deferred anchor. Known generated render output remains unplanned and cannot acquire source membership. Measurements and repeated visual copies do not execute or admit the source again. Preserve all existing geometry, wrappers, diagnostics, cache behavior, zero-render cases and unplanned entrypoints. No new public API/default or phase is authorized here.
