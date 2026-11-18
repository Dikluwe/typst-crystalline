# Prompt L0 — entities/scope
Hash do Código: c5a23adc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-root-retention.toml sha256:08e9f23ba773c1fd5ed43bf67a985c2593f0c002b0fb6084ef9f883a0ef020c7


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/scope.rs`
**References:** ADR-0023, ADR-0017, ADR-0018.


## Evidence before decision

At received HEAD `0a15ccebf861e3cc0ea16d0397d55dc057664556`, before this reservation change,
`01_core/src/entities/scope.rs:63-69` stores an ordered
`IndexMap<String, Binding, FxBuildHasher>` and constructs an empty map;
`:74-75` inserts bindings and `:96-97` exposes its current length.
`01_core/src/compiler/eval/stdlib_snapshot.rs:80-85` receives an immutable
original Scope and fills a fresh destination in declaration order. Its known
cardinality permits an initial RAM reservation under ADR-0029/0030. This is
construction mechanics, not a language-parity criterion under ADR-0107.
The inference that reservation preserves language behavior is refuted by any
change to values, declaration order, replacement, aliases or identities.

## Domain contract

Scope maps names to Binding in declaration order using
IndexMap<String,Binding,FxBuildHasher>. Preserve order during iteration and
when replacing an existing name. Binding holds only value: Value, with
new(value)->Self, value(&self)->&Value and into_value(self)->Value.
The original Value(()) examples describe the construction-era placeholder,
not a requirement to replace the current Value domain with unit.

Preserve Scope::new()->Self, Default, define(&mut self,name:impl Into<String>,
value:Value), get(&self,&str)->Option<&Value>,
get_binding(&self,&str)->Option<&Binding>,
iter(&self)->impl Iterator<Item=(&str,&Binding)>, len()->usize and is_empty()->bool.

Add public `Scope::with_capacity(capacity: usize) -> Self` as an additive
constructor for an empty Scope. Construct the same map with
`IndexMap::with_capacity_and_hasher(capacity, FxBuildHasher)`, reserving room
for at least the requested number of distinct bindings. A zero request has
zero allocated capacity. The reservation is an initial provision, never a
binding limit; later insertions may grow it. Preserve declaration order,
in-place replacement, lookup, mutable lookup, cloning and growth semantics.
Keep every existing production method and Default behavior unchanged. Do not
add a capacity field, capacity getter, observer or public test portal.

Capturer is a Debug/Clone/Copy/PartialEq/Eq enum with Function and Context.
It records why a scope was captured, enabling the distinct mutation diagnostics
"...outside the function..." and "...outside the context expression...".
It is one value per closure invocation, stored in ClosureRepr.capturer and
propagated to Scopes at call time, not a per-binding BindingKind flag.
See `00_nucleo/prompts/core/entities/func.md` and the compiler scope owner.

## Boundaries and rationale

BindingKind, Span, Option<Category> and Option<Box<Deprecation>> remain omitted
fields under the original ADR-0017 scope; Span availability alone does not
authorize adding it. Their historical unmigrated-type rationale is preserved
in the archive and is not a claim about current migration status. Scopes<'a>
(parent-lifetime stack) belongs to eval mechanics, outside this entity;
consider inclusion only if required for Module through an explicit contract.

IndexMap preserves declaration/import/shadowing iteration semantics and O(1)
lookup. FxBuildHasher follows ADR-0018 for typically short identifiers;
the original faster-than-SipHash rationale is historical, not a fresh benchmark.

## Acceptance

An empty Scope returns None from get, is_empty true and len 0. A defined x
returns its Value. Insertion [z,a,m] iterates [z,a,m]; redefining a after [a,b]
keeps [a,b], not [b,a]. Preserve both Capturer variants and diagnostic distinction.

Independent inline regressions must preserve all existing tests and inspect
private map capacity only from this entity's test module. Cover zero capacity
without allocation and empty length, then requests 1, 3, 4, 7, 8, 17, 257 and
4096 using unsorted owned keys. Initial capacity must be at least the request;
it must stay unchanged while inserting that many distinct names. Do not
assume exact capacity or an IndexMap growth factor. Check replacement position,
get_mut and capacity preservation, lawful growth beyond actual initial
capacity with order/values intact, clone independence, and fresh small scopes
after a large scope using the current small length. Use pure primitive Values;
no I/O, clock, globals, unsafe or allocator instrumentation. These assertions
verify the reservation contract mechanically; language acceptance remains
values, ordering and scope behavior.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/scope.md`. Historical observations are not a new parity certification.

## Laboratory builtin root retention — 2026-11-13

### Measurement before decision

Frozen line-products source at2026-11-13T06:46:29.065697+00:00 (exact UTC/hash in
freeze-line-products.json), HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2,
uncommitted lab composition. Its single twelve-pair campaign records plain
3.381ms vs vanilla1.706ms,100paragraphs14.820vs3.764,outline24.697vs4.208.
Twelve raw profiles show evaluation42.8% inclusive of264plain samples,
11.67% of1508layout samples and20.99% of2630outline samples. Counts overlap
and are not exact time nor promised savings. Scope::define, symbol_from_codex,
emoji registration and symbol destruction appear directly in plain samples.
eval/mod.rs:877-917 builds builtin stdlib each root compilation;
stdlib_snapshot.rs:42-65 only reuses an import-tree seed and80-248 rebuilds
scopes/modules/identity-bearing functions; scope.rs:64-66 owns and deeply
clones its ordered map. module.rs:65-69 uses Module allocation identity;
func.rs:407-432 preserves Element/With identity. Reusing those wrappers would
change observable identity. Pure namespace storage sharing is mechanics;
ordered bindings, values, diagnostics and per-file identities are language.
User authorizes all laboratory experiments without another approval, no integration.

### Owned responsibility

Use Arc around the private ordered map for O(1) clone, keeping every public signature. new/with_capacity preserve map capacity behavior. define/get_mut detach with Arc::make_mut before write. Read methods borrow current map. Test actual shared binding storage before mutation and independent values/order after mutation; no new observer API. This is an immutable storage mechanism, not a requirement to share Module identities.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.
