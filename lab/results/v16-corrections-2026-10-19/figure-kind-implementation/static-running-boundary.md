# Static running content: measured carrier boundary

The frozen additional rendered-control worker includes a source using
`set page(header: ...)`. Its strict failure remains in
`candidate-target-controls-{1,2}/receipt.json`; neither the source nor expected
result has been changed. It must not be reported as a successful control.

The approved semantic catalogue treats `SetPage` as an operational leaf and
`PageRun` as the content-bearing page element. Current `SetPage` retains a delta
until layout; `wrap_page_columns` does not convert it into `PageRun`. Realization
and introspection cannot infer physical repeated header occurrences from that
configuration marker. Moving callbacks into layout or counting a configured
header once per marker would not establish physical-page parity.

The separate `native-page-running/receipt.json` measures actual source-level
`page(header: ...)`, which constructs `PageRun`: the pinned oracle and frozen
candidate-build-2 both render `H Figure 1: HC B Figure 2: BC (2,)`; the identified
baseline renders header 2/body 1/global 1. Exact source hashes, executable hashes,
commands, UTC, HEAD and dirty state are retained. The candidate is the stable
binary bound to `candidate-build-2/receipt.json`, not an unrecorded current build.

This identifies the supported carrier, rather than certifying all page
configuration or repeated physical-page admission. The source `SetPage` header
gap remains visible and open; no new SetPage phase is implemented or silently
classified as passing. Final reports must distinguish approved PageRun static
content coverage from this additional unresolved source-configuration path.
