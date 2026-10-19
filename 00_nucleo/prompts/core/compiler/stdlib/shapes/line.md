# L0 — stdlib/shapes/line
Hash do Código: b53ebd37

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

`shapes/line.rs:30-37,106-140` converts malformed legacy dx/dy to zero; the
polar length route separately validates its input. Pinned `visualize/line.rs`
has no dx/dy surface, and every baseline upstream dx probe is an unexpected
argument. The current hub L0 explicitly preserves typed Length/numeric dx/dy
and the zero/zero omitted default; that local contract, not upstream dx parity,
authorizes rejecting malformed extension arguments.

### Contract and limits

Make the private point extractor exhaustive and fallible. Float/Int/Length
retain current point projection. Ratio/Relative keep the existing zero projection
for legacy dx/dy as explicit unresolved RM1 debt; do not invent dimensional
resolution. All other variants report `expected length, found {vanilla type}`
instead of becoming zero. Use the last named occurrence value_span followed by
occurrence.span and Args.span, with dx checked before dy.

Keep stroke validation before geometric parsing. Keep end and polar conflicts,
start/end coordinate validation, ignored length/angle when end is present, and
the separate existing polar-length Ratio rejection/message. No broadening of
polar accepted forms follows from the legacy helper. Preserve libm formula,
negative/zero lengths, default legacy zero line and nonzero-origin Path geometry.

Tests cover malformed legacy inputs, supplied-value span, valid absolute/numeric
and negative/zero controls, stroke/conflict precedence and named dimensional
debts. No native public signature/default/phase changes and no suppression.
