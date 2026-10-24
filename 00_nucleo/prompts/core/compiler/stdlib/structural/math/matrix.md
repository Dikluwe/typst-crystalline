# L0 — compiler/stdlib/structural/math/matrix
Hash do Código: ea33d738

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/matrix.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9

## Evidence before specification


At the uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-matrix-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:725-1042` contains `MathMatrixOptions`, relative-length and augment casts, named option application, row interpretation, rectangular padding and the `math.mat` native. Static binding occurs later in the parent. The `p1431` test filter passed 29 tests before movement. This is a structural and regression baseline, not a performance result or a new bilateral vanilla measurement.



Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.


`lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/final.json.gz` records
`augment-json` and `augment-fields`: a supplied `2pt` is serialized as
`2pt + rgb("#000000")` instead of the pinned vanilla's `2pt`.
`lab/typst-original/crates/typst-library/src/math/matrix.rs:434` uses
`Stroke::from_value`; `01_core/src/compiler/stdlib/layout/casting.rs:179–197`
leaves `specified` empty for Length/Color shortcuts. The existing carrier
`entities/geometry.rs:37–44` already represents presence.



`lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/before.json.gz` augment-array/keys, delim-int/align-int/gap-str and Code true/false/left/center match errors but expose aggregate Tekt spans; matrix.rs:263-271 uses args.span despite individual value_span. V17 T259/T260 at 305/308: `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz` fixes dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/status/stat/hashes/sources/L0,28 guards/21 owners/CI rule. V16 M4 augment at 114: tekt-v16-math-value-2026-10-16/before.json.gz fixes same exact state/39 Value variants and uniform fallback. These are Rust-evolution/maintenance measurements, no functional/AI-benefit/upstream-intent claim.

## Exclusive language contract


Own `math.mat` with variadic positional rows and named `delim`, `align`, `augment`, `gap`, `row-gap` and `column-gap`. Unknown named and duplicate occurrence are errors; `rows:` is not accepted. Preserve validation order and the source spans and wording of each diagnostic. `gap` fills both axes first; specific row/column gaps override it. The defaults and explicit-presence bits are retained, including when a supplied value equals its default. Alignment must be horizontal. Relative length accepts Relative, Ratio and Length without losing its relative component. Delimiter conversion is called from the exclusive `00_nucleo/prompts/core/compiler/stdlib/structural/math/cases.md` owner; this prompt does not duplicate that mapping.

Scalar positionals form one row. In array mode all positionals must be arrays; in delimited-row mode they must be supported parenthesized MathDelimited rows. Mixed incompatible shapes keep their existing errors. Convert each cell with the existing Content/LocatedContent/None/Symbol/value-to-content mapping. Pad shorter rows with Empty to the maximum width. Preserve row order, empty rows and comma splitting in delimited rows. Construct the complete semantic `Content::MathMatrix`; do not execute layout or separate `&`.

`augment` accepts none, an integer vertical offset or a dictionary with `hline`, `vline` and `stroke`. Offsets accept an integer or array of integers and are checked against dimensions under the same current source-span gate. The dictionary consumes these three recognized fields and tolerates other internal keys, matching the measured upstream asymmetry; this does not permit invalid types for known fields or unknown top-level named arguments. Stroke extraction uses the existing stdlib helper. Preserve the existing `MathMatrixExplicit` bits and `MathMatrixAugment` carrier.

The parent `math.rs` owns only static registration of the native binding. The structural hub reexports `MathMatrixOptions`, `math_matrix_augment` and `apply_math_matrix_option` directly from this child with unchanged visibility; eval and layout consumers keep their established bridge. No dynamic dispatch, public signature change, default, pipeline phase or compatibility break is authorized.



 the matrix augment conversion must mark only thickness for a
Length shortcut and only paint for a Color shortcut after the existing
validated extraction. Preserve supplied Stroke presence, geometry, defaults,
diagnostics and all other extract_stroke consumers. This is an internal
parity correction, without a public contract change. Acceptance: a local
RED→GREEN test proves thickness present and paint absent for `2pt`; the
bilateral augment JSON and fields controls coincide.



For named casts use corresponding non-detached occurrence value_span else args.span; keep arity/gap-then-axis order/duplicate rules/signatures/Args transport. RED→GREEN individual augment-array anchor and bilateral errors.

Select rows by tuple(array_mode,delimited_mode,value): (true,false,Array) converts cells; (false,true,_) interprets delimited row; every other mode case yields same array error/span. Keep prior detection/order/padding and scalar mode when both false; both true remains error. No helper just to hide conjunction/lint suppression/config change.

M4 after validated extract_stroke: Length sets only thickness, Color only paint; explicit other 37-variant group preserves empty body/bits. Current productive remainder is Stroke; unreachable-by-extractor cases still enumerate nominally. Keep Stroke cap/join/dash/overhang/specification and external validation/errors/spans. Replace only wildcard, shared body/visible informative V19; no duplicated bodies/facade helper/catch-all/isemption/config. Nominal unused payload `_` allowed. Future Value requires compiler decision.

## Acceptance and limits

Preserve local padding/gap-row override, P1431/core suite/build/crystalline-lint/1:1 lineage and language defaults/compatibility/phase. Functional preservation controls accepted/rejected/morphology/presence pass before/after. Structural RED is V17family 28/V16selected plus real-workspace future Value sentinel currently swallowed; GREEN zero covered gates/E0004 at selected matches, only intentional V19/no new diagnostics. Do not claim 39-variant execution or global parity. Check successful/error/fallback/short-circuit/effects, same expected values. Owner-only direct/reverse reseal after valid graph; build/full workspace/strict all-target Clippy including unfulfilled/fmt/Rustdoc/strict V1/V5/V15/V26/reverse/SARIF under sequential 12GiB/swap 0/jobs 1, preserve unrelated code and source/command/hash/UTC evidence. No memory/speed/AI gain without independent measurement.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Preserve the selected raw Symbol as Content::Symbol when casting a content argument; preserve quoted/numeric/manual legacy carriers and argument diagnostics. Matrix comma syntax recognizes the native Symbol comma without altering the payload.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
