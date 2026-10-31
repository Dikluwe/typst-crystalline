# L0 — infra/pipeline/tests
Hash do Código: 7789b388

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compilation-metric-cache.toml sha256:389926b89934d625412cbbb22bad324ea6579533d79aa6b2dcf92fa5877095e4
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425




**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/tests.rs`

## Current evidence

- `03_infra/src/pipeline/tests.rs:18` — `p1247_contexto_svg_filtra_pagina_e_posicao_homologa`.
- `03_infra/src/pipeline/tests.rs:41` — `MockWorld`.
- `03_infra/src/pipeline/tests.rs:47` — `library`.
- `03_infra/src/pipeline/tests.rs:50` — `book`.
- `03_infra/src/pipeline/tests.rs:53` — `main`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/tests.rs`.

## Owned contract

- Own only the integrated pipeline suite and fixtures, using the existing public facades and descendant helpers. Tests do not authorize production behavior or a new pipeline phase.
- Preserve integrated format precedence, feature separation, contextual snapshots, callbacks, bundle, resources and diagnostics oracles. Use structure, text, DOM, dimensions and positions rather than complete serialized byte hashes.
- Characterize fractional boxes on finite lines: suffix position for 1fr, proportional positive shares, zero share, isolated negative share and safe zero-sum/nonfinite ratios. Preserve repeat ceiling 1000, link scope and strictly-more-copies-at-greater-width relation.
- Keep repeat controls with/without prefix at equal width, distinct sibling 1fr repeats/suffixes, a later block body and nested boxes. Observe paged output, not private repeat/fr indices; do not adapt expectations to the candidate.

Verify that resource collection and resolution for a paged document containing only shapes succeed with a World whose catalog and font accessors reject any access. Construct layout before applying the rejecting World: shape layout can legitimately require baseline and cap-height metrics. This checks export resource demand independently of installed host fonts or wall-clock timing.

Verify with the real Ubuntu Sans variable fixture that unknown-axis-only resource requests retain source bytes even with an unavailable configured Python. Keep a matching `wght` control: it must still fail with the existing dependency diagnostic when the tool is absent. Do not mutate process-global environment in tests; the isolated test command supplies the missing interpreter.

## Verification and boundary

Run `cargo test -p typst-infra --lib pipeline::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Carrier, bundle and single-evaluation controls

RED source→PDF probes/handler inspection in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` preceded public layout_types approval 2026-09-28. Adapt fixture projections/exhaustive traversal for TextSeparator with position transported and neutral point-count/ink/existing measurements. Approved identidade-hifen-discricionario-2026-09-28 initial/before/approved-before eight U+002D/SHY equal-bbox controls require transparent TextReplacement descendants, unchanged text/order/coordinates/calculation, recursive/literal-hyphen/text-box tests, no new advance/algorithm/PDF-byte/new-selection criterion.

`lab/results/construction/diagnostics/auditoria-composicao-construcao-2026-09-29/probe.json` shows standalone UA1 missing-title error while bundle emits claimed-UA1 PDF without title. Through existing World fixture require exact missing-title error/no returned file in bundle, supplied-title success and no non-UA title requirement. Test only canonical PDF/bundle owners' acceptance, never authorize production through tests. Successful bundle uses entry metadata `#document("one.pdf", title: [Okay])[...]`; #set document belongs to standalone eval, not bundle-entry metadata. Verify PDF title/UA1 identification and no inheritance by a second untitled document.

Context expansion after show keeps prepared producer/unified index: Original | Generated. A recipe carrying captured context and creating another producer preserves both; realization id allocator never recycles captured eval ids. These exercise productive pipeline, not L1-test projection adapters. Internal helpers preserve root Sequence without ContextBlock through preparation/outlines, observable content/no warnings/order/queryable headings. Arc::ptr_eq witnesses avoided mechanics, not language identity; shortcuts must not suppress callback/error/outline/context tests.

## Complete initial-candidate and local-cell sentinels

Canonical `00_nucleo/prompts/infra/pipeline/paged.md` supplies evidence/decision. Before implementation count wrapper entries per thread only cfg(test): simple multipage table without numbering uses one layout, preserves all PageStore logical bindings/text; ordinary explicit numbering uses two. Keep page-total context and outline/callback/error controls. Reject hidden SetPage context/unlisted types/residual show/unqualified Realized original. Counter measures work, not parity; preserve exact RED state/GREEN suites.

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/layout-call-counts-before.json.gz` records release/HEAD/tree/UTC/fixtures/commands: local-set-1600 two complete passes versus no-fill one. paged.rs:13–53,434–468 ties excess to local-recipe rejection. Test multipage Table with set table.cell(fill:red), no numbering, one wrapper entry/all cells+pages. Captured local show and numbering retain two entries. No arbitrary ShowRecipe/function independence inference; test owner does not define algorithm/API/phase/default. Preserve text/boxes/pages/ink.

## Page snapshots and contextual metric lifetime

Test paged counter(page).display default/callback, two pages, explicit logical start. Compare emitted per-page text to1/2 and7/8, not PDF bytes; keep nonpage counters. RED precedes display-owner repair, no pipeline algorithm change here. Require current/final/both RED→GREEN for single/multipage, initial/intermediate/late update, page-aware Func callbacks, final pages without context, nonPage counters, standalone HTML and PDF/HTML/PDF bundles with distinct totals/updates. Verify language text/context selection before stability; unreproduced cases prove no parity. Preserve exact-state prior CLI/code Page/current/final/callback/HTML RED, then GREEN before suites/build/lint with full provenance.

Empty/multilevel Page states' realized get/final project one level including zero physical delta; prior oracle/source live with `00_nucleo/prompts/core/compiler/introspect/from_tags/counter_updates.md`. Small/large table retention fixtures verify returned current/final share one generation and per-occurrence finals equal realized total. Report entries/capacities/Vec payload separately from RSS/time. Invalid Page callback return fails; next valid compile in same World has complete correct snapshot. No unmeasured upstream-message equality.

Under paged/context owners' measured contracts, selected-context table using counter(heading.where(level:1)).final() must preserve text/pages and create no independent layout metric caches across attempts. Thread-local cfg(test) counts only fresh-wrapper branch, zeroed before compile, not other helpers; RED fresh→GREEN session handle. Selected final reconciliation similarly expects zero fresh metrics with preserved text/logical numbers; prepare without selection transports no settled result. Selected RTL text+Latin/numeric suffix tests bidi handle reuse, zero fresh coordinator metrics, preserving prior layout/reconciliation sentinels. Owner's baseline299 bidi computations is historical causal evidence, not a new parity threshold. No productive instrumentation/reconciliation-equation ownership change.

## Full-pipeline bidi envelopes

bidi-contexto-implementacao-2026-10-05/verification.json.gz preserves first candidate RTL mixed/LTR numeric still matching wrong baseline. pipeline-context.log RED shows all leaves lack spans; realize/mod.rs:38–44 inserts Located, unexercised by direct eval. Located is transparent only for certifying its complete textual body; never ignore opaque bodies or Location registration. Lexical-direction and collection traverse same wrapper. Require spans in every textual leaf and RTL order with leading Latin segment; keep failed-candidate artifacts.

## Ratified bidi and effective face fixtures


`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
records unit references/1:1 owners; confirmed manifesto 1ae7513e requires None for
uncertified constructors and `..` read patterns, preserving language observations
under the pinned origin nucleus without phase/API expansion.

`identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}`
distinguishes regular GIDs from Oblique selection. Human approval of public
TextShaped.font: Option<Font> is manifesto
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`;
implementation state/inventory: identidade-fonte-shaped-implementacao-2026-10-05/.

Migrate only this owner's references: legacy constructors None, transforms preserve
Font, shaped comparison/join distinguishes face/axes. The shaper supplies the actual
Font clone even on hits without semantic/style changes to force selection. Collectors/
subset/CMap/PDF bind the same original face and style coordinates. PNG/SVG prepare
local bindings and instance once per face/request before render, keeping original
resource index separate from instanced bytes; None follows legacy resolution.
Preserve public APIs/FontKey, no fabricated fonts/unsafe/globals/new phase; private
functions may receive Font/bindings explicitly.

Association uses original face bytes plus axes, never names/slots alone. Table
lifetime is export-local; preserve supplied resource order and append missing faces.
For multifont distribute items once before per-font collectors; no cross-face
GID/CMap reuse. References never share ownership. Require RED→GREEN/bilateral
miss/hit/variable-font controls without a promised speedup. Shared invariants remain
in pinned shaped-face-identity, no second productive owner.

## Source identity and compiled-query tests

diagnostic-paths-2026-10-06/identity-next-audit.json freezes source/UTC/HEAD/tree/binaries/commands at historical9f2b6e8715237cb086899a42873660fe744622e8. eval/mod.rs:609–615 and wiring/eval.rs:66–95 reuse world.main id, cache.rs:58–78 recycles ids, upstream typst-cli/eval.rs:159–179/world.rs:176–190 distinguishes expression/stdin/no input. Shown source/position/relative base are language, integer/global-interner mechanics not required. Owner `confirmo`2026-10-06 approved `lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json`. Test document error/warning versus expression error with distinct Source ids/final introspector/parser failure. Prove one document eval via observable effect/test World, not one layout/fixpoint pass; keep import/callback origins/read base/legacy facades.

query-residuos-2026-10-06/before.json.gz freezes HEAD/tree/UTC/pin/binaries/source/commands/output/status: callback/nested error lost, contextual metadata missing, selector quote/where rejection and compilation-before-retrieval precedence. query.rs:34–66,86–105 and query_helpers.rs:488–522/pipeline.rs:182–214 support the compiled-query correction. Owner confirmed2026-10-06 in query-compilada-aprovacao-2026-10-06.json. Test public compiled facade: unknown callback variable fails original span/trace, generated metadata appears in final introspector, warnings retained once/no export artifact. Use isolated existing Sources/Worlds, not helper/match-only observations.

## Private fixture representation and exact decisions

pipeline/tests.rs:1492 and infra-imports-docs-ci-2026-10-13/before.json.gz preserve source/owner/spans/compiler output/HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty status/diffstat/UTC and productive/test/comment audit. Remove only proven unused private imports, retain public/required ones; no public/default/phase/formula/acceptance/suppression change. All-target/infra names+suite/build/workspace Clippy/Rustdoc/lineage, no import/comment mirror tests.

pipeline/tests.rs:3030 and type-complexity-2026-10-15/before.json.gz preserve hash-checked source/L0 and all-target-discovered caller. page_snapshot_retention_keeps_one_complete_generation destructures PagedCompilation.document/introspector as existing doc/intr, retaining entire program/cases4, 320/asserts/counts/retention probe. No tolerance/expected/carrier-mirror test. Paged owns result/compile; private continuous flow, workspace/lint/build/scoped lineage.

pipeline/tests.rs:388 clippy::redundant_guards, clippy-expressions-2026-10-16/before.json.gz/initial-diagnostics.json preserve source/L0/HEAD/tree/status/diffstat/UTC/suggestions/spans/type-use audit. Destination point counter matches LinkTarget::Destination(_) directly inside FrameItem::Link pattern instead of matches guard. Keep other wrapper recursion/count/fixtures/assertions. Internal expression change, no new API/default/phase/dependency/suppression/parity/performance; L0 first/scoped lineage/selected RED→GREEN/full workspace/build/all-target Clippy/fmt/Rustdoc/lineage, no syntax-mirror tests.

## Exhaustive existing FrameItem decisions


### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

Only replace frame_items_text's `_` at pipeline/tests.rs:750 (historical F067) and contains_bold_text's `_` at:764(F068), each with FrameItem::TextSeparator/Line/Glyph/Image/Shape.
Preserve complete fallback bodies/prior arms/guards/transforms/calls/returns/order.
Callbacks/effects/metadata/clipping/fonts/positions/ink/width follow current owners;
no expanded traversal/wrapper-leaf change/text-or-ActualText geometry replacement.
Tests retain fixtures/expectations/tolerances/counts/Unknown failures; V19 group
notes stay visible without pretending one arm covers all alternatives.

Continuous ADR-0127 adds no enum/field/trait method/public signature/default/
dependency/compatibility/phase and approves no historical pending proposal.
L0 first, scoped bidirectional resealing; no suppression/higher threshold or
match→if-let evasion. Before code run new FrameVisitor control/workspace suite
with unchanged expectations and confirm AST RED for all 70 family warnings.
Afterward zero scoped-family V16/no new warnings; outside V16 remains pending.
For productive core matches reached by cargo check -p typst-core, use a disposable
FrameItem sentinel variant to observe absent E0004 before/present after. That
probe covers only compiled sites, never unbuilt cfg(test) or blocked L3 sites.
For the full batch replay patterns/guards/bodies, verify enum/signatures unchanged,
real build/tests, no stub bodies.

Revalidate full workspace suite/cargo build/strict workspace-all-target Clippy
(-D warnings, -D unfulfilled_lint_expectations)/fmt/Rustdoc -D warnings/four HTML
indexes/strict V1/V5/V15/V26/reverse lineage/SARIF/diff --check. Freeze sources/L0/
configuration during runs; heavy commands sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1. Structural cleanup is not new global parity,
memory gain or measured AI benefit.

## Ratified complete text-size fixture adaptation

text-argumentos-2026-10-17/before.json.gz, before-measurement.json.gz, morph-before-measurement.json.gz, audit.json preserve HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty tree/UTC/status/diffstat/source/L0/binaries/argv/historical9f2b6e8715237cb086899a42873660fe744622e8. named-em0pt versus24pt, affine3versus27, nested18→0versus36; six Size/Color/body orders accepted upstream/rejected locally; positional18pt ignored12versus18. Pinned text/mod.rs:290, 303, 924–930, 1136–1142, args.rs:178–247 establishes typed search/named priority/rest body/composition. style_chain.rs:513–517/constructor.rs:123 drops em, collapse:394–395, 419–422 top-wins instead of fold. Restoration/Raw/heading/math/capture/show/duplicates/spans/zero-negative/metrics refute incomplete fixes. Equality probe upstream text(size:2em)[A]==text(size:24pt)[A] true, content/mod.rs:779–782 compares child: equality is a separate defect, not closed by complete type.

Human ratified manifest 771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba and its two public contracts; S1–S16/12GiB/swap0, no release/merge/push, lineage only after implementation/valid graph. RG1 independent. This test owner only adapts Style::Size(Pt) to Size(absolute Length) and StyleDelta.size expectations to Option<Length>. Preserve language/functional observables, never replace correct geometry with representation snapshots.

## Context textual inheritance and occurrence acceptance

context-paridade-2026-10-18/before.json.gz, before-sources.json.gz, baseline-build-resources.json preserve clean productive HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731/UTC/argv/diffstat/binaries/new recipe at current bd5ebec285505889e8ff1d4f68804ab147a699f3. Global/local probes A/A versusR/L, creation before rule/nesting/regex/product lose rules; inner-local L matches. realize/mod.rs:240–269 captures only Equation/Table, recipes.rs:343 treats Context generic leaf, textual.rs:46–103 already preserves baseline/local RuleId delta. Upstream context.rs:78–86 uses contextual-point styles, typst-realize/lib.rs:353–418 prepares/applies/visits. Lost-rule cause is refutable by scope/nesting/product/parent-hides-child controls. Internal correction within ratified R3-T1 preserves delayed execution/API/phases; no Location offset/hash/removing Located/public contract change, ordinary Styled stays opaque to textual matching/mapper unchanged.

Add paged/HTML global/local inheritance, pre-rule creation, nesting, regex, product, Styled, parent-hides-child regressions with body-local-rule controls. Record identity REDs(two equations,Heading prefix,it.location) separately, no ignored/relaxed expectations. Occurrence approval is `lab/history/construction/records/00_nucleo/materialization/context-occurrences-2026-10-18/review.md`, SHA 500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388 and approval-receipt.json; additive arena API only, no unrelated phase/API. Evidence at before.json.gz shows EQ EQ Q1/Q2, H EQ Q0/Q1, query00/01 from fresh per-producer allocators/dedup Location. Test distinct Context equations, Heading prefix, callback locating own it, equal producers/aliases/fresh products/nested Context/grow-shrink selected replay. Use real text/count/identity relations, no synthetic expected Locations/PDF bytes; functional RED before code, preserve textual/prior suites.

green-occurrences1.log/receipt/occurrences-candidate1-sources.json.gz preserves seven approved controls/three failures. Context owner retains unselected contributions; only actual filtered-counter/HTML-position reads join replay. equation-grow-numbered/shrink-numbered observes query in unselected producer, freezing Q0; equation-alias asserts initial empty query before legacy pass. Keep these divergent probes/external evidence/expected values, never declare GREEN. Isolate arena acceptance with observable Q counter instead of premature alias assert, and actual filtered-counter read in the query observer for grow/shrink, comparing upstream. This neither expands selection nor changes error tolerance. Core separately tests slot reuse/no ghost tags.

occurrences-html-before.json.gz/candidate2 recompiles textual freeze/arena:25 probes same status/text/primary error; ten textual match upstream, fifteen equations still diverge. prepare_html selects Position/Page, retains unselected query and rejects standalone filtered counters. Arena repairs collision but broader HTML query/it.location parity remains open; do not describe expanded battery as entirely green or alter that contract.


## Approved physical page occurrence integration regressions

Evidence precedes this test specification: the source-qualified baseline at
HEAD63b7b96b99109b61655b373dc0d248beb7838f86 and its immutable binary/input/UTC
receipts live under lab/results/set-page-header-2026-10-21. The original header
renders Figure1 twice/final1; the pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
oracle requires header1/body2/final2. Repeated static headers/footers require four
distinct actual members with per-page ordered counter actions. Contextual headers
are missing, unused overridden templates stay inert, and repeated references
report missing labels instead of the oracle's multiple-occurrence diagnostic.
Ambient probes observe header20pt then30pt despite creation at10pt; explicit
inner18pt/12pt remains separate. These are language facts, not internal IDs or PDF
bytes. Full measurement identities/sources/commands are frozen in the receipts.

The human approved review SHA256 cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039;
record lab/results/set-page-header-implementation-2026-10-21/approval.json.
This test owner adds only integrated regressions and authorizes no productive
algorithm. Actual implementation belongs to approved page-running L1 and L3
owners, retaining the exact public declarations and contextual budgets.

Before implementation, require genuine semantic RED through current compiled
query/paged facades for physical_page_header_one_page_counter_membership,
physical_page_header_repeats_as_distinct_occurrences,
physical_page_footer_actions_follow_body,
physical_page_context_header_uses_real_pages_and_effective_styles,
physical_page_inner_styles_override_page_environment,
physical_page_repeated_header_reference_reports_ambiguity and
physical_page_all_four_slots_share_one_causal_order. Keep the positive
physical_page_overridden_dormant_template_is_inert control. Never classify a
compiler/setup error as RED. Assertions exercise language count/final/at/page
relations, actual retained metadata and sourced duplicate-label errors, not
synthetic numeric Location identities, mechanical equality, private flags or
function invocation counts. Preserve all historical fixtures and expectations.

The tests require correct completed candidates, genuine repeated occurrence
identity, page-effective styles and header-before-body/footer-after-body order.
No warning/error suppression, ignored tests or weakened assertion is permitted.
After focused GREEN repeat current F/G policies and frozen CLI sources, complete
workspace validation and canonical paths/strict ownership/Nucleus/lineage checks.

## Physical request diagnostic transactions

The approved request reuse protocol retains exact realization diagnostics even
when the physical template creates no Context producers. Add source-level
controls for a show rule emitting the existing Hebrew-zero numbering warning in
a repeated header, and the same warning followed by an actual panic. Require the
existing Sink's single-source diagnostic policy, original source spans and final
terminal error; no extra recipe execution or replacement warning is acceptable.
These supplement the initial eight semantic RED controls and must run against a
frozen implementation with the same productive pipeline entry point.

## Source-owned deferred and container acceptance

Measure before deciding: pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
observations are bound by
`lab/results/set-page-header-implementation-2026-10-21/oracle-extended/receipt.json`.
The exact input programs start at `oracle-extended/{footnote-child-order,
top-float-order,bottom-float-order,hidden-child-membership}/source.typ:1` under
that result directory. All four record semantic header/FIRST/deferred/SECOND
counter states 1/2/3/4 and four distinct query members on page 1; footnote/float
paint order does not redefine those language facts. Grid, Table and segmented
Columns programs at their corresponding `source.typ:1` record header/FIRST/
SECOND states 1/2/3 and three distinct members. The prepared-show alias program
records four members across two pages, header states 1/3 and body states 2/4,
despite duplicate visual output of each prepared occurrence. These are language
semantics/identity observations under ADR-0107, not required frame algorithms.

Add integrated source-level regressions for those four deferred/hidden routes,
three cell/column routes and prepared header aliases. Require exact query
cardinality, distinct Locations and causal counter state at named source labels;
never infer those values from extracted PDF word order or weaken acceptance to
successful compilation. Keep physical page acceptance tied to the measured
program's actual page break/configuration. Further cross-page measurements remain
separately qualified and cannot silently replace these controls.

## Bundle physical-channel integration acceptance

The current Bundle owner requires one global index and per-document Page scopes;
HTML occupies no physical page and cannot execute operational page templates.
Test two Paged documents with an HTML document between them: both physical
headers participate in global Figure membership/counter order and exports use
only complete retained layouts. A failing HTML header remains inert. Separately,
Context headers must retain target `paged`, effective 20pt/30pt environments and
each document's logical Page reset to 1; body assertions observe the complete
shared metadata generation. These enforce existing target/scope contracts,
without introducing a Bundle option, new global counter policy or oracle claim
about an unmeasured experimental target. Require three actual emitted files and
preserve the ordinary Bundle regressions and diagnostic policy.


### Supported floating source route and inherited Figure surface debt

Measure before selecting the fixture: `03_infra/src/pipeline/tests.rs:3996,4003`
used `figure(placement:top/bottom)`. The immutable frozen-baseline and pinned-oracle
receipt at `lab/results/set-page-header-implementation-2026-10-21/baseline-float-surface/receipt.json`
records baseline rejection (`unexpected argument`) for both forms while the
pinned oracle accepts both. Both frozen binaries accept the existing
`place(top/bottom,float:true)[figure(...)]` route. The independent oracle receipt
at `lab/results/set-page-header-implementation-2026-10-21/oracle-place-floats/receipt.json`
requires header/FIRST/FLOAT/SECOND states 1/2/3/4 and four distinct Locations
for that supported route; its later-page controls show semantic source order
independent of geometric page order.

Use existing Place float syntax for this owner's two source-routing controls,
retaining every identity, membership, query and counter assertion unchanged.
Preserve the original Figure fixtures and observations in the diagnostic archive
as an explicit inherited parity debt. This correction neither implements nor
claims support for Figure's placement argument, and the failed Figure probes
must never be reported as successful controls. No public contract, acceptance
threshold, warning policy or retry limit changes.


### Effective page style versus partial body style

The pinned-oracle source/UTC/hash receipt
`lab/results/set-page-header-implementation-2026-10-21/oracle-partial-style/receipt.json`
measures a root20pt page with a contextual metadata header. Inline local10pt at
the end, a block-local10pt as the only body, and local10pt between root text all
produce header20pt. A full-root change to30pt produces header30pt. These are
language style observations, not style-chain pointer or layout-step requirements.

Add one integrated control with those four exact source forms through the
compiled-query facade. Require exactly one metadata value with the measured size
and no warnings. Do not let the last visited partial child redefine the physical
page insertion environment; preserve full-root style changes and inner marginal
overrides governed by the existing tests. No relaxed expectation or global retry
budget increase is authorized.


### Contextual marginal descendants under a body query

`lab/results/set-page-header-implementation-2026-10-21/oracle-context-descendants/receipt.json`
measures a header Context creating one captioned Figure, one body Figure and a
body Context observing query membership/identity/counter snapshots. The pinned
oracle compiles all assertions and returns metadata `(2,(2,))`. Preserve the exact
source in an integrated control: two members, two distinct Locations, ordered
states1/2 and final2. Actual source-generation reconstruction must not cause
unbounded descendant allocation churn; this does not authorize execution reuse
by Context id, payload equality, physical key or retry-budget changes.


### Changed running contribution must precede final paint

`lab/results/set-page-header-implementation-2026-10-21/oracle-changed-running-paint-numbered/receipt.json`
freezes the numbered-heading condition: a contextual header admits a Figure once
the heading counter is nonzero. The pinned oracle paints header Figure1:HC and
body Figure2:BC. The preceding unnumbered-heading measurement stays a separate
zero-counter control with no header; it does not establish this acceptance.

Use the exact numbered source through the integrated paged compiler and observe
actual page-layer text (whitespace-independent), requiring both measured caption
number/text combinations and no warnings. A discovery Complete made against an
older index is provisional: new running membership must reach the final paint
through the already approved source-build then sealed replay stage. Neither
caption numbers nor source selectors belong in production decision logic.

### Existing Outline realization under the physical channel

The exact source, binary, input and UTC receipts at
`lab/results/set-page-header-implementation-2026-10-21/route-probes11/receipt.json`
and `route-probes-baseline/receipt.json` distinguish a regression from inherited
debt. Both prepared own-show entries and ordinary outlines containing Metadata
or Figure bodies compiled with the preserved baseline. The new channel rejects
the prepared forms with a source-plan error and the ordinary forms with a
stability error. Raw own-show entries already fail in the baseline. The pinned
oracle additionally observes two borrowed occurrences where the baseline sees
one; restoring compilation alone does not certify that separate parity debt.

Add an integrated control for the four formerly compiling sources. Require
successful paged compilation, preserved original body facts, and no warnings.
For the default outline, require exactly one outlined source heading: a generated
title must not recursively enter its own TOC. The pinned source's existing
Outline show-set disables heading outlining and numbering
(`lab/typst-original/crates/typst-library/src/model/outline.rs:397-401`).
Do not bless the inherited one-versus-two borrowed-occurrence difference, delete
the real visual body, run a second Context solver, or replace the existing five
attempt budget. Preserve the ordinary outline and Bundle regressions.

The derived-producer branch additionally needs its own control; the four source
forms above do not establish it. Store a Context value before the outline's show
rule, return that exact value from an entry show callback, and let a body Context
observe its metadata. This makes a real producer appear only when the existing
Outline phase emits an entry. Require one value carrying the captured20pt
environment and no warnings. The same Session must retain its allocation
authority across candidates; source ordinals, integer ids, and result equality
are insufficient continuity proofs. This is a focused verification of the new
integration, not a claim that the earlier RED measured this branch.


### Deferred note and Place occurrences across physical pages

Measured evidence precedes the regression contract. The immutable source and
observation receipts under
`lab/results/set-page-header-implementation-2026-10-21/running-acceptance/extras/`
record `oracle-crosspage/note-causes-next-body-page` and
`note-child-on-later-page`, plus
`oracle-place-floats/top-place-float-next-page` and
`bottom-place-float-next-page`. The pinned vanilla oracle
`bd5ebec285505889e8ff1d4f68804ab147a699f3` compiled each without warnings
and emitted two physical pages; exact source hashes, observation hashes, original
measurement times and current dirty-tree provenance are retained in
`lab/results/set-page-deferred-corrections-2026-10-21/agents/tests/before.json`.
The measured candidate rejected all four with the source-plan verification error.
These results establish language membership, actual Location identity, source
causal counter order and physical-page observations; they do not prescribe an
internal allocation, traversal order or serialized PDF byte representation.

The first four observers were refuted only as isolation of the original
verification error by the actual RED run
`lab/results/set-page-deferred-corrections-2026-10-21/red-1/receipt.json`
and `tests.log`: all four compiled as Rust but failed on an in-flight Typst
assertion observing three members before running headers were admitted. They
did not reproduce the original source-plan verification error. The first test
and owner bytes, readiness and hashes remain in
`agents/tests/refuted-first-observer.json` under that correction directory.
Do not treat that run as evidence of the intended original cause. The Typst
assertion programs are valid language and expose an additional earlier Context
failure, not invalid fixtures or invalid assertions. Preserve those exact four
programs and retry them after the original programs pass; their final guarantees
remain mandatory and their earlier failure stays open until revalidated.

Keep four integrated `physical_page_deferred_*` regression tests using the exact
original complete source bytes, including their non-aborting contextual metadata
observer. Compile through the existing paged-with-introspector facade, then read
the actual returned introspector's metadata in Rust after successful compilation.
Require all nine typed dictionary fields from the measured observer, with five
Figure members, five distinct actual Locations, states `(1,)` through `(5,)`,
and final Figure counter `(5,)`. Require two actual document pages, an empty
warning sink and the original measured per-label snapshots. Do not add Typst
assertions to a provisional Context: doing so changes the original program and
can prevent the approved discovery/replay cycle from reaching its final index.
For both note cases, the complete queried page sequence is `(1,1,2,2,2)`; headers
have page/state `(1,(1,))`, `(2,(4,))`, the first Figure `(1,(2,))`, the
deferred Figure `(2,(3,))`, and the second Figure `(2,(5,))`. For both Place
cases, the complete page sequence is `(1,1,2,1,2)`; headers instead have
`(1,(1,))`, `(2,(5,))`, and the second Figure has `(1,(4,))`, while first
and deferred observations remain unchanged. This intentionally separates source
causal counter order from physical page order: never sort expectations by page.

The initial inference is that successful compilation must preserve these
observations. An actual mismatch after the verification error is repaired refutes
completeness of that repair; it does not authorize relaxing the test. These are
private tests and a parity correction under ADR-0127 continuous flow. They add
no public contract, phase, default, tolerance, warning suppression or retry
budget. Preserve all earlier acceptance programs and their failed receipts.


### Paint controls for the measured partial note continuation

The same two immutable note observations above include extracted page text:
`note-causes-next-body-page` paints `FIRST` and `NOTE TEXT` on page1, while
`note-child-on-later-page` paints `FIRST` and `START` there. Both paint the
Figure of the note and `SECOND` on page2. The oracle's numbered captions
identify the note Figure as `Figure 3: NC`, and the second body Figure as
`Figure 5: C2`. These are measured visible text/page observations, independent
of serialized PDF bytes or internal frame representation. Moving the entire
note to page2 would violate this measured partial continuation even if final
query metadata happened to match.

Strengthen only those two existing original-source regressions: after successful
compilation, read text from actual document page layers, remove whitespace only,
and require page1 `FIRST` plus `NOTETEXT` or `START` as applicable. Page1 must
contain neither `SECOND` nor the note's `Figure3:NC` caption. Require page2
`SECOND`, `Figure5:C2` and `Figure3:NC`. Preserve original complete source bytes,
all nine typed metadata fields, distinct actual Locations, source-causal counter
states, two physical pages and the empty warning sink. Place source programs and
acceptance observations remain unchanged. No test-derived constants enter
production behavior; these controls change no public contract or pipeline phase.


## Fractional geometry observers preserve world coordinates

### Measurement

`lab/results/nested-fractions-repeat-2026-10-21/agents/acceptance/affine-observer-proof.json`
records current source SHA and the historical full-suite observation
`middle=0` versus the unchanged `65.533 ±0.05` bound. The corresponding
`lab/results/cmyk-box-corrections-2026-10-21/final/validation/tests-domain-infra.log`
and rejected candidate artifacts remain preserved. Source inspection at
`03_infra/src/pipeline/tests.rs:449` and `:601` establishes that both existing
position observers recurse into Group while discarding Group.pos and its affine
matrix. `01_core/src/entities/layout_types.rs:1127-1140` defines the projection:
ancestor matrix, then Group translation, then Group matrix, then leaf position.
A leaf (3,4) inside translation (10,20) and scale (2,3) is at world (16,32),
while either flat observer incorrectly reports x=3.

The fresh exact-state bilateral measurement
`lab/results/nested-fractions-repeat-2026-10-21/original-before/measurement.json.gz`
records the pinned oracle and candidate, immutable sources, UTC, dirty-tree
state and unchanged inputs. Unequal ordinary sibling fractions preserve their
world origins within the existing 1e-8 geometric bound despite different local
translation decomposition. Nested-child and Repeat cases remain real language
RED. Observer correction must not relabel those failures as representation.

### Decision and acceptance

Add a causal translated-and-scaled Group witness before correcting either
observer and retain its actual RED. Compose every ancestor affine transform
exactly once when observing text or dots. Traverse Text and TextShaped and the
existing transparent Link, Semantic and TextReplacement wrappers exhaustively;
do not change each observer's matching predicate, order or multiplicity. This
is private test observation under ADR-0107/0108/0127, with no public API, product
default, phase, diagnostic or compatibility change.

Preserve all existing P1407/P1408/P1502 source programs and assertions, especially
middle 65.533 ±0.05, suffix >150, positive-negative difference <0.01, zero suffix
<40 and zero-sum suffix <50. Preserve source text, copy counts, complete pages,
styles, gap/justify and destination controls. The new affine witness retains
1e-8. Correcting an observer is not permission to alter an expected value, loosen
a threshold, suppress a warning, admit a formerly rejected language source or
claim unmeasured parity. Require witness RED then GREEN, all existing focused
controls and final test/hash/lineage gates.

## Ordinary Repeat physical membership integrated acceptance

Exact phase review SHA-256 `2609d1a9231b1b8a665f2d8284b7343366a1bc44b0471fdd6008b0f180a52bb2` is approved in `lab/results/nested-fractions-repeat-2026-10-21/phase-approval.json`. Before implementation, add pipeline tests embedding the immutable paired source language, not only synthetic Content/trace construction. No-header and empty-header zero-copy programs must compile without new warnings, display STATE0/COUNTER0/LABELS0, retain BEFORE/TAIL exactly once and emit no body X. Positive-copy, nested sibling, reached contextual Repeat, rollback, outside Label and conditional-reference controls must retain exact source authority without extra execution. Keep strict malformed/foreign/missing/duplicate proof tests, unchanged real-font55 corpus and1e-8 geometry, original target sources and baseline GREEN controls. Test HTML and mixed Bundle target separation; do not add an extra attempt, ignored failure or weakened diagnostic expectation. Raw observations, source state, original RED and final GREEN remain immutable evidence.

The actual-source Repeat tests must install their declared DejaVu Sans face from
`03_infra/fixtures/fonts/DejaVuSans.ttf`, SHA-256
`ae7b7855e115a5966d8b1b3f80f254ccc117ec86f9965e202ee2940453837280`,
using the existing SystemWorld/embedded FontSlot interface. Preserve the actual
source font and `warnings.is_empty()`; an unknown-font warning from a fontless
MockWorld is a failed test precondition, not the intended semantic RED. The
fixture retains its DejaVu license alongside it. No system-font discovery,
external path assumption, skipped assertion or warning suppression is admitted.


### Actual outside-Context position regression

The corrected phase receipt under `prototype-3/approved-phase-controls-corrected`
in the nested-fractions-repeat evidence root measures no-header here().position()
x=0pt/y=0pt versus pinned oracle158.33pt/16.36pt; empty-header returns those oracle
values. Extend the unchanged actual-source helper with both paired position
controls using its existing declared embedded font. Keep all original effect,
marker, metadata, label and warning assertions. Require PAGE1, POSPAGE1,
POSX158.33pt and POSY16.36pt in actual language output; page1 alone cannot
certify location lookup because absent paginated positions default to page1.
Stage RED before internal location transport changes. This test does not alter
terminal policy or invent a public API; the private Session owner separately
checks exact retained execution/current candidate Location and read proof.

The position observer removes exactly its single `POSX` field label before the
unchanged body-glyph X absence check; the label's X is reporting syntax. Assert
that field label occurs once so replacement cannot erase an extra marker. The
original nonposition controls retain their complete text observation.


## Complete finite-page observation — 2026-10-22

### Evidence before specification

`lab/results/remaining-parity-2026-10-22/baseline-nine/measurement.json.gz`
(SHA-256 `320e226fc9eb1b606cb155c4bfcba26046c2398f6fe0339102f1fd870ebd2aa1`)
retains UTC2026-10-22T11:48:44.824207+00:00–11:49:15.381299+00:00, exact
uncommitted HEAD`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, dirty diffstat,
full sources/fonts/commands/build identity and unchanged source inputs. Product
SHA-256`9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`
versus ratified vanilla commit`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
already matches both PDF pages and complete word sequence for finite-rollback.
Missing second product SVG is not GREEN: `pipeline/svg.rs:113` and its owner
explicitly preserve first-page string export. No default/API change follows.

### Exclusive test contract

Add an always-executed integrated test embedding byte-identical finite-rollback
source (SHA-256`c68a9a010261dabb2314f2a4c815f55f54c7ea6e9351f2bf903eb3651a800d69`)
and the declared existing DejaVuSans test font. Compile through the real common
paged pipeline, require empty warnings, two actual settled pages and complete
page text. Export every page using the existing resolved font bindings,
per-page destination context and glyph-font associations. A distinct
`TEKT_FINITE_SVG_PAGE` record carries each numbered actual SVG into the captured
LAB test log for unchanged strict geometry comparison, including page2. Do not
re-evaluate rewritten input, reconstruct a layout from PDF, skip the test based
on environment, ignore missing pages, or widen the first-page public facade.
Full geometry closure requires the independently recorded complete-page
comparison, not this test's page-count assertion alone. This is test observation
within existing phases and contracts under ADR-0127 continuous flow.

## Measured remaining HTML and Length parity correction

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/agents/html-length/measurement-before-decision.json`
pins receipt SHA-256 `96025a9ff9d2c5a6057e9873285e43e4e81501bb40b55fe43b1244743e8ed09c`,
UTC `2026-10-22T11:53:10.657191+00:00`–`2026-10-22T11:53:35.064880+00:00`,
uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, complete dirty stat,
source archive, commands/fonts/fixtures/build/tool and verified oracle pins.
All provenance checks pass. Oracle commit is `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Length rendering, numeric Raw paint, page warnings, native page aliases and two
container refusals differ. False-condition/uninserted/scalar controls pass. The
coordinator froze exact source rejection in `lab/results/remaining-parity-2026-10-22/html-negative-policy.json`
before implementation; no failed render is waived. Pinned `foundations/value.rs:192–205`,
`text/raw.rs:563–606` and `typst-realize/src/lib.rs:635–655` distinguish actual
content morphology, token styles, target-specific warnings and container errors.

### Exclusive correction and acceptance

Add actual source tests for HTML direct/alias/empty-tail page warnings with exact
SetRule byte ranges, false/uninserted controls, nested and HtmlFrame exact refusal,
and mixed Bundle target separation. Compare message, span, hints/order and body;
never weaken warnings or classify missing-render observations as success. These
regressions authorize no independent productive policy beyond their owning L0s.

Internal continuous ADR-0127 parity correction: no public field/trait/signature,
new mode/default, phase or compatibility break. L0 precedes code and meaningful
RED→GREEN; validate paths, valid owner/Nucleus graph, restricted hashes, tests,
build, strict Clippy/rustdoc, reverse lineage and complete unsuppressed Tekt.

## Measured HTML diagnostic occurrence deduplication

### Measurement before correction

`lab/results/remaining-parity-2026-10-22/agents/html-length/heading-review-before/receipt.json`
SHA-256 `d30007c52b81b2d8a0b55c3c3715333619f1e17191a0b3f28f88a40b559c42ac`
records UTC `2026-10-22T13:26:12.931919+00:00`–`2026-10-22T13:26:13.108127+00:00`,
exact coordination HEAD/dirty stat, source, commands, cached product's separate
verified archived build and ratified oracle pins. Reusing one actual Heading
constructor twice emits two visual nodes and exactly one oracle warning at that
constructor span. The contrast receipt `agents/html-length/heading-distinct-before/receipt.json`
under the same result root, SHA-256 `bccdd179d7e4ff7f03aaa550b0e38c8175e6c13fa9ce89120254cc5af1722046`,
UTC `2026-10-22T13:27:25.099502+00:00`–`2026-10-22T13:27:25.429862+00:00`,
retains two distinct constructors with an intervening Page rule. Both Heading
warnings remain, ordered Heading/Page/Heading, with true individual AST spans
and unchanged two-node visual body. All provenance guards pass. Pinned
`lab/typst-original/crates/typst-library/src/engine.rs:224–233` deduplicates
warnings by actual `(span,message)` while retaining first occurrence order.
These are observable diagnostic semantics, not an alert waiver.

### Exclusive obligation

Test the exact reused-constructor source with two identical div nodes, exactly
one Heading warning, its true constructor byte range and both original hints.
Contrast two actual constructor spans separated by Page: retain both Heading
warnings and the intervening Page warning in causal order, with their individual
source byte ranges and complete two-node visual body. A GREEN contrast is a
preservation control, not fabricated RED evidence. Tests authorize no separate
productive diagnostic policy.

## Candidate HTML and Raw replacement refuters

### Measurement before correction

`lab/results/remaining-parity-2026-10-22/candidate-2/qualification/html-length-18/receipt.json.gz`
SHA-256 `8c7f679fd7633958fa7402be97c67b3f7b0c2ef846357b023076309601175c05`
retains UTC `2026-10-22T13:43:20.929159+00:00`–`2026-10-22T13:43:43.693372+00:00`,
exact uncommitted HEAD/dirty stat, full source archive, fonts, argv/artifacts,
and source-bound candidate SHA-256
`feb025296531c92350e29bff032f655cc2fa47542cf8ab4aca0e239108bc4e8f`.
The build receipt is `candidate-2/candidate-build/receipt.json` in that result root,
SHA-256 `df1ee28f39d889a05e9aba69b306736c7a500abfc8264781d8f2a9e7853ff5b2`.
Oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
All provenance guards pass. The ignored Page-only source has correct exact
warning/source but produces `<body><p></p></body>` instead of empty `<body>`.
Raw show replacement preserves actual text/paint/root extents, but glyph origins
differ by up to `5.005859375pt`: replacement glyph advances use exterior Sans
rather than the scoped Mono font; direct Raw passes. These are separate real
language failures, not a reason to relax diagnostics or geometry criteria.

### Exclusive correction

Add a real-source test requiring exact empty HTML body for a nonempty page-only
rule while its ignored-page warning survives exactly once, and for an empty page
rule with no page warning. Preserve all original assertions and negative policies;
the earlier empty-tail test's warning-only coverage was insufficient body evidence.

## Preserve Figure product origin through semantic HTML projection

### Measurement before correction

`lab/results/remaining-parity-2026-10-22/agents/html-length/c11-figure-boundary-before/receipt.json`
SHA-256 `e5985c169d029992f1c972ad27ddd3af3ea2a682898e47d29f05c5488f8c2ff7`
retains UTC `2026-10-22T16:34:08.846371+00:00`–`2026-10-22T16:34:09.036410+00:00`,
exact HEAD/dirty stat, unchanged source/config inputs, C11 source/build archive
bindings, binary pins and the original failed F target receipt. Candidate SHA-256
`140ea6f5c8d78355bd9912739bcefc586d3d638c8dda5d50f50e393a6e19810d`
and ratified oracle SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
compile the exact original four-line source (SHA-256
`49560c2a3e13e9bb32fed8ea4a0a762b631b7f9eaefa5237f61ce5838e41a934`)
through the same source path. Both exit zero with identical complete experimental
HTML warning. Candidate body is `<p>raw table</p>`; native body is
`<p>raw</p><p>table</p>`. Kind values themselves are correct. The original F
expectation `rawtable` observes concatenated paragraph text; deleting spaces or
changing that expectation would hide a real language block-boundary failure.

The existing exporter owner `00_nucleo/prompts/infra/export/html/document.md`,
"Figure show-product paragraph morphology" and "Captured Figure product origin
transport", already specifies this measured Figure boundary. Its actual
project_visual_root needs Realized.original to recognize the consumed Figure
without rendering it. The newly reused earlier bundle projection discarded that
original before the exporter could observe it. Generic map_content's Realized
branch in entities/content/traversal.rs:438 maps only the visual child and retains
original unchanged; keeping the carrier executes no original recipe or body.

### Exclusive obligation

Add the exact original F source as a real standalone HTML pipeline regression.
Require two actual paragraph nodes containing raw and table in that order, with
no pipeline diagnostics. The CLI F/G control separately retains the exact
experimental HTML warning emitted by wiring. Keep the original source, semantic
kind values, diagnostic and F observer expectation unchanged. This test owns no
independent paragraph classifier or producer policy; bundle projection retains
the carrier and the existing export owner classifies Figure boundaries.

### New pipeline-warning precondition adjudication

`lab/results/remaining-parity-2026-10-22/green18-figure-html-carrier/receipt.json.gz`
SHA-256 `eea9c4a67000710c7662a94ab7c169b05beceaae3cb9703315a1e10c74a7a2b5`,
UTC `2026-10-22T16:40:19.645412+00:00`–`2026-10-22T16:40:39.316742+00:00`,
records that the new exact two-paragraph DOM assertion passes, then the added
warning-count precondition fails with zero rather than one. This test directly
calls the infrastructure pipeline with Html enabled. `04_wiring/src/compile.rs:359–360,579–585`
owns the experimental CLI envelope and prints it after the pipeline result;
`03_infra/src/pipeline/html.rs:71–91` only emits the feature diagnostic when the
required feature is missing. Requiring the CLI envelope inside the pipeline
warning vector was an incorrect new test assumption, not a product regression.
Preserve the exact DOM assertion; require the pipeline warning vector to remain
empty, while immutable CLI F/G retains its complete original warning requirement.
No runtime or diagnostic-emission code changes follow from this adjudication.

### Empty visual preservation before carrier-normalization decision

`lab/results/remaining-parity-2026-10-22/agents/repeat/realized-empty-probes-before/receipt.json`
SHA-256 `d06880fcd432de80a343585cb17b9c9067aa58ffa68fbb93eda1c9efc58dade9`,
UTC `2026-10-22T16:41:17.865825+00:00`–`2026-10-22T16:41:18.335595+00:00`,
retains exact sources, C11 source-bound build/archive, binary hashes, commands,
full CLI diagnostics and actual native/product DOM. Two suppressed nonFigure
Headings have an empty body in both C11 and the oracle. A suppressed Figure
between A/B instead preserves two oracle paragraphs; C11 incorrectly joins AB.

Add these exact real-source pipeline tests: `#show heading: none` followed by
`#heading[A]#heading[B]` must keep an empty HTML body; `#show figure: none`
followed by `A#figure[X]B` must keep `<p>A</p><p>B</p>`. Both enabled-feature
pipeline calls have zero diagnostics; the independent CLI policy still requires
its experimental warning. The nonFigure control is prior GREEN preservation,
not fabricated historical RED. Retaining Realized may expose it as a new RED;
measure that before selecting any further production owner/correction. Never
collapse Figure Empty before its existing classifier preserves the boundary,
strip generic empty paragraphs or change either expected DOM.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.

## Stack origins across physical fragments (2026-10-27)

Measure first: the finite image bindings in `lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/effects-candidate3/binding.json` and shape-context/receipt.json disagree with vanilla after correct image/shape frame geometry. The stored Here Location and captured integer do not consistently match the child's final page. Add an integrated regression through the existing MockWorld and paged compilation: twenty source-ordered contextual children, each with metadata `(index, here().page(), here())` and a 30×20pt fixed rectangle, on a400×400pt page with5pt margins. Require two pages, source indices0..19, and both captured and final Here pages1 for first19 and2 for last; expose exact observed IDs/positions on failure. This specification owns only tests, authorizes no production fix or new phase, and retains all prior assertions. Image CLI variants remain independent acceptance.

## Approved physical eligibility: migrate only the obsolete wrapper counter

Measurement before decision: physical-candidate-execution.json and
physical-candidate-source.json.gz under
lab/results/construction/diagnostics/readiness-fixes-2026-10-28/ preserve exact
HEAD, dirty file list, UTC and sources. Its physical-preservation log reports
numbered_table_keeps_second_layout with correct margin 1 / 1 but zero ordinary
wrapper calls instead of two. The physical eligibility review SHA-256
9d52d041904a94221cae76e4d9d394afd3a758be31e0e4b8845c6839a0dcdc87 was approved
by the human, as retained in physical-number-outline-approval.json. This
explicitly supersedes the old ordinary-wrapper call requirement only for this
newly eligible source. That counter measures mechanics, not language parity.

Keep its historical name and exact source. Require successful compilation, one
page, unchanged exact margin 1 / 1, and A/B/C/D each once in body text. Require
zero ordinary-wrapper calls for the approved physical route. Do not claim a
performance improvement from this counter. Preserve all other table controls.
The failures p1160_page_run_restaura_numbering_externo, p1406 query-one and
p1486 mixed Bundle remain substantive regressions: keep all their expected
values and sources unchanged. No test adaptation authorizes production code.

## Actual Outline entry query preservation

readiness-fixes-2026-10-28/outline-entry-before/receipt.json binds physical4
4f4144d8… and pinned64c5e8c0 with source/build/UTC/dirty state. Exact default,
show-set and identity sources fail query-one as0/0/3; hidden passes one in both.
The alias receipt separately proves repeated Locations, not distinct sources.
Add independent actual-pipeline tests for all four exact modes, requiring one
query entry and Alpha twice when visible, once when hidden. Require two genuinely
separate outlines to retain two Entries. Keep all prior source/expected values.
This authorizes tests only; production fixes remain in their exclusive owners.

Functional RED receipt: entry-text-weight-red.json/log and source archive retain
exact inputs/HEAD/dirty list/UTC and --no-fail-fast execution. Core records one
actual suffix at81.391pt instead of the regional right edge; four actual Entry
query tests and one real static-font emitter fail in infra. The hidden Entry
control and all prior readiness tests remain green. Preserve those logs rather
than treating later tests as historical RED.

## Figure geometry integrated note regression diagnostics — 2026-10-29

readiness-fixes-2026-10-28/physical13-validation1 retains2179 frozen inputs,
source archive/UTC/diffstat, six passing gates and two existing integrated
failures at the domain/infra gate. physical_page_deferred_note_causes_next_body_page
and physical_page_deferred_note_child_on_later_page observe pages1,2,2,2,2 rather
than their unchanged1,1,2,2,2, with all states/identities retained. Keep both
programs, expected vectors and every assertion unchanged. Add only actual
per-page visible text to the failing metadata assertion's diagnostic context
through the existing frame_items_text observer. This distinguishes displaced
paint from wrong source attachment without authorizing a production fix,
rewriting observations or promoting a failed gate. Use the exact fontless
MockWorld, existing facade/features and source archive for reproduction.

## Rustfmt gate repair (2026-10-29)

The frozen `physical13-validation2/fmt.log` under
`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/`
records a genuine formatting RED. Apply Rust 1.92.0 rustfmt to this consumer
without changing executable tokens, fixtures, expectations or assertions.
Preserve all behavioral obligations above; reseal both directions and rerun
the unchanged gates against the final frozen source. Internal ADR-0127 flow.


## Issue 8894 — occurrence-relative stroke integration

The official fixture and probe-existing.json in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` expose lost em
thickness independently of PDF cap omission. Under the approved entity contract
in `00_nucleo/prompts/core/entities/geometry.md`, add real-source regression
controls through MockWorld/paged compilation: dictionary 10em round stroke at
11pt and 20pt effective font, shorthand, mixed pt+em and reused aliases. Inspect
prepared shape width/cap and preserve coincident/nondegenerate path endpoints.
Use absolute 110pt controls; test expectations precede implementation and must
fail on lost em. No test-only font-size approximation or broad parity claim.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Add source-level RED tests for the official rectangle, independent relative
radius and side stroke controls, aliases/fonts, None/default and mixed lengths.
Observe prepared geometry/paint, not XML tag choice. Keep all prior tests.
