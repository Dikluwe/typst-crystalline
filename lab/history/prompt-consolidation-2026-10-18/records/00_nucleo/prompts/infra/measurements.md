# L0 — Infrastructure measurement counters
Hash do Código: 7c7a5ab1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/measurements.rs`

## Current evidence

- `03_infra/src/measurements.rs:43`, `:48`, `:86` — eviction counters and fixed method matrix.
- `03_infra/src/measurements.rs:150`, `:158`, `:171`, `:182` — snapshots, reset and recording.
- `03_infra/src/measurements.rs:199` — `CountingIntrospector`.
- `04_wiring/src/eviction.rs` — the caller records eviction before delegating to comemo.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own the L3 diagnostic counters and the transparent `CountingIntrospector<I>` wrapper. `CacheStats` exposes eviction calls and last max age; it does not report cache hits/misses. `CallCounts` exposes total and fixed-order per-method observations, with `count_for` returning zero for unknown names. `reset` clears both matrices. Atomic operations remain Relaxed: snapshots are approximate concurrent observations, not synchronization guarantees.

Preserve the `INTROSPECTOR_METHODS` order and its 28 observed methods:

```text
query_by_kind, query_by_label, query_first, query_unique, position_of,
figure_number_for_label, query_metadata, formatted_counter, counter_values_at,
state_value, state_final_value, query, formatted_counter_at, bib_entry_for_key,
bib_number_for_key, figure_number_at_index, flat_counter_at, resolved_label_for,
headings_for_toc, headings_for_bookmarks, query_labelled, label_count, pages,
page, page_numbering, page_supplement, counter_key_for_label, counter_final_values
```

The wrapper owns its inner value and preserves `new`, `inner` and `into_inner`. Every implemented Introspector method delegates its existing return value unchanged. Increment only methods listed in the matrix at their fixed index. Auxiliary queries, including `element_at`, equation numbering/supplement content and `html_position_of`, delegate without a new counter. HTML positions are neither injected nor converted/stored; `page_numbering` preserves its typed `Option<&Numbering>` return.

Keep instrumentation in L3; no mutable globals enter L1 and the wrapper does not alter Introspector, layout, fixpoints or production defaults. Do not equate invocation counts with comemo cache hits. `00_nucleo/prompts/wiring/eviction.md` owns eviction composition; `00_nucleo/prompts/wiring/compile.md` owns the `CRYSTALLINE_MEASUREMENTS=1` opt-in dump and its default silence.

## Verification boundary

Existing tests check snapshots, observed delegation, count lookup, reset, eviction growth and uncounted HTML-position transparency. Do not infer a successful benchmark or upstream performance comparison from these diagnostic counters.
