# L0 — introspect/from_tags
Hash do Código: 948da922

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/from_tags.rs`.
The current module has no from_tags constructor. Delegate filtered/ordinary counter replay to `00_nucleo/prompts/core/compiler/introspect/from_tags/counter_updates.md`, legacy explicit State function replay to `00_nucleo/prompts/core/compiler/introspect/from_tags/state_updates.md`, State/counter display to `00_nucleo/prompts/core/compiler/introspect/from_tags/displays.md`, and equation realization to `00_nucleo/prompts/core/compiler/introspect/from_tags/equations.md`. This consumer remains the import boundary and retains its tests. The historical extraction locations for the delegated bodies are
resolve_filtered_counter (`:36`), apply_state_funcs (`:123`),
apply_counter_funcs (`:157`), bundle Page replay (`:185`), state/counter
display passes (`:310`, `:411`) and equation passes (`:462`, `:523`, `:570`).
`01_core/src/compiler/introspect.rs:1376` builds stores during the walk.
Both runtime entrypoints and the bounded helper coordinate these passes;
none of that construction belongs to this child.

## Delegated explicit State and ordinary counter boundaries

The legacy explicit apply_state_funcs helper evaluates after walk with Engine/EvalContext and an
empty local Scopes whose closures retain their lexical captures.
For each StateUpdate::Func Start, read the state at its Location and apply
the function to that value as one positional argument. Store a successful
result at the update Location. Propagate callback diagnostics; an update
without an available initialized value is ignored. The pure walk does not
evaluate Func. The later ratified object-reader contract removes automatic eager
State replay only; this explicit compatibility helper remains available.

Ordinary counter Func receives the current components as separate positional
integer arguments, not a single array argument. Missing state uses [0].
Accept a non-negative integer or an array of non-negative integers as the new
complete state; reject other returns with the existing diagnostics.
Store Set at the same Location and propagate errors. Do not promise a causal
replay correction for all ordinary keys: the observed implementation appends
function results after deterministic walk snapshots.

The legacy explicit bundle replay clears/rebuilds only Page snapshots, then replays each
body boundary Page = [1] and its Page Set/Step/Func in source order.
A Page function receives its own body's state; later actions see its result,
and the next body starts at 1. Other keys retain their existing paths.
Realized Page snapshot construction is governed by the separately approved
counter_updates owner and facade below; this legacy boundary does not override
its document-aware action/physical-page fold.
Do not execute a callback twice or conceal an error. Boundary/seed collection
belongs to `00_nucleo/prompts/core/compiler/introspect.md`.

## Delegated on-demand filtered counter boundary

For a selector key containing Element, use the immutable event log, not
differences between previously computed counter snapshots.
Select automatic events by the concrete selector query and manual events by
language counter-key equality. That equality is owned by
`00_nucleo/prompts/core/compiler/eval/operators/equality.md`; derived Rust equality
is not a substitute. Order selected events causally, retaining Location as identity, and fold from [0].
The counter owner's recorded-order contract supersedes numeric Location ordering;
retain historical fallback only without recorded order.
Apply Step(level), Set and Func in causal order, retaining owned history.

Resolve the entire selected sequence before a caller chooses an at/get
prefix or final state. An invalid callback later on the same key can therefore
fail an earlier get. Unrequested keys are not eagerly executed merely to
prepare a contextual read. The ordinary global counter-function pass skips
these Element-containing keys.

Each replay Func uses a new EvalContext without introspective context,
preserving the observer's target/features and closure captures. Pass components
as separate positional arguments and validate the same return domain as
ordinary updates. Capturing a Counter is allowed; querying one from inside
this callback must not inherit the outer contextual read or recursively
initiate contextual replay. Do not mutate the original event log or public
eval snapshot into artificial Sets. Cache/optimization must not change error
ordering or observed effects without its own evidence and specification.
Context-generated actions and stabilization are coordinated by eval/pipeline
owners, not by a global preparation pass here.

## Delegated display conversion boundary

After update passes, state display reads its value at (key, Location), using
None when absent. Its callback receives that single value.
Without callback, Content passes through, LocatedContent explicitly converts
with into_content(), and String becomes text; other values become Empty.
With callback, Content/String become display content and other successful
returns or errors become Empty, preserving the current defensive behavior.

Counter display callbacks receive one array of integer components (empty when
absent), unlike update callbacks' separate arguments. Without callback, join
existing components with dots; missing state becomes Empty. Callback
Content/String succeeds; other returns/errors become Empty.
Store realized display Content by (typed key, Location), never execute these
callbacks in physical layout or rewrite Values held in state, arrays or closures.

## Delegated equation realization and snapshot boundary

For block equations whose captured numbering gate is active, obtain the
location-aware number. Format a captured string pattern (numeric fallback on
format failure), or call the numbering function with that single integer.
Require Content/String from numbering callbacks and propagate errors.
Store realized number Content by Location; keep the raw Func/pattern view.

Resolve supplement from the captured target specification: Auto uses
`compiler::lang::equation_supplement_for_lang` through its existing owner;
None is Empty; Content/String use display conversion; Func receives the base
public equation Content and its result uses display conversion. Callback errors
propagate. Never use the reference's current language or a private translation
table.

After numbering and supplement passes, the equation child publishes its
authoritative occurrence snapshot while preserving original semantic Content:
block, raw numbering, effective number-align, realized supplement, effective alt,
body, and causal label last when present. Its measured identity/field regression
and precise publication contract belong to
`00_nucleo/prompts/core/compiler/introspect/from_tags/equations.md`.
An existing present snapshot remains verbatim, including empty/known fields;
the normal walk's absent snapshot is the publication input. Do not introduce
Styled constructor markers, refresh semantics, or alter Heading snapshots.
Only explicit conversion to rendering discards a snapshot, not the original
stored Value. Shared invariants belong to the pinned content-snapshot nucleus.

## Acceptance and boundaries

Verify state errors versus display defensive errors, separate update arity
versus array display arity, non-negative return validation, filtered
Step/Func/Step and later-error ordering, absence of callback introspective
context, immutable logs, bundle Page resets and no duplicate callbacks.
Check lexical equation numbering/supplement/alt and unchanged Heading snapshots.
Do not reinstate retired observation carriers/hooks; instrumentation needs its
own approved specification. Consumer existence proves implementation, not
independent parity or completion of every contextual stabilization gate.


## Approved Page snapshot facade

Reexport realize_page_counter_snapshot with the public signature already
ratified in `00_nucleo/prompts/core/compiler/introspect/from_tags/counter_updates.md`.
This facade contains no formula. That owner retains the realized Page/action/
partition/read-normalization contract, including its causal fallback and generation
boundaries; declaration does not transfer productive ownership here.

## Ratified compact carrier fixture adaptation

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/before.json.gz`
retains UTC/sources/L0/hashes/status/git diff HEAD --stat. Existing owners were
read before proposing. probe-details.json retains rustc 1.92/argv/rlib/binary/
probe SHA-256/output; probe-result.json records 12 GiB/zero swap/one job/events.
This synthetic carrier/register-copy probe is not compile/layout/RSS benchmarking
or a real workload distribution. Candidates mirror real domain types; inline
uses productive Tag/ShowRule.

Tag is 416 bytes, boxed/Arc Start 48. For 4,096 Start/End pairs with heap-free
Heading, requested bytes/allocations are inline 3,407,872/1, Box 2,031,616/4,097,
Arc 2,097,152/4,097. Cloning boxed Tag allocates per Start; inspected productive
fixpoint.rs:86–136 and entrypoints.rs:44–66 use temporary Vec<Tag>/borrowed reads,
without cloning the vector. Walk:325–356 populates the index by reference before
moving info to Start. Synthetic clone frequency is not assumed productive.

Transformation is 328 and ShowRule 376 bytes; Box/Arc<Styles> candidates are 120
and 168. rules/show.rs:464–470 copies to_vec before every registration;
apply.rs:108–112,349–352 filters/clones rules. Across 128 registrations of simple
100% show-set without retained old snapshots, requested bytes/allocations are
inline 12,276,960/384, Box 8,194,560/8,640, Arc 5,530,624/512. With retained
snapshots, requested peaks are 3,203,856, 4,141,744 and 1,477,808. A custom-channel
control copying 4,096 rules requests 4,107 inline, 8,203 Box and 11 Arc
allocations, including Vec growth. apply/intercept/outline/table_cells borrow
styles; application still clones a delta when needed.

Mandatory counterexample: building one simple 100% show-set collection of 4,096
rules requests 1,540,096 bytes inline, 2,031,616 Box and 2,097,152 Arc. Arc may
retain excess custom-channel capacity; the rich 128-registration control without
old snapshots ends with 78,064 live bytes inline and 156,432 Arc. Claim no
universal gain or ignore atomic/indirection cost. This is ADR-0030/0107 storage
mechanics, not language parity or measured AI benefit. Real relevant memory/time
regression refutes the favorable copy-cycle inference; retain those controls.

### Ratification and exclusive responsibility

The human approved manifest
`282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`
records approval. The former ADR-0127 public-payload/compatibility proposal gate
is satisfied; this contract, including T1–T6 and no release, is in force.

Migrate only this consumer's fixture Tag::Start constructors to Box::new of the
same ElementInfo. Preserve Locations/payloads/labels/End hashes/calls/assertions
and every productive body. Borrowed reads/value hashes retain their contracts;
add no hash algorithm/error and change no expectations to hide a difference.
The public payload belongs only to `00_nucleo/prompts/core/entities/tag.md`.

### Acceptance and owner boundaries

Ratified L0 precedes tests and real RED; reseal strictly changed owners. No
large_enum_variant suppression, threshold/dependency/flag/default/phase change.

- T1: construction Box<ElementInfo>/Arc<Styles> and relative budgets (Tag at most
  three Location; Transformation at most Content plus alignment) fail before
  and pass after without freezing a universal ABI. Those payloads stay owned by
  their entity prompts.
- T2: preserve Start/End/Location/payload/label, value equality/structural hashes,
  independent Box clone and mutation isolation; hashes are not persistence format.
- T3: existing introspection/convergence/nesting/heading/label/state/counter/query/
  metadata tests retain expectations/errors.
- T4: Transformation::Style clone shares Arc; Arc::make_mut on a copy preserves
  original styles including custom; Func/Content/Str variants remain unchanged.
  This is a dependent entity boundary, not implementation in convergence.
- T5: show-set/func, precedence/scope/labels/outline/table.cell/textual/regex
  controls preserve semantics/morphology/errors with no extra execution.
- T6: full workspace/build/all-targets Clippy with prior expectations/lints as
  errors, rustdoc -D warnings with four indexes, fmt/diff/strict/reverse lineage
  and no Tekt increase, under 12 GiB/zero swap/one job. Record HEAD/working tree/
  hashes/diffstat/UTC and distinguish requested sizes/bytes from RSS.

No release. Any signature/field/phase change beyond these ratified payloads
requires a separate proposal. This owner adapts only its fixtures and keeps all productive pass/reexport contracts unchanged.

## Heading supplement import boundary

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree/UTC/hash are in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz`.
before-pdf-probe.json.gz: heading-supplement-panic succeeds in Tekt and errors
A1_HEADING at vanilla `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
heading-supplement-valid is accepted in both, but only vanilla uses Custom in
references. Upstream model/heading.rs:249–289 synthesizes per-occurrence
supplement; model/reference.rs:393–415 applies function/display. Set.rs:286–320
ignores supplement/clears numbering; heading.rs:55–68 snapshot_fields fixes the
translation. This is observable callback/value behavior, not upstream mechanics.


### Exclusive responsibility

Import child headings and expose its finalization helper only within
compiler::introspect. Logic belongs to
`00_nucleo/prompts/core/compiler/introspect/from_tags/headings.md`, not this hub.
Add causal callback-panic/field-body-capture/Content-none/snapshot tests using
existing Engine runtime, without reinferring from render. The withdrawn State
proposal did not authorize reader init on updates: init belongs to each reader;
the later independent approved reader contract below governs demand evaluation.
ADR-0127 internal parity correction adds no public Rust API/field or eval/layout
logic transfer. Existing post-walk Engine is the callback boundary. L0 first,
RED/bilateral controls, no callback in encoder/walk/layout or all-fields parity
claim.

## Ratified State reader checked-display boundary

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
before.json.gz and final-freeze.json.gz under
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
preserve UTC/status/diffstat/sources. state-counterproof.json.gz records bilateral
RED: readers initialized to 1/10 with the same key require 2/11 after +1; an
unread callback never executes; reused updates execute per occurrence. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`; upstream
introspection/state.rs:226–245,419–455,477–520. The reader's init starts the
sequence; document order, not the Location integer, orders it. The measured
method treats Func as Set and eager postprocessing loses that semantics. These
are language values/errors, not Rust equality or a mandated algorithm.
The earlier State proposal was withdrawn; the owner ratified the object-reader
runtime gate in that session. Review:
`lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`,
SHA-256 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`.

### Exclusive responsibility and partial supersession

Reexport only the checked State display internally; preserve public exports and
legacy explicit apply_state_funcs. Add real Engine tests for two reader init
values, prefix/final Func/Set/Func, unread callback/error on demand and context/here
not inherited but restored on success/error. Legacy explicit-helper tests remain
independent of automatic runtime. This supersedes historical automatic State
phase only.

### Acceptance and boundaries

The object-reader gate is ratified for its five signatures and read phase.
L0 first, functional RED before code; GREEN against the stated pin for language
and independent controls for Tekt extensions. Do not broaden other public
contracts. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt,
V1/V5/V15/V26 and reverse lineage; no increase in V16. Limits:
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; heavy tasks sequentially. Measure consumption
without inferring global benefit from representation alone.

## Ratified before-query Heading finalization

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
final-freeze.json.gz/target-controls.json.gz in the same callback diagnostic retain
UTC/diffstat/sources/argv. Pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
heading-raw-constructor-func passes baseline/vanilla but fails candidate because
Styled transport changes func. Query sees a temporary causal wrapper and HTML
ignores callback panic. Sources: walk.rs:930–945, pipeline/paged.rs:419–427 and
pipeline/html.rs:101–106 prepare query through pure walk. These are language
values/errors, not a required upstream algorithm/structure. Owner confirmation
is in `lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`,
SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.


### Exclusive responsibility

Retain internal finalizer reexport/causal tests. Add pure-snapshot/two-facade-call
control: only the first invokes the callback. Verify semantic Content/fields and
immutable old clones. Preserve State/legacy explicit tests; no child callback
logic moves here.

### Acceptance and limits

Ratified H1–H8 with bilateral RED before code. No public contract beyond this
Heading facade is authorized here; the five State helpers have their separate
ratification. No new public fields. Ordinary Styled/other callbacks keep their
boundaries. Validate workspace/doctests/build/strict Clippy/Rustdoc/fmt/
V1/V5/V15/V26/reverse and no V16 increase, under 12 GiB/High 8 GiB/swap0/jobs1 with
heavy commands sequentially. RAM benefit requires measurement.
