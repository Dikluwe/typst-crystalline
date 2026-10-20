# Draft: prepare rich Figure supplements at emitted numeric references

Status: **unratified ADR-0127 §2.1 items 1 and 3: additive public classifiers and phase extension**. No implementation is
authorized by this draft. Batch F stays open pending this decision. This is a
newly measured path, not a retroactively invented pre-existing debt.

## Evidence before decision

The exact pre-classification test, L0 and integration-tests-5 actual RED are
retained with hashes in `nested-matrix-before-phase-classification/receipt.json`.
`nested-supplement-oracle-v3/receipt.json`,
`nested-supplement-effects-oracle/receipt.json` and
`nested-supplement-identity-oracle/receipt.json` bind exact sources, argv, UTC,
dirty state, pinned vanilla commit bd5ebec285505889e8ff1d4f68804ab147a699f3,
tree cac836c07e4dec2a89219370428292393a3216b5 and executable SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
Candidate-one replay is separately bound to the frozen candidate build receipt.
No counter result inferred from a failed/syntactically invalid probe is evidence.

For both explicit `supplement: inner` and `supplement: it => inner`, a captionless
outer Figure followed by `@outer` emits the nested Figure, with figure count 2,
table final [1], image final [1]. Removing that reference leaves only the outer
Figure: count 1, table [0], image [1]. An inner supplement callback panic occurs
for the reference and numbered caption, but not no-caption/unused/own-hidden or
parent-hidden controls. This establishes a missing language effect and content
occurrence, independent of render bytes or Rust equality.

## Exact present contract and missing boundary

Read owners: `00_nucleo/prompts/core/entities/elements/ref.md` explicitly declares
Ref a **nonlocatable terminal leaf**, preserving terminal content/text maps;
`core/compiler/realize/recipes.md` realizes only emitted products and never
Realized.original; `core/compiler/realize/figure.md` prepares reached Figure before
its own replacement. `core/compiler/realize/mod.md` owns initial document and
contextual insertion realization. `core/compiler/layout/references.md` resolves
numeric reference content against immutable Introspector. The approved Figure
counter design validates demands after actual action indexing; it does not add a
reference content producer. Complete Figure caption synthesis remains separate.

Current code: `eval/mod.rs` passes an empty previous introspector to initial
`realize_document`; `realize/mod.rs` performs recipes, occurrence finishing,
normalization, then initial introspection. `layout/references.rs` chooses explicit
Ref.supplement, otherwise target Figure.resolved_supplement, only after label/key
resolution. Ref entity maps are terminal. Thus initial realization cannot select
a forward target, and pure demand collection cannot legitimately create or
execute a missing nested occurrence. Executing callbacks in layout or indexing
unrealized raw retained content would not repair that causal omission.

## Proposed bounded phase and private transport

Admit **normal, nonbibliographic numeric Figure references** to the existing
context-producer stabilization boundary, after an initial document label/action
index exists and before the retained iteration's final counter-demand validation
and layout. No new public entity field, Content variant or trait method is proposed. Three
exact additive doc-hidden public transport signatures below bridge the private
L1 producer protocol to L3; keep Ref entity and generic maps terminal. Page references,
bibliography/cites and HTML implicit reference rendering retain their current
contracts; do not eagerly inspect their supplements.

A new feature owner `compiler/realize/reference_supplements.rs` wraps an actually
emitted eligible Ref only after the Ref root's own show-rule/replacement decision.
A parent hiding the Ref, show ref:none, or a replacing Ref show rule must prune
the selected supplement effects. Preserve the existing recipe guards on a
replacement/alias and admit only its actually emitted eligible Ref product; do
not capture a producer before executing the root decision. Wrap that Ref in the existing ContextBlock/ContextRecipe mechanism. Its
private NativeWithEngine producer captures the raw Ref, lexical insertion styles,
show rules, source span and stable context occurrence through existing Args and
context-occurrence arena. Selection is a real contextual label query against that
iteration's immutable introspector; record that dependency through the existing
ContextReadRequest::Query machinery. Read the first matching label, require a
prepared Figure and enabled numbering, then select explicit reference supplement
or the target's resolved supplement. Do **not** rerun the outer Figure supplement
callback: the selected Content is its existing resolved value.

The producer realizes a fresh clone of selected Content through existing recipe,
Table and occurrence preparation with its exclusive frame in the existing shared
Session ContextOccurrenceArena (never a separate arena), then returns a
prepared Ref whose existing supplement field carries that realized contribution.
Keep the original Ref in Realized.original. A compiler-private root marker in an
existing Styled wrapper distinguishes this generated Ref from a fresh source
Ref: an internal native Func identity token, not a public field/string comparison.
The exact marker stays attached to the prepared Styled(Ref) transport for every
later accessor/map call. While visiting the selected child, omit that root-only
marker from the inherited style snapshot; do not physically remove it from the
retained Ref carrier. It cannot suppress nested references or leak into user
styles/fields. The marker performs no callback and its exact typed
identity is not forgeable by a same-named user function. Canonical action walk and
demand traversal visit the prepared Ref's selected supplement at that reference's
actual position; layout consumes this prepared field and executes no callback.
A marker-free raw Ref remains terminal until its producer executes.

Preserve the selected template before any caption child realization: use existing
`Realized { original: Located(prepared Figure before child realization, L), visual:
Located(same Figure after actual child realization, L) }`. Canonical introspection
indexes that original Figure root only, then actual visual children (the existing
Table-original pattern); it does not descend retained raw supplement/body twice.
The target's element_at snapshot therefore retains the already resolved outer
supplement result, before a caption use assigns descendants their Locations.
This also applies to the no-show/no-rule Figure path; an existing show original
already retains the corresponding snapshot. Figure kind/supplement getters,
fields and queries observe this prepared original's resolved language value,
not a synthetic cache or a newly added seventh entity field.

Each caption/reference use clones that immutable selected template. Raw descendant
Figures have no Located carrier and acquire distinct Locations through the
respective actual producer frame. A Located descendant already present in the
template is an intentionally prepared alias and retains its existing snapshot;
never blindly strip all Located/Realized carriers or alter known field snapshots.
Thus two references plus a caption of a raw nested Figure produce three inserted
occurrences, while an intentionally preexisting prepared alias obeys the approved
alias contract. Do not source a new use from a prior caption's realized visual.
The original/template and visual may share unchanged Arcs; no callbacks rerun to
reconstruct templates.

Raw ContextBlock descendants in a selected template also need fresh producer IDs
per actual raw insertion. During reached child instantiation, allocate each raw
ContextBlock ID from the existing ctx.next_context_id counter, advance that
counter, clone its closure and capture its actual insertion recipe. Do this after
ancestor/root replacement permits the child; do not previsit hidden template
children. An already Located ContextBlock is an intentionally prepared alias and
retains its ID/Location. Session's next_id floor and its shared occurrence arena
remain authoritative across nested producers; no separate ID domain, second
arena or source-hash offset is introduced. Repeated raw reference uses must not
be rejected as duplicate ContextBlock IDs or accidentally share contributions.

## Exact additive transport contracts and actual-view traversal

Export through the exact additive public module
`compiler::realize::reference_supplements` (all other items remain private):

```rust
#[doc(hidden)]
pub fn is_reference_supplement_producer(
    block: &crate::entities::elements::context_block::ContextBlockElem,
) -> bool;

#[doc(hidden)]
pub fn prepared_reference_supplement(
    content: &crate::entities::content::Content,
) -> Option<&crate::entities::content::Content>;

#[doc(hidden)]
pub fn map_prepared_reference_supplement<F>(
    content: &crate::entities::content::Content,
    transform: F,
) -> crate::entities::source_result::SourceResult<
    Option<crate::entities::content::Content>,
>
where
    F: FnOnce(&crate::entities::content::Content)
        -> crate::entities::source_result::SourceResult<
            crate::entities::content::Content,
        >;
```

The producer classifier and borrowed child accessor are pure and execute nothing;
unrelated input returns false/None. The map facade invokes transform exactly once
on the selected actual child of a recognized transport, rebuilds that transport
with only that child replaced, propagates transform errors, and returns Some. For
unrelated input it returns Ok(None) without invoking transform. Producer recognition requires exact FuncRepr::With shape whose
base is the private NativeWithEngine producer's typed function pointer and whose
bound Args has the specified one raw Ref Content argument plus its source span;
additional/malformed bindings fail classification. Do not compare Func names or
Func PartialEq (native Func PartialEq includes name). The prepared accessor and map facade
recognize only the exact Styled(Ref) carrier with one reserved compiler-private
native pointer token in Styles, no inherited marker and no unknown wrapper
search. L1 alone owns marker recognition, Ref unwrapping and reconstruction. L3 only
consumes the borrowed actual child or supplies its existing recursive transform
to the map facade; it makes no marker/Ref-field business decision. Private
pointer/token construction remains L1-owned. Source-created homonyms and
ordinary native functions never qualify. These three added signatures are explicit
item-1 approval scope, not claimed as already existing protocol.

Paged Session::execute sets a separate pipeline-private reference-producer bit
from the producer classifier when the actual emitted producer executes. Its
selected state is filtered-counter OR this new bit OR existing HTML-position;
ordinary Query/Context producers retain existing selection. Latch admission and
retain source failures/warnings even before the first successful target/counter
read. Query logging alone does not currently select a Paged producer, and
has_filtered_counter_reads keeps its current pure getter semantics unchanged.
Bundles use the same classifier/admission contract rather than relying on their
current all-producers selection accidentally masking a missing standalone path.
Validation still records/replays the actual Query and any actually required typed
CounterAt; no fake selector/counter observation is added merely to force selection.

Generated ContextBlock is execution transport, not a new source-language Context
object. The canonical walker recognizes the producer classifier, reserves its
real existing allocator Location and fills the internal context_block_locations
entry needed by Session, but does not insert a fabricated semantic ContextBlock
payload into elements/query membership. Raw Ref original remains the language
object and stays nonlocatable. This is an explicit owning walk/populate/locatable
review obligation, not an assumption that an internal wrapper is invisible.
Ordinary source ContextBlock indexing remains unchanged. Preserve label-on-Ref
classification/target: a label surrounding the transformed Ref continues to label
the original Ref morphology, not the internal producer or its first nested Figure.
The compiler source/label provenance wrapper and existing labelled walker must
register execution transport separately from the ordinary semantic label target.
No hash/id-derived
Location or fake counter action is permitted.

Generic Ref remains terminal. Compiler actual-view traversal explicitly recognizes
the prepared Styled(Ref) carrier through the borrowed accessor: enter its actual
selected supplement with the same lexical style snapshot, then retain the Ref
numeric placeholder. The token remains on that single retained Ref transport, but is excluded from
styles inherited by its selected child; it is never inherited to nested
references. The getter/map recognize it again after substitution without relying
on an inherited token. L3 collect_context_blocks and ordered_blocks recurse via the borrowed child
accessor; substitute_context_blocks delegates rebuilding to the map facade with
its existing recursive substitution transform, so nested Context
producers are discovered, retain order and substitute their accepted contributions.
The pure canonical action walker and figure demand visitor mirror that exact
admission; layout/references consumes the prepared field in its existing numeric
link position. No generic Ref entity map or unrelated semantic visitor changes.
The current Numbering None/missing/unnumberable checks and bibliography/page
precedence still gate actual supplement admission before any nested effects.

## Strict observation audit

Existing source proof: observation/value.rs NativeWithEngine branch compares the
typed pointer, name and namespace; With recursively observes base Func and Args;
Args observes items/named/span/occurrences. observation/elements.rs Ref observes
name/supplement/form and ContextBlock observes id/closure. NativeWithEngine has
no Rust captures: use its existing Func::with(Args) carrier, not a closure pointer
that pretends to carry data. Bind only immutable raw Ref Content and real source
span; lexical rules/styles/file are captured by prepare_context_recipe's existing
ShowRecipe carrier, not hidden mutable globals. ContextBlock ID uses the existing
next_context_id allocation and shared arena frame, never hash/name offsets.

ObservationEq Same is not assumed from Arc/debug equality. Preserve strict
Different/Unproven invalidation: user Element/Plugin Funcs or Dynamic payloads
whose equality is unproven do not become Same because a compiler producer wraps
them. Add controls for equal captures, changed Ref supplement/form/span, changed
native namespace, closure Arc aliases and deliberately Unproven capture. If the
existing observer cannot certify a retained contribution, report its existing
verification/nonconvergence diagnostic; do not suppress it or introduce a result
cache. Any further observational support needs its own owner specification and
RED→GREEN evidence. The transport classifiers prove transport identity only; they do
not certify arbitrary user capture equality.

## Ordering, stabilization and diagnostics

Forward and backward references use the same provisional-index dependency.
An absent provisional target contributes no speculative nested effects; retry
under the existing contextual stabilization protocol. A retained stable missing
or unnumbered target reports the existing reference error at the source Ref span.
First-label selection and bibliography precedence must match current reference
resolution. On each retained iteration: build current actual action index;
resolve reference producers with that immutable snapshot; realize selected
children; rebuild actual action index; validate full typed counter demands; then
publish only that iteration's content, introspector and diagnostic contribution.
Discard validation-only diagnostics/effects under the existing transaction rules;
never consume retained diagnostics via shared result caches. Cycles and newly
produced references obey existing stabilization/depth bounds and report an error
rather than succeeding with partial nested rendering. Restore styles, rules,
context/current Location and defer flags on all errors. No callback-once promise
is introduced across discarded stabilization iterations.

## Owners and gate

New L0 `core/compiler/realize/reference_supplements.md` owns the producer and all three
additive doc-hidden transport functions. Update current owners of realize/mod (export),
recipes (actual Ref admission), figure (original template boundary), and
introspect/walk (original root-only template plus actual prepared-Ref traversal
and internal producer registration without semantic source membership), and
introspect/populate/extract_payload/locatable/labelled only where the existing generic
registration must delegate this classifier before payload publication; context occurrence/insertion
coordination only if its existing arena API cannot express the described stable
producer; introspect/walk, figure_demands and layout/references own their actual
prepared-Ref branches. Update infra pipeline/context_stabilization and bundle L0
only for required ordering/wiring; preserve existing public signatures and target/features.
Also update `infra/pipeline.md` for collect/substitute selected-field traversal and
`infra/pipeline/context_stabilization.md` for ordered_blocks, producer admission
and retained diagnostics; associated independently owned pipeline tests specify
forward/multiple/nested producer controls. `core/compiler/eval/observation/*`
changes are not authorized unless an actual missing observable comparison is
measured and its owner updated without weakening proof. All affected owner hashes
must be captured before implementation. Ref entity and generic Content map
contracts stay unchanged. The only authorized public additions requested are the
three exact functions above; any further public carrier/signature requires a
separate review before implementation.

## Acceptance before closure

Reinstate the archived Ref regression as an executable test after approval and
observe RED first. Run both explicit and callback supplements with forward and
backward references, label-on-Ref target/morphology, repeated raw ContextBlock
templates with fresh producer IDs versus prepared Located aliases, two uses plus
caption, show ref:none/replacing/identity
alias rules and hidden-parent Ref controls, inner panic/Metadata/typed Table
counter reads, no-caption with no reference, unused and hidden parents, own-hidden
outer, explicit override/empty supplement, first duplicate label, unnumbered and
missing target, bibliography and page controls, nested references and cycle
termination. Require source error origin, retained warnings, query membership,
distinct stable locations and actual Paged rich render in addition to typed
counter values. Preserve generic terminal Ref-map and original-field snapshots,
all frozen Figure/Grid acceptance, user-visible query membership (including
ContextBlock queries: generated producer transport must not become a new semantic
source element), raw nonFigure references, both-target explicit reads, complete lineage,
paths, V15/V26 and zero new lint violations. No ignored failure, suppression,
neutral leaf, fake pre-indexing or changed expected result counts as passing.
