# L0 — introspect/from_tags
Hash do Código: f66f2d18

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/from_tags.rs`.
The current module has no from_tags constructor. Its productive bodies are
resolve_filtered_counter (`:36`), apply_state_funcs (`:123`),
apply_counter_funcs (`:157`), bundle Page replay (`:185`), state/counter
display passes (`:310`, `:411`) and equation passes (`:462`, `:523`, `:570`).
`01_core/src/compiler/introspect.rs:1376` builds stores during the walk.
Both runtime entrypoints and the bounded helper coordinate these passes;
none of that construction belongs to this child.

## State and ordinary counter updates

Keep state-function evaluation after the walk, with Engine/EvalContext and an
empty local Scopes whose closures retain their lexical captures.
For each StateUpdate::Func Start, read the state at its Location and apply
the function to that value as one positional argument. Store a successful
result at the update Location. Propagate callback diagnostics; an update
without an available initialized value is ignored. The pure walk does not
evaluate Func.

Ordinary counter Func receives the current components as separate positional
integer arguments, not a single array argument. Missing state uses [0].
Accept a non-negative integer or an array of non-negative integers as the new
complete state; reject other returns with the existing diagnostics.
Store Set at the same Location and propagate errors. Do not promise a causal
replay correction for all ordinary keys: the observed implementation appends
function results after deterministic walk snapshots.

For bundle runtime, clear and rebuild only Page snapshots, then replay each
body boundary Page = [1] and its Page Set/Step/Func in source order.
A Page function receives its own body's state; later actions see its result,
and the next body starts at 1. Other keys retain their existing paths.
Do not execute a callback twice or conceal an error. Boundary/seed collection
belongs to `00_nucleo/prompts/compiler/introspect.md`.

## On-demand filtered counter resolution

For a selector key containing Element, use the immutable event log, not
differences between previously computed counter snapshots.
Select automatic events by the concrete selector query and manual events by
language counter-key equality. That equality is owned by
`00_nucleo/prompts/compiler/eval/operators/equality.md`; derived Rust equality
is not a substitute. Sort selected events by Location and fold from [0].
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

## Display conversion

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

## Equation realization and snapshots

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

After numbering and supplement passes, replace each equation's canonical
entry with its public Styled view: raw numbering, default/effective
number-align, realized supplement, effective alt, unchanged body/block.
Use IntrospectedContent with absent fields to preserve that existing realized
projection; do not erase the realization or alter Heading's present snapshots.
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
