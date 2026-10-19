# L0 — Counter display
Hash do Código: 7967c100

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/counter/display.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before decision

compiler/stdlib/counter.rs:666–845 contains display location/value/both/numbering/
callback policy; :304–315's native wrapper preserves public entry. This is tree
structure at commit 320c2149b, not performance comparison or upstream intent.

## Exclusive contract

Receive the facade's same Counter, arguments, scopes, context, Engine and span.
Validate contextual location/named arguments in the existing order; obtain
filtered history from the sibling owner; observe CounterAt/CounterTotal at the
same points; apply default numbering, explicit pattern or callback in the same
order and return the same Content/diagnostic. Public wrapper/counter_display
remain in the facade with unchanged signatures. Preserve defaults/casts/eval
phase. Acceptance retains text/content/spans/errors/warnings/call order for
implicit/explicit pattern, callback, both and filtered counters, with build,
focused tests and lint passing.

## Causal filtered-counter order

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at,
StateRegistry::value_at, before/after queries and counter events compare
Location.as_u128. Realization prepares identity before transforming, so the number
no longer orders originals/products. CLI probes and
single_evaluation_counter_follows_causal_order retain evidence in the
implementation diagnostic. Apply the pinned realization-order nucleus; keep
historical fallback only without recorded order. A filtered counter prefix at
Location uses the introspector's causal ordinal, never raw identity order.

## Logical-page display resolution

### Evidence before decision

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree recorded in
`lab/results/construction/diagnostics/contexto-page-display-2026-10-05/before-state.json.gz`,
#context counter(page).display() prints 0 versus historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8` printing 1. counter/display.rs:61–77
reads only CounterRegistry; counter.rs:378–390,463–482 already resolves get/at
using physical Location and logical PageStore. Upstream
introspection/counter.rs:271–291,456–472 shares resolved state for display_at/at.
Diagnostic receipts retain CLI evidence.

### Current responsibility and acceptance

Apply the facade's private page_counter_value for current display when
CounterKey::Page has realized Location/logical page. Without that view preserve
history/causal fallback; never invent page 1. Other keys retain history/order.
The observed CounterAt value is the value used by pattern/callback, coherent
with get/at replay. Preserve callbacks/patterns/validation/errors/public signature/
phase. The original CounterTotal/final scope-out is superseded only by the
ratified snapshot contract below; never use one bundle PDF's final state to
fabricate another's total.

Default/explicit pattern and callback use the occurrence's logical page;
pagebreak increments and a predocument update is honored; get/at stay coherent.
String/heading counters receive no physical page. Require integrated RED→GREEN,
existing counter/context tests, ratified-pin CLI and build/lint. Do not reduce
stabilization cycles/budget.

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

## Exhaustive numbering-style lookup

### Evidence before decision

At baseline HEAD `91ea6ee911`, `compiler/stdlib/counter/display.rs:124-133`
uses string-specific, bare-Kind-specific and Page-specific style keys. Its final
wildcard represents the eleven current non-Kind Selector forms, not an unknown
CounterKey. `entities/selector.rs:28-79` defines those forms; `render:59-113`
reads and observes the complete counter key before selecting a numbering style.
This source measurement is structural evidence, not a claim of complete upstream
counter parity.

### Contract and acceptance

Keep CounterKey exhaustive and decide inside Selector explicitly: Kind uses its
lowercase kind numbering key; NativeElement, Element, Label, Location, And, Or,
Regex, Where, Within, Before and After use `counter.numbering.pattern`. Preserve
full selector identity for history, filtering and context replay; do not normalize
these keys to a base Kind or change default discovery. String and Page keys retain
their existing paths. Context/location/named-argument validation precedes reads;
CounterAt/CounterTotal observations, explicit patterns and callbacks remain
unchanged. Tests must distinguish kind versus generic style keys, preserve
filtered identity and check context/errors/callback output. Require attributable
future-Selector exhaustiveness failure at this nested decision. Internal nominal
coverage follows ADR-0127 continuous flow with no public/default/phase change.
