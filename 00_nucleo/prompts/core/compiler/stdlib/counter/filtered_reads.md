# L0 — Filtered counter reads and replay
Hash do Código: c210c7a2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/counter/filtered_reads.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before decision

compiler/stdlib/counter.rs:317–632 shares filtered-key identification, location
resolution, history reconstruction, *_owned methods and contextual replay.
eval/context_reads.rs:268 calls replay_context_read through the facade. This
measures mechanical tree ownership at commit 320c2149b, not upstream intent.

## Exclusive contract

Own only filtered-read/replay policy. Preserve validation order, causal diagnostic
origin, filtered-read marking, observe_context_read and causal event order.
Unfiltered keys delegate to existing facade methods. Replay only the Counter*
variants assigned to this owner; keep facade signatures and Counter representation
unchanged. Display belongs to
`00_nucleo/prompts/core/compiler/stdlib/counter/display.md`.

Acceptance preserves get/at/final/replay results and errors, including filtered
selectors, labels and absent context; static dispatch, build and lint must pass.
Claim no unmeasured performance benefit.

## Causal filtered-counter order

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at,
StateRegistry::value_at, before/after queries and counter events compare
Location.as_u128. Realization prepares identity before transforming, so the number
no longer orders originals/products. CLI probes and
single_evaluation_counter_follows_causal_order retain evidence in the
implementation diagnostic. Apply the pinned realization-order nucleus; keep
historical fallback only without recorded order. A filtered counter prefix at
Location uses the introspector's causal ordinal, never raw identity order.

## Ratified realized Page snapshot

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`; exact working tree/binaries are in
`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/measurement.json.gz`.
single prints B1/0 F0 versus B1/1 F1; two-page total/final 0 versus 2; initial
update 7 keeps total 7 versus 8; an intermediate update changes the previous
page (7/7 and 8/7 versus 1/7 and 7/7); final step prints final 1 versus 2.
Bundle prints 1/7 in its first PDF versus 1/1, final 7 in the second versus 8;
mixed HTML returns 1 versus 7. Historical paged.rs:360–365,840–913 uses the last
update as every page's start; context_stabilization.rs:969–1010 does the same by
body. compiler/stdlib/counter.rs:490–513 and counter/display.rs:96–110 read the
global causal final. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`,
introspection/counter.rs:246–254,730–773,934–963 combines document scope, updates
and physical advancement. This is language parity, not a requirement to copy
upstream memoization.

### Ratification and current contract

The human confirmed
`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/l0-manifest.json`,
SHA-256 `1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e`.
Its ADR-0127 gate is satisfied. Thus the page-counter-snapshot nucleus supersedes
the historical final scope-out for this unit. The public contract and page-aware
update realization were approval-gated; the archived preapproval instruction is
not a new pending gate. Subsequent internal clarification follows continuous
flow, L0 before code and RED→GREEN.

Preserve existing signatures, cycles and budget; do not guess page/total before
a known snapshot. Page readers use the realized snapshot at their Location when
present: current get/at/display reads page_counter_values; final/both reads
page_counter_final_values for the relevant contextual Location. Until then retain
the existing causal fallback without mixing partial input and another generation's
total. CounterAt/CounterFinal/CounterTotal replay uses that same resolution,
including errors and empty state. Other keys remain unchanged. The older logical
PageStore view cannot repair absent snapshots by inferring the final PDF.


## Modern final requires a contextual Location

### Measurement before decision

Pinned `lab/typst-original/crates/typst-library/src/introspection/counter.rs:471–478`
requires `context.location()` before `CounterFinalIntrospection`. In contrast,
`lab/typst-original/crates/typst-library/src/foundations/context.rs:38–49`
distinguishes location access from generic introspection: known styles alone
satisfy the latter, but do not provide the former. This is an observable language
capability boundary, not a requirement to copy upstream implementation mechanics.

The source-bound current-pin oracle and product measurements are preserved in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters/math-callback-capability-oracle/receipt.json`
and
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters/math-callback-capability-product-source14/receipt.json`.
The product's math callback has known styles and no Location; modern
`counter("x").final()` nevertheless succeeds with a default zero after an inserted
update7, while vanilla rejects unknown context. Source
`01_core/src/compiler/stdlib/counter/filtered_reads.rs:194–225` delegates
unfiltered keys before checking context and checks only in_context for filtered
keys. The public materialized helper then falls back to zero in the absence of
an actual final snapshot. That fallback cannot establish language permission to
read a document at an unavailable contextual Location. Exact pre-repair source,
owner bytes, hashes, HEAD and dirty-file provenance are preserved in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/realize/counter-final-capability/before.json`.

### Owning private correction

Within `counter_final_owned` only, classify the key once, mark filtered demand if
and only if the key is filtered, then require both in_context and an actual
current_location. If either is absent, return one Error at the actual call span
with `can only be used when context is known`, followed in order by the measured
hints ``try wrapping this in a `context` expression`` and
``the `context` expression should wrap everything that depends on this function``.
Do not synthesize a trace at this private read boundary. The same pinned
counter-final oracle receipt preserves the exact message, source range and both
hints. Marking remains before this error, preserving existing demand-selection
and causal ordering. Do not resolve
or execute a fold, callback or update on a rejected read. After this guard,
unfiltered keys delegate to the existing public materialized helper; filtered
keys use the existing complete history and final projection. Bound and static
modern language calls share this owned path, including replay with its recorded
context and Location.

The public Rust no-Engine `counter_final` signature, materialized behavior and
fallback remain unchanged. Keep legacy string-native routes unchanged. Do not
change get/at/query/state/display guards, construct a Location, install a snapshot,
force zero, broaden any public contract, add a context capability field, or add a
phase. This bounded correction follows ADR-0127 continuous language-parity flow;
the pinned page-counter-snapshot and realization-order Nuclei remain unchanged.
Known styles alone remain sufficient for the separately specified style reads;
this clause does not decide other introspection capabilities.

### Acceptance

Require an actual focused functional RED before implementation and GREEN after
it. Preserve the source-bound math counter-final rejection independently of
known-style text.size acceptance and here/get rejection without Location. Test
modern Str, Page and native Element keys with known context but no Location,
including populated materialized snapshots: all reject with that single sourced
Error, ordered pair of hints and no fabricated trace.
A filtered key with an available Func update must reject before executing that
Func; demand selection is still marked. Both bound and static calls retain the
same capability boundary. A real located context returns its correct final value;
noncontext execution remains rejected even with an assigned Location. Preserve
an explicit public no-Engine materialized-helper success control. Existing Page
snapshot, causal order, replay, never-touched contextual zero, diagnostics and
full counter acceptance remain required. Do not weaken tests, suppress alerts or
claim complete query/at/state/math parity from this correction.
