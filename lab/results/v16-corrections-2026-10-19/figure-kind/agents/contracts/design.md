# Figure kind: proposed bounded contract

Status: ready for integration into the review package; not approved, not active L0,
and not authorization to write productive code. The integrated package must bind
this text, the full owner manifest, the counter design, and the frozen acceptance
cases. `../counter/design.md` supplies the final counter API and lifecycle; its
additive bundle-target facade is separately listed in the integrated public gate. This document revises the old FG1 candidate; it does not retroactively
approve that candidate.

## Evidence before decisions

At baseline `d4e72102df01f9c248dffac761662d0f3cbf699e`,
`compiler/stdlib/figure_image.rs:60–70` treats only Image/Table/Raw/Sequence as
inferable and silently accepts every other Content. Lines 129–145 discard valid
element-function identity; `entities/elements/figure.rs:25` can store only a
String. Its L0 explicitly leaves typed kind, semantic recursion, and synthesis
open. `compiler/realize/recipes.rs:410–444` selects rules before
`compiler/eval/rules/apply.rs:600–624` folds matched show-set styles. One
constructor-time decision cannot reproduce occurrence-dependent styles.

The independent audit's `../audit/boundaries.json`, `supplement-hidden.json`, and
`source-verification.json` bind current product inputs and the fixed oracle
`bd5ebec285505889e8ff1d4f68804ab147a699f3` (not a moving version). Typed functions,
omitted/auto/explicit kinds, scoped reuse, Metadata/Array traversal, opaque
Dictionaries/Context/later show products, final show-set kind, custom supplements,
and supplement/counter effects are measured language behavior. The relevant
upstream docs state typed kinds and kind-specific counters. The precise Rust
carrier, cache, or callback invocation count is not a language requirement.

The existing Func `PartialEq`/Hash at `entities/func.rs:411–450` uses names for
native functions. Deriving FigureKind identity from it would admit homonymous
functions and reject aliases with a different diagnostic name. Existing selector
function identity at `compiler/eval/operators/equality.rs:205–235` uses typed
function pointers and Format discriminants instead. This is a concrete reason to
specify FigureKind equality separately, not to rewrite global Func equality.

`entities/content/traversal.rs:11–342` is an active visual visitor: Metadata and
Ref are opaque, TableCell visits visual(), Outline visits realized_body, and
Realized visits visual. It cannot implement the semantic field traversal below.
The new traversal remains feature-owned and static; the generic visitor is unchanged.

## Exact public entity changes for approval

In the existing `entities/elements/figure` owner, add:

```rust
#[derive(Debug, Clone)]
pub enum FigureKind {
    Element(crate::entities::func::Func),
    Name(ecow::EcoString),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FigureSupplement {
    Auto,
    None,
    Content(crate::entities::content::Content),
    Func {
        function: crate::entities::func::Func,
        span: crate::entities::span::Span,
    },
}
```

FigureKind receives manual PartialEq/Eq/Hash, preserving nominal Element versus
Name. Within Element, Native compares typed native call pointers; Format compares
its FormatElementKind and typed pointer; NativeWithEngine compares its distinct
ABI typed pointer. Dynamic Element functions compare the constructor Arc identity,
not the registered name. Closures, Plugin and With are invalid language kind
inputs; synthetic invalid Element values remain nominally distinguished by their
Func representation/Arc identity and are never executed by identity. Hash follows
those exact equality partitions. Do not alter global Func PartialEq/Hash or use
Debug, display names, a hash, or pointer ordering as semantic identity.

Change FigureElem's fields to:

```rust
pub body: Content,
pub caption: Option<Content>,
pub kind: Option<Option<FigureKind>>,
pub resolved_kind: Option<FigureKind>,
pub supplement: Option<FigureSupplement>,
pub resolved_supplement: Option<Option<Content>>,
```

Outer kind None is lexical omission; Some(None) is explicit auto; Some(Some(k))
is explicit kind. Resolved fields describe one prepared occurrence only. Resolved
supplement distinguishes unprepared from prepared explicit-none. No numbering
field is restored. Additions are public Rust API changes under ADR-0127 §2.1/2.4.
The phase contract below independently requires §2.3 approval.

Keep Content::figure(body, caption, kind: Option<String>, numbering) unchanged as
a compatibility adapter: Some(name) becomes explicit Name, None is omission,
all resolved fields and lexical supplement begin None; numbering transport stays
in existing Styles. This preserves the Rust signature but intentionally changes
the meaning of legacy Some("table")/Some("image") from implicit element-like
strings to custom names. That semantic compatibility migration is part of the
approval, not a claim of unchanged behavior. Audit all callers: productive native
construction becomes typed; materialize_time reconstructs the full entity instead
of calling this lossy adapter; tests that intend actual table/image element
identity must construct that typed intent. Tests of custom strings remain Name.
The language constructor builds the typed entity directly.
Every Figure reconstruction copies lexical presence and resolved fields. Both
maps recurse body/caption and the selected semantic supplement Content once: if
resolved_supplement is Some, transform only its Content and preserve the lexical
supplement as origin; otherwise transform lexical FigureSupplement::Content.
Preserve None/Auto/Func, all spans and map order; never invoke the transformer
twice on lexical/resolved aliases or execute supplement callbacks, and never
convert omitted caption to explicit None. Prepared alias
metadata is copied, not regenerated. Caption position/separator projection remains
the existing incomplete contract; these maps do not synthesize missing fields. ObservationEq conservatively includes every causal
field; it is separate from language equality and cannot trust Func names.

Change `ElementPayload::Figure.kind` to `Option<FigureKind>` and add
`supplement: Option<Option<Content>>` to that variant. It carries the occurrence's
resolved values, with None only for documented synthetic unprepared input. No
new Introspector public signature is required: the counter proposal retains the
legacy string adapter and uses existing typed CounterKey + Location readers for
productive captions/references. Existing synthetic payload fixtures must migrate.

## Lexical construction and style transport

Preserve body/caption/kind validation order and causal Args occurrences. Existing
native element recognition (`native_element_fields` typed function-pointer
registry, or an actual FuncRepr::Element) validates Func kinds without executing
them. A registered name by itself is never proof. Type/Symbol/With/error controls
remain. Kind:Auto no longer loses its explicit presence. An omitted kind is not
inferred in the constructor.

After these existing arguments, validate supplement and then numbering in
upstream declaration order; freeze mixed-invalid causal diagnostics before code.
Validate supplement as Auto, None, Content,
LocatedContent, Str/Symbol through the existing Content cast, or Func. Retain
function source span. Preserve unknown/excess argument order; tests pin casts,
last named occurrence and detached fallback. Figure set and show-set transport
`figure.kind` and `figure.supplement` in the existing lexical StyleChain, without
storing an entity in Styles or reevaluating expressions. Explicit constructor
kind/supplement override inherited settings; explicit auto overrides a set kind.

The constructor's explicit numbering uses the existing Styles transport (Str,
None, Func) rather than restoring baked entity state. Existing set-numbering
behavior remains; any newly accepted function-numbering transport is identified
in the amendment. Count eligibility is non-None numbering, independent of caption.
Function numbering formatting remains a separate, measured debt unless included
by the counter/layout integration. A Func is enabled for counting/demand and its
invalid results/errors must be exercised by the accepted scope. Preserving an
existing missing visual prefix is recorded as debt; inventing a decimal fallback
and presenting it as oracle-equivalent is forbidden.

## Semantic body inspection

Add `compiler/realize/figure_kind.rs`, with sole matching L0 owner. It owns the
closed Content and Value decisions and exact element-kind identity helper.
`semantic-partition.json` enumerates every current variant and its policy. The
algorithm checks a node before its fields, then ordered semantic fields;
Content-valued fields and recursively nested Arrays recurse, Dictionaries and
other value kinds do not. First matching Table/Raw/Image wins for auto; default
is the actual image element function. For supplement callbacks with explicit
Element kind, the same ordered visitor finds the first matching element of that
exact identity, including heading; custom Name always receives the original body.
No Context/callback/recipe evaluation, layout, or rendering occurs in inspection.

For explicit-kind supplement target search, inferable Table is not an unconditional
terminal: a nonmatching Table root descends its lexical children, without using the
resolved visual plan. The two `../audit/table-target.json` ordinary cell cases
are required, including first-cell precedence. Native Table currently extracts
and can merge separated headers; original mixed header/cell order for an
explicit non-Table supplement target is not recoverable. That exact complex
supplement-target ordering remains debt; auto inference is unaffected because
it stops at the Table root. This package adds no Table lexical carrier and makes
no complete semantic-field traversal claim for that supplemental path.
Image and Raw have no Content-valued children after a
nonmatching root. StateUpdate.update is internal and opaque, including Set(Content);
only Metadata public Value/Array descends (see audit/state-update-fields.json).

The catalogue includes semantic content behind Ref.supplement, Metadata.value,
Figure body/caption and explicit supplement content, Quote attribution,
Bibliography title, FootnoteEntry separator/note, and public math rows/slots.
It excludes generated Outline body/prefix and prepared TableCell visual copies;
source/lexical data is inspected. A returned/retained Realized carrier follows its
original semantic contribution, not a later show-created Table. Carrier metadata,
style dictionaries, snapshots, closure captures, and resources are not fields.

Dynamic extension content has no field enumeration in the current public
Element/DynElement contract. Its explicit policy remains opaque, with no callback
or dyn_map_content invocation, and is an identified extension limitation rather
than a claim it has no children. This does not certify automatic kind inference
inside arbitrary external extensions. A future extension enumeration contract is
a separate public gate; no fake neutral-container claim or global warning
suppression is permitted. Tests must include a Dynamic container with a Table
and assert preserved opacity/zero callback effects as a bounded compatibility
control, not as vanilla parity.

## Selection preview and final occurrence preparation

Add private feature coordinator `compiler/realize/figure.rs`, sole matching L0.
After installing the effective lexical occurrence frame and before root selector
matching, produce an immutable kind-only preview for a reached Figure. Resolve
lexical explicit kind first, explicit auto by inspection, omission through the
current `figure.kind` then auto. Preview has no supplement/counter callback,
Location allocation, registry write, or child realization. Use it consistently
in `has_root_effect`, root rule filtering and show-set eligibility. Other element
selection remains unchanged.

Accumulate matched show-set styles using current precedence against that same
preview. Do not rematch against a partially updated kind: table→raw show-set
must select from table while final callback sees raw. Then prepare a distinct
occurrence with the complete effective styles, typed kind and supplement before
invoking its selected function/Content replacement. In `apply::realize_occurrence`,
retain the preview for selection, but pass the final prepared Located node to the
callback and store it as Realized.original. At the no-rule Figure path, perform
that same final preparation before descending into emitted children and before
query/indexing. Preserve existing Location/RuleId guards, source spans, recipe
restoration, depth and alias behavior. A returned prepared it retains the snapshot;
a fresh occurrence from reusable raw input resolves independently.

Only reached roots prepare: an ancestor replacement that hides a Figure prevents
its supplement/child effects. A Figure's own replacement happens after its final
supplement resolution; no-caption and show figure:none still propagate supplement
errors. This order is specifically approved; it is not constructor-time baking.

Final supplement: Auto for known localizable element identities uses the existing
locale table after exact function classification (never Name text); custom Name
or other nonlocalizable kind with enabled numbering errors `please specify the
figure's supplement`. With numbering None it resolves to known empty content.
Explicit None remains no supplement; explicit Content remains its content; Func
is called with the first exact-kind semantic descendant or original body, using
a style-known context with no Location (not the caller Location and not counter
Context::none), with original argument span/traces. Convert the callback result
through the existing value-to-display-content operation, not the strict Content
argument cast: None displays empty, numeric/displayable values retain display
semantics, and existing rich-display limitations remain visible. Retained evaluation
and isolated validation follow existing diagnostic ownership. No cross-transaction
cache or universal callback-once assertion is introduced.

The pinned constructor-validation receipt fixes invalid precedence
body→caption→kind→supplement→numbering→unknown/excess arguments, independent of
named source order. Invalid supplement reports `expected content, function, none,
or auto, found {type}`; invalid numbering reports `expected string, function, or
none, found {type}`. Preserve causal last-value spans.

Kind is the sole newly certified selector-preview synthesized field. Final getter,
fields/query/show projections expose resolved kind and supplement; raw explicit
kind/auto/supplement remains lexical, omitted raw fields stay unknown. Preserve
known-empty field snapshots. Supplement-dependent where selection, complete
caption-synthesized fields, and other unrelated Figure fields remain explicitly
uncertified; do not claim complete upstream synthesis from this package.

## Counter integration, scope and closure

Use the counter agent's independent demand-snapshot contract as the owner of
count history and retained effects. No string namespace maps element kind to a
custom name. Automatic eligibility is numbering-only, not caption. Caption/ref
are demand; own show-hidden originals can still be counted by explicit reads.
Every demand folds the complete relevant history before projecting a Location;
manual events are not Figure ordinals. No shared result cache may consume a
validation transaction's diagnostic contribution on behalf of retained execution.

Close V16-055 only with real functional RED→GREEN, exhaustive Content/Value and
FigureKind extension probes at the owned decision, the frozen current-pin
kind/set/show-set/Metadata/Array/Dictionary/custom supplement/counter cases,
parent-hidden versus own-hidden effects, aliases/homonyms, field snapshots,
clone/map/observation, actual Paged caption/reference text, and both-target
kind/reflection/explicit counter reads. Implicit HTML caption/reference rendering
remains the counter package's explicit debt; do not imply both-target rendering
parity from these tests. Revalidate
all twelve established gates, strict and reverse lineage, paths, V15/V26 and
full differential lint. Existing V19 notes remain visible; runtime group execution
is not every-alternative coverage. Record source state, UTC, argv and binary
identity for every decision measurement.

This is a bounded typed-kind/semantic-inspection/occurrence/counter repair, not
full Figure parity. Caption omission versus explicit none, caption position and
separator/default locale transport, caption.figure_location, every callback
number-format rendering, supplement-dependent preview selection, and general
Dynamic extension field enumeration remain separate visible obligations. The
old image loading and decoding contract sharing figure_image.rs is unchanged.

## Additional causal Grid order carrier

`../audit/grid-order.json` refutes deriving semantic order from Grid's separated
cells/header/footer storage: header(Table) before Raw infers table, after Raw
infers raw. The accepted header-order cases directly expose lost inference order and are
required closure cases. Footer-before-body validation is independently classified
by `../audit/grid-footer-validation.json`: direct invalid Grid errors in both,
show-hidden Figure succeeds in both, and raw fields reveal an existing validation
phase debt. The old missing-kind error masks which error would follow; do not
claim a newly observed footer error from that fixture. Preserve its oracle result
as visible residual evidence, not an all-green requirement for this bounded fix.

Add to `entities/elements/grid` the exact public metadata:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridChildOrder {
    Cell(usize),
    Header,
    Footer,
    HLine(usize),
    VLine(usize),
}
// Added to GridElem:
pub source_order: Option<Vec<GridChildOrder>>,
```

The language constructor records Some in original positional order while doing
its existing extraction. Recognize existing Source wrappers for header/footer/line
classification without losing their child or source provenance; this corrects
classification of transparent transport, not invalid-layout acceptance. Each accepted body child appears exactly once; indices
are in bounds; Header/Footer descriptors occur once iff that field exists.
Current duplicate-header/footer restrictions prevent overwritten accepted fields
and stay as separate existing Grid limits. No Content snapshot is added. Existing
Grid maps transform each actual child exactly once in their current contracted
map order and copy descriptors; temporal/placement reconstructions and observation
preserve descriptors. None is the explicit legacy synthetic input contract with
canonical cells/header/footer order, not recovered source order. A malformed Some
is a rejected invariant, never an ignored descriptor or silently repaired list.
The private semantic resolver returns SourceResult and validates the entire order
before visiting children: every index is in bounds, every cell/line index occurs
exactly once, and Header/Footer occurs exactly once iff present. Missing, repeated,
out-of-range or absent-field descriptors produce `invalid grid source order` at
the Grid Source span (detached only for originless synthetic Rust input). Both
kind preview/final preparation and explicit supplement-target search propagate
that error; no partial-kind success hides invalid metadata. Productive native
constructors generate valid Some, while None alone selects the explicit legacy
synthetic order. No user-created hash or length coincidence certifies validity.

Do not change Grid validation phase, rendering, duplicate-header/footer limits
or existing emitted-invalid diagnostics. Only source-order metadata is added;
the new public Grid contract is included in ADR-0127 §2.1/2.4 approval. The final
phase gate concerns Figure preparation, not a new Grid realizer.

Require accepted source-order header-before/after, valid footer order, preserved
emitted-invalid diagnostic, nested body, line descriptors and
source-order map/clone/observation tests. No duplicate child effects or layout
reordering is authorized. The source/owner manifest includes entity, constructor,
reconstruction, placement and every literal fixture.

## Required implementation acceptance inventory

The frozen independent oracle cases are acceptance inputs, not tests already run
against a candidate. Require all in-scope cases in `../audit/boundaries.json`,
`../audit/supplement-hidden.json`, `../audit/constructor-validation.json`,
`../audit/state-update-fields.json`, `../audit/table-target.json`, and the accepted
header-order cases of `../audit/grid-order.json`, plus counter/design.md's exact
location/demand controls. `../audit/selector-supplement.json` is preserved as
explicit supplement-preview debt: kind selection and explicit-constructor
precedence controls remain required, supplement-dependent selection remains open.
`../audit/grid-footer-validation.json` retains the separate validation-phase
limits, not a mandate to move Grid validation.

Before code, generate and freeze the candidate case manifest classifying each
case required-change, required-preservation, or named prior-debt against the
current baseline. Do not revise a required result after a failed candidate. Add
local tests for raw omitted/auto/function/name; exact alias versus homonym
identity; ordered semantic wrappers, all current Value classes and known-empty
LocatedContent snapshots; every metadata carrier; map count/order and prepared
alias conservation; actual paged caption/reference text from location snapshots;
HTML preservation without implicit paged demand. Run the established full
workspace, wiring, doc, all-target, Clippy/rustdoc/fmt, CI/path/strict+reverse
lineage gates. The independent test author must observe pre-code semantic RED,
then GREEN from candidate sources with immutable expectations.
