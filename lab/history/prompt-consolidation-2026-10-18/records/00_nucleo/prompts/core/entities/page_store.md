# L0 — Sealed page metadata store
Hash do Código: 2f81dfd9

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/page_store.rs`

## Evidence and resolved specification

Retained architectural evidence is
`lab/typst-original/crates/typst-layout/src/introspect.rs:38-58,22-145`.
Sealing uses a separate immutable sub-store. Existing later raw-Numbering and
realized-view clauses supersede the old EcoString-only schema; this comes from
the L0 amendments, not from adopting code as authority. Earlier staging and
conflicting derive notes are recorded in the sanitation diagnostic.

## Construction and queries

`PageStore` is pure L1 with private storage, Debug/Clone/Default, immutable after
construction/injection. `empty` is the default sentinel: total_pages is None,
`is_empty` is true, and every page query returns None.
`from_total_pages(NonZeroUsize)` supplies only total and empty metadata:
`is_empty` is false even when individual queries return None.
`from_runtime(total, Vec<Option<Numbering>>, Vec<Content>)` remains compatible;
it uses physical one-based indices as logical snapshots and supplies no realized
numbering views. Caller-provided numbering/supplement slots align with total.
`from_realized` accepts total, raw Numbering slots, logical numbers, spans,
visible-numbering slots, reference-numbering slots and supplements in physical
page order. All vectors align one-to-one with total pages.
Raw numbering, the realized two-argument marginal view and the realized unary
reference view remain distinct. Missing view slots differ from present empty
Content. No Engine or callback is executed by the store.

`total_pages` returns the optional total. Nonzero one-based accessors
`numbering_for_page`, `logical_number_for_page`, `numbering_span_for_page`,
`visible_numbering_for_page`, `reference_numbering_for_page` and
`supplement_for_page` expose corresponding snapshots. Raw numbering returns
`Option<&Numbering>`; views/supplements return `Option<&Content>`.
Outside available slots, return None without panic. An absent supplement slot
differs from a supplied empty supplement. `is_empty` tests only absence of total.

## Boundaries and unresolved restriction

Capture, injection and query routing belong to their pipeline/introspector
owners, including `00_nucleo/prompts/core/entities/introspector.md`. Tracking remains
at the Introspector boundary, not on a reference-returning store impl.
The earlier explicit no-PartialEq restriction conflicts with the current derive
and is unresolved; sanitation does not approve a public trait change.
No I/O or mutable global state is allowed. Verify sentinel/minimal/full stores,
aligned slots, distinct raw/realized views and nonpanicking range queries.
