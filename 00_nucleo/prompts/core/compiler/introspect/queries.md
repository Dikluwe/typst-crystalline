# L0 — introspect/queries
Hash do Código: ea7e0edc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:7e7373acce2b00247d18b9c5ea1b9d53833275997cd2c304e9d919d2cce10648

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/queries.rs`.
At clean-tree state `df72a6c4a701917ab56a7c6169764a92ea775693`, `01_core/src/compiler/introspect.rs:287-673` contains the behavior assigned to this unit. Source and state are recorded in `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. Separation is by behavior, preserving the API and without a performance claim.

## Contract

Implement Introspector for TagIntrospector through occurrence indexes. Filters, selectors and positions preserve causal order, Location identity, Before/After limits and snapshot projection under the existing contract.

Language invariants, causal order and parity limits remain defined in `00_nucleo/prompts/core/compiler/introspect.md`; this owner must not create a second interpretation. Code uses static L1 calls without vtables, I/O or mutable global state.

## Acceptance

Preserve introspection tests and the L1 suite, build, and pass `crystalline-lint .` with consistent hashes. Future semantic changes require language measurement and an L0 update before code.




## Causal query order

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at,
StateRegistry::value_at, before/after queries and counter events compare
Location.as_u128. Realization prepares identities before transformation, so
numeric IDs no longer order originals/products. CLI probes and
single_evaluation_counter_follows_causal_order retain evidence in the
implementation diagnostic. Apply the realization-order nucleus, retaining
historical fallback only without recorded order.

Before/after and native-element query ordering use recorded causal ordinals.
Locations remain opaque identities; after never infers position from their
integer. Sort native-selector unions by causal order and deduplicate only
prepared identity.

## Label filters on legacy absent snapshots

Evidence: after.json.gz records equation.where(label:<display>) returning zero
versus one upstream. walk.rs:242 retains Equation fields None;
from_tags/equations.rs:161–166 realizes fields None. LabelRegistry already maps
the Location, transported by query_helpers.rs:544 for None results.

Only when entry.fields is None, a Value::Label filter resolves through that
Location's LabelRegistry; other fields use existing element_selector_matches/
equality. Some, including empty, is authoritative and gets no index-completed
field. Fabricate no snapshot or text-derived label; preserve causal order and
other selectors. No signature/phase change, ADR-0127 continuous flow. Test positive
and Some-empty before code; invalid/absent cases require measurement.
label-oracle.json additionally measures none and string "display" failing,
Label <display> matching and <unknown> failing. Absence is not Value::None,
String is not Label; only Value::Label uses index fallback, other values retain
prior projection/equality without type coercion.

## Ratified bare table.cell selector boundary

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz`
retains UTC/diffstat/inputs/commands/source and executable hashes. Historical pin:
`9f2b6e8715237cb086899a42873660fe744622e8`. selector(table.cell) returns cell;
where() returns cell.where(:), comparison is false, and query/counter reject
nonlocatable cell. A genuine CodeBlock alias preserves those results. The
previous document-only alias fixture did not expose its variable to eval and
was not positive alias evidence; table-aliases-before corrects it.

Upstream foundations/selector.rs:309–321 distinguishes dict None from Some;
:421–450 validates locatability separately. Local entities/selector.rs:34–37
represents only a present Element group; stdlib/foundations/selector.rs:69–77,
348–354 omits native cell. d08-owner-audit.json retains other boundaries/hashes.
Form/identity/query rejection are language; the closed carrier is chosen
mechanics. Source explicitly distinguishes the states; do not infer intent from
output alone. The inference that the new variant covers selection/locatability
is refuted by a public absent-group/identity case it cannot represent. This does
not presume correct Cell realization.

### Ratification, responsibility and incomplete scope

The human confirmed the selector-only manifest
`f80f85c4ac3ab747d20d98ef3f5758c59727bcf5f5d762082660099aef4dcc9c`;
receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-selector-aprovacao-2026-10-06.json`
at 2026-10-06T22:36:30.591466+00:00. Its original ADR-0127 gate is satisfied;
the archived proposal-only instruction is not a new pending approval.

Consume NativeElement by real native-identity matching against registered elements, preserving causal order/Locations. Add no filter or fabricated snapshot: absent group is not empty filtering. Do not hardcode [] for the variant; prior validation must reject table.cell as nonlocatable before a valid query. Compositions retain limits/identity. Broaden no indexable catalog and register no TableCell.

This selector package covers carrier/construction/repr/equality/composition/
locatability only. The broader Cell domain needs separately measured direct show,
body/field projection and explicit/implicit cell preparation, including positions,
padding/spans/header/footer/lexical scope. Its own measured L0 and public/phase
gates are not authorized by selector approval. Preserve negative controls and no
universal parity claim. Atomic promotion, language RED before code, static/safe
Rust, 1:1/V26 before restricted owner resealing, build/core suite/linter remain
required. Revalidate bare/where/alias, true/false equality, direct/composed
query/counter, homonymous function and partial/closure; preserve D05/D07/query.


## Complete label-selector occurrences

### Evidence before decision

The frozen Circle replay failed `circle-control-geometry-shared-occurrences`:
only the first 40pt measurement was returned instead of both 40pt and 80pt.
`lab/results/v16-corrections-2026-10-19/circle-origin/agents/audit/context-investigation/receipt.json`
binds exact HEAD, dirty diffstat, UTC, source/binary hashes, commands and outputs.
Its direct-metadata control loses the second value even without Context or
Circle, while unique-tag controls distinguish occurrence production from
label selection. These language observations refute attributing the missing
result to Circle geometry, closure caching or its new semantic origin.

At the audited source, `01_core/src/compiler/introspect/queries.rs:129` delegates
Selector::Label to the first-only query_by_label. The existing multi-label
registry already exposes all registered Locations; its add preserves duplicate
pairs. `00_nucleo/prompts/core/entities/introspector.md:145` retains the separate
query_by_label compatibility projection, while allowing the later compiler-owned
selector contract. The realization-order nucleus requires recorded causal order
and deduplication of prepared identities, not sorting by raw Location integers.
`lab/results/v16-corrections-2026-10-19/circle-origin/label-query-before.json`
and its source archive preserve the audited owner/consumer before this decision.

### Exclusive correction and acceptance

For Selector::Label, collect every Location from LabelRegistry::lookup_all, sort
by the existing causal order key, and deduplicate identical Location identities.
Distinct occurrences remain distinct even when their content or label is equal.
Unknown labels return empty. Preserve query_by_label's first-insertion lookup
and Option<Location> signature, label_count/registry behavior, all other selector
branches and existing composition/boundary semantics. Perform no callback,
layout, cache invalidation, re-evaluation or new occurrence allocation.

This narrowly supersedes the historical first-only Label selector staging and
the parent catalog's preservation clause for that branch. It does not change
public APIs, defaults, pipeline phases or authorization of unrelated selectors.
Use ADR-0127 continuous parity flow: L0 before independent semantic RED, then
implementation and restricted reseal. Test duplicate labels, nonnumeric causal
order, repeated prepared identities, unknown labels, preserved first-only trait
lookup and current composition/boundary behavior. Re-run the unchanged frozen
Circle replay, including both occurrence-font measurements, plus introspection
and full suites, paths, direct/reverse lineage and exact lint delta. Never change
the frozen replay policy to accept the lost second occurrence.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Find prepared Figure histories by counter_keys_eq before registry fallback. counter_values_at, counter_final_values, flat_counter_at and formatted_counter_at agree and project by recorded causal order. A matching present empty Vec stays Some(&[]) for borrowed reads; absence is distinct. Never execute callbacks. Preserve figure_number_at_index implementation exactly; productive snapshots no longer promise automatic string history. Explicit filtered readers continue their independent full fold; no diagnostic cache.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.
