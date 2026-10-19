# FG1 typed Figure kind and occurrence preparation — candidate decision

Status: **draft under owner-graph review; no approval and no implementation**.
This file is a diagnostic proposal, not an active L0 owner. V16-055 stays open.

The measured baseline is HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`.
`../agents/geometry/baseline-query/receipt.json` records ten source queries whose
show callback reads `it.kind`: the product fails with an unknown field; the pinned
oracle returns a function-valued table/raw/image kind through wrappers. The
separate baseline eval receipt distinguishes raw unknown kind, explicit function,
custom string and auto. Oracle commit is
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Upstream model/figure.rs:345–363 resolves kind before show; content/mod.rs:408–464
queries semantic content-valued fields and arrays. This supplies language
observations; its dynamic Rust representation is not a Tekt requirement.

## Concrete proposed public contract

Keep `Content::Figure(Arc<FigureElem>)`, without a new Content variant. Declare the
following closed type in the existing Figure entity owner:

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FigureKind {
    Element(crate::entities::func::Func),
    Name(ecow::EcoString),
}
```

`Element` retains an already validated element function identity. It never stores
a callback obtained by calling a constructor, a user closure, or a function name
as a substitute for identity. `Name("table")` differs from `Element(table)`.

Change `FigureElem.kind: Option<String>` to
`kind: Option<Option<FigureKind>>`: outer None is lexically omitted, Some(None)
is explicit auto, Some(Some(kind)) is an explicit function/name. Add
`resolved_kind: Option<FigureKind>` to the immutable entity, initially None.
Only an occurrence-prepared clone receives Some; the reusable raw constructor
and already captured occurrences are not mutated. This is semantic preparation,
not render-style baking. The existing body and caption fields retain their types.

Retain `Content::figure(body, caption, kind: Option<String>, numbering)` as a Rust
compatibility adapter: Some(name) becomes an explicit Name; None is omitted;
resolved_kind starts None. The language constructor builds the typed entity
using the same existing argument validation/causal spans and no premature body
inference. No new public constructor is necessary.

Change `ElementPayload::Figure.kind` to `Option<FigureKind>`. Producers publish
the prepared kind, or preserve absence for explicitly synthetic unprepared data;
consumers must not conflate an arbitrary custom name with an element identity.
Change the existing public Introspector method to:

```rust
fn figure_number_at_index(&self, kind: &FigureKind, idx: usize) -> Option<usize>;
```

All implementors, including L3 counting instrumentation and test implementations,
migrate in the same materialization. This is an explicit Rust API compatibility
change requiring ADR-0127 approval. Other Introspector signatures stay unchanged.

Use the **existing** CounterKey::Selector and Selector::Element vocabulary for
per-kind counter identity: the native figure function plus ordered `kind` filter
containing Value::Func or Value::Str. Do not add another CounterKey variant, use a
hash as identity, or keep automatic counters in the user-string `figure:...`
namespace. A compiler-owned static helper constructs this key; no compiler
imports enter entities. Ordinary manual `counter("figure:table")` remains a user
counter. Layout's private progress state uses the same typed key, and checkpoints
copy/restore it exactly.

## Proposed phase and owner boundaries

Add one private form-B unit `compiler/realize/figure_kind.rs`, with canonical owner
`00_nucleo/prompts/core/compiler/realize/figure_kind.md`. It owns pure semantic
body inspection and per-occurrence kind preparation. The realization hub only
registers it. The recipes coordinator invokes it before eligibility/where-filter
selection for an emitted Figure root, after installing the effective lexical
frame and applying relevant show-set style semantics. Preparation happens even
without a matching function show rule so subsequent query/index and layout agree.
This is an explicit phase decision replacing constructor-time inference; approval
must name it. No source reevaluation or layout callback is permitted.

Preserve pre-order causality: a parent recipe may hide the Figure, so do not
prewalk the entire document to prepare hidden descendants. Inspect body data
without executing body/caption/Context callbacks. The actual traversal partition
must cover every current Content variant explicitly and distinguish source/
occurrence transport, semantic content fields, arrays and opaque callback-bearing
values. Do not reuse map_content in a way that changes data or descends through
unrelated referenced records. First inferable semantic descendant wins; default
is the actual image element identity. The remaining dynamic/extension and
contextual-content frontier must be covered by negative/reference probes before
promotion, not declared neutral for lint.

Explicit lexical kind wins; explicit auto requests inference; omission inherits
an applicable set/show-set kind before default inference. `rules/set.rs` currently
only handles figure.numbering: typed kind set validation/transport must be added
under its owner without silently evaluating ignored unrelated arguments or
changing numbering precedence. Source/Located wrappers, recipe frames and
existing occurrence identity are preserved. Reusing prepared `it` keeps its kind;
a fresh insertion of the raw value resolves in its own effective frame.

Entity getters expose explicit raw kind where known; omitted raw kind retains
unknown-field behavior until preparation, and explicit auto remains observable
as auto before preparation. Prepared getter/fields/JSON/YAML/query/show values use
the resolved typed kind. Extend strict known Figure fields by `kind`; preserve
Some snapshots as authoritative, including empty snapshots. No extra fields store
or encoder-specific synthesis. Language equality/morphology and internal
ObservationEq must be specified separately: raw lexical presence and prepared
kind are not silently excluded from reuse proof, while Rust derives are not the
language acceptance oracle.

Populate/query/selectors/counters/labelled/reference/layout consume the same
prepared kind and typed counter key. Existing counter gates, causal order,
Location allocation, caption count policy, numbering callbacks and page maps are
not implicitly redesigned. Numeric reference precedence and explicit supplement,
including empty, remain. Element kind may use its known existing locale mapping;
custom strings must not gain an element's supplement merely by matching its name.
Any required new supplement callback/runtime behavior is excluded and must be
measured in a separate contract; this package must report, not hide, its residual.

## Acceptance before closing V16-055

1. Raw omitted/auto/function/custom string retain their distinct kind observation;
   aliases preserve identity, homonymous functions and invalid callable values
   reject with existing causal precedence and no constructor execution.
2. Reproduce all ten failed baseline realized-query cases with typed values;
   add first-descendant ordering, Source/Styled/Strong/Emph/Hide/Box/Block/nested
   Figure, Table versus Raw and default image, plus an unrelated semantic-record
   target and callback-hidden target as discriminating negatives.
3. `figure.where(kind: table)` and kind:"table" distinguish source/show/query;
   bare/alias/refiltered selectors use semantic equality and authoritative
   snapshots, never function names. Include known-empty snapshot controls.
4. Function table and custom string "table" counters are independent, including
   manual `counter("figure:table")`, explicit updates, interleaving, references,
   labels, no-caption cases and numbering:none. No added/dropped automatic event.
5. Two insertions of one raw figure under different effective kind settings
   resolve independently. Returning/repeating `it` retains prepared identity and
   does not execute a rule twice. A parent hiding a child executes no child
   effects. Context-generated and recipe-generated figures follow the same phase.
6. Preserve retained body/caption morphology and text; test original clones,
   maps, equality, hash/reuse controls, source spans, styles and snapshot ordering.
   Render/reference tests must measure actual text/semantic kind and errors,
   not Rust carrier equality or output PDF bytes.
7. Tests fail semantically before materialization, then pass without expected
   value relaxation. A disposable Content extension must fail at the new inference
   decision; no whitelist/config/suppression or new catchall may hide055.
8. Full workspace, both targets, all-target build, strict Clippy/rustdoc/fmt,
   paths, ownership/pins and strict/reverse lineage; preserve prior query/context/
   counter/Equation/Cell regressions and verify state with source hashes/UTC.

## Explicit remaining scope and gate

This proposal does not complete all FG1. Caption omission versus explicit none,
caption position/separator transport/auto presence, default localized separator,
supplement/numbering callback synthesis, figure_location and full caption-known
fields remain separate obligations. It does not certify all Figure rendering,
PDF accessibility, or dynamic callback traversal. Their current known limitations
remain visible in tests/reports; no sentinel/string/Empty stand-in is approved.

Approval is pending under ADR-0127 §2.1 (entity/payload/trait public changes),
§2.3 (pre-show semantic preparation phase) and §2.4 (trait/field compatibility).
The prior Context occurrence approval authorizes none of these changes.
Active L0s must be updated and pinned together before any implementation. This
candidate is not yet a permission question: the exact full owner graph review,
new inference traversal partition and supplement boundary probes must be finished
and recorded in the manifest before it is promoted as a final gate package.
