# L0 — Bounded evaluation products across source revisions
Hash do Código: bd549fd4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-root-retention.toml sha256:08e9f23ba773c1fd5ed43bf67a985c2593f0c002b0fb6084ef9f883a0ef020c7


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/subtree_session.rs`
**Status:** laboratory experiment; user waived laboratory approval gates on
2026-11-12. No integration. This increment is intentionally incomplete: it
establishes effect-certified loop product reuse; general evaluation and layout
reuse remain the next experimental scope, not an implemented promise.


## Evidence before decision

HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2, uncommitted laboratory composition,
freeze-incremental-source.json and incremental-source-validation/reuse-boundary-audit.json
bind exact source hashes. eval/mod.rs:779–920 creates fresh contexts/scopes and
evaluates the root; control_flow.rs:101–192 reevaluates the iterable, binds every
item in a local scope, ticks the budget and executes/joins every body. The old
whole-root prototype misses whenever main text changes. The current Source
preserves intact CST spans but has no result cache: twelve pairs measured
19.901/19.798ms base/candidate versus3.342ms vanilla for100paragraphs, mixed
paired deltas. These are mechanics, not semantic equality claims.
markup_stream.rs:118–145 consults current World Source and parent to establish
content origin; textual.rs:373–431 can capture contextual recipes. Thus span
or text alone, arbitrary Value equality, or retention of an old Engine is not
a valid key. Side effects and source origins are required inputs.

## Explicit session and admission

Expose SubtreeSession::new()/Default and evaluate(world,source,full_error,target,
features)->(SourceResult<Module>,Vec<SourceDiagnostic>), plus stats for mechanical
hit/miss observation. The session belongs to one watcher and owns only bounded
RAM data; no World, clocks, files, mutable globals or thread-local cache. The
ordinary public eval facades remain cold. Inject the memo through a private
EvalContext field using an additive internal entrypoint; keep fresh stdlib identities (with the separate bounded root seed below),
syntax validation, root execution and realization. No root/module result reuse.

The initial cache boundary is the completed for-body accumulation, after the
iterable was actually evaluated in the current context. Admit only simple
identifier patterns, primitive items/bindings and a ContentBlock comprising
plain text/space/paragraph boundaries, literal scalars and identifier reads.
Inspect the real syntax, not fixture names or marker strings. The initial experiment declined calls; the certified call/markup extension below
supersedes only that restriction. Continue declining closures, code mutation, nested loops, labels, state/counters, context/math,
show/set rules and unknown syntax. Empty active show rules/guards and absence
of control flow/context/realization are required. The body's source and parent
must be present and match in the current World; failed source queries decline.

Key exact syntax payload/shape and every causal span, source/file identity,
pattern name, ordered current iterable items, all free primitive bindings,
ordered raw lexical style deltas and target/features/execution flags. Scalar
keys retain variants and floating-point bits, never numeric language equality
or a digest alone. Custom style values unsupported by the exact scalar proof
decline. Compare complete typed style state mechanically, never use this as a
definition of language equality. Keep raw levels, not only collapsed styles.

Replay only successful products. No error or warning cache. Syntax diagnostics
are emitted normally before evaluation; this admitted body cannot generate
callbacks, escaping bindings, rules, locations, state events or World reads
apart from the independently validated Source origin. Account for exactly the
skipped loop iterations; if the remaining budget cannot accommodate them,
execute the ordinary path so first error and span remain correct. Decline on
flow or incomplete proof. Misses preserve the original evaluation order and
failure behavior; store only after successful completion.

Retention is bounded by entry count and admitted syntax/item/value payload
budgets. Define/check these bounds explicitly and expose retained count in
tests. They are logical payload limits, not an allocator/RSS guarantee; record
actual RSS separately. Never grow indefinitely across revisions. Drop all
entries with the session; independent sessions must start empty.

## Acceptance and refutation

Tests first: a cold facade with zero hits must fail the warm untouched-body
case at runtime. Compare warm versus fresh text, semantic values and diagnostics
after prefix/suffix structure changes, changed body/iterable/free bindings,
inputs/import-derived values, style changes, show/context/effect refusal and
source origin rebinding. Exercise budget accounting, independent sessions,
bounds and negative mutations removing key dimensions. Report counters only
as mechanism evidence; use real watch PDF/SVG/query/geometry/diagnostic/font
controls for language preservation. Measure paired CPU against frozen baseline
and pinned vanilla; no success claim from hits alone. Full suites/build/lint/
Clippy/Rustdoc/fmt and lineage gates remain mandatory before freezing a release.


Initial bounded policy: at most16 products,8MiB of charged logical payload,
1MiB per product,8KiB body syntax and4096 primitive items. Charge syntax bytes
plus descendant count times retained scalar size and four Content slots per
iteration, plus keys/styles. Checked arithmetic overflow declines. This is an
admission estimate, not exact allocated bytes. LRU eviction preserves bounds;
measure live RSS separately. Empty iterable, over-budget and unsupported cases
retain the original path. Counters are cumulative per explicit session.


Charge pattern/binding names and font-list names/variants/regex patterns retained
by the key, including optional spot colorant names. Keep exact numeric bits for
typed style Length/TextEdge and spot tint in addition to complete typed state
comparison, avoiding signed-zero conflation. Compiled regex allocator footprint
is not the declared logical-payload metric. Budget/bounds tests exercise actual
eval_for with current scopes/Engine and compare its failure span with cold eval.

## Previous builtin root experiment — superseded by laboratory graph contract

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

Own a separate initially empty RefCell<Option<Arc<StdlibSnapshot>>> root seed. A private stdlib(inputs,features) delegates to snapshot owner. Default/new remain lazy; loop memo budget/statistics unchanged. No new public API, no retention of registry constructors or user-produced bindings. Root seed is independent of delayed import snapshots.

## Measurement before decision — laboratory builtin graph, 2026-11-13

Base is frozen builtin-root, HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2,
uncommitted laboratory composition. freeze-builtin-root.json records exact
UTC/source/binary hashes. Its census still observes1411 Scope::define calls
per plain/layout warm compilation despite zero registry builder calls.
stdlib_snapshot.rs:42,80-248 recreates Modules/functions; eval/mod.rs:877-917
and modules.rs:60-87 repeat global composition. This is construction mechanics.

builtin-identity-audit/receipt.json records pinned vanilla64c5e8c0 queries at
2026-11-13T07:16:50Z and negative controls at07:19:05Z, source/fixture hashes,
argv, HEAD and checkout diffstat. Language comparisons root/import sym,std,
calc,raw.line return true upstream and false in line-products. Local aliases
stay equal and a user module named calc remains unequal in both. Upstream
typst-library/src/lib.rs:231-247 constructs one Library graph, and typst-eval/
src/lib.rs:42,67,105,151 gives each file that library. This corroborates observed
language behavior, not an assertion of upstream intent from behavior alone.
Sharing the immutable builtin graph is an inference supported by these probes;
stale inputs/features, authority leakage, changed diagnostics or failed negative
homonym/registry tests refute it. Separately created raw.line.with() equality
is a known independent debt and is outside this experiment.

User authorizes experiments without another permission, laboratory only.
The current contract below supersedes the old per-file builtin instantiation
requirement, including the prior deliberate cross-file std equality debt.
No changes to Module/Func/Value equality or user-module evaluation are authorized.

## Current exclusive laboratory graph contract

The separate lazy stdlib slot now retains the immutable composed graph. Its private stdlib method returns Arc<StdlibSnapshot> through root_resolve. Loop cache budgets/statistics remain unchanged; admission follows the certified extension below. Root binds the graph into the fresh EvalContext, allowing imports to reuse only matching builtin state. Same-session same-key exposed sym/raw.line stay equal across evaluations, while newly evaluated user With stays fresh under its separate existing contract. Changing inputs/features replaces the graph. Separate sessions remain isolated.


## Certified markup and native-call loop products — laboratory 2026-11-13

### Evidence before decision

Frozen builtin-graph, HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2,
uncommitted lab tree, exact files/UTC/recipe in freeze-builtin-graph.json.
One twelve-pair campaign: outline24.126ms versus vanilla4.266ms, six better
pairs versus base and no demonstrated gain. Independent census leaves12loop
bindings each warm compilation; body_reads atsubtree_session.rs:216-254 rejects
Heading and FuncCall. The intact outline fixture repeats heading,lorem(30),
pagebreak twelve times. The profile records248direct native_lorem samples out
of2548outline samples; these are samples, not exact exclusive milliseconds.

pure-loop-audit/receipt.json binds source/lock/corpus hashes, HEAD/diffstat/UTC.
markup.rs:29-51,60-117 pushes only local style deltas for Strong/Emph/Heading;
intercept.rs:16-27,154-156 returns these unchanged when show rules are empty.
pagebreak.rs:17-63 validates args and constructs content without using ctx/World.
lorem.rs:33-143 validates args, mutates only context-private immutable training
and deterministically generates text. call_dispatch/apply.rs:41-96 restores
temporary context-read state; native calls do not consume closure route depth.
Func Native stores the actual call pointer; names alone do not certify effects.
Lipsum0.9.1learn uses split_whitespace; measured ASCII corpora have maximum
token lengths13and17bytes and no consecutive -- tokens. Thus64bytes per
requested non-dash word plus one terminal byte is a conservative output bound,
including separators/en-dash and capitalization. Keep a corpus qualification
test so dependency/data changes refute this bound rather than silently widen it.

This is bounded reuse of successful evaluation products, not a change to Typst
semantics or Rust equality. Inference is refuted by changed output morphology,
diagnostics, skipped effects/budget, stale inputs/bindings/styles/source, unknown
native acceptance or retained charge exceeding the stated bounds. User already
authorizes laboratory work without further approval; no integration.

### Exclusive implementation contract

Extend the existing body certificate with Heading/HeadingMarker, Strong/Star,
Emph/Underscore and pure FuncCall. Existing source-origin, full style/float-bit,
features/target/flags, show-rule/active-guard/context/flow and budget guards stay.
No namespace, phase, public signature, World method or global mutable state change.

For a call, require an identifier callee distinct from the loop binding and
resolve its CURRENT value from scopes before lookup. Accept only Native with
no namespace and actual pointer equal to native_lorem or native_pagebreak;
all other FuncRepr variants and unknown pointers decline. Retain callee-name
plus the private closed builtin kind in the key, compared in order. This
supports aliases without trusting spelling or Func's name-based equality.
Do not retain arbitrary functions, closures, modules, Engine or World.

Arguments must remain in the effect-certified syntax subset. Reject spreads;
visit positional expressions and named expressions, never treating a named key
as a free binding. Lorem requires one positional literal Int or primitive Int
identifier read (loop binder or free scope), with all values in0..=4096 for
admission. Compute the maximum across ordered loop items; include every actual
item/free read in the existing exact key. Wrong shape/type/range declines into
normal evaluation and its current diagnostics; never cache errors. Pagebreak
accepts named pure expressions; native validation remains on the ordinary miss
path, so only successful results can be retained. No new user-facing limit.

Add every call's conservative output bound to the per-iteration charge using
checked arithmetic, before the existing1MiBentry/8MiBtotal/16entry gates.
Include retained callee names/kinds. Bound exhaustion must preserve ordinary
execution; never truncate source/output or alter old cache entries. Existing
loop-budget replay and successful-result storage boundary remain unchanged.
No widening to arbitrary calls, Raw, math, labels, nested loops, set/show,
context, state/counters, field/method access, mutations or user constructors.

### Tests and acceptance

Runtime RED must show that the real unchanged outline-like body, then repeated
aliases/variable arguments and styled headings, are evaluated again. Compare
warm versus cold full values/content morphology and diagnostics, not merely
hit counts or plain text. Check changed literal/free/loop arguments, body and
style revisions, Source::reparse prefix/suffix changes, inputs, builtin alias
shadowing by a closure, unknown native with the same name, With/engine-native
refusal, show/state/context refusal, wrong arguments with current error spans,
budget near limits, independent sessions and output-charge refusal. Corpus
qualification checks ASCII/maximum token bound/no consecutive -- and generated
boundary cases. Preserve the prior controls, including exact float bits.
Full suites/build/lint/Clippy/docs/fmt, paired release timing against frozen base
and pinned vanilla, source/font/semantic/output controls and independent work
census/profile remain required. Gains are unproven until measured.



### Control qualification

The RED effect-refusal fixture compared independently constructed state-update
closures with Rust PartialEq and failed even on two cold evaluations. Preserve
that evidence; it is not a cache regression or language equivalence test.
For refused effectful cases, compare evaluated text/warnings and require zero
reuse; full watch state/context fixtures remain the end-to-end semantic control.
For the admitted pure subset, retain exact content comparison as a mechanical
regression guard AND exercise the actual Typst equality operator for morphology.
Direct classifier tests must reject homonymous unknown natives, namespaces,
With, engine natives and user elements without executing them. Validate zero,
upper admitted count4096 and refused4097 through real session evaluation.

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

## Approved text gradient package

On 2026-11-24 the owner approved (“Aprovo o pacote”) the scope in
`00_nucleo/diagnostics/text-gradient-fill-package-2026-11-24.pt-BR.md`. Gradient text fill is outside the bounded style payload and stays cold.
Verify a gradient case that asserts paint and a solid control.
