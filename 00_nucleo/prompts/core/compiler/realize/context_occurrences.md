# L0 — compiler/realize/context_occurrences
Hash do Código: 3a14fe56

**Layer:** L1. **Exclusive consumer:** `01_core/src/compiler/realize/context_occurrences.rs`.
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.
**Status:** Ratified; materialization in progress.



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42


## Context — ratified occurrence identity, 2026-10-18

### Measurement before decision and gate

`before.json.gz` in `lab/results/construction/diagnostics/context-paridade-2026-10-18` preserves HEAD `b7e7c83223e5daa67bbdf124c36b5b7f93200731`, sources, UTC, diffstat, argv, and binary SHA; `baseline-build-resources.json` contains the new build recipe. Vanilla pin: `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Two equations produce EQ EQ Q1/Q2; a Heading prefix produces H EQ Q0/Q1; query of `it.location` returns 00/01. `context_stabilization.rs:257` restarts `EvalContext` per producer; `eval/mod.rs:438` restarts Locator; `realize/mod.rs:41` allocates without a shared domain; `introspect/walk.rs:375–390` deduplicates by Location. This is language identity. Upstream `typst-realize/src/lib.rs:353–418` prepares identity before the recipe but does not require copying representation, vtables, or algorithm. The collision hypothesis is refuted by failing count, alias, callback, or replay controls.

The human approved proposal `review.md` in `lab/history/construction/records/00_nucleo/materialization/context-occurrences-2026-10-18`, SHA-256 `500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388`; `approval-receipt.json` is in the same directory. ADR-0127 §2.1 is satisfied for the specified additive API. This does not authorize unrelated phase or API changes.

### Exclusive responsibility

A pure arena of Locations for contextual producers within one Session. Additive public API and opaque types, without `Clone`:

```rust
pub struct ContextOccurrenceArena { /* private */ }
pub struct ContextOccurrenceFrame { /* private */ }
impl ContextOccurrenceArena {
    pub fn new(origin: &Content) -> Self;
    pub fn frame(&mut self) -> ContextOccurrenceFrame;
    pub fn evaluate<R>(
        &mut self,
        frame: &mut ContextOccurrenceFrame,
        ctx: &mut EvalContext,
        operation: impl FnOnce(&mut EvalContext) -> R,
    ) -> R;
}
```

`new` reserves Locations already carried by `origin` and those assigned by canonical introspection of that same origin. Do not use another pass's results as an increasing offset. Reservation order does not change identity: `observe` takes a maximum; it does not allocate. The arena has one Locator; a frame has private `Vec<Location>` slots and an execution cursor. An immutable `Arc` token may verify that a frame came from the same arena; its address never creates a Location.

The crate-private `next` helper delegates to the ordinary Locator when there is no active frame. With a frame, it reuses the slot in the order allocations are actually reached; when a new slot is needed, allocate from the single Locator and append the slot. Already `Located` aliases do not call `next`. Do not derive Location from morphological equality, span, text, hash, or Context ID. Slots unused after a retraction remain reserved, without phantom tags or contributions. Growth allocates at the shared free tail.

In `evaluate`, temporarily install the Locator and slots into the existing `EvalContext`, reset the cursor, call `operation` once, recover Locator/slots, and restore prior context state on every normal return, including `Result::Err`. Do not catch panic or alter compiler unwind policy. A frame belongs to its actual producer and persists across that producer's re-executions; do not clone it for another producer. Do not retain frames/arenas across compilations or create global state, I/O, unsafe code, or dynamic dispatch.

### Acceptance

Integrated RED for count and `it.location`, then bilateral GREEN on the pinned version. Local tests cover reservation of the real origin, producer disjointness, slot reuse/growth/retraction, and restoration after `Result::Err`, using real locatables and introspection. Pipeline controls cover aliases, nesting, and selective replay.

Validate build, full suite, strict Clippy/Rustdoc, formatting, lineage, and SARIF with no new error/warning, under 12 GiB/High8 GiB/swap0/jobs1. Do not claim general parity.

## Documentary provenance

The pre-consolidation owner is preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/realize/context_occurrences.md`.
The ratified API, current materialization status and acceptance remain authoritative.

## Raw selected-template Context IDs

Measured source context_stabilization.rs:execute initializes next_context_id from
Session.next_id for every retry and stores the increased floor afterward. A
scalar-only raw template clone would allocate different nested Context IDs on
retained retries; equal contributions would fail stability or share unrelated
producer IDs. The approved Ref proposal explicitly requires fresh insertion sites
and stable retries. Extend only private OccurrenceSlots with retained Context ID
slots/cursor and a scoped template-instantiation flag, alongside Location slots.
Reset both cursors at frame replay; reused ID raises ctx floor to at least ID+1,
fresh slot consumes actual ctx.next_context_id. Preserve public Arena/Frame APIs.
A private with_template coordinator sets/restores only this flag in existing slots
(or temporary slots using the caller's same existing Locator outside a frame),
never a separate allocation arena. Only actually reached raw ContextBlock visits
reconstruct their ID; preexisting Located aliases and generated Ref producers
retain their own identities. Test separate sites, retained retry/growth/error
restoration and aliases. No mutable globals/hash/offset IDs or second arena.

## Retained allocation domain and original boundary seeds

### Measurement before correction

The physical-page implementation baseline and frozen diagnostic runs retain the
exact source/L0 audit under
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/arena-domain/before.json`.
Current context_occurrences.rs:27–35 stores Locations and cursors but no installed
arena owner; :57–76 checks the frame against its arena before installing ownerless
slots; :80–94 returns only Location. Merely finding Some slots cannot prove that
a later request belongs to that arena, and reading the live Locator checkpoint
on replay loses the original source-boundary seed after interleaved producers.
These are private protocol invariants already required by the approved physical
page review SHA cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039,
not a new public Context capability or generic language identity law.

### Exclusive private protocol

Retain the existing immutable arena owner Arc token in its installed slots.
A compiler-descendant allocation_owner helper returns that exact capability only
for a genuine installed arena frame; no frame and temporary legacy with_template
slots return None. It certifies allocation domain only, never a Location or
language object. Existing evaluate frame/arena validation and restoration remain.

Store the original opaque LocatorSeed with every retained Location slot. The
private next_with_seed helper uses exactly the same slot cursor/allocation path
as next, requiring an actual owner capability and returning the stored pair on
replay. A fresh pair records the checkpoint immediately before the existing
Locator.next operation. Do not reconstruct seeds from Location integers or the
later live tail. Existing no-frame next fallback, public Arena/Frame declarations,
Context-ID slots/floors, template flag, alias reuse, Err restoration and budgets
remain unchanged.

The physical source plan retains the actual domain capability and one optional
original document boundary pair through this private helper. Contribution plans
reuse the same capability but allocate no additional document boundary. Planned
realization/binding rejects a missing or foreign domain before callbacks or new
allocations. Legacy temporary slots still serve their existing selected-template
contract and cannot impersonate a physical request frame.

Require actual public-body preparation/replay with interleaved frames, foreign
Arena binding rejection, same-Arena acceptance through another legitimate plan
frame, missing/temporary-frame rejection and unchanged legacy arena tests. L0
precedes semantic RED→GREEN; no public signature/field/variant/default/phase,
observer law, second arena, arbitrary offset or seed arithmetic is introduced.

### Reserve dormant operational aliases before producer execution

Source measurement: Arena::new first calls reserve_locations and scratch
introspection; both keep SetPage operational fields opaque. Existing body
Context discovery can execute a producer before the later public physical-body
plan calls reserve_source. Thus a prepared header alias at the next free Location
can collide with that earlier body producer. The approved reserve-only obligation
must apply at arena construction, not only after discovery. A functional control
uses body Located(ContextBlock,0), dormant header Located(Heading,1), then a real
arena producer frame allocating a Heading; the result must not use Location1,
and no dormant Heading may enter the source semantic index or execute a closure.

Add only a feature-owned reserve scan of existing aliases in operational fields
at arena initialization, recursively including nested operational fields and
preserved originals. Do not alter generic traversal, admit or prepare a dormant
header, derive locations numerically, run its show/closure/callback or replace
the legacy scratch-index compatibility path. The new control must fail before
changing Arena::new. Context-ID/rule floor reservation and first-body-producer
ordering remain explicit associated obligations, not proven by the Location
control alone.

### Independent dormant Context and rule-floor controls

The source evidence is independent of Location reservation: Session initialization
computes next_id from collected actual body blocks, and execute resets a fresh
EvalContext.next_context_id to that value. Thus earlier realization observation
does not by itself retain a dormant operational template's higher Context floor.
At context_occurrences.rs:18–21 and :40–46 the arena currently stores only Locator
and owner; evaluate installs only those allocation resources. The exact source
and owner before these tests are archived under the physical-page implementation
agent's arena-domain/floors-before receipt.

Before changing production, require separate failing controls for an existing
dormant raw Context ID and a captured ShowRecipe's Context/rule floors. Neither
control invokes its captured native closure/transformation. In the implementation,
the same feature-owned reserve-only scan retains private maximum floors in the
arena and evaluate raises the incoming EvalContext floors before the body producer
executes. Existing higher caller floors must never be lowered; retained Context
slots still replay their own IDs. This does not admit templates, allocate their
contents, alter public APIs or prove a language identity from equal Content.
The original Location-only test remains unchanged and is not evidence for these
independent floors.

### Reserve-only traversal and floor application

The independent Location, raw Context and captured-recipe-floor controls all fail
on the unchanged reservation path in reservation-red-all-2/receipt.json, without
compiler errors or input drift. A feature-owned pre-order scan explicitly reserves
Located handles, both Realized representations and raw Ref supplements; ordinary
containers delegate their existing closed traversal. At SetPage/PageRun it scans
only the four operational templates plus the lexical PageRun body, then stops
generic descent at that node. This is reservation, never source admission or
execution. Context IDs reserve id+1 and ShowRecipe reserves its Context floor and
each captured rule id+1 using the existing saturating maximum convention.

Retain those Context/rule maxima privately on the arena. Before each evaluate
operation, max-raise the incoming EvalContext counters without lowering existing
higher floors; Locator and slots retain their existing installation/restoration.
Keep the prior reserve_locations and scratch canonical-index compatibility step.
The three test bodies/expected conditions remain unchanged through the repair.

## Private operational capture and allocation cohorts

Source evidence: rebuilding the body plan retires L3 marginal frames although
body query results compare actual Locations. Fresh marginal allocations then
change those query values and trigger another body execution. The actual
focused-diagnostic-4 failures and source-bound round snapshot retain this churn;
no generic observation equality may be weakened.

Retain private operational capture-token slots with their own replay cursor in
the actual producer frame. A raw reached operator consumes one creation slot;
frame replay retains its opaque Version Arc token, different producer frames do not.
No owner/temporary legacy slots receive an unretained fresh token. Reset this
capture cursor alongside the ordinary Location/Context cursors on evaluate.

Maintain dedicated private running allocation cohorts inside installed slots,
separate from ordinary document-boundary slots. An actual request selects a
cohort by operation stage, physical key and exact owned operator-token Arc within
the installed arena. A changed token resets only that stage/key cohort, leaving
old identities reserved in the same Locator. Matching tokens retain only allocation
slots, never prior execution output or effects. A scoped helper installs the
selected cohort on the same live Locator/owner, resets its replay cursors, calls
once and restores the caller on every normal return including Err. Realize and
bind have separate stage meanings even when a caller uses one legitimate frame.
No second arena, shared result cache, public field/signature or identity from
Content/Version equality, hashes, span or numeric offsets is introduced.

The private cohort token domain distinguishes captured operator Version-Arc authority
from exact source-plan Arc plus template packet authority for legitimate unmarked
raw synthetic bodies. These alternatives never compare equal. The latter is
within-plan only and supplies no cross-plan producer claim.

Carrier source correction: value.rs:177–181 actually stores Version in Arc,
whereas value.rs:191 stores Args by value. The compile-only diagnostic refutes
the earlier Args-Arc carrier assumption. Use a fresh empty Version Arc only as
an opaque owned token authenticated by the private Native marker; neither
Version components/equality nor PARITY_VERSION/sys.version participates. This
changes no public language value, version contract or generic observer.

## Actual contextual producer retention inside a running cohort

### Measurement before correction

The source-bound runtime-observation-1 execution preserves the actual failing
physical_page_body_query_with_contextual_header_retains_descendants control:
body execution versions 1,3,5,7,9 never complete. recipes.rs:240–248 reconstructs
ContextBlockElem on every template insertion, even when the certified running
cohort replays the same creation slot and actual owned closure. The source/L0
bytes before this correction are retained in the implementation evidence under
agents/realization/stable-context-before. The pinned language control observes
two Figures and final count two; this is producer/replay correctness, not a
requirement to equate arbitrary Rust functions or cache execution results.

### Bounded private producer witness

Only the slots installed by with_running_cohort retain contextual producer
objects. At the existing Context-ID creation cursor, keep an optional owned
ContextBlockElem Arc. The existing next_context_id remains the sole ID allocator
and advances exactly once. Reuse the retained product only if its ID is the
current retained ID and both live Func owners have the identical representation
pointer. A changed actual Func owner replaces that one product even when its
native function address, displayed name, ID or body appears equal. A changed
operational capture token already resets the whole stage/key cohort. Different
slots and parent arena frames remain disjoint. Unused slots remain reserved.

No-frame/temporary slots and ordinary non-running selected-template insertion
keep fresh immutable ContextBlock construction. The helper does not execute a
closure or retain results, observations, chains or diagnostics. Context producer
Arc continuity alone never certifies the surrounding captured styles or execution
version; callers must replay under the actual current environment. Ref-generated
producer exclusion and prepared Located aliases retain their existing behavior.
This private correction implements the approved retained producer/arena contract;
public functions, entity fields, allocation identity, phases and budgets remain.

Require actual facade replay across rebuilt source plans, changed closure owner
at the same certified creation slot, different operator/cohort separation and
ordinary non-running template freshness. The language-level integrated RED is
preserved before implementation; local tests must not invoke the deferred body.

## Independent running-cohort reuse witnesses

Measurement before correction: `lab/results/set-page-report-blockers-2026-10-21/agents/domain/before.json.gz` retains the
exact source/L0 bytes, UTC, HEAD and dirty-file inventory. At the retained
`context_occurrences.rs:364–380`, the reuse match guard conjoins exact operator
token continuity with the presence and exact Arc identity of both arena owners.
The retained full lint reports V17 at that compound decision guard. These are
two independent private authority requirements, not interchangeable evidence.

Express the token decision and owner decision separately before selecting the
retained slots. A changed token rejects reuse immediately. A matching token
still requires both actual owner capabilities and their exact Arc identity;
missing or foreign ownership creates fresh slots in the caller's owner domain.
Preserve stage/key selection, removal and reinsertion, all replay cursors,
caller restoration on normal Err return, and the requirement to execute the
operation once. Do not derive authority from value equality, hashes or numeric
identifiers, cache outputs, or relax either witness. This internal clarification
follows ADR-0127 continuous flow; signatures, phases and behavior are unchanged.
Revalidate retained running producers, changed captures, foreign/missing arena
rejection, frame separation and existing allocation/error-restoration controls.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
