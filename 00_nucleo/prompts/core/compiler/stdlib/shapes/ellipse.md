# L0 — stdlib/shapes/ellipse
Hash do Código: 6e95aad8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shapes/optional-stroke.toml sha256:016cad80570d8133fcb275e6e0f693e088dc921e4d6d907a51888898ae8b00ce

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/shapes/ellipse.rs`. At HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` with a clean tree, `01_core/src/compiler/stdlib/shapes.rs:229-350` contains the extracted behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. This is internal separation by behavior.

## Contract

Construct ellipse and circle; circle converts radius to diameter and uses ShapeKind::Ellipse. Preserve fill/stroke and fallback.

Detailed rules, historical evidence, parity limits and diagnostics remain in `00_nucleo/prompts/core/compiler/stdlib/shapes.md`. Extraction preserves public signatures, defaults, phase and compatibility. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and shapes tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.


## Explicit absence correction

Before decision, visual-rect/visual-circle probes in
`lab/results/construction/diagnostics/paridade-dominios-2026-10-06/measurement.json.gz`
compile at historical vanilla pin 9f2b6e871 but Tekt rejects the stroke cast.
Upstream shapes.rs:125–129,335–339 distinguishes fill-dependent Auto from
Custom(None). The consumer always sent None to the required Stroke parser and
its fallback ignored argument presence. Correct language acceptance/form
without inferring historical intent.

Apply the pinned optional-stroke nucleus to argument reading/fallback for both
constructors. Do not change the shared required parser, still-unrepresented
per-side strokes, signatures, dimensions or radius. Test None with/without fill
and omission without fill, RED→GREEN. This ADR-0127 internal parity correction
adds no API/phase/default, unsafe code or dependency. Revalidate PDF/raster and
macro controls against the qualified ratified pin.
