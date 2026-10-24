# L0 — compiler/realize/page_running
Hash do Código: 8a5f6c46

**Layer:** L1.
**Exclusive productive consumer:** `01_core/src/compiler/realize/page_running.rs`.
**Governance:** ADR-0026/R1, ADR-0029, ADR-0030, ADR-0107, ADR-0108,
ADR-0109, ADR-0127, ADR-0128, ADR-0129, ADR-0130 and ADR-0131.


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42


## Measurement and exact ratification

At the implementation baseline, `compiler/layout/page_running.rs:89–150` draws
explicit raw running Content at finite page-slot subframes, after source-derived
semantic indexing. `compiler/realize/recipes.rs:609–660` already realizes captured
recipes with borrowed environment restoration; `realize/context_occurrences.rs:53–78`
installs a retained frame and allocation domain, retaining slots even on error.
`compiler/eval/rules/textual.rs:46–82` retains persistent lexical style snapshots.
These are measured owner boundaries, not a mandate to copy upstream execution.

The frozen bilateral `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json`
compares source-bound executables against vanilla commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`: raw SetPage headers/footers
render but their Figure/Context contributions are absent from productive
semantic stores; overwritten headers remain inert. Native PageRun already
prepares raw static fields before knowing physical repetitions, so repeating
that prepared raw object would incorrectly turn independent insertions into
aliases. These are language effects, morphology, order and Location/page
observations, not a requirement to copy upstream mechanics.

The separate `lab/results/set-page-header-2026-10-21/ambient-baseline/receipt.json`
SHA-256 6291efca219a3558ad2192f99f1208d64ad3e70b38ec96db2d417527549fcb05
refutes freezing creation-time 10pt as insertion styles: the measured header
uses page-effective 20pt, then inherited page-effective 30pt on the next page.
Explicit inner header18pt/footer12pt remain binding preservation controls.

The approved review is `lab/results/set-page-header-2026-10-21/review.md`,
SHA-256 cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039.
The direct human decision is preserved in
`lab/results/set-page-header-implementation-2026-10-21/approval.json`.
It authorizes only the frozen five public facades, opaque carriers and physical
page phase; it does not authorize additional public contracts or generic
observer/traversal changes. The implementation baseline is HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`. Exact owning source/L0 bytes,
SHA-256, UTC and working-tree state before these amendments are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/before.json`.

## Exclusive public boundary

Expose exactly these two pure functions through public module
`compiler::realize::page_running`, using the existing approved opaque layout
request/resolution types and existing contextual types:

```rust
pub fn realize_page_running(
    request: &PageRunningRequest,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<PageRunningResolution>;

pub fn bind_page_running_content(
    resolution: &PageRunningResolution,
    expanded: Content,
    ctx: &mut EvalContext,
) -> SourceResult<PageRunningResolution>;
```

`compiler/layout/page_running_requests` owns private authority, capture and
geometry; `compiler/introspect/page_running` owns complete source plans and
source-derived stores. This consumer does not redefine their fields, enum
variants or public constructors. Internal cooperation uses parent-descendant
visibility; no additional public setter/getter/type or generic observer bridge.
L1 remains pure: no filesystem, environment, clock, network, global mutable state,
layout Engine callback or independently constructed World.

## Capture without admission

Own bounded pure helpers shared with recipe/hub/Table coordination for four
operational slots: Background, Header, Footer and Foreground. A reached SetPage
captures raw Content into existing immutable ShowRecipe data (`textual=false`)
after parent/root admission; a PageRun captures those fields and realizes only
its lexical body before physical requests. Retain omission/Auto/None/explicit
Empty distinctions and every original field. Existing captured recipes and
prepared Located aliases retain their original snapshots. Nearest proven Source,
rules, guards, file and Context-ID floor remain lexical data.

Do not execute any descendant for capture, including a Figure kind preview or
supplement, Table preparation, Context closure, counter/state action or arbitrary
Func. The generic entity mapper stays unchanged. Feature-owned reserve-only
inspection must observe IDs in dormant templates before fresh allocation without
index membership, query hits, automatic actions or diagnostics from unexecuted
children. Overridden/unemitted templates receive no physical request/effect.

## Requested physical insertion

Realize only an actual opaque request admitted by pure layout for its selected
physical slot. Require an existing shared ContextOccurrenceArena retained frame
installed in ctx; compare its actual private allocation-domain capability to the
request source plan and retained contribution plan before callbacks, reuse or
allocation. Temporary legacy template slots are not a physical frame. Missing
frame/invalid ownership is a sourced fallible invariant
error, never a private fallback Locator or a second arena. Use the supplied Engine,
resources, target/features, reads, Sink and existing recipe pipeline. Raw repeated
insertions allocate actual descendants through retained arena slots; existing
Located aliases keep their identity. Raw Context IDs are fresh by actual insertion
and stable by retained replay. Do not derive identity from page/slot, coordinates,
text, equality, hashes or span.

The private resolution root is protocol authority, never a synthetic Located,
Context/Figure element, Location or public language field. Figure supplement
callbacks retain known styles without a Location; actual Context producers and
prepared aliases keep only their real Locations. Deferred ContextBlocks cross
through the existing L3 Session and its retained read/error/warning transactions;
this function does not execute them eagerly or fabricate a read to select Session.

An actual request may carry a previously retained resolution only after the
layout protocol proves the same immutable source/template authority, effective
environment path, physical key and finite geometry. Validate its contribution
ownership and exact request correspondence, then return the retained resolution
without executing recipes or allocating identities. Publish the exact retained
realization diagnostic contribution into the current candidate Sink through its
existing warning policy, preserving severity, origins, hints, traces and causal
order. An empty Context producer list does not prove that realization was silent.
Actual L1 execution uses a separate existing Sink transaction and forwards all
its diagnostics even on failure; a successful resolution retains that exact
contribution for proven reuse. Session producer diagnostics remain independently
owned, without new deduplication, filtering or key/emptiness-based ownership. Return
its retained unexpanded contribution view so the existing Session sees the same
actual raw producer Arc objects and can validate retained producer records; do not
feed the already substituted bound tree back as new source. This is existing transcript reuse under opaque provenance, not a value/effect cache.
Missing/stale/obsolete requests are not reused and do not remain in the new store.


Capture and insertion environment are different authorities. Retain immutable
lexical template/rules/guards/file/floors/source independently from the actual
page-effective styles carried by the request. Rebase only the operational ambient
capture root onto that page's effective insertion environment; preserve explicit
inner Styled18/12pt, genuine local captured recipe deltas and prepared alias
snapshots. Refuted creation10pt must not override page-effective20/30pt; do not
collapse raw style chains or choose arbitrary last-body styles. Restore every
borrowed Engine/context value on success and error. Existing callback capability
guards remain: known style does not manufacture a current Location or authorize
modern counter final/get, here(), position or page reads without actual context.

## Session binding and provenance

`bind_page_running_content` accepts the actual retained expanded contribution
from the existing Session. Retain the immutable private request authority,
validate its retained root/source ownership, then rebuild the complete typed
source plan through the approved `prepare_page_running_body` facade under the
same installed shared-arena plan frame. Any new unlocated descendant uses that
allocation domain. Preserve the input resolution and its initial unexpanded contribution, returning
a new bound value. Retained replay never overwrites that initial causal input.

Reject wrong/missing/conflicting retained root ownership. Sealing/layout owners
reject stale/mixed body-plan, capture or page-effective environment authorities,
including changes with the same physical key/geometry. Content alone, pointer
comparisons of unrelated values, Debug/hash equality, generic Func/Value equality
or matching final coordinates cannot certify a candidate or arbitrary execution.
Private Arc identity may establish the exact immutable protocol authority only;
the existing Session remains responsible for actual producer execution,
substitution and strict read replay. No cache, synthetic closure proof or unknown
public mutation API is authorized.

## Acceptance and limits

L0 precedes genuine semantic RED→GREEN and exact approved API/phase verification.
Require actual all-four-slot order, fresh raw repetitions, prepared alias reuse,
None/Auto/omission/Empty, overridden/parent-hidden no effects, PageRun lexical body,
source/rule/context floors, retained retry identity, captured local rules,
page-effective20/30 and inner18/12. Require actual Figure numbering/labels/query,
Context/state/counter location and page values, nearest errors, retained warnings,
forward/repeated references, mixed target/global Bundle order and no physical
admission for HTML. Existing attempt budgets, Route limits and diagnostic Sink
ownership stay with their established owners.

Require meaningful malformed/missing frame/provenance tests and restoration on
errors, source-current full gates, strict forward/reverse lineage, valid 1:1
owners/Nucleus graph and exact-scope seals, full differential diagnostics and
future-variant compile-negative evidence at new closed decisions. A public API
or phase not exactly approved requires a new ADR-0127 classification.

No overall parity claim follows from this package. Direct Func marginal cast
ordering and ignored-page HTML warning are retained separately measured gaps;
accessibility/decorative geometry obligations remain with their existing Nucleus.
Deferred footnote/float source causality versus physical placement requires actual
source-bound controls rather than a universal paint-order assumption.

### Scoped operational Context style carrier

The request realizer installs a private typed native-function identity token in
its Engine style chain while realizing actual source children. The realization
hub may retain its existing context-recipe style carrier with empty rules only
under this exact token. This transports page-effective styles into the existing
later Context Session; it does not execute the token, expose a new language field,
set current_location/in_context or reinterpret an unknown token by name/equality.
Prepared aliases keep their existing capture. Restore the caller chain on success
and error; no new default path outside actual page requests is admitted.

## Operator capture provenance and allocation-only continuity

Source evidence precedes this correction: a rebuilt PageRun Arc is not stable
through ordinary body substitution, and the actual body-query controls in
focused-diagnostic-4 fail contextual stabilization as header Locations change.
Preserve the strict Location observer. Exact pre-correction owner bytes are in
agents/realization/allocation-lineage-before under the implementation evidence.

Allocate one private operator occurrence token shared by its four captured
operational fields. Transport the owned Version Arc plus exact private Native marker
in existing ShowRecipe styles. Each fresh raw operator occurrence consumes a
retained capture slot in its actual parent frame; sharing a raw header or raw
operator value cannot merge those slots. A per-EvalContext weak memo recognizes
only actual captured output SetPage/PageRun Arcs for completion idempotence, not
raw input Arc equality. Body-only substitution carries the token unchanged.
Existing captured rules/styles/source and prepared Located aliases remain intact.

Before actual realization/binding, validate the request/arena and extract that
exact operational capture token from its selected source-plan template. Enter a
private stage/key/token allocation cohort using the same installed arena Locator.
An unchanged cohort retains occurrence IDs across a new SourcePlan or effective
environment, while realization still executes with current input/styles and
publishes current diagnostics. A changed token resets only the cohort. Stronger
full-request provenance still governs existing reusable execution contributions;
physical key alone never certifies either claim. Retain distinct realize/bind
allocation stages and preserve the caller's document-boundary slots.

For a raw synthetic body prepared without lexical capture, the exact immutable
SourcePlan Arc and template packet remain a valid within-plan allocation authority.
This fallback cannot retain a cohort across a new plan and cannot stand in for
actual captured operator lineage. Keep the existing public same-arena raw-body
binding control; foreign/missing owners still reject before any allocation.

Carrier source correction: value.rs:177–181 actually stores Version in Arc,
whereas value.rs:191 stores Args by value. The compile-only diagnostic refutes
the earlier Args-Arc carrier assumption. Use a fresh empty Version Arc only as
an opaque owned token authenticated by the private Native marker; neither
Version components/equality nor PARITY_VERSION/sys.version participates. This
changes no public language value, version contract or generic observer.

## Captured lexical rule base at operational insertion

Evidence before correction: textual.rs:86–104 composes only recipe-owned rule
additions above the captured baseline, expecting the inherited rule set to supply
that baseline. realize_actual currently starts the operational template with the
caller's empty show_rules. The actual text-header warning and warning-then-panic
controls still fail after round7, whereas source capture and direct Figure
contributions run. The root captured baseline's text rule is therefore removed
without an inherited replacement; this is a concrete scoped composition failure.

At an actual captured operational ShowRecipe root, install its captured rules as
the inherited lexical rule base before entering the existing recipe realizer.
Nested local recipes continue to use their normal baseline/owned composition;
existing guards and lexical file restoration remain in realize_pending. Restore
the caller's show_rules after the operation on both success and Err, together
with the existing style/context restoration. Raw synthetic unwrapped templates
keep the caller's prior rules. Prepared aliases' operational wrapper captures no
new rules, preserving their no-reapplication boundary. Keep current page-effective
styles independent from this lexical rule authority. No generic compose_rules
change, layout callback, additional effect cache or new phase is authorized.

### Contextual descendant producer acceptance

The runtime-observation-1 body-query control is actual semantic RED: repeatedly
rebuilt marginal templates contain freshly reconstructed ContextBlock Arc objects.
Keep the exact template operator/cohort authority independent of current captured
styles. The context-occurrence owner may retain the immutable Context descriptor
at an actual creation slot only while the owned Func representation is unchanged;
no contextual execution or effects are cached by this facade. Require facade
replay across rebuilt source plans with stable actual Context Arc, and rejection
of that proof for a changed closure owner or a different operator occurrence.
These tests use deferred natives that panic if called, proving capture remains
inert. Ordinary selected-template materialization outside a running cohort remains
fresh. Exact reusable resolutions still use their existing separate owner proof.

## Passive operational Context environment transport

### Measurement before correction

source-boundary-9 proves actual request chains20/30 while the unchanged three
integrated style controls in focused-diagnostic-8 observe11. The Context capture
hub retains full styles only inside custom metadata; L3 starts collection from
its ordinary default and cannot obtain that chain from the custom-only delta.
An existing ShowRecipe already has a full-chain collection boundary. Before bytes
are archived in agents/realization/context-transport-before under implementation
evidence; this is a private transport repair within the approved phase/API.

Construct a non-textual passive ShowRecipe around the already prepared Context
Styled carrier, with the exact unchanged persistent Engine chain, no executable
rules/guards and existing file/floor provenance. Place a distinct private Native
marker only in the *own Styled body's local delta*, never in the outer recipe's
ambient chain. Authenticate that local Native by its actual nominal call identity,
namespace absence, Context-only prepared body and its existing context-recipe
snapshot bound to the identical persistent outer chain head. Located/Source around
that actual Context remain transparent aliases; arbitrary Styled or pending
recipe bodies do not qualify. The marker is not a language field or public API.

A marker inherited through Engine styles cannot certify a prepared recipe. Fresh
Table/text/local captures may copy Engine styles directly, so head-only ambient
marker checks are insufficient. Recognize only the body-local prepared transport.
Ordinary pending recipes must still execute even when their ambient metadata
contains a copied marker. Reusing the exact already prepared body as an alias
retains its existing descriptor; it never executes the Context during capture.

The recipes owner returns the authenticated passive root unchanged on re-entry,
so Table coordination cannot reinstantiate its Context or execute rules. Generic
ShowRecipe semantics, map contracts, morphology and observations remain unchanged.
Existing morph_canon strips ShowRecipe and planned layout routes ShowRecipeBody;
no rendered size delta is applied a second time. Test actual facade20/30, original
Context Arc/ID across recipe and unrelated-Table re-entry, fresh local/Table
pending recipes despite inherited metadata, ordinary capture and inner18/12.


## Full-style capture at source-owned insertion boundaries

The measured operational Context regression and source-boundary9 prove that a
correct request chain can still become default11pt when only a rule snapshot is
transported. The source-owned Outline completion boundary now creates equivalent
new Context products under the same retained arena. Factor only the existing
private authenticated environment-marker operation so the parent realization
owner can request the same full-style capture while completing those new
products. Keep the marker recipe-local and the prepared-transport predicate
unchanged: no ambient inherited-token bypass, new mode, public getter/signature,
or generic Context capture change. Ordinary page insertion uses the identical
operation and retains existing operator provenance, rebase and diagnostics.


## Test identifier compatibility

The pinned tree-sitter grammar rejects references such as `&raw` in the newly added test code as raw-pointer syntax. A direct parse probe isolated the rejected tokens; the first tuple-pattern hypothesis was refuted and its source edits were restored. Name these test-local content values `template` instead, retaining exactly the same variant checks, assertions and failure boundaries. No productive behavior, criterion or public contract changes. Exact preflight failures and prior bytes remain under `lab/results/set-page-header-implementation-2026-10-21/seal-preflight-repair/` and `lab/results/set-page-header-implementation-2026-10-21/seal-preflight-repair-2/`.


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
