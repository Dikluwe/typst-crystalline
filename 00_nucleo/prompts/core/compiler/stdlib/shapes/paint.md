# L0 — stdlib/shapes/paint
Hash do Código: 3c4ed4ca

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stroke-paint-inputs.toml sha256:b92ae8f26e4ebe8d5b7d0f1ee0e346e97210a9afb4c00ed1c9c184f690a263a8

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/shapes/paint.rs`. At HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` with a clean tree, `01_core/src/compiler/stdlib/shapes.rs:26-76` contains the extracted behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. This is internal separation by behavior.

## Contract

Convert typed colors, Paint and Stroke for shapes, preserving type, thickness, diagnostics and fallback only where contracted.

Detailed rules, historical evidence, parity limits and diagnostics remain in `00_nucleo/prompts/core/compiler/stdlib/shapes.md`. Extraction preserves public signatures, defaults, phase and compatibility. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and shapes tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.

## Paint and uniform-dictionary strokes

### Evidence before decision

stroke-paint-2026-10-14/before/results.json.gz retains HEAD/working tree/diffstat/
UTC/sources/owners/commands and both binary hashes. layout/stroke.rs:53–63 limits
named paint to Color; shapes/paint.rs:45–64 sends other paints to the legacy
Length/Color/Stroke caster; shapes/rect.rs:40–43 discards all Dict. The uniform
control renders 1pt black in Tekt; named Gradient/Tiling/Auto and circle Gradient
are rejected. Historical vanilla pin 9f2b6e871 accepts them. Empty Dict draws a
black Tekt stroke and no vanilla stroke. Pinned stroke.rs:236–243 and
sides.rs:238–258 distinguish smart Paint, empty and uniform dictionaries.
square(size:...) fails Tekt dimensions and is not stroke-cast evidence; its
dimensions/defaults remain open and outside this contract.

### Exclusive contract and acceptance

parse_shape_stroke forwards Gradient, Tiling and uniform Dict to existing
positional_stroke without reducing Paint to Color. Preserve legacy Color/Length/
Stroke and thickness behavior and String-specific rejection. The constructor
owns absence. Existing uniform conversion validates rich fields/unknown keys.
Do not change the generic layout caster or add render logic. ADR-0127 internal
parity flow requires L0 first, RED→GREEN, restricted resealing, build/suite/
lineage. Preserve paint meaning and available form; vanilla byte equality is not
a requirement. Collect PDF/raster/errors and retain unfixed discrepancies.
