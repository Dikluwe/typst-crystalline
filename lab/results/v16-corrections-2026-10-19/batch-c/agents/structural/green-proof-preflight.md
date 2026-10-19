# Green structural-proof preflight

Read-only preflight at 2026-10-19T13:17:34.528185+00:00.
Worker SHA-256: `1b4e95ec4c49531ab235bd26e1f6b676e7d498a004fb3cad1f04e1cb78b5cbb8`.
No Cargo command or productive edit was performed.

## Readiness

The targets resolve to the intended productive decisions in the current tree:
027 selects the exhaustive identity match after `node_kinds!` at
`selector_matching.rs:328`, not the earlier transport passthrough; 011 selects
its Content decision at `expression_dispatch.rs:822`, with the separate nested
Metadata Value decision at `:773`; 012 selects the outer conversion at
`markup_stream.rs:505`. Type and font-name nested decisions resolve at
`call_dispatch/instance_methods.rs:45` and `rules/set/text.rs:330`.
The remaining owner-scoped anchors resolve to their selected productive matches.

The worker copies complete sources/manifests, injects a unit sentinel into actual
Content/Value/Type/Selector/ElementKind/Expr/P1293HtmlNative enums and, for green,
FuncRepr, then compiles the full crate. It restores every original Rust file
between jobs. Stored baseline mutation files confirm actual declaration changes;
Value and Type each receive a sentinel in their shared file. No stub substitute
or original-tree enum mutation is used.

For each pair axis, covering all right-new cases leaves left-new/right-old
uncovered; covering all left-new cases leaves left-old/right-new uncovered;
covering both mixed classes leaves new/new uncovered. The same isolation is
applied separately to Selector and FuncRepr. Each selected failure must be E0004
at the owner's primary source path, line and anchor byte interval, not merely an
unrelated build failure. Pair jobs additionally require sentinel text. The enum
is the only domain extended in each isolated pair job.

## Baseline scope and limits

`structural-baseline/results.json` records exactly the 19 original selected IDs
in its all-domains job, each lacking a selected E0004 diagnostic; each isolated
Selector pair job also lacks the selected E0004. Exit101 arises from unrelated
exhaustiveness failures, with no non-E0004 errors. Its freeze receipt reports no
changed frozen inputs. The separate `structural-baseline-native/results.json`
records lack of E0004 at the old native-address decision when actual FuncRepr is
extended, with its own unchanged freeze.

The new nested Type/font-name/Metadata Value, receiver and contextual-validator
matches are green additions, not part of the original19 baseline claim. The
worker filters these green-only targets out of baseline selection. Native
identity baseline is separate from both groups. Green remains unexecuted by this
review; only resulting diagnostics can establish candidate success.

## Small receipt defect

The worker writes `results.json` before assigning `rec['isolated_axis']`.
Each following iteration persists the prior annotation, but the final
`native-new-new` annotation is not flushed. The actual isolation assertion and
job name/mutation evidence remain valid; this is a metadata persistence defect,
not a false E0004 pass. Assign that field before serializing, or serialize once
again after the loop. Reported to the coordinator before the expensive run.
