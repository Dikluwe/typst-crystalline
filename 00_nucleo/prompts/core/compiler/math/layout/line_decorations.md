# L0 — compiler/math/layout/line_decorations
Hash do Código: 67ce3c48

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/text-gradient-paint.toml sha256:fa3c28e2415136777f2f215bb4e3e2cf8501a15ecb41b063d5368e232e7ccd4d

## Evidence before decision

At HEAD ea7ae25ab (clean tree, 2026-09-30), the implementation was in `01_core/src/compiler/math/layout/mod.rs:1312-1401`. This measurement identifies the extraction boundary; it neither measures performance nor changes language parity.

## Owner and contract

Layer L1. Exclusive productive consumer: `01_core/src/compiler/math/layout/line_decorations.rs`. Lay out mathematical overline and underline using the existing font constants, baseline-relative coordinates and styles. Extraction preserves the public contract, defaults, pipeline phase, compatibility and L1 purity.

## Acceptance

Dispatch remains static. L1 tests, build and crystalline-lint must pass. Do not infer performance gains from separation.

## Approved text gradient package

On 2026-11-24 the owner approved (“Aprovo o pacote”) the scope in
`00_nucleo/diagnostics/text-gradient-fill-package-2026-11-24.pt-BR.md`, including
the measured RED cases and the limits. Inherited line paint is the text Paint; explicit color is Solid.
Apply the pinned text-gradient-paint nucleus. Verify a gradient case that asserts
paint, a solid control and the absence of tiling on this route.
