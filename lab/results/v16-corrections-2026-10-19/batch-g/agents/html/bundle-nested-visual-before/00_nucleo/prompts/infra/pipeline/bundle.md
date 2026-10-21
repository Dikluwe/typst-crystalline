# L0 — infra/pipeline/bundle
Hash do Código: 3b6c781d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:199640afc0a7243ca509288840d9c9e0c431f850f67919148fc8dac3b1cb33a0
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/bundle.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

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

### Contract and acceptance

Project only these already HTML-supported Table/Cell/Header/Footer visual
children. A resolved Table transforms active plan cells and preserves dormant
raw source fields; a raw Table transforms existing header, children and footer
in their visual order. A Cell transforms its selected resolved or standalone
visual when present, otherwise its body. Clone and preserve nonvisual metadata,
presence, placement, spans, kinds, repeat flags and complete snapshots. Caption
remains currently unrendered data and contributes neither new output nor warnings.

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
