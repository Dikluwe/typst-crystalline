# L0 — Introspection query contract and stores
Hash do Código: f83e99bd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde


**Layer:** L1. **Exclusive consumer:** `01_core/src/entities/introspector.rs`.
This owner specifies the public query interface, concrete data, construction,
clone and injection methods. The concrete `impl Introspector for
TagIntrospector` and its query algorithms belong to
`00_nucleo/prompts/core/compiler/introspect.md`, with query responsibility in
`00_nucleo/prompts/core/compiler/introspect/queries.md`.

## Evidence and specification status

The original architecture inventory is
`lab/results/construction/diagnostics/inventario-tipos-introspection-vanilla.md`
(2026-04-30). Upstream separates the Introspector trait and generic
`ElementIntrospector<P>` for paged/HTML targets. Crystalline uses explicit
sub-stores and a concrete TagIntrospector. Its initial five-method, untracked,
paged-only milestone and `position_of -> Option<()>` stub were bounded staging
contracts, not permanent restrictions on later approved integration. The later
HTML contract explicitly assumes a tracked trait and separates HTML from
physical positions. There is no Clone requirement on the trait; derive Clone
on the concrete data. Old lack of external consumers and future migration
plans describe construction history, not today's acceptance criteria.

Historical R5 measured HEAD `b303f1f15b610e09872b567027e0d806387fde8c`
with an uncommitted working tree and full diff/stat. Its independent receipt
preserves sources, times and executables against historical upstream
`a51e02804`. At `entities/introspector.rs:372,727`, the store was
HashMap<Location, Content> and element_at projected directly. R5 and
`compiler/introspect.rs:1350` show that tag payloads cannot reconstruct all
public fields. The resulting public elements-type proposal below retains its
ADR-0127 approval-evidence gap; later observed use does not itself supply a
human receipt.

A later boundary measurement at HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4` found the concrete impl at
`entities/introspector.rs:559` and empty Where/Regex query branches at
`:634–724`; elements already stored IntrospectedContent. Native identity and
language equality belong to compiler. Moving the whole impl preserves that
boundary; an inherent helper would not make entities→compiler lawful.

The earlier equation-supplement measurement found the consumer choosing its
default from the reference language (`layout/references.rs:235–237`), while
upstream synthesizes from the target (`math/equation.rs:173–188`). A language
change between target and reference therefore distinguishes the contracts.
The page-numbering measurement returned repr `(..) => ..` for a callback,
refuting an EcoString-only return type. The typed trait contract was approved
with the page package completed in historical P1158; the earlier P1157
instruction to wait for that gate no longer describes this approved package.

For HTML, Position { page, point } cannot represent external HTML positions:
external HTML has none, while upstream returns x/y within `html.frame`.
Reusing the paged store would require a synthetic page that could leak. The
owner confirmed the separate HTML method on 2026-09-24 under ADR-0127.

All source line numbers and old parity pins in these measurements identify
historical snapshots. They do not certify the current upstream pin.

## Trait and data boundary

Pure L1: no I/O or mutable global state. Queries are read-only on `&self`.
Construction/injection supplies the stores; reads expose materialized data,
never execute Func or conceal mutation. The concrete data derives
Debug/Clone/Default. Empty/default construction initializes all sub-stores
empty. Public sub-stores provide explicit composition and direct access to
callers/tests, rather than boilerplate getters. The initial statement that
all sub-stores were private is superseded by their explicit public schema.
Its claimed immutability through absence of `pub(crate)` mutators is not an
additional guarantee against mutation of public fields: the operative query
contract is read-only after construction/injection, with existing visibility
preserved. Do not change public visibility to repair prose.

Keep the trait, struct, fields, derives, constructors and inherent injection
methods in this consumer. The concrete compiler impl retains every signature,
default and external implementation compatibility. Its Rust source location
is not part of the API. Do not import compiler here, install a callback or
comparator in the data, add a generic Kind, create another fields store or
introduce a global cache to avoid transferring the impl. Borrowed reads
continue returning materialized slices without Func or hidden mutation.
Moving the impl neither transfers authority over the trait nor authorizes
counter-method changes. Element-query extensions belong to the concrete
query owner. Construction/clone tests remain here; algorithm tests belong to
the concrete impl owner.

The baseline construction schema, with later additions listed below, is:

```rust
#[derive(Debug, Clone, Default)]
pub struct TagIntrospector {
    pub labels: LabelRegistry,
    pub counters: CounterRegistry,
    pub kind_index: HashMap<ElementKind, Vec<Location>>,
    pub figure_label_numbers: HashMap<Label, usize>,
    pub label_to_counter_key: HashMap<Label, EcoString>,
    pub metadata: MetadataStore,
    pub state: StateRegistry,
    pub bib_store: BibStore,
    pub resolved_labels: ResolvedLabelStore,
    pub positions: SealedPositions,
    pub page_store: PageStore,
    pub figures_for_lof: Vec<(usize, String)>,
    pub tables_for_lot: Vec<(usize, String)>,
}
impl TagIntrospector {
    pub fn empty() -> Self;
}
```

This is the retained baseline field subset, not a request to delete later
stores or recreate the compiler impl here. from_tags preserves tag/walk
insertion order in the kind index. The public Label→counter-key map is
populated for numbered elements labelled through Content::Label; the earlier
Content::Labelled legacy path does not populate it. Figure-label numbers are
populated only for numbered, captioned figures (`is_counted == true`) with an
associated label. Metadata, state, bibliography and resolved labels retain
their dedicated stores. Their initial empty-in-production windows ended at
their separately specified producer integrations; do not impose permanent
emptiness or duplicate their sources of truth here.

## Query surface and observables

All signatures below are retained interface obligations. Duplicate historical
back-reference/LoF/LoT declarations describe one method each, not duplicate
Rust items.

```rust
fn query_by_kind(&self, kind: ElementKind) -> Vec<Location>;
fn query_by_label(&self, label: &Label) -> Option<Location>;
fn query_first(&self, kind: ElementKind) -> Option<Location>;
fn query_unique(&self, kind: ElementKind) -> Option<Location>;
fn figure_number_for_label(&self, label: &Label) -> Option<usize>;
fn query_metadata(&self) -> &[Value];
fn query(&self, selector: &Selector) -> Vec<Location>;
fn query_labelled(&self) -> Vec<(Label, Location)>;
fn label_count(&self, label: &Label) -> usize;
fn element_at(&self, location: Location) -> Option<&Content>;
```

query_by_kind returns every matching Location in insertion/walk order, or an
empty vector. query_by_label delegates one-to-one to LabelRegistry::lookup
and returns None for absence. query_first equals the first copied kind
Location. query_unique returns Some only for exactly one matching Location;
zero or multiple matches return None. These derived operations must remain
consistent. figure_number_for_label supplies the one-based number of a
numbered/captioned labelled figure, replacing the corresponding legacy map
read. query_metadata returns metadata(value) values in traversal order.

The first Selector implementation supported only Kind by delegation to
query_by_kind; its deferred Label/And/Or/Where variants are historical staging,
not a restriction on the later compiler-owned query contract. query_labelled
returns `(Label, Location)` handles sorted alphabetically by Label,
deterministically; no labels means an empty vector. It delegates iteration,
clone and copy with O(n) cost, retaining the handle-based architecture of
ADR-0073/0074 rather than upstream EcoVec<Content>. Callers use Location to
obtain materialized content. label_count delegates to LabelRegistry::count,
equivalently lookup_all(label).len(): absent=0, unique=1, duplicates≥2. The
multi-label registry is a precondition; do not scan the complete labelled
query just to count one label.

element_at exposes the stored element with accessible fields. A synthetic
introspector without a walked element retains absence; the query consumer's
contracted fallback is Value::Location, not fabricated content. The complete
snapshot proposal below refines this same store instead of adding a parallel
one.

### Counter and State reads

```rust
fn formatted_counter(&self, key: &str) -> Option<String>;
fn formatted_counter_at(&self, key: &str, location: Location) -> Option<String>;
fn counter_final_values(&self, key: &str) -> Option<&[usize]>;
fn flat_counter_at(&self, key: &str, location: Location) -> Option<usize>;
fn state_value(&self, key: &str, location: Location) -> Option<&Value>;
fn state_final_value(&self, key: &str) -> Option<&Value>;
fn is_numbering_active(&self, key: &str) -> bool;
fn is_numbering_active_at(&self, key: &str, location: Location) -> bool;
fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>;
fn counter_key_for_label(&self, label: &Label) -> Option<&str>;
```

formatted_counter returns the final hierarchical formatting, corresponding to
legacy format_hierarchical. formatted_counter_at uses the historical snapshot
at the supplied Location: missing key, empty history or all updates after the
Location returns None. counter_final_values delegates to CounterRegistry::value
and supplies counter.final(). Borrow the materialized slice.
flat_counter_at delegates to counters.value_at(key, location)?.last().copied().
Missing key/history returns None. For flat figure/equation counters this is the
current one-based number; for hierarchical headings it is only the deepest
level, so callers requiring the full number use formatted_counter_at.

state_value applies ordered updates through the supplied Location;
state_final_value returns the last applied update. is_numbering_active reads
state.final_value(key); the location-aware variant reads state.value_at(key,
location). Only Some(Value::Bool(true)) yields true; missing state, Bool(false)
or another variant yields false. Key convention: `numbering_active:<feature>`,
for example `numbering_active:heading`. The two variants are distinct reads,
not permission to replace a local snapshot with the global final value.
Earlier location-aware consumer migration depended on Layouter current_location;
its old not-yet-consumed state is historical, and the proposed two-axis ADR
reference did not grant a separate public change here.

figure_number_at_index(kind, idx) constructs `format!("figure:{}", kind)` and
delegates to CounterRegistry::value_at_index. idx is zero-based among figures
of that kind in walk order, the returned counter one-based. Missing kind or
out-of-range index returns None. Resolving an omitted kind to `"image"` is the
caller's responsibility, not this method's default. counter_key_for_label
borrows from label_to_counter_key; examples are `"heading"`, `"figure:image"`,
`"equation"`, `"table"`. Return None for an absent label or one not associated
with a numbered element through Content::Label.

### Bibliography and caption inventories

```rust
fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>;
fn bib_number_for_key(&self, key: &str) -> Option<u32>;
fn citation_number_for_key(&self, key: &str) -> Option<u32>;
fn citation_order(&self) -> &[String];
fn back_refs_for_key(&self, key: &str) -> Vec<usize>;
fn figures_for_lof(&self) -> &[(usize, String)];
fn tables_for_lot(&self) -> &[(usize, String)];
fn resolved_label_for(&self, label: &Label) -> Option<&str>;
```

Bibliography entry/number reads delegate to BibStore::entry_for_key and
number_for_key; empty store or unknown key gives None. Number assignment is
one-based; the first assigned number for a key persists across multiple
Bibliography occurrences (`or_insert`), preserving the earlier bibliography
assignment clause. citation_number_for_key is the one-based first-citation
position; citation_order borrows first-citation order. Both delegate to
BibStore and support numeric citation rendering and bibliography ordering.
back_refs_for_key delegates to bib_store.back_refs_for_key and returns all
one-based citation positions, empty if never cited. The bibliography consumer
uses these for ` ↑[1][3]` after an entry body.

figures_for_lof and tables_for_lot borrow `(counter_value, caption_text)`
inventories populated by the respective Figure/Table walk arms when
caption_text.is_some(). The ElementPayload Figure/Table caption_text is
Option<String>. These inventories support layout_lof/layout_lot in the outline
consumer; a generic kind index is not a replacement for their caption data.
resolved_label_for delegates to resolved_labels.get(label), returning a
borrowed resolved string or None. It is a deterministic final label→text
snapshot and has no *_at variant. The original producer handoff deliberately
used `resolved_label_for(label).or_else(|| state.resolved_labels.get(label))`
as substitution-with-fallback while the new store was empty; retain that
bounded compatibility provenance, not a new unconditional legacy dependency.
The old parallel CounterStateLegacy/introspector construction and planned
return of both likewise describe a completed migration boundary, not a new
entrypoint requirement.

### Physical page reads and injection

The original `position_of(location) -> Option<()>` always-None stub is
superseded by physical Position and sealed post-layout injection. Preserve
position_of and positions as the paginated channel. None before injection or
for a nonlocatable Location remains meaningful absence.

```rust
fn pages(&self, location: Location) -> Option<NonZeroUsize>;
fn page(&self, location: Location) -> Option<NonZeroUsize>;
fn page_numbering(&self, location: Location) -> Option<&Numbering>;
fn page_supplement(&self, location: Location) -> Option<&Content>;
```

pages deliberately ignores its Location argument and returns document total,
matching upstream PagedIntrospector::pages: None for PageStore::empty(), Some(N)
after injection. page delegates to SealedPositions::position_of(location)?.page
and returns a one-based nonzero physical page or None. page_numbering combines
page(location)? with PageStore::numbering_for_page; absence of a position or
numbering slot yields None. The old direct-pattern Option<&EcoString> schema
and minimal-injection always-None numbering window were superseded by the
approved Numbering enum contract. Additional read-only methods expose realized
visible and reference numbering views as Option<&Content> per page, without
callbacks. They are distinct from the raw Numbering. page_supplement combines
page(location)? with PageStore::supplement_for_page, returning None for no
position/no captured supplement, while present empty Content remains present.
The type matches upstream Content under ADR-0026.

positions: SealedPositions defaults empty and is supplied post-layout by
inject_positions. page_store: PageStore is the separate sealed metadata store
for total_pages, numberings and supplements, supplied by inject_pages in
parallel. Preserve this dedicated sub-store choice rather than merging it
into positions. Its complete raw/logical/realized view schema belongs to
`00_nucleo/prompts/core/entities/page_store.md`; the sealed position contract
belongs to `00_nucleo/prompts/core/entities/sealed_positions.md`.

The public inject_positions and inject_pages examples are executable Rust
doctests, with complete imports/inputs. They verify a real position for a
locatable Location and the injected nonzero page total respectively. Do not
obtain a passing result with no_run, ignored pseudocode, I/O, external fixtures
or private APIs.

## Realized element snapshots: retained public proposal

The R5 proposal changes the single public elements store to
`pub elements: HashMap<Location, IntrospectedContent>`, retaining its existing
hasher type and Location key. Keep
`Introspector::element_at(&self, location: Location) -> Option<&Content>` and
return entry.content(). No new required trait method: the L1 query already
has the concrete TagIntrospector. empty/default/clone include the carrier.
Do not execute callbacks here. This public type change still requires the
specific ADR-0127 approval receipt; later measured use is distinguished from
that unresolved approval evidence.

Equation runtime realization keeps updating this same entry. Never create a
parallel public-fields map or replace realized Equation values with Heading
defaults. Historical Equation calculation maps retain their own roles and
must not become a second fields store. Acceptance: element_at and the complete
entry refer to the same tree; clones preserve snapshots; synthetic missing
elements remain missing. Carrier authority and public fields semantics remain
with `00_nucleo/prompts/core/entities/value.md` and the pinned snapshot nucleus.

## Heading, Equation and outline data

Keep heading_numbering: HashMap<Location, bool>, populated by the Heading arm
of populate_intr_from_tag_start from ElementPayload::Heading.numbering_active.
The reference consumer distinguishes a heading with a counter but inactive
numbering, reporting `cannot reference heading without numbering`, from the
successful supplement + formatted-counter case.

Equation numbering is materialized Content keyed by Location, filled
deterministically after fixpoint. It is the single source for the visual
numbering gutter and references: string/function numbering is already
materialized; none, absent numbering and inline equations have no entry.
The read-only trait does not expose Func to layout or execute callbacks.

`equation_supplement_content(Location) -> Option<Content>` exposes the
pre-layout materialized supplement. Some(Content::Empty) distinguishes
none/empty content from technical absence. Materialize auto defaults too, for
every locatable Equation, from the target's language. Functions never escape
to the layouter. Instrumented/test implementations explicitly delegate or
return None and update counts where applicable. Store effective alt per
Location as Option<EcoString>, preserving an empty string. element_at returns
the realized equation view, not its bare child after style loss. Layout and
query consume that same semantic entry/store; do not add an export method or
callback to the trait. Query stays read-only.

The concrete store preserves clones of OutlineElem occurrences in occurrence
order as private construction data, for one entry per realized outline and
included heading. It also preserves the Locations of effectively outlined
headings in headings_for_toc order; do not infer Location from an auto-label.
No public Introspector change is authorized by that private store.
outline_prefix_widths is private, indexed by opaque outline key and one-based
level, with the maximum absolute inset observed in outline.prefix-info
carriers. Internal borrowed reads do not change the public trait, create
query results or execute callbacks. Ignore level zero and malformed payloads.

The historical instrumented L3 CountingIntrospector added counter_final_values
as call index 27 (28 measured calls total); element_at delegates without
record_call, like heading_has_numbering. These counts are historical API
instrumentation provenance, not a current completeness claim or permission
to alter the trait for a counter.

## Ratified local HTML positions

Add `html_position_of(Location) -> Option<HtmlPosition>` to the tracked trait,
with default None for implementations without HTML. TagIntrospector stores
Location→HtmlPosition separately and injects once per generation by replacing
the complete map. The contract states clone/default start empty for this HTML
store; preserve that explicit restriction separately from ordinary concrete
Clone snapshot preservation. The relationship between that special clone
restriction and broader snapshot cloning requires an explicit owner resolution
if a consumer relies on cloning injected HTML positions; sanitation does not
silently choose a different clone contract.

Read the HTML channel only when the causal EvalTarget is Html; an enabled
feature does not select the store. position_of, positions, PageStore and
paginated queries retain their contracts. L3 fails closed on cross-island
Location collision, a position that cannot be associated with topology or
mixed generations. The trait cannot repair these with a synthetic page.
The public method was confirmed by the owner on 2026-09-24.

## Ratified realized Page counter snapshots

### Measurement

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, exact working tree and
binaries in
`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/measurement.json.gz`.
Single-page output was B1/0 F0 versus B1/1 F1. Two-page total/final was 0 versus
2; initial update 7 retained total 7 versus 8; an intermediate update changed
the preceding page (7/7 and 8/7 versus 1/7 and 7/7); final step printed final 1
versus 2. Bundle first PDF printed 1/7 versus 1/1, second PDF final 7 versus 8.
Mixed HTML returned 1 versus 7.

`03_infra/src/pipeline/paged.rs:360–365,840–913` used the last update as every
page's start; context_stabilization.rs:969–1010 did the same per body.
`compiler/stdlib/counter.rs:490–513` and `counter/display.rs:96–110` read a global
causal final value. Historical pinned upstream
`9f2b6e8715237cb086899a42873660fe744622e8`,
`lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963`,
combines document scope, updates and physical advances. This is language
parity evidence, not a requirement to copy upstream memoization mechanics.

### Approved data contract and acceptance

Apply the pinned page-counter-snapshot nucleus. Human confirmation of
`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/l0-manifest.json`
(SHA-256 `1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e`)
satisfied this package's ADR-0127 gate on 2026-10-05. Thus its earlier proposed
status, old final-only scope-out and wait-to-materialize instruction are
superseded within this package. Subsequent internal clarifications follow
continuous flow, L0 first and RED→GREEN. Keep existing signatures, cycles and
budget; never guess a page/total before its snapshot exists.

Preserve this consumer's ownership and other fields, adding:

```rust
pub page_counter_values: HashMap<Location, Vec<usize>>,
pub page_counter_final_values: HashMap<Location, Vec<usize>>,
```

Both start empty. They are complete views of the realized generation, per
existing occurrence, including recognized contexts, updates and aliases.
Store the entire state, including an empty vector: absence differs from empty
state. Each Location's final belongs to its own document. Clone/Default/equality
copy and compare both maps. Do not change the public Introspector trait for
these fields. A producer cannot reuse prior-generation entries to complete a
new one. Account for retained memory; introduce neither globals nor persistent
caches in this correction.

## Acceptance, scope and provenance

Verify empty construction: empty stores and None/empty query results; numbering
flags false and counts zero where their signatures require those sentinels.
With one label, populated counter registry and a kind index containing two
Locations, verify ordered query_by_kind, first Location, unique=None and the
associated label lookup. One kind Location gives unique=Some; zero gives None.
Before injection position_of remains None, rather than a fabricated position.
Exercise clone/snapshot preservation, synthetic missing-element absence, both
executable injection examples, typed page metadata and isolated HTML routing.
These are data/construction obligations; concrete query algorithms and their
language-level acceptance remain with the compiler owner.

The productive result is the trait/data/inherent-method consumer above and
its construction tests, with reexports of trait and struct in entities/mod
under that owner's contract. Do not recreate an impl here because an early
result list included it. Historical plans to return CounterStateLegacy beside
TagIntrospector, migrate reference consumers incrementally, and introduce
metadata/state/query as the source of truth remain recoverable in the archive;
they do not authorize a second runtime source or an always-empty store.

The complete pre-consolidation owner, including dated migration milestones,
revision table, initial five-method interface, staged empty stores and
superseded forms, is preserved at
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/introspector.md`.
Its historical construction references P165/P168–P177/P181–P207, P462/P468/P472,
P788/P844, P1140/P1157/P1158, P1307/P1339/P1402/P1405/P1470/P1493 are provenance,
not active execution-step organization. The original inventory cited ADR-0033,
ADR-0066 and the then-applicable handle/page ADRs; apply current governing ADRs
without reviving revoked guidance. R5 approval evidence and the HTML clone
scope conflict above remain explicit pending issues.
