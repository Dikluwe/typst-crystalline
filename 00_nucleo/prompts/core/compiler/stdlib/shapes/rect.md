# L0 — stdlib/shapes/rect
Hash do Código: 1f122f72

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shapes/optional-stroke.toml sha256:016cad80570d8133fcb275e6e0f693e088dc921e4d6d907a51888898ae8b00ce

## Evidence before specification

L1 exclusive consumer `01_core/src/compiler/stdlib/shapes/rect.rs`; clean HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9`, former shapes.rs:77-228 and `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz` preserve initial hash/time/state for behavior extraction. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

`lab/results/construction/diagnostics/paridade-dominios-2026-10-06/measurement.json.gz` visual-rect/visual-circle compile in historical oracle but reject Tekt stroke None. Pinned shapes.rs:125-129,335-339 distinguishes fill-dependent Auto from Custom(None); required-stroke parser/fallback ignored argument presence. No inferred historical intent.

stroke-paint-2026-10-14/before/results.json.gz preserves HEAD/dirty tree/diffstat/UTC/sources/owners/commands/both binary hashes. Named paint is Color-only; old shape caster accepts Length/Color/Stroke and rect discards Dict. Uniform control draws 1pt black; Gradient/Tiling/Auto named and circle Gradient reject despite oracle acceptance. Empty Dict draws black versus no stroke in oracle. Pinned stroke.rs:236-243/sides.rs:238-258 separates smart Paint/empty/uniform Dict. square(size:...) fails dimensions, so proves nothing about stroke casting; those defaults/dimensions remain open.

## Exclusive contract

Construct rect/square with width/height/fill/stroke/radius, preserving common Rect/no new Square type and parent's defaults/diagnostics/limits (`00_nucleo/prompts/core/compiler/stdlib/shapes.md`). Static L1 extraction changes no signature/default/phase/compatibility or I/O/global state, with no speed claim.

Apply optional-stroke nucleus to argument presence/fallback in both constructors. Do not change shared mandatory parser, dimensions/radius or unsupported per-side strokes. Uniform nonempty Dict paint/thickness/cap/join/dash/miter-limit goes to parse_shape_stroke; unknown uniform keys/invalid paint cannot silently drop. Empty(:) is explicit absence like none, even without fill; omission keeps fallback. square uses existing width, no new size/default correction.

Dict left/top/right/bottom/x/y/rest remains separate unsupported per-side entity gap, including mixed keys; do not turn it into uniform stroke or claim complete validation/render. Previous scope-out does not authorize dropping all dictionaries; separate decision/materialization needed for that gap.

## Acceptance and limits

RED→GREEN None with/without fill and omission/no fill; PDF/raster/macro matrix against pinned current oracle when measured. Preserve L1/shapes suite/build/lint/ownership/pins, no unsafe/dependency/public mode/phase. Internal ADR-0127 continuous-flow parity correction: L0 first, meaningful RED→GREEN, restricted reseal/build/suite/lineage, PDF/raster/error recollection retaining uncorrected differences. Paint meaning/available form is acceptance, never binary equality. No public API/entity/default/phase expansion.
