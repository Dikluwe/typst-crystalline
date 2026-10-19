# L0 — entities/gradient/interpolation
Hash do Código: e589a56f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/gradient/interpolation.rs`

## Evidence before owned interpolation

At clean HEADb7c6bbe0a0bfe3bebf0bb071b33fa0da51574229, gradient.rs:285-620
contained the extracted behavior; SHA256/time/state are in
`lab/results/construction/diagnostics/atomizacao-gradient-entity-2026-09-30-before.json.gz`.
Own supported-space conversion/interpolation, preserving alpha,hue paths,
public contracts/defaults/sampling and
`00_nucleo/prompts/core/entities/gradient.md` acceptance. Pure L1,no I/O/globals.

## Measured transcendental implementation

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b with uncommitted tree is preserved
with hashes/UTC/status/diffstat in cores-ci-2026-10-13/{before,constructors-before}.json.gz
under lab/results/construction/diagnostics; evidence.json.gz holds probes/
numerical proof. Historical pin9f2b6e8715237cb086899a42873660fe744622e8,
oracle SHA2567fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
clippy.toml prohibited native f32 sin/cos/atan2, still used in conversion.rs:60-61/246
and interpolation.rs:200. Upstream palette disables default features in favor
of libm; palette0.7.6 num/libm.rs:5-41 calls sinf/cosf/atan2f, lockfiles libm0.2.11.
Probes retain colors/gradients/alpha/hue, pre-existing Color-constructor rejection
and sample(25%) chroma discrepancy without normalizing them.

Use pinned libm sinf/cosf/atan2f at corresponding calls with explicit whitelist.
Preserve formulas/order/alpha/hue/Spot identity/API/default/phase. This ADR-0127
internal mechanics change is accepted through language components/types/colors/
alpha, not identical implementation. Eighteen coefficient spelling suggestions
were compiled as f32 and to_bits-identical: only those spellings may change,
not values/order/truncation or decomposition of fmaf/gamma. Keep the facade's
precision distinction and add quadrant/alpha/short-hue controls.

## Measured bounds and precision refinements

The same HEAD/tree/pin/oracle is recorded by limites-cor-2026-10-13/
{before,parents-before,evidence}.json.gz. Measured differences: red.rotate(45deg)
had negative blue locally versus upstream zero; rgb(10%,20%,30%,40%) exposed
10.2%/30.2% instead of 10%/30%; Linear RGB accepted256/alpha-1% rejected upstream;
negative Oklch chroma became colored locally versus gray. Full extra digits
remain evidence, not silently rounded outputs.

Upstream visualize/color.rs:1825-1895 distinguishes bounded FromColor, identity
and from_linear; palette convert/from_into_color.rs:53-59 clamps after conversion.
encoding/srgb.rs:100-120 uses reciprocal multiplication for linear decode and
mul_sub for encode; num.rs:718-722 implements it without fusion. This refutes
the older fmaf-encode clause. oklab.rs:355-368 clamps chroma before polar
conversion; hues.rs:155-157 uses sincosf through libm.

In this owner change only the two linear sRGB decode helpers to multiply by
(1.0_f64/12.92) as f32. Preserve weights,hue route,sampling,other operation order
and proved f32 spellings. sample(25%) chroma remains open. No public contract/
binding/default/phase/compatibility changes; L0→observable RED→code→revalidation,
without generic parity claims for remaining differences.

The first implementation still failed rotate without a tolerance change.
hue-intermediate.json in evidence.json.gz measured equal L/chroma but upstream
hue 28.442533493041992 versus local 28.442523956298828; +45deg propagated to RGB.
Palette hues.rs:134-146 uses PI+atan2(-b,-a) then degrees; local atan2(b,a)/
rem_euclid rounded differently in f32. Use PI+libm::atan2f(-b,-a), then degrees
and existing circular normalization in this owner's Color→Oklch helpers.
Add independent nominal-red hue control, retain quadrants/tolerances; no
special red constant or relaxed rotate tolerance.

## Explicit Spot guard

T289 at interpolation.rs:107 was source/L0/hash checked before decision.
tekt-guards-2026-10-16/before.json.gz under the same diagnostics root retains
HEAD/tree/UTC/status/diffstat; inventory 28 guards across 21 owners, pinned CI
rule requires splitting or moving conditions into bodies. This is Rust
structure, not proof of functional error or AI readability/upstream intent.

Combine Spot/Spot arm and move colorant comparison into its body. Compatible
keeps a's Arc and identical tint formula; incompatible reconstructs the same
Color::Spot values for process projection/interpolate_oklab. Generic pair stays
unchanged. Preserve clamp/alpha/identity/numeric order without extra clones/
allocations. No helper hiding the conjunction, suppression or config change.

## Acceptance

Keep color/gradient regressions and functional controls before code with
unchanged expectations. Structural RED is 28 V17; GREEN requires zero.
Check success/fallback/error,short-circuit/effects; restricted owner reseal.
Require build/full workspace,Clippy all-targets with warnings/unfulfilled
expectations as errors,fmt,strict Rustdoc,V1/V5/V15/V26 strict/reverse,SARIF
without new warnings, under 12GiB/no swap/one job. No global parity or memory/
performance gain without separate measurements.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/gradient/interpolation.md`. Historical observations are not a new parity certification.
