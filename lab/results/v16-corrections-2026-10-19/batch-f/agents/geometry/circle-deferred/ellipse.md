# L0 — stdlib/shapes/ellipse
Hash do Código: 415c5bab

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

## Exhaustive argument decisions and measured rejection

### Evidence before decision

At baseline `91ea6ee91105b97b1aedf172e34780dddde14c94`, the exact source/L0
hashes and UTC are retained in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/initial-audit-state.json`.
The baseline eval/query receipts in that directory bind the rebuilt product
SHA to the ratified oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
They measure acceptance, diagnostic meaning, supplied-value provenance and
geometry, not Rust representation or rendered bytes. The initial audit separates
source evidence from inferred intent and records refuters.

`shapes/ellipse.rs:84-100` turns unsupported radius values into zero and
truncates Length.em before doubling. Pinned `visualize/shape.rs:272-294` parses
Smart<Length> and doubles the whole length. The baseline geometry query measures
2em at 10pt/20pt: upstream dimensions 40pt/80pt, local dimensions zero. Existing
layout `shape.rs:28-51` already resolves Length against occurrence font.

### Contract and limits

Use a private exhaustive fallible radius-to-diameter conversion. Float/Int retain
legacy point behavior as doubled Float. Length doubles both absolute and em
components into the existing Length dimension carrier; no region/font resolution
in eval. Unsupported None/Bool/String/Array/Dict and all other non-radius variants
report `expected length or auto, found {vanilla type}` at the last radius
occurrence value_span, then occurrence.span, then Args.span. Ratio/Relative are
invalid radius types upstream and reject here too. Preserve omission and the
existing fill/stroke validation order and optional-stroke nucleus.

Explicit Auto remains the measured zero-dimensional legacy path in this bounded
repair: upstream automatic shape fallback is 30pt, while the existing generic
Shape Auto consumer does not implement that contract. Do not hardcode 30pt in
eval, claim automatic-size parity, or reinterpret zero as the intended default.
The separate automatic-shape carrier/layout debt remains open. Negative/zero
absolute radii and numeric compatibility remain unchanged.

Tests must measure the actual constructed Content through existing layout at
10pt/20pt, preserve absolute 2pt geometry, invalid-type diagnostics/spans, numeric
extensions, omission/Auto limitations and paint/stroke controls. RED precedes
implementation; this internal parity correction changes no public ABI or phase.

### Identity refutation gate for the dimensional candidate

Before accepting the candidate, `agents/geometry/baseline-equality/receipt.json`
under the Batch F evidence root records eleven additional language expressions
against the frozen baseline product. In particular, circle radius 2pt versus
ellipse width/height 4pt, and circle radius 2em versus ellipse width/height 4em,
are false on both baseline product and oracle. The existing numeric radius
extension remains equal to its absolute-length counterpart locally. Source
`compiler/eval/operators/equality.rs:150-172` projects Shape without normalizing
its dimensions, so changing their carrier may expose a constructor-identity
regression. The candidate must preserve those measured language boundaries,
not merely improve rendered dimensions. If it cannot do so without a new public
identity contract, defer this owner correction rather than weaken the gate or
invent a private marker. Direct tests must refute or confirm this risk before
closing the dimensional change.
