# L0 — Filtered counter reads and replay
Hash do Código: 7691b231

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
