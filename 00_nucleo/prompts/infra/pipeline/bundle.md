# L0 — infra/pipeline/bundle
Hash do Código: 6f24ba6d


**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/bundle.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:199640afc0a7243ca509288840d9c9e0c431f850f67919148fc8dac3b1cb33a0
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/page-rule-container-boundary.toml sha256:54dec4a08272a6ddbb91b63d388af8c25aaaa8afc7864b72226e1d125026682e


## Current evidence

- `03_infra/src/pipeline/bundle.rs:13` — `introspect_bundle_with_runtime_for_pipeline`.
- `03_infra/src/pipeline/bundle.rs:55` — `BundleFile`.
- `03_infra/src/pipeline/bundle.rs:63` — `BundleCompileOptions`.
- `03_infra/src/pipeline/bundle.rs:77` — `bundle_error`.
- `03_infra/src/pipeline/bundle.rs:82` — `compile_to_bundle_files`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/bundle.rs`.

## Owned contract

- Own `BundleFile`, `BundleCompileOptions`, ordered entry collection, path validation/collision checks, global snapshot composition and per-format rendering. Return the complete collection or diagnostics, never partial success; filesystem publication belongs to wiring.
- Preserve all option fields and explicit absence (`None`) until the relevant format owner resolves document precedence. Keep PDF stream/serialization modes independent and `is_html_document` based on semantic format, not an asset extension.
- Evaluate the root as Bundle only with its explicit feature. Accept documents, assets, tags and whitespace separators at top level; reject loose content, invalid/duplicate normalized paths, unknown formats and multi-page image documents before publication. HTML additionally requires its own feature.
- Preserve ordered bodies and inherited styles. Use one global introspector and causal Location seed per body; delegate preparation of the complete set to `00_nucleo/prompts/infra/pipeline/context_stabilization.md`. Never export bodies from different validated generations or fall back to isolated snapshots.
- Keep heading/state/label history global and Page boundaries local per body. Explicit HTML Page updates remain observable. Do not fabricate metadata padding, derive seeds from element count, hardcode page 1 or repair rendered numbering strings.
- Font resource collection and acquisition follow the resource child owner, without an unconditional catalog request for an empty set.
- Paged bodies use seeded layout; HTML remains semantic except its intrinsic `html.frame` islands. Bind logical page numbers from the final global PageStore with an offset advanced only by paged PDF/SVG/PNG bodies; HTML and assets consume no pages. Misalignment is an error, not a physical-index fallback.
- For a paged body with nonzero offset, project only marginal `logical_numbers` and `visible_numberings` into the layout copy's local indices. Keep total, reference numberings, spans and supplements global and leave the original bundle introspector untouched.
- Treat `SpaceFromNewline` as a separator only outside top-level Document/Asset entries. Preserve its origin inside bodies, preparation and HTML semantic projection until the L1 normalization owner acts.
- Cross-document `.final()` generalization is not authorized by the Page-boundary contract. No new public API, target/default or partial-write policy is introduced here.

The resource owner's private matching-axis predicate governs the early instancer dependency check. Unknown-axis-only requests must not produce a missing-tool diagnostic; retain dependency errors for actual font axes and preserve export dispatch/options.

For PDF entries delegate instancer guard/diagnostic to the private resource owner
`00_nucleo/prompts/infra/pipeline/resources.md`. Preserve its measured Portuguese
message, shared by standalone PDF, rather than the divergent English wording.
Export font dispatch is unchanged; its equivalence requires separate measurement.

## PDF entry validation and metadata

### Evidence

auditoria-composicao-construcao-2026-09-29/probe.json observes UA1 missing title
rejected standalone but bundle publishes one.pdf with pdfuaid:part1/no title.
bundle.rs:463–513 resolves standards/direct exporter while pdf.rs:404–426 first
validates. Shared-gate omission hypothesis is refuted if bundle already calls
those gates; measured source does not.
correcao-bundle-pdf-validacoes-2026-09-30/green.log then refutes delegation alone:
valid-title control also fails because bundle.rs:156–194 discards title/author/
keywords and render_bundle_document does not set document_info, unlike paged.rs:463
module info. Existing structural/document.rs:25–102 validates/preserves these fields.

### Contract and acceptance

For each PDF after pages/tags/standards resolve, call PDF owner's private pre-export
UA1 title/alt/duplicate-attachment-path helper before fonts/bytes. Propagate exact
diagnostic without prefix/conversion, abort whole collection, preserve all-or-nothing
publication. No duplicate rules/HTML-SVG-PNG/default/public API change. Matching
invalid title fails both with same message/hint; valid title bundle publishes,
no-UA1 requires no title. Alt/attachments follow PDF owner. Existing invariant
parity repair, continuous ADR-0127/RED→GREEN, no public/phase change.

Project each Content::Document metadata into its own PagedDocument before validation/
export. Present title Content→Some plain text even empty, absent→None; author/
keywords ordered joined by ", " like standalone set-document arrays. Date keeps
existing carrier/no new DocumentInfo field. No last-document title leak/filename
inference/global set-document replacing explicit per-entry metadata in this slice.
Collect bundle entries through visual carriers without provenance duplicating
Document/Asset.

## Transparent final HTML projection

bundle.rs:308 does not descend Source/Located/Realized/ShowRecipe before heading
projection. auditoria-producao-tekt-2026-10-13/html-probes.json.gz global-introspection
fixtures pass verified vanilla/reject Tekt. At final HTML render-copy projection
traverse Source.body/ShowRecipe.body/Located.body/Realized.visual before heading/
label conversion. Global snapshot already prepared: no recipe reevaluation/original
provenance publication/retained occurrence-identity mutation. Keep styles/order/
heading diagnostics/HtmlFrame island. Wrapped-heading and visual-versus-original
unit RED plus cross-document query CLI; no API/phase/default change.

## Private result and box representation

type-complexity-2026-10-15/before.json.gz retains HEAD/tree/UTC/diffstat/sources/
owners/hashes at bundle.rs:160–176; red-clippy.log selected error, reviewed tuples/
alias/static contracts, not functional/AI/parity proof. Consume context_stabilization's
PreparedBundle through named prepared_bodies/bundle_intr/document_starts with same
values/moves/order/seed/offset/global candidate/full publication. No external API.
Transparent abbreviation needs identical expansion, no module/impl/crate suppression/
higher threshold; only nominal justified historical exceptions override generic
bans, other corrections stay. Existing functional expectations/selected RED→GREEN/
unfulfilled errors/build/all-target Clippy/workspace suite/fmt/Rustdoc/lineage/
scoped seals, no alias/field-name mirror tests.

clippy-expressions-2026-10-16/before.json.gz retains sources/L0/HEAD/tree/status/
diffstat/UTC, initial-diagnostics.json inspected types/suggestions/spans at :287
(clippy::replace_box). Reuse body Box, assigning through **body the same
Content::styled(body.clone(),styles.clone()), evaluating clone before replacement.
Keep envelope/order/style/origin/causality; outer address/allocation is private
mechanics. Continuous ADR-0127 no public/default/phase/dependency/suppression;
L0 first/scoped seals/unchanged controls-tolerances/selected RED→GREEN/workspace
suite/build/all-target Clippy/fmt/Rustdoc/lineage, no syntax-mirror tests.

## Nominal per-document arity exception

too-many-arguments-2026-10-15/before.json.gz/classification.json retains full
sources/prior L0/signatures/hashes/UTC/status/diffstat at HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
dirty tree2026-10-15T16:39:18.873477+00:00. render_bundle_document :379 has10args/
threshold7, historical C100. Body/format/metadata/global snapshot/causal seed-offset/
services-options/warnings have different scopes; no proved need for new aggregate.
Arity alone proves no functional/parity benefit. Nominal materialization evidence
clippy-excecoes-2026-10-16/before.json.gz and clippy-locais/clippy-inventory.json
(79warnings) reconfirm sources/reasons/signatures/hashes/HEAD/tree/UTC/diffstat.
Only this function receives:
`#[expect(clippy::too_many_arguments, reason="C100: Fronteira por documento do bundle recebe corpo/formato/metadados e o snapshot global, seed/offset causais, serviços/opções e warnings")]`.
Keep signature/body/calls/cfg/order/other attributes, item-only, no module/impl/crate/
threshold/other lint. Remove/review if no longer over threshold, never silence
unfulfilled expectations. Old audit-only pending status/generic bans superseded
only for C100; C047/C066/pending corrections not annotated. Public/external need
requires concrete ADR-0127, private callers/L0 first.

Metadata-only internal flow/no executable/public/default/phase/dependency/architecture
change, L0 first/scoped seals; deleting added attrs/header recovers prior rustfmt
source. Real RED then historical batch exactly46arities/five-test-warnings/49expects,
no new warning/unfulfilled errors, 11correction arities visible; build/fmt/strict-
reverse lineage/Tekt/existing Counter-coverage tests under12GiB/swap0/job1. Prior
full suite is same-executable evidence, not fresh validation; no attribute-mirror/
performance test. No scope expansion to pending bundle standard CLI override.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Extend only private introspect_bundle_with_runtime_for_pipeline with actual html_targets metadata and invocation Features. Convert booleans to EvalTarget and call approved additive L1 facade. Own a fresh Sink and return candidate diagnostics; no fold/key/selector logic in L3, no syntax guesses or HTML eager postprocessors.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


## Supported table visual descendants in final HTML projection

### Evidence before decision

At baseline4382891, `03_infra/src/pipeline/bundle.rs:321–390` converts Heading in
explicit prose/HTML containers but leaves Table, TableCell, TableHeader and
TableFooter unchanged. Current-pin probes in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/residual-container-probes/receipt.json`
show plain Table text accepted by both engines, while table-cell, table-header
and isolated Cell Headings are accepted by the oracle and rejected by the
product. The independent footer source is retained in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/footer-oracle-control/receipt.json`.
Link text itself is unsupported, so Link-heading failure does not justify adding
Link projection. Existing level6 heading DOM matches; standalone warning omission
and detached bundle warning origins remain uncertified diagnostic boundaries.

`03_infra/src/export/html/table.rs:70–126` selects resolved plan rows including
footer rows, but the raw fallback emits only header and ordinary children.
`00_nucleo/prompts/infra/export/html/table.md` explicitly preserves that fallback
without inventing separate footer rendering. The field projection still retains
and converts a raw footer; using the same diagnostic sink for that dormant field
would create a deep-heading warning for absent output. This is an observed
internal renderer boundary, not a ratified language default or an exemption from
language parity. The exact current-pin public resolved-footer source, output,
message and two hints are measured before the decision in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/raw-footer-deep-oracle/receipt.json`;
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/raw-footer-readonly-plan.json`
retains the source-level distinction and prior owner hashes.


### Contract and acceptance

Project only these already HTML-supported Table/Cell/Header/Footer visual
children. A resolved Table transforms active plan cells and preserves dormant
raw source fields; a raw Table transforms existing header, children and footer
in their visual order. A Cell in a Table row slot transforms its resolved visual when present, then
standalone visual, then body. A Cell emitted through ordinary inline content
transforms standalone visual when present, otherwise body, even when a dormant
resolved snapshot also exists. Recurse into the selected visual with ordinary
inline selection: a resolved table plan can contain an inner Cell with both
snapshots. Preserve every unselected snapshot. This distinction follows the
actual renderer; never transform both snapshots or emit duplicate warnings. Clone and preserve nonvisual metadata,
presence, placement, spans, kinds, repeat flags and complete snapshots. Caption
remains currently unrendered data and contributes neither new output nor warnings.

Keep retained field conversion distinct from emitted diagnostic effects. A raw
Table footer remains projected exactly as a field, with its metadata and nested
content intact, while its currently unrendered branch creates no deep-heading
warning. A private emitted/dormant state follows every owner-local recursive
edge, including Source, Styled, Realized.visual and nested Tables; dormant state
is monotonic and never reactivates through a wrapper or nested container. This
state is independent of Table-row versus ordinary-inline Cell selection. Only
an emitted Heading may create its existing diagnostic. Do not create then
filter/discard warnings, match messages to suppress them, or change any global
walker, renderer, supported kind, public API, target/default or pipeline phase.
The same standalone Footer and resolved Table footer remain emitted and retain
the exact deep-heading warning and both hints once.


Preserve recursive causal carriers/styles and keep HtmlFrame a terminal paged
island, including frames in cells. A blanket map must not preconvert its body.
Unsupported semantic containers retain their export error. No callback, recipe
replay, semantic reindexing or global-snapshot mutation is allowed.

Verify actual bundle Table/Cell/Header/Footer heading output, sections/spans,
and resolved/standalone forms through public preparation. Inactive raw/caption
snapshots remain intact. Heading offset/deep warning occurs once in visual order;
frames containing Headings stay unconverted and emit no semantic HTML heading
warning. Preserve unsupported Link behavior and existing raw footer-emission
limits. This bounded ADR-0127 parity correction changes no public API, pipeline
phase, target/default or table-layout algorithm; it certifies no complete
Table/HTML or diagnostic parity.

Require raw deep-footer controls for direct content, Source→Styled→Realized.visual
and a nested Table: retained fields are still converted to div/aria-level7,
raw-rendered footer output remains absent and no phantom warning is produced.
The emitted standalone and provenance-bound public resolved-footer positives
must preserve the complete warning message and the two exact hint texts once.
Keep every preexisting raw-field, snapshot, caption, frame, section and public
preparation assertion unchanged. This preserves the observed raw fallback; it
does not declare its footer omission an intended language divergence.


### Nested prepared cell selection evidence

The provenance-bound diagnostic run at
`lab/results/v16-corrections-2026-10-19/batch-g/bundle-diagnostic/receipt.json`
records body/header/footer Table heading failures while the standalone-cell
control succeeds. `01_core/src/compiler/realize/table_cells.rs:934–972` can retain
resolved data while constructing standalone data; `:1347–1357` places that Cell
product inside the outer resolved visual. `03_infra/src/export/html/table.rs:44–50`
selects resolved data only for direct Table row slots, whereas
`03_infra/src/export/html/inline.rs:110–115` selects standalone data or body for
an ordinarily emitted Cell. These are source-mechanical causes of the measured
language failure, not a new language distinction.

The owner-local projection must follow those exact emission boundaries. Carry
Table-row selection through the renderer's flattened Sequence/Header/Footer
slots, then resume ordinary selection inside the chosen Cell visual. Do not
invent Source flattening, change the generic walker, widen unsupported content,
replace the renderer, or discard raw/resolved/standalone snapshots. The existing
four unchanged public-preparation source assertions remain required acceptance.


## Approved physical running-content candidate integration

Evidence before this decision is the immutable source-qualified current-pin
SetPage baseline and ambient20/30 measurements under
lab/results/set-page-header-2026-10-21. Header-only/repeated/footer/context/style
and duplicate-label failures are language observations; the twelve existing gates
passing is structural preservation only. Actual seven semantic REDs and one inert
template control are recorded in
lab/results/set-page-header-implementation-2026-10-21/initial-red/receipt.json.
Human approval of review SHA256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
is recorded in that implementation directory's approval.json. ADR-0127 items1/3
are satisfied for exactly the frozen five L1 facades/types and their phase/order.
This section supersedes earlier preserve-phase/default boilerplate only for the
approved physical channel. It adds no public L3 API, source reevaluation, new
solver/budget, generic observation/equality rule, or unrelated behavior change.

Use the approved owned Content/source plan and immutable request/resolution/trace
facades as opaque L1 transports. L3 coordinates the actual World, Engine, retained
arena frames, existing Context Session and candidate diagnostic sinks; no selector,
Figure-kind, semantic child-edge, temporal action, style-rebase or render business
logic moves to L3. The pure layout renderer executes no language closures. The
initial canonical body identity comes from the existing reached realization and
its protected operational templates; preparation publishes no invented body index
or PageStore. Only a matching actual trace/store pair builds the complete Paged
source-derived index. Global Bundle input uses actual ordered target metadata,
features and document seeds; HTML has no physical requests.

Keep the Session alive through physical discovery, including initially unselected
body producers. Actual Pending requests select the approved physical contribution
channel, without fabricating filtered/position reads or running unselected source
producers a second time. Raw operational SetPage/PageRun templates are excluded
from body discovery/substitution; realize only requested actual insertions in one
retained frame per actual request lineage/key. Inspecting a dormant template for
allocator reservation is not execution, query/counter admission or a new producer.
Register actual generated Context descendants in the same Session, preserve their
source/style/target/features, and rebind expanded Content through the approved L1
function under the shared frame. No synthetic operational Located wrapper/Context
query element or page-derived numeric Location is allowed.

Reuse a body plan only with unchanged actual retained contribution versions and
immutable source provenance. Content Eq/hash/Debug, equal counters/page counts or
freshly reconstructed closure equality are not evidence. New source generations
invalidate prior plan/request authority; pure layout reflow of the same retained
plan keeps its authority. Candidate-local stores/transcripts must be complete and
agree with current geometry/effective page environment; no partial candidate may
reach shaping/export. Maintain the existing five contextual/page attempts and
separate eight math/par.line request attempts, no hidden sixth execution or second
context solver. Discovery/replay/replaced sinks remain discarded; publish only the
retained final transactions through existing Sink policy, preserving source error
precedence and independently sourced identical warnings.

Own only this consumer's integration. Its existing private helpers and a private
descendant page_running module may coordinate the protocol; each new productive
file has its own canonical owner. Other owner's clauses do not authorize bodies
here. Preserve legacy facade contracts, non-running paths, PageRun lexical restore,
page snapshots and numbering, source parents/original morphology, HTML semantics,
Bundle validation/resource/write policy and all unrelated F/G debts. Require actual
focused GREEN and frozen CLI/counter/query/style/error/alias/dormant/template and
rollback controls, followed by full twelve-gate validation, canonical paths,
strict forward/reverse lineage, ownership/Nucleus pins/DAG and complete diagnostic
comparison. No suppression, test relaxation, opaque catchall or convenience global
hash reseal is authorized.

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

The final correction is specified in the measured-refutation clause below.

Internal continuous ADR-0127 parity correction: no public field/trait/signature,
new mode/default, phase or compatibility break. L0 precedes code and meaningful
RED→GREEN; validate paths, valid owner/Nucleus graph, restricted hashes, tests,
build, strict Clippy/rustdoc, reverse lineage and complete unsuppressed Tekt.

## Measured refutation and final causal HTML boundary

`lab/results/remaining-parity-2026-10-22/agents/html-length/empty-warning-before/receipt.json`
SHA-256 `8bd8aac399399978418fd9a5c82cdbfa07bf93c6fe9779401901b55cc489a4d6` records UTC
`2026-10-22T12:42:21.170883+00:00`–`2026-10-22T12:42:21.559949+00:00`, exact HEAD/dirty stat, sources, argv,
stdout/stderr and the cached baseline's own verified source archive/build, oracle
and font pins. The cached binary is not claimed as current uncommitted source.
Empty page/empty nested page do not warn or error. Returned ContentBlock content
and nested initial Bundle content warn rather than error; the construction-marker
proposal is refuted and must not be materialized. Direct standalone Html ContentBlock
insertion and semantic HtmlFrame/Block reject page configuration. Both Heading/Page
warning orders are measured; missing standalone Heading warnings cannot justify
late sorting. This supersedes only the refuted construction/Document-consumption
clauses, preserved as diagnostic history in that task's LAB records.

Make the existing internal HTML projection fallible and reusable by standalone
HTML. Carry nearest Source origin and private container state through its existing
emitted visual walk. Nonempty SetPage warns outside containers and errors inside
marked or semantic HtmlFrame/Block containers; an empty delta stays inert. Preserve
heading warning hints, set their actual Source spans, and produce both warning
families in one traversal order, never a later location sort. Initial Bundle target
creates no direct-Html insertion marker, so returned/nested document fragments
retain the measured warning behavior. Paged siblings and dormant table/footer/page
templates remain inert. Frame validation may inspect its actual static body without
mutating or HTML-converting the body delivered to the existing paged resolver.

Meaningful before-code RED and final bilateral exact source/message/hint/order/body
checks remain required. Internal ADR-0127 continuous flow; no new public contract,
default or phase. Preserve full unsuppressed lint/test/path/hash/lineage gates.

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

In the existing causal HTML projection, retain only the first Heading warning
for the same actual source span and message, matching existing Sink semantics
and the Page-warning policy. Preserve both visual nodes, all hints, independent
constructor warnings and their original traversal order. Do not sort the sink,
deduplicate by message alone, suppress warnings or change public API/phase.
Require the new real-source reused-constructor regression RED before changing
the body; independently sourced contrast warnings must remain GREEN.

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

After validating and emitting the existing ignored-page diagnostic, project
an emitted SetPage to Content::Empty because that page rule has no semantic HTML
visual. This includes inert empty deltas without inventing a warning. Preserve
container refusal, dormant data, exact source/message/hints and target separation.
Do not hide arbitrary empty paragraphs or suppress any warning in the exporter.
Require an actual page-only empty-body regression RED before this body changes.

## Empty transparent HTML projection carriers

### Refutation before follow-up correction

`lab/results/remaining-parity-2026-10-22/green9-infra-empty-html/receipt.json.gz`
SHA-256 `7a891d21163a31fd06aded24c03916bd830212e82048f84ca76751beb42e1bd0`
records UTC `2026-10-22T14:16:40.511601+00:00`–`2026-10-22T14:16:41.011754+00:00`,
exact source/L0 inputs, HEAD/dirty stat, asset hashes and immutable failed log.
The genuine page-only RED8 assertion still fails after SetPage becomes Empty.
`entities/content/sequence.rs:15–20` normalizes only vector cardinality, not Empty
children; `entities/content/construction.rs:27–32` always preserves Styled.
Thus transparent projected carriers can remain visually empty while the exporter
correctly reserves explicit semantic paragraphs. This refutes closing the debt
by changing SetPage's immediate leaf alone. It is not a constructor defect or
permission to strip arbitrary empty paragraphs.

### Exclusive follow-up

After the existing ordered emitted-child projection, discard literal Empty
children from its transparent Sequence and normalize its cardinality. A Styled
whose projected body is literal Empty projects to Empty as well; otherwise retain
its exact style/body. Existing Source/Located/ShowRecipe visual projection remains
unchanged. Do not collapse explicit Par, HtmlElem, HtmlFrame, Text, Strong or other
semantic nodes, and do not execute dormant bodies or change diagnostic order.
Keep the original meaningful page-only RED assertion intact. Preserve explicit
empty Par through projection/export as a positive control, and retain the existing
Frame-island preservation control. No generic exporter empty-HTML suppression,
public API or new phase is authorized.

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

Preserve a Realized carrier through this render-copy semantic projection by
cloning its existing record and recursively projecting only visual. Keep original
and every other field unchanged. The existing HTML exporter remains sole owner
of Figure product block classification; do not duplicate that classifier, add
blanket paragraph breaks, change kind inference or whitespace policy, execute
origins, or expand public API/phase. Existing diagnostic order, dormant footer/
page templates, frame islands and target separation remain unchanged.

The existing synthetic carrier-heading unit observer expected a bare HtmlElem
for a Realized wrapper. Update only that representation observation: require the
retained original to remain unchanged, then check its projected visual against
the same h2 tag/text and zero-warning assertions. Non-Realized cases keep their
original checks. This is an explicit internal-carrier observer adjudication, not
an acceptance relaxation. Add a hidden-origin Heading/SetPage control requiring
zero diagnostic or rendered contribution, and a real-source exact Figure DOM
regression. The original F expectation and all existing domain assertions remain
unchanged. C11's immutable failed F test and fresh bilateral source measurement
are pre-implementation RED evidence; final targeted HTML, F/G and full gates must
pass on the new source-bound candidate.

## Bundle PDF export warning integration

For each PDF document entry, destructure the approved `PdfExport` result and
append its diagnostics to the bundle's existing warning vector in occurrence
order before returning the unchanged bytes/file record. Preserve the public
bundle return signature, document order, page counts, timings, validation,
standards, tags, serialization, font resolution and all unrelated HTML carrier
work. Bundle export performs no direct stderr publication and never filters or
deduplicates PDF export warnings.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

## Approved issue 8954 — exclusive carrier reader migration

The human-approved schema is recorded in approval.json, patch SHA
18e5019618e3c08e71c3a9d2f51a4f50d9b4a2e624fc70ab6d8de6c4aac6ee20
under lab/results/construction/diagnostics/issue-8954-2026-10-31/.
carrier-audit-before.json.gz retains exact owner/source/UTC/dirty state. Manual
Table readers currently inspect only legacy children/header/footer: with the
new ordered stream this incorrectly certifies empty/context-free raw subtrees.
In this owner's existing traversal/certificate use Complete visual cells, else
authoritative Some(items), else legacy children/header/footer, and caption once.
No parallel visitation, new allocation, context evaluation, phase or public API.
Retain existing policies and negative certificates for all other carriers.
