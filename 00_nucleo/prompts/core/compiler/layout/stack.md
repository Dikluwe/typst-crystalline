# L0 — stack
Hash do Código: d66bf6ce

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/stack.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/stack.rs:15`, `01_core/src/compiler/layout/stack.rs:25`, `01_core/src/compiler/layout/stack.rs:115`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

FrameItem sites: F009 at `01_core/src/compiler/layout/stack.rs:88` (walk), F010 at `:252` (first_baseline).

AST spans, patterns/guards/bodies and complements were checked before code. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, sources/L0/configuration, hashes, status and git diff HEAD --stat. classification.json records 70 cases/complements across 26 owners using pinned CI-parser tree. Relevant L0 clauses were read with source; this proposes neither new architecture nor inferred upstream intent. Rust patterns/exhaustiveness are mechanics; product semantics/morphology/geometry/effects/diagnostics remain unchanged.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Close preceding text, establish initial baseline and preserve the incoming block boundary/bbox. Empty children retain the current no-emission path. Measure through `00_nucleo/prompts/core/compiler/layout/measurement.md`, lay out each child once in an isolated subframe and recompose according to the source's vertical/horizontal direction branches and resolved inter-child spacing.

Vertical composition uses measured baseline/ascent/descent and prior-child descent; horizontal composition uses actual child width and the shared top/baseline offset. Preserve current parent-block entry adjustment and outgoing last-bottom/pending-below state. Direct HTML markers do not influence child width or ink metrics but are translated and emitted. Existing non-text approximations and direction-branch limitations are not replaced by a new reverse-direction/balancing promise. Restore outer bbox; no independent page dispatcher is created.

In horizontal Stack, transport TextSeparator but exclude it from child terminal width/descent; shift only its position by the same delta as other items.

Treat TextReplacement as a transparent visual envelope in this owner's traversals; descend into children preserving text, order, coordinates and existing calculations, without a new algorithm or advance. The pinned text-replacement nucleus shares invariants only; this prompt owns only its declared consumer.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Publish composed-subframe causal width: maximum for vertical composition, sum with spacing for horizontal. Do not count Place as child width. Preserve existing vertical policy in this correction.

F009 walk: replace only wildcard pattern with nominal TextSeparator/Line/Image/Shape/Link group under existing FrameItem prefix. F010 first_baseline: replace only wildcard with TextSeparator/Line/Image/Shape/Group/Link. Retain full existing fallback bodies, preceding arms, guards, transformations, calls, returns and order.

Preserve callbacks/effects/metadata/clipping/fonts/positions/paint/width under their current owners. This work does not expand traversals, turn wrappers into leaves or substitute text/ActualText for geometry. Test helpers preserve fixtures/expected outputs/tolerances/counts/Unknown failure; cleanup cannot accept missing observation. Nominal alternatives retain the prior shared body and visible informative V19 notes; one arm's execution does not prove every alternative.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

This internal parity correction under ADR-0127 adds no public contract/default/phase. L0 precedes RED→GREEN/restricted resealing. Causal width is only part of the ongoing block-region/width/transport correction: Auto height/backing, late alignment, fractions, fragmentation, outset and PDF Parent retain their own criteria. Testing a carrier alone does not approve render. Preserve full bilateral controls/refuters, suite/build/doc/fmt/lineage/inventories.

Internal continuous-flow ADR-0127 maintenance introduces no enum/field, trait method/public signature, default, dependency, compatibility or phase. Do not materialize historical pending proposals. L0 precedes restricted bilateral resealing. Add no suppression, threshold change or match→if-let rewrite to hide V16.

Before code, run new FrameVisitor control and workspace suite with unchanged expected outputs, and establish structural RED for 70 family warnings through AST reading. Afterwards require zero covered-family V16 and no new warning, retaining external V16 as pending. For productive core matches reached by cargo check -p typst-core, use a disposable copy with a sentinel FrameItem variant to measure absent E0004 barriers before/present after. The probe covers only compiled sites, not cfg(test) helpers or L3 blocked by dependency. Across the whole package replay patterns/guards/bodies, verify unchanged enum/signatures, real build/tests and no stub bodies.

Revalidate full workspace suite, cargo build, Clippy workspace/all-targets -D warnings and -D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes, strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/configuration during commands. Heavy commands run sequentially with MemoryMax 12GiB/MemorySwapMax 0/CARGO_BUILD_JOBS1. Structural cleanup does not establish new global parity, memory gain or measured AI benefit.

## Source-owned physical page child transport

### Measurement and approved scope

At the implementation baseline, `stack.rs:168–180 and :234–246` selects the
actual body/subframe or deferred-body operation. Exact source/L0 texts, full
SHA-256, UTC, HEAD and working-tree state before this amendment are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/child-adapters/before.json`.
The approved review SHA-256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
and human `approval.json` under the implementation evidence ratify this finite
physical-page source-routing phase. Existing geometry is preservation, not a new
claim of upstream intent, global page parity or completed historical proposals.

### Exclusive routing contract

Both direction branches identify each actual child by StackChild(source_index), carry that cursor into the existing finite subframe, then retain current baseline, width, spacing, item/decor translation and composition. The preliminary stack measurement remains unplanned and contributes no semantic event. Never use the child value or final coordinates as its source address.

Use private parent-descendant protocol helpers with explicit typed source edges;
no new public API, entity field/variant, generic mapper, callback execution,
source re-evaluation, result cache or layout-from-Frame semantic reconstruction.
Retain legacy behavior when the planned cursor is absent. Actual source packets,
source parentage and physical placement remain distinct; measurement cannot
publish actions, consume transcripts or leak tentative diagnostics. Preserve
existing current styles, original/visual identity, per-store position sealing and
all stated geometry/effect/error limits. In particular, deferred footnote/float
causality versus later physical placement requires source-bound language controls;
this routing amendment does not declare that universal paint order is semantics.

Require actual nested source Context/Figure/label/counter cases, repeated pages,
no-paint children, wrong-edge/foreign cursor rejection, measurement inertness,
retained/deferred/rollback and existing geometry/baseline/clip/float/note tests.
No old assertion, fixture, diagnostic or explicit pending limitation is weakened.
L0-first source-current RED→GREEN, scoped lineage/graph/seals and full established
gates remain mandatory before completion.
