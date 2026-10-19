# L0 — entities/color/conversion
Hash do Código: 52a4849c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/color/conversion.rs`

## Extraction and numerical evidence

Clean HEAD `ea7ae25ab` on 2026-09-30 places the extracted block at color.rs:280–714. This establishes ownership, not performance/parity improvement. Own process-space conversions and sRGB byte/float representation while preserving public signatures, rounding, formulas, spot identity, defaults, pipeline phase, compatibility and L1 purity/static dispatch.

`lab/results/construction/diagnostics/qualidade-ci-2026-10-13/color-literals-before.json.gz` preserves the Rust 1.92 Clippy inventory, originals/proposals, independent probe and tree provenance. All 50 excessive-precision literal replacements compile to identical f32 bits. Use those representable spellings without changing formula/order/signature/output; do not shorten f64 contexts. That original slice excluded transcendental changes, which the subsequent separately measured decision below permits.

`lab/results/construction/diagnostics/cores-ci-2026-10-13/{before,constructors-before,evidence}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/hashes/UTC/status/diffstat and historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. clippy.toml disallows native f32 sin/cos/atan2, still used at conversion.rs:60–61,246 and interpolation.rs:200. Upstream configures palette without defaults and with libm; palette0.7.6 num/libm.rs:5–41 calls sinf/cosf/atan2f; both locks pin libm0.2.11. Color/gradient/alpha/hue probes retain preexisting constructor rejection and gradient.sample(25%) chroma discrepancy without normalization.

`lab/results/construction/diagnostics/limites-cor-2026-10-13/{before,parents-before,evidence}.json.gz` preserves the same HEAD/tree/pin/binary provenance. red.rotate(45deg) exposes negative local blue versus upstream zero; rgb(10%,20%,30%,40%) exposes 10.2%/30.2%; LinearRgb accepts256/alpha−1% rejected upstream; negative Oklch chroma becomes chromatic locally versus gray. Preserve complete public digits in the evidence. Upstream color.rs:1825–1895 distinguishes clamped FromColor, identity and from_linear; palette convert/from_into_color.rs:53–59 clamps after conversion. encoding/srgb.rs:100–120 uses reciprocal multiplication for linear decode and mul_sub for encode; num.rs:718–722 does not fuse mul_sub. oklab.rs:355–368 clamps chroma before polar conversion; hues.rs:155–157 uses libm sincosf. The old fused-encode clause was wrong for this source.

The first rotate repair still failed without changing tolerance. hue-intermediate.json within evidence measures equal L/chroma but hue 28.442533493041992 upstream versus 28.442523956298828 locally, propagating into RGB after+45°. palette hues.rs:134–146 uses PI+atan2(-b,-a) then degrees; local atan2(b,a)/rem_euclid had observably different f32 rounding. No red-specific constant or relaxed tolerance is justified.

## Conversion contract

Use pinned libm sinf/cosf/atan2f at corresponding sites, with explicit symbol whitelist; retain formula/order/alpha/hue paths/spot identity. Encode sRGB as powf*1.055−0.055 without fusion. Keep fused exponential decode and linear `(1/12.92 as f32)*c`. Private polar conversion clamps chroma minimum to zero and uses pinned sincosf, shared by Oklch→sRGB and Oklch→LinearRgb; whitelist sincosf. Color→Oklch helpers use measured PI+libm::atan2f(-b,-a), degrees and existing circular normalization.

For to_space(Srgb), clamp converted channels/alpha to 0..1 only on upstream FromColor paths; preserve Srgb identity and raw LinearRgb→Srgb from_linear without added clamp. to_rgba_f32 remains raw for existing consumers, never a global clamp. For to_space(LinearRgb), convert Oklab/Oklch directly by matrix without intermediate encode/decode, clamping converted FromColor results; other sources use existing decode and identity stays identity. Do not change CMYK/ICC or other-space formulas in this repair.

## Explicit enum domains

The numerical-repair SARIF in limites-cor evidence identified V16 at conversion.rs:383. Oklab/Oklch have direct routes; enumerate Srgb/Luma/LinearRgb/Hsl/Hsv/Cmyk/Spot for the remaining LinearRgb decode arm, keeping its body. Future variants require a compile-time decision.

The later `lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` freezes sources/owner/config/hashes/UTC/status/diffstat at the same uncommitted HEAD, with eleven warnings and one note across seven owners plus nonemitting fallbacks. At conversion.rs:230, to_oklab_p476 names Srgb/LinearRgb/Oklch/Hsl/Hsv/Cmyk/Spot fallback, retaining direct Oklab/Luma. to_oklch_p476 names every variant except Oklch in fallback. Target-Spot ProcessToSpot and to_process process-clone branches name the eight process spaces, retaining both Spot arms/order and Spot projection. Do not change any formula, operation order, rounding, error or identity. Rust exhaustiveness is mechanics, not inferred upstream intent or AI benefit.

## Acceptance

L0 precedes observable RED tests and implementation under continuous ADR-0127 without public API/binding/default/compatibility/phase changes. Replace empty smoke coverage with independent oracle-referenced conversions, quadrants and alpha. Verify rotate channels/Oklch reconversion, target bounds, raw LinearRgb→Srgb control, alpha, negative chroma, independent public component values and nominal-red hue at unchanged tolerances. Retain color/gradient regressions, L1 suite/build/lint and ownership hashes. Remaining probe differences stay visible, without generic parity or performance claims.

For the recorded V16 repair, obtain functional local tests and scoped lint RED first. In a disposable workspace only, add sentinel variants to Color, ColorSpace, MathStyleKind, FuncRepr and StateUpdate. Run `cargo check --offline --locked -p typst-core` with one job and shared artifact cache; record sources/hashes/argv/UTC/diagnostics. Each of 21 selected matches must change from absence of its own E0004 to E0004 at that match, regardless of other failures. Unsentinelled compile and real behavior tests approve production; the negative probe does not prove parity. Never edit productive definitions during probe commands.

This is ADR-0127 continuous internal maintenance: no enum/field/trait/signature/default/dependency/compatibility/phase change or ownership move. Restrict direct/reverse resealing. Revalidate build, workspace/all-target Clippy with `-D warnings -D unfulfilled_lint_expectations`, fmt, full workspace tests, Rustdoc `-D warnings` and four HTML indices, strict V1/V5/V15/V26 and reverse lineage, no-new-warning SARIF and diff checks. Heavy jobs are sequential under MemoryMax 12 GiB, swap 0 and one job. Do not relax historical fixtures/tolerances. External V16 debts, including nonemitting T027/T158/T160, remain open.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/color/conversion.md` preserves the original evidence and chronology.
