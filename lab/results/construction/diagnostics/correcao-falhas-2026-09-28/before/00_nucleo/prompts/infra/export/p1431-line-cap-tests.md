# L0 — infra/export/p1431-line-cap-tests
Hash do Código: 2bec9e0b

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/p1431_line_cap_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/p1431_line_cap_tests.rs:29` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Independent raster/PDF line-cap observations

Own only the existing cfg(test) consumer and its fixtures/assertions. Construct
Butt/Round/Square through math.mat, isolate the transported line and observe actual
exporters. Keep the frozen effective automatic thickness 0.05*11pt=0.55pt.
Raster endpoints must distinguish Butt < Round < Square; automatic augment matches
the explicit Square witness. Preserve the existing deterministic local byte guards
for that witness and the legacy Butt line, not a universal byte-parity claim.

The PDF parser tokenizes operators, verifies balanced q/Q, selects the internal
scope containing w/m/l/S, and attributes its sole J before the line path in that
same scope. Round requires 1 J, Square 2 J, and Butt no redundant 0 J at top/local
drawing. An outer scope alone cannot attribute cap.
Missing/truncated/unattributable stream, pixel or line identity is exactly
UNKNOWN_EXPORT_OBSERVATION and never receives acceptance credit.

## Ownership boundaries

`00_nucleo/prompts/infra/export/render.md` owns raster Line cap mapping;
`00_nucleo/prompts/infra/export/stream/shape.md` owns PDF cap primitives;
`00_nucleo/prompts/infra/export/stream/draw.md` owns their drawing scope.
This suite does not claim equivalent SVG or Shape-stroke cap support.
