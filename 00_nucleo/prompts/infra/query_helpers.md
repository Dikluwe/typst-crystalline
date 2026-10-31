# L0 — Query parsing, summaries and located results
Hash do Código: 9f6ceb5f


**Layer:** L3
**Exclusive consumer:** `03_infra/src/query_helpers.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


## Current evidence

- `03_infra/src/query_helpers.rs:110` — `parse_selector`.
- `03_infra/src/query_helpers.rs:154` — `summarize_query`.
- `03_infra/src/query_helpers.rs:225` — approximate content counts.
- `03_infra/src/query_helpers.rs:274` and `:357` — text/variant walkers.
- `03_infra/src/query_helpers.rs:455` and `:475` — summary and located-result entrypoints.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own selector parsing, the existing approximate summary projection and transport of located query values. Do not implement JSON/YAML/repr formatting, field realization, CLI --field/--one selection or a new L1 selector model. Presentation belongs to `00_nucleo/prompts/shell/cli.md`; command composition belongs to `00_nucleo/prompts/wiring/query.md`.

`parse_selector` trims input and recognizes simple ElementKind names, nonempty angle-bracket labels and the `math.equation` alias. Preserve the internal standalone equation spelling. Quoted-text/regex selectors produce the distinct `text is not locatable` error. Empty/unknown/complex selectors preserve their current QueryError messages; dots/parentheses are rejected after the alias check. Diagnostic literals containing a historical case identifier remain exact output, not chronology authorizing a contract.

`query_to_summary(world: &dyn World, source: &Source, selector: &str)` returns `Result<QuerySummary, QueryError>`. Parse before eval; reuse the pipeline eval adapter, obtain module content, and introspect the pre-show tree when supplied. Preserve EvalFailed, NoContent, InvalidSelector and WorldError. This legacy entrypoint discards evaluation warnings. `summarize_query` also takes the raw selector string and retains it in the result. QuerySummary keeps selector, SelectorKind, count, kind_name, label_found and metadata_values without Serialize. Label summaries count zero/one and retain a matched label. Kind summaries count introspector matches except the existing approximate document-element projection. Metadata uses its current plain-text/debug-string projection, not generic language serialization.

`count_element_in_content` retains its distinct List/Enum group walkers, boolean paragraph-text approximation and Link/Raw/Quote/Footnote variant counts; other kinds return zero. Do not promote approximate matches into located values. The list/enum walkers count all-item Sequences and recurse as currently implemented; the singleton discrepancy is recorded in `lab/results/construction/diagnostics/sanitization-infra-wiring-history.md`. The text/variant walkers descend through their explicit content containers, including PageRun.body without interpreting PageConfig. OutlineEntry text consults element; variant counting descends element/fill after the predicate precheck. A matching predicate returns once without counting descendants again.

Mathematical carriers are terminal in this approximate projection, including limits/class/style wrappers, root/fraction/sequence, lr/mid/stretch, primes, accents and over/under decorations. MathUnderline, MathVec and Flush do not invent text/located occurrences or descend into math internals. Space and SpaceFromNewline remain neutral, without guessing newline origin or removing CJK spacing. Public structural traversal/repr/layout semantics remain with their L1 owners; these approximate walkers do not redefine them.

Preserve the current public located-result signature:

```rust
pub fn query_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

Invalid selectors become detached SourceDiagnostics without panic. Evaluation errors and warnings remain separate; a module without content returns an empty successful vector. Use one causal realization index with source and generated occurrences. The later approved realized-index reuse below supersedes the older tree-by-cardinality selection; equal fields never collapse distinct elements. Preserve selected location order and cardinality for every recognized locatable kind and labels, without a heading-only filter.

For an entry with captured fields (`Some`), clone the complete IntrospectedContent and transport its exact Location as LocatedContent. Preserve the captured label; never rebuild a captured-field entry from a selector/registry, discard its snapshot or replace Some by None. For an entry with no captured fields, preserve the legacy labeled-content projection by constructing a separate None entry when required, without modifying the original entry. A location without an element fails with `query result has no locatable element` instead of silently dropping it. No output serialization or default-field fabrication belongs to L3.

## Verification boundary

Existing tests cover selectors, summaries, match ordering, label transport and neutral math/newline-space projections. Located values preserve public fields/labels through the L2 serializer. This prompt records the current consumer; it does not resolve archived approval/provenance gaps or certify fresh upstream parity.


## Causal realization index

Legacy counts/visibility project internal carriers to emitted visual only, never
count consumed original as emitted content. Semantic query instead uses causal
realization provenance. Inspection query_helpers.rs:478,519 found initial index
reconstruction despite realization already producing it. Borrow
Module::realization_introspector under
`00_nucleo/prompts/core/entities/module.md` for both query paths while Module lives;
build fallback only when absent. Never clone read-only stores/select visual tree
by match count/persist across separate evaluations. Keep causal order/fields/labels/
originals/generated occurrences and distinct equal-field semantic elements.

## Approved compiled-query API

### Evidence and approvals



`lab/results/construction/diagnostics/plano-debitos-2026-10-06/before.json.gz`
measured error → document warnings → deprecation and the specific replacement hint
against the historical pin. Upstream query.rs:43,51–64 and compile.rs:779 order
these explicitly. Human `confirmo` on 2026-10-06 approved replacing ADR-0045 §8's
prior warnings→errors order through its amendment; candidates/hashes are retained
in `lab/history/construction/records/00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json`.

`query-residuos-2026-10-06/before.json.gz` retains HEAD/tree/UTC, pin/binary hashes,
sources/commands/stdout/stderr/status. Missing-file controls (relative, ./relative,
absolute) show the pinned oracle exits 1, empty stdout and one line:
`error: input file not found (searched at INPUT)`. Tekt exited 2 using MainNotFound
Display. `infra/world.rs:150–155` already distinguishes typed MainNotFound from Io.

The same diagnostic's query-stdin-callback succeeded where vanilla reported error
and trace; query-context-success lost generated metadata; nested-error disappeared;
query-selector-quotes failed at the old selector limitation before a document
error; where-level was rejected rather than returning [2]. Document-error-invalid-
selector establishes compilation precedence; valid-document-invalid/text-selector
establishes retrieval failure without deprecation. Contextual eval reference and
where controls already match through the current paged compilation. Upstream
query.rs:34–66 compiles before retrieval; :86–105 evaluates selector in default
Scope then converts to LocatableSelector. Locally query_helpers.rs:488–522 consulted
only Module, while pipeline.rs:182–214 already compiled with the final introspector.

Human `confirmo` on 2026-10-06 approved the new compiled-query API/path, receipt
`query-compilada-aprovacao-2026-10-06.json`. Historical candidates/manifests remain
receipts; this owning contract is current. These measurements do not certify the
new upstream pin or require copying upstream execution mechanics.

### Exclusive helper contract

```rust
pub enum CompiledQueryError {
    Compilation(Vec<SourceDiagnostic>),
    Retrieval(String),
}
pub fn query_compiled_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, CompiledQueryError>, Vec<SourceDiagnostic>);
```

First compile through compile_query_context; failure retains raw Compilation
diagnostics. On success evaluate selector with existing code evaluator in separate
stdlib scope, empty expression introspector/default Features/transient code Source.
Never interpolate selector into query(...) or run document again. Expression Source
stays internal; its errors are retrieval text, never formatted against document
Source. Eval error is Retrieval("failed to evaluate selector: "+messages joined
in order by ", "). Expression warnings unpublished as upstream retrieve; compilation
warnings stay separate.

Reuse public native_query with Args of one evaluated value and final-introspector
EvalContext. Str/Regex yield Retrieval("text is not locatable") before legacy
string-name conversion, never reinterpret text as kind. L1 owns composition/
locatability, no L3 .where parser/function-name matching/new Selector model. Preserve
original locatable-conversion message in Retrieval. Return Array Vec<Value> with
LocatedContent/exact captured fields, no fabricated fields/cardinality-chosen index.
Keep legacy parse_selector/query_to_summary/query_elements signatures/contracts
and historical consumers, no approximation promotion; CLI uses compiled helper.
This is an additional ratified public signature, no removal.

## Label transport without captured snapshot

green-cli-refined.log in query-compilada-2026-10-06 finds <fig-one> figure missing
JSON label while legacy p1285 transports label through None projection. native_query
returns store element, registry retains Location/label. Do not modify L1 store/
frozen fields. For LocatedContent fields=None, query final introspector's
query_labelled by exact Location, carry found label through Content::label_auto
in new IntrospectedContent(None), preserving already-Label body. Some retains
all original fields/label exactly, never replace with None/rebuild from registry.
Order/cardinality unchanged; CLI label RED→GREEN, internal parity/no new phase/API.

## Ratified prepared-table index boundary

The full table insertion evidence/refuters at
`00_nucleo/prompts/infra/export/html/table.md` retain HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/
dirty tree/UTC/diffstat/sources/binaries/argv and historical9f2b6e871. The owner
ratified insertion package2026-10-07, `comfirmo`, manifesto
`5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, record
`lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`.
This supersedes old candidate status only for its five fields/two carriers/facade/
pre-Cell-show phase; fields remain owned by domain, not this helper. Query current
prepared-view index with authoritative snapshots, never hidden sources/cardinality
choice. Cell remains nonlocatable. Require RED→GREEN/suites/build/lint/restricted
preservation/table lexical/raw/smartquote/matrix/query controls from archived
contract; language/effects/real geometry, no bytes/whitespace closure. Isolated
intrinsic defaults/fresh-generation behavior belong only to separate approved
isolated package; cell-set/style/default Table/rich paint-cast/gutters/multipage/
PDF-UA/global-nonlocatable scope remains open. No legacy-cast/syntax/flag changes.

## Approximate walker expression maintenance

ci-simplificacoes-2026-10-14/before.json.gz retains source/owner/spans/HEAD/tree/UTC/
diffstat at :304, 305, 310, 328; red.log selected errors, uses/types checked at1.92.0.
has_any_text presence of header/footer/caption/title uses
as_ref().is_some_and(has_any_text), absent false, Some same call once, OR short-
circuit/order preserved. No case/prepared-table selection change or approximation
promotion to semantic query. Continuous ADR-0127/no public-default-phase-unsafe-
dependency-suppression; L0 first/scoped seals/existing fixtures-expectations-
tolerances/selected RED→GREEN/full suite/build/all-targets/Rustdoc/fmt/lineage/
full inventory, no simplification-mirror tests.

## Closed approximate-summary decisions

### Evidence before classification

The clean batch-G baseline is commit
`4382891f0d5f0839669597cb5c90a98f24cfb8bd`. Exact source/owner bytes, hashes,
UTC and the current working-tree stat are retained in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/queries/audit.json`.
`03_infra/src/query_helpers.rs:154-177` routes seven document kinds to the
approximate helper and other kinds to the real introspector.
At `:225-243`, direct calls for non-document kinds return zero.
At `:246-272`, an all-item Sequence counts one group, including a manually
constructed empty Sequence because `all(empty)` is true. A bare ListItem or
EnumItem descends into its body; ordinary wrappers are terminal here.
`01_core/src/entities/content/sequence.rs:15-21` normalizes empty/singleton
inputs, so a raw Sequence is not interchangeable with `Content::sequence`.
These are source-level observations of the existing approximate API, not
fresh vanilla measurements or proof of a reachable CLI defect.

### Decision and acceptance

Preserve the existing List/Enum/Par/Link/Raw/Quote/Footnote branches and give
every other current ElementKind an explicit zero disposition. Keep the
summary's real-introspector route separate: Heading and Metadata can have
positive summary counts while the direct approximate helper returns zero.
For both group walkers, retain Sequence and matching-item recursion exactly;
name every other current Content variant as intentionally untraversed by this
legacy projection. Never infer that its children lack semantic content.

Meaningful preservation controls distinguish raw empty from canonical empty,
bare items from explicit one-item groups, opaque Styled wrappers from mixed
Sequences, nested group bodies, and direct-helper zero from positive actual
introspector summaries. Existing compiled-query and located-result tests remain
binding. Closed-enum mutation in a disposable full workspace must change the
three selected decisions from no diagnostic to exact primary E0004 witnesses.
This is structural protection, separate from language acceptance.

No public signature, target, phase, selector grammar or default path changes.
The known approximate-group limits remain open; this repair does not ratify a
new approximation or close list/enum language parity. Preserve all existing
tests and diagnostics, validate ownership/Nuclei before scoped resealing, and
require focused/full tests, all-target build, strict Clippy/Rustdoc, formatting,
path and forward/reverse lineage gates plus the complete lint delta.


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
