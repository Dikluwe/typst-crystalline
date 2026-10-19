# L0 — introspect
Hash do Código: 7d367f7e

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29
```

## Ownership and evidence

Layer: L1. Sole productive consumer: `01_core/src/compiler/introspect.rs`.
ADRs: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

The current bodies of `01_core/src/compiler/introspect.rs:287`, `:718`,
`:782`, `:828`, `:885`, `:922`, `:1376` and `:1897` establish the
concrete trait implementation, entrypoints, store population and causal walk.
They return `TagIntrospector`, not a separate counter-state object.
`03_infra/src/pipeline/context_stabilization.rs` uses the runtime entrypoints;
the full reevaluation helper is specified separately in
`00_nucleo/prompts/core/compiler/introspect/fixpoint.md`.

This owner retains the public `introspect` compatibility wrapper and
coordinates the static child units. The behavioral owners for bibliography
conversion, concrete queries, entrypoints, outline synthesis, time
materialization, store population, parent indexing, target classification and
causal traversal are respectively the individual Prompt L0 files under `00_nucleo/prompts/core/compiler/introspect/`.
Their extraction preserves the contracts below as shared context and does not
introduce a second implementation. The parent wrapper remains the sole
productive behavior of this owner. Child owners specify payload extraction
(`00_nucleo/prompts/core/compiler/introspect/extract_payload.md`), classification
(`00_nucleo/prompts/core/compiler/introspect/locatable.md`), heading projections
(`00_nucleo/prompts/core/compiler/introspect/heading.md`), label text
(`00_nucleo/prompts/core/compiler/introspect/labelled.md`), runtime postprocessing
(`00_nucleo/prompts/core/compiler/introspect/from_tags.md`), tag hashing
(`00_nucleo/prompts/core/compiler/introspect/convergence.md`) and the bounded helper
loop (`00_nucleo/prompts/core/compiler/introspect/fixpoint.md`).
Shared snapshot invariants belong to the pinned content-snapshot nucleus,
not to competing parent/child definitions.

## Entry contracts

`introspect(&Content) -> TagIntrospector` delegates to
`introspect_with_introspector(&Content) -> TagIntrospector`. These pure
entrypoints convert bibliography references to citations, create a fresh
Locator, tags, introspector, auto-label counter and default StyleChain,
walk once, synthesize outline entries and construct parent relations.
They do not evaluate deferred functions.

`introspect_with_runtime(content, engine, ctx) -> SourceResult<TagIntrospector>`
performs that preparation and one walk, then coordinates the child-owned
runtime passes: counter functions, fallible State runtime reads/displays, counter
displays, equation numbering, equation supplements, equation realization and
Heading finalization before queries. The separately ratified demand-driven State
reader contract supersedes automatic eager State replay; the explicit legacy
helper remains child-owned.
Fallible passes propagate diagnostics. This API processes supplied content;
it does not reevaluate the whole document or establish pipeline convergence.

`introspect_bundle_documents_with_starts(bodies: &[Content])` returns
`(TagIntrospector, Vec<LocatorSeed>)`.
`introspect_bundle_documents_with_runtime_and_starts(bodies, engine, ctx)`
returns `SourceResult<(TagIntrospector, Vec<LocatorSeed>)>`.
Both use one global Locator and introspector, collect bibliography keys across
all bodies and return one checkpoint taken before each body, in input order.
They perform global outline/parent preparation once. The runtime variant
coordinates the same passes with the child-owned Page-aware counter replay.

Before each bundle body, record Page = [1] at its next Location without
consuming the Locator or adding a Content/Tag occurrence. An empty body still
has a seed but consumes no Location. Local Page updates take precedence;
headings, other counter keys, labels, states and cross-document queries stay
global. Do not define multi-document `.final()` semantics here. Seeds must
come from the causal walk, never element-store sizes or approximate offsets;
a transformation that changes occurrence order invalidates their alignment.
These APIs execute no layout, context expansion or output writes.

## Walk and causal capture

Keep the closed Content dispatcher exhaustive and static. Walk mutates only
local analytical stores; no I/O, global mutable state, FontMetrics, visual
frames or Engine callback execution belongs here. Thread Locator, tags,
TagIntrospector, the local auto-label counter, optional language, StyleChain
and optional parent label through the traversal.

For a payload-producing occurrence, allocate its Location, capture effective
lexical properties, populate stores and record the content/snapshot, then emit
Start before descending. Emit End at the same Location with
`hash_content(content)` after descending. Derived Labelled, TOC and bookmark
records reuse the target Location and paired zero-hash tags; they do not create
a new real occurrence or overwrite parent relations. Build parents from real
occurrences, including NativeElement, not from those derived records.

Styled pushes its styles only for its body and forwards the causal label.
Sequence visits children in order without spreading a label to all children.
Transparent structural wrappers keep their established descendant traversal;
Repeat visits its body once, not once per physical repetition. Bibliography
entries are data; its title is traversed. ContextBlock and Dynamic are leaves
in this walk, not an invitation to execute a closure or invent child traversal.

SpaceFromNewline is cloned and classified like Space without normalizing its
origin; it creates no tag, Location, counter or state action. Mathematical
carriers are terminal in these time/reference/counter walkers, including
MathLr, MathMid, MathStretch, over/under wrappers, MathPrimes, MathUnderline,
MathVec and MathLimitsOverride. Do not repeat the separate structural Content
transformation traversal. Flush is terminal and has no introspection effect;
its float behavior belongs to `00_nucleo/prompts/core/compiler/layout/flush.md`.

At Start, Heading uses the child-owned complete modeled snapshot.
Strong captures ordered fields delta = 300, body, then the causal label if
present; Emph captures body then label. Each own Strong/Emph node, including
empty and repeated bodies, has its own occurrence and descends once.
Styled is not an extra occurrence. The fixed delta covers the modeled
producer only: new variable-delta producers require their own L0 and causal
capture; this contract does not silently accept them. Other initial entries
retain absent snapshots and their contracted projections; equation runtime
realization belongs to the postprocessing child. Never recover a snapshot
from body equality, a later caller's styles or synthetic TOC labels.

Time materialization freezes location-aware CounterDisplay text while keeping
semantic container structure and existing Boxed width_fraction data. It does
not query the counter anew at outline render time or reinterpret box fractions.

## Indexes, actions and labels

The effective chain is the source of numbering gates; payload fields freeze
their values for consumers that no longer have that chain.
Heading always enters its kind index but advances its hierarchical counter
only when heading.numbering is Bool(true), at its depth.
Equation advances only for block equations with Str/Func numbering; None or
absence disables it. Capture numbering specification, supplement and target
language, effective number-align (default end + horizon) and alt at the target.
Figure requires a caption and numbering not explicitly None, with the default
enabled; maintain its per-kind key (default image) and element-function key.
Table requires a caption and string numbering. Footnote advances its
element-function counter once per note, without resetting at column boundaries.
A user string counter named "footnote" is a different key.

Record automatic effective actions and manual full-key actions, including
Func, in the immutable counter event log at the same occurrence. NativeElement
and Citation automatic events use Step(1); auxiliary figure writes and derived
records are not duplicate automatic events. The child runtime resolves
Element-containing filtered keys; query and the pure walk never run Func.

Populate metadata, bibliography/citation order, state initialization, Set
updates and typed counter Step/Set snapshots during Start processing.
Func remains deferred. Numbered targets retain their label-to-counter mapping;
Heading records its key even when its numbering gate is false. Do not restore
the obsolete removal of numeric mappings to force the resolved-text fallback.

The exhaustive walk delegates its Heading and Label arms to static free
functions in their existing same-layer child modules. The parent allocates
real occurrence locations and brackets real Start/End; each child owns its
branch recursion and derived records under the same causal chain.

For an auto Label, walk its target first with the causal label, identify the
first newly emitted Start and attach supported resolved text to that same
Location. Capture chain.lang() at the wrapper, falling back to the explicit
language argument only when the chain has none. A target without a real Start
is classified for the non-referenceable diagnostic, not assigned an invented
identity. Explicit labels preserve known associations and classify unsupported
targets without a competing counter-state object.

Heading traversal freezes context-dependent title content at the heading's
Location before later counters can change it. Coordinate child heading helpers
and emit TOC/bookmark records according to independent outlined/is_bookmarked
gates; generate the sequential auto label regardless of those gates.
Outline title registration is synthetic Heading indexing only when the title
is default or present: no section counter and no self-entry in the TOC.

After the walk, synthesize one OutlineEntry for each eligible heading and each
heading-target outline within depth. Pair the recorded heading Location with
its snapshot; retain public level, LocatedContent element and fill, and private
prefix/source Location. No outlines means no synthesized entries.
Record outline.prefix-info maxima by key/level while traversing Styled,
without a new occurrence or text. Realized outline bodies can be traversed for
those observations; rendering/reevaluation is owned by
`00_nucleo/prompts/core/compiler/eval/mod.md` and the pipeline L0s, not a second query
implementation here.

## Concrete queries and positions

Element selectors use the language matcher in
`00_nucleo/prompts/core/compiler/eval/selector_matching.md`. Present snapshots are
authoritative; absent snapshots use only contracted projections. Do not derive
functional identity from NativeElement or reconstruct fields from caller styles.
If a selector tree contains Element, return unique Locations in document order,
including interleaved Or branches. Other trees retain existing order and
behavior: And intersects, Or deduplicates first appearances, Within tests
strict ancestors, and legacy Regex/Where query branches return empty.

Before/After filter the base result against the first boundary-selector result
using canonical Location order. Inclusive admits a matching boundary; no
boundary returns the whole base. They are implemented behavior, not a dormant
proposal. Query is read-only and runs no constructor, update, callback or layout.

Trait projections read the existing stores: paginated position_of delegates to
SealedPositions; html_position_of reads the independent generation-local HTML
map. Missing data returns None. Never infer HTML islands, convert paginated
positions, or replace either view with an intrinsic logical page.
Page supplement/raw numbering and realized visible/reference views are read-only
store projections; their injection and realization belong to
`00_nucleo/prompts/core/entities/page_supplement.md` and pipeline owners.

## Acceptance and boundaries

Verify independent calls, forward/backward labels, lexical numbering changes,
numbered/un-numbered headings, independent TOC/bookmark inclusion, causal
snapshots surviving reuse/style changes, Strong/Emph identity/order/parents,
filtered keys with manual actions, and query boundaries.
Check bundle pure/runtime seeds and Page resets, including empty bodies and
Page functions, without altering other keys. Check independent HTML/paginated
position stores and newline-space/math terminal behavior.

Language semantics, syntax and morphology are acceptance observables
(ADR-0107); tag/hash mechanics are not universal parity proofs. No new public
API, default, phase change or compatibility decision is authorized by this
sanitization. Audited limitations and unresolved classifier/extractor alignment
are recorded in
`lab/results/construction/diagnostics/sanitization-introspection-columns-history.md`.


## Single-evaluation identity reservation boundary

The internal walk identity-reservation service is accessible to layout, while
algorithm ownership remains in
`00_nucleo/prompts/core/compiler/introspect/walk.md`.

## State import evidence and current facade boundary

state-green.log in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
records an unused StateUpdate import after populate delegates actions to the
registry. Remove only that import from this hub. from_tags retains its own
import and public legacy explicit-application helper. The automatic read phase
was separately ratified in
`lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`,
SHA-256 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`.
Move no child logic and expand no hub contract.

## Ratified Heading finalization before queries

### Evidence and approval

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
`final-freeze.json.gz` and `target-controls.json.gz` in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
record UTC/diffstat/sources/argv. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`.
`heading-raw-constructor-func` passes on baseline/vanilla and fails on the
candidate because transport `Styled` changes `func`. Query also observes a
temporary causal wrapper; HTML ignores a panic callback. Sources:
`walk.rs:930–945`, `pipeline/paged.rs:419–427`, `pipeline/html.rs:101–106`
prepare queries through the pure walk. These are language values/errors, not a
requirement to copy vanilla's structure or algorithm. The owner confirmed A1-H2
on 2026-10-17 in
`lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`,
SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

### Exclusive facade and acceptance

Reexport entrypoints' new realize_heading_supplements facade with its exact
ratified A1-H2 public contract. No realization logic belongs in this hub and no
new trait is authorized.

Require H1–H8 in the ratified review, with bilateral RED before code. No public
contract beyond the Heading facade is authorized by H2; A1-S2 separately covers
five State helpers. No new public fields. Ordinary `Styled` and other callbacks
retain their boundaries. Require workspace/doctests/build/Clippy/Rustdoc, fmt,
V1/V5/V15/V26/reverse lineage and no V16 increase; use
12GiB/High 8 GiB/swap0/jobs1 with heavy commands sequential. RAM gain requires
measurement, not intention.
