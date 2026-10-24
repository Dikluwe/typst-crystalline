# V16 absence classification and exhaustive dimension projection

The nine V16 findings are resolved. All nine pointed to bare `None` patterns
that the linter parser classified as catch-alls. Qualifying the existing unit
variant as `Option::None` retains exhaustive matches and language behavior.
Executable controls confirm the distinction and retain detection of real
wildcards with the original configuration and severity.

The audit also found the actual `Some(_)` evolution gap in `resolve_pt`.
It now names the complete current Value complement. A temporary extension of
the real enum produces E0004 at this exact match after repair; the original
accepted it there. The enum was restored byte-for-byte after both measurements.
No public API, product default, pipeline phase, lint rule or exclusion changed.

Validation: 9,816 workspace tests passed, zero failed, 7 existing
ignored; build/all-targets, strict Clippy, rustdoc, fmt and CI helper tests pass.
Installed and CI-pinned full linter scans have zero errors/warnings/V16.
Informational V19/V20 remain visible. Both lineage directions and mature paths
pass; 1,003 consumers have no reverse-lineage errors. Only eight owner/consumer
pairs changed in this batch; prior unrelated work remains intact.

Provenance, source deltas, exact snapshots, controls, mutation/restoration times,
binary identities and complete gate receipts are retained in the
[completion report](../../lab/results/v16-option-exhaustiveness-2026-10-23/README.md).
This supersedes the nine-open-V16 status at the PDF-warning batch's closure.
