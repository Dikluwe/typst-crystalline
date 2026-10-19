# L0 — stdlib/calc/logarithms
Hash do Código: f216c805

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/calc/logarithms.rs`. At HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` with a clean tree, `01_core/src/compiler/stdlib/calc.rs:848-995` contains the corresponding behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz`. This decision only separates algorithms by behavior.

## Contract

Implement exp, ln, log, log10 and deg/rad extensions. log defaults to base 10, handles named/positional base and chooses exact e/2/10 branches before the generic quotient; preserve domains and diagnostics.

Parity details, types, diagnostics and bindings remain in `00_nucleo/prompts/core/compiler/stdlib/calc.md`. Extraction preserves public signatures, module functions, phase, results and error policy. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and calc tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains from this separation.

## Deterministic transcendental implementation

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/matematica-ci-2026-10-13/before.json.gz`
retains sources/SHA-256/UTC/status/diffstat. evidence.json.gz (before-probes.json)
retains language probes and binary identity. Historical oracle pin:
`9f2b6e8715237cb086899a42873660fe744622e8`, ratified SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

clippy.toml:3–24 prohibits platform-dependent native transcendentals.
01_core/Cargo.toml:42 and lab/typst-original/Cargo.lock:1620–1623 both pin libm
0.2.11. Historical upstream foundations/calc.rs:313–608 and layout/angle.rs:81–113
use libm for these families. Measured probes have matching types/quadrants/domain
rejections, with final-digit differences in cosh(1), exp(1) and log(81,base:3).
Error messages already differ between implementations.

### Contract and acceptance

ADR-0127 internal continuous flow: use pure functions from already-pinned libm
0.2.11 without a new dependency. Remove this consumer's local disallowed_methods
exceptions. The whitelist names added functions. Transcendental implementation
choice is mechanics; types, coercions, numeric values, argument order and errors
are language observables. Require neither universal bit equality nor general
parity inferred from probes. Preserve signatures, bindings, guards, messages,
defaults, phase and extensions in `00_nucleo/prompts/core/compiler/stdlib/calc.md`.

Use independent constants/explicit tolerances, result types/quadrants/domains,
existing calc regressions, Clippy without prohibited methods or local exceptions,
build and lineage. Quality-gate RED precedes repair; do not fabricate a language
failure where the platform already passes. Later probes must record preexisting
differences outside this correction.
