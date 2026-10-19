# L0 — infra/pipeline/tests
Hash do Código: 770fb24d

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
