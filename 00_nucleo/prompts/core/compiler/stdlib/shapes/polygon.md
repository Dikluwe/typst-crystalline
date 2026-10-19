# L0 — stdlib/shapes/polygon
Hash do Código: a5cac0a4

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/shapes/polygon.rs`. At HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` with a clean tree, `01_core/src/compiler/stdlib/shapes.rs:633-888` contains the extracted behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. This is internal separation by behavior.

## Contract

Construct polygon and polygon.regular as Path with points and analytical bbox, vertex count, fill/stroke, defaults and closure. Preserve the regular formula and degenerate cases.

Detailed rules, historical evidence, parity limits and diagnostics remain in `00_nucleo/prompts/core/compiler/stdlib/shapes.md`. Extraction preserves public signatures, defaults, phase and compatibility. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and shapes tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.


## Ratified Circle constructor origin

### Evidence and approval before implementation

At HEAD `954d13185e26a1ed90f6d7c60f6ba27760ca9889`, the source/owner audit in
`lab/results/v16-corrections-2026-10-19/circle-origin/l0-before.json` freezes
UTC, exact diffstat and hashes. The approved proposal
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md`
(SHA-256 `5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`)
binds the baseline equality and rejected full-Length candidate receipts.
`stdlib/shapes/ellipse.rs:92-104` erased the em component;
`entities/elements/shape.rs:21-29` used one geometry carrier for both constructors.
Those observations concern language identity and geometry, not mandatory upstream
Rust structure. The explicit user approval is recorded in
`lab/results/v16-corrections-2026-10-19/circle-origin/approval-receipt.json`;
it satisfies ADR-0127 section 2 for exactly this public type/field and migration.

### Exclusive approved responsibility

Migrate this consumer's existing generic ShapeElem literals to explicit
`ShapeOrigin::Geometry`, preserving every other field, fixture, expectation,
argument, algorithm and output. Equal axes or Ellipse kind do not infer Circle.
This is only the required migration for the approved field, not a new geometry
behavior or permission to materialize any unrelated pending proposal. Full-target
compilation and existing tests must retain all current observations/tolerances.
