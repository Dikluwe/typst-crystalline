# Independent raw-footer implementation review

No substantive finding. Entry admits emitted content; of21 recursive child edges,20 inherit emission state and only raw Table.footer enters dormant state. No recursive path reenters the emitted wrapper, so Source/Styled/Realized/nested Table/Cell cannot reactivate dormant diagnostics. Table-row-slot selection remains separate.

Heading fields still convert. The one warning constructor is guarded before creation, with unchanged literal payloads; no collect/filter/discard operation exists. The renderer is unchanged. Old/new test inputs, literals and expression tokens remain intact after allowing root formatting's whitespace/optional trailing commas. Fresh pinned resolved-footer evidence requires real tfoot output and the exact warning/two hints.

This preserves an existing renderer/effect boundary rather than suppressing emitted warnings. The L0 explicitly leaves raw-footer omission as an internal rendering debt, not desired language behavior. Actual GREEN and final source-bound CLI/structural/lint/API/lineage gates remain separate obligations. No Cargo, productive or L0 changes were made by this review.
