# L0 — stdlib/shapes/line
Hash do Código: 1e3a12bb

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/shapes/line.rs`. At HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` with a clean tree, `01_core/src/compiler/stdlib/shapes.rs:351-506` contains the extracted behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. This is internal separation by behavior.

## Contract

Construct line with start/end/length/angle and dx/dy extension. Preserve precedence, incompatibilities, Path for nonzero origin and causal box dimensions.

Detailed rules, historical evidence, parity limits and diagnostics remain in `00_nucleo/prompts/core/compiler/stdlib/shapes.md`. Extraction preserves public signatures, defaults, phase and compatibility. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and shapes tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.


## Pure numeric projection

### Evidence before decision

shapes/line.rs:134–135 contains disallowed_methods calls reported by current
Clippy. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz`
retains source/owner/config/libm/compiler/UTC/status/diffstat. host-math compares
backends before choosing: 4968 cases, 69 trig/atan2 differences up to 1 ULP,
without numeric-class or signed-zero changes; powers/hypot agree in the sample.
This is host measurement, not universal proof or new vanilla parity.

### Contract and acceptance

Use libm::cos/sin to project length/angle into dx/dy. Preserve start/end, argument
precedence, rejections, Path origin and causal dimensions. Add no rounding,
clamp or default. ADR-0127 internal continuous flow adds no public contract,
default or phase. Preserve existing geometry/language tests, compare measured
SVG examples before/after and report differences. Geometry comparison does not
certify all cases/platforms. Validate build/all targets/core/Clippy/lineage.
Do not suppress the lint or substitute compiler-byte identity for acceptance.
