# L0 prompt — color values, process spaces, and spot identity
Hash do Código: d007c43c

## Ownership and evidence

This L1 owner specifies `01_core/src/entities/color.rs`, a pure domain facade with zero I/O or global state. Color conversion implementation belongs to `00_nucleo/prompts/core/entities/color/conversion.md`; stdlib argument validation and language errors belong to `00_nucleo/prompts/core/compiler/stdlib/color.md`; export belongs to its canonical exporter owners. Keep the `pub mod color;` and `pub use color::Color;` re-export from entities. The migration replaced the simplified `Color { Rgb, Rgba }` in `entities/layout_types.rs:638–654`; the old estimated 250–350 lines is historical, not an acceptance gate.

The complete original specification, chronological amendments, old code sketches, and execution-step provenance remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/color.md`. Historical pin observations below apply to their snapshots, not current-upstream certification. ADR-0107 requires parity of language semantics/syntax/morphology, not the original “structural parity” or exporter byte identity as a universal gate. The initial same-output regression for `Color::rgb(255, 0, 0)` and the four existing PDF `to_rgba_f32` paths remains a compatibility control; it does not impose byte equality on unrelated current output. ADR-0029 governs RAM purity; the revoked ADR-0028 reference does not reinstate its architectural rules. The original ADR-0083 **proposed** scope-outs remain historical scope evidence, with only the explicit later refinements below superseding them.

### Process-space source evidence

At historical vanilla `e0e8ca4d`, `crates/typst-library/src/visualize/color.rs:26–28` aliases Hsl/Hsv/Luma to `palette::hsl::Hsla<encoding::Srgb, f32>`, `palette::hsv::Hsva<encoding::Srgb, f32>`, and `palette::luma::Lumaa<encoding::Srgb, f32>`. Hsl/Hsv's “standard algorithm” means palette's hexcone over sRGB encoding, not an invented vanilla algorithm; HSL's public source is `color.rs:686–702`. Luma conversion uses `Luma::from_color` at `:1771–1774`. Attribution to Rec.709/sRGB primaries is an **inference from encoding**, not a source quotation, refutable by numeric comparison with palette coefficients. The observable controls below support the measured results separately.

The same source at `color.rs:30–38` loads a `LazyLock<ColorProfile>` from `typst_assets::icc::CMYK_TO_XYZ`, a minimal CGATS TR 001-1995 profile. Thus the local naive CMYK formula is a **declared observable result divergence**, not merely different mechanics. The historical source also states at `:640–641` that PDF/A does not support CMYK; that observation does not declare every current exporter route unsupported. The original upstream enum/ColorSpace references at `:194`/`:1798` had eight process variants; the approved spot extension below supersedes that count and Copy assertions.

### Operator observations that precede the contract

Historical vanilla `969087ec`, `visualize/color.rs:311,1549–1653`, measured named `red = rgb(1.0, 0.254902, 0.211765)`, **not** pure `#ff0000`. Palette 0.7.6 scratch calculations reproduced the measured bytes exactly:

| Expression | Measured language result |
|---|---|
| `red.lighten(20%)` | `rgb("#ff675e")` |
| `red.darken(20%)` | `rgb("#cc342b")` |
| `red.negate()` | `rgb("#004b74")` |
| `red.rotate(90deg)` | `rgb("#87a100")` |
| `red.mix(blue)` | `oklab(61.08%, 0.075, -0.031)` |
| `red.saturate(20%)` | `rgb("#ff372b")` |
| `red.desaturate(20%)` | `rgb("#ff675e")` |

These observations refute and supersede the old Oklch lighten/darken, sRGB-complement negate, and Oklch-chroma saturation definitions. Existing binary mix acceptance is retained subject to the corrected space/input-specific results below.

## Current process and spot data contracts

Retain eight f32 process carriers:

```rust
Srgb { r: f32, g: f32, b: f32, a: f32 },
Luma { l: f32, a: f32 },
LinearRgb { r: f32, g: f32, b: f32, a: f32 },
Oklab { l: f32, a: f32, b: f32, alpha: f32 },
Oklch { l: f32, c: f32, h: f32, alpha: f32 },
Hsl { h: f32, s: f32, l: f32, a: f32 },
Hsv { h: f32, s: f32, v: f32, a: f32 },
Cmyk { c: f32, m: f32, y: f32, k: f32 },
```

The 2026-09-23 owner-confirmed spot package additionally defines `SpotColor { colorant: Arc<SpotColorant>, tint: Ratio }` and `Color::Spot(SpotColor)`. Color is no longer Copy; retain Clone and value PartialEq. The original claim that derived f32 PartialEq is bitwise was incorrect: the current manual implementation compares process fields with `f32::to_bits`; Spot compares structurally. Preserve signed-zero and NaN behavior without tolerance. This Rust mechanism is not the language-equality acceptance criterion.

The public constructors remain `Color::rgb(r: u8, g: u8, b: u8) -> Self`, `rgba(r: u8, g: u8, b: u8, a: u8) -> Self`, `srgb_f32(r: f32, g: f32, b: f32, a: f32) -> Self`, `luma(l: f32) -> Self`, `linear_rgb(r: f32, g: f32, b: f32, a: f32) -> Self`, `oklab(l: f32, a: f32, b: f32, alpha: f32) -> Self`, `oklch(l: f32, c: f32, h: f32, alpha: f32) -> Self`, `hsl(h: f32, s: f32, l: f32, a: f32) -> Self`, `hsv(h: f32, s: f32, v: f32, a: f32) -> Self`, and `cmyk(c: f32, m: f32, y: f32, k: f32) -> Self`. RGB byte channels normalize by `channel as f32 / 255.0`; rgb and luma use alpha 1.0, rgba preserves explicit alpha, srgb_f32 stores direct f32, HSL/HSV hue uses degrees, and CMYK components range over [0,1]. Constructor validation corrections belong to the stdlib/conversion boundaries below; do not quantize direct percentage inputs through bytes.

Retain the public domain API, including its approved fallible conversion extension:

```rust
impl Color {
    pub fn to_srgb(&self) -> (u8, u8, u8, u8);
    pub fn to_rgba_f32(&self) -> (f32, f32, f32, f32);
    pub fn lighten(self, factor: f32) -> Self;
    pub fn darken(self, factor: f32) -> Self;
    pub fn saturate(self, factor: f32) -> Option<Self>;
    pub fn desaturate(self, factor: f32) -> Option<Self>;
    pub fn negate(self, space: Option<ColorSpace>) -> Self;
    pub fn rotate(self, angle_deg: f32, space: Option<ColorSpace>) -> Self;
    pub fn mix(self, other: Self, weight: f32, space: Option<ColorSpace>) -> Self;
    pub fn space(self) -> ColorSpace;
    pub fn to_space(self, target: ColorSpace) -> Result<Color, ColorSpaceError>;
    pub fn to_process(&self) -> Color;
    pub fn components(self, include_alpha: bool) -> Vec<ColorComponent>;
    pub fn to_hex(self) -> String;
    pub fn transparentize(self, factor: f32) -> Self;
    pub fn opacify(self, factor: f32) -> Self;
}
pub enum ColorComponent { Ratio(f32), Float(f32), Angle(f32) }
```

ColorComponent does not depend on Value; the language adapter maps these categories. Ratio components print as percentages, Float carries Oklab a/b and Oklch chroma, Angle carries hue in degrees normalized with `rem_euclid(360)` as measured `hue_angle`. Components (`color.rs:1455–1522`) are Srgb/LinearRgb `[Ratio r, g, b, (a)]`; Luma `[Ratio l, (a)]`; Oklab `[Ratio l, Float a, Float b, (alpha)]`; Oklch `[Ratio l, Float c, Angle h, (alpha)]`; Hsl `[Angle h, Ratio s, Ratio l, (a)]`; Hsv `[Angle h, Ratio s, Ratio v, (a)]`; Cmyk `[Ratio c, m, y, k]`. Parenthesized alpha is omitted when include_alpha=false; CMYK always ignores alpha. Spot exposes only tint as Ratio.

## Conversion formulas and narrow refinements

`to_srgb` returns byte sRGB and `to_rgba_f32` normalized sRGB [0,1], preserving the existing exporter hot-path API. Srgb is identity before byte projection; Luma replicates its channel and preserves alpha under the later rule below; LinearRgb uses the exact inverse sRGB transfer curve, superseding the initial shorthand “gamma 2.2.” Oklab uses LMS/linear RGB and the sRGB transfer curve; Oklch uses polar→Cartesian Oklab; Hsl/Hsv use palette's sRGB hexcone. Process CMYK projection remains `(1-c)(1-k), (1-m)(1-k), (1-y)(1-k)`; the reverse naive hub uses `k = 1-max`, `c = (1-r-k)/(1-k)` with corresponding other channels under its existing implementation. The ICC difference remains declared. Conversion cannot collapse all origins through sRGB when a measured origin-specific route is specified.

For sRGB→Luma, linearize by the sRGB curve, use exactly palette 0.7.6's Y coefficients `0.2126729`, `0.7151522`, `0.0721750`, then re-encode sRGB. Abbreviations `0.2126/0.7152/0.0722` are forbidden because they change public red luminance from `54.02%` to `54.01%`. Independently, `to_space(ColorSpace::Luma)` preserves the source alpha. Historical `a51e02804` lost alpha for non-Luma origins through Luma::from_color but preserved already-Luma identity. Local preservation is a protected `Known-Upstream-Bug` divergence; it cannot excuse a separate luminance delta or promote any SVG pair.

The linear sRGB→Oklab path used by to_space(Oklab), mix, and to_oklch uses full f32 constants from `palette 0.7.6/src/oklab.rs::linear_srgb_to_oklab`:

```text
0.4122214708 / 0.5363325363 / 0.0514459929
0.2119034982 / 0.6806995451 / 0.1073969566
0.0883024619 / 0.2817188376 / 0.6299787005

0.2104542553 / 0.7936177850 / -0.0040720468
1.9779984951 / -2.4285922050 / 0.4505937099
0.0259040371 / 0.7827717662 / -0.8086757660
```

Truncation changes public components and SVG subdivision and is forbidden. Preserve each transfer direction's numerical order: exponential decode uses `c.mul_add(1/1.055, 0.055/1.055).powf(2.4)` (fmaf and powf); linear decode multiplies by the reciprocal. Encode uses `powf(1/2.4) * 1.055 - 0.055` with separate multiplication/subtraction, **without fusion**. The former encode-fmaf clause is superseded by measured MulSub source. Cube root must be bit-compatible with no_std palette's `libm::cbrtf`; the std intrinsic is not numerically interchangeable. Pure `libm 0.2.11` authorizes `powf`, `fmaf`, `cbrtf`, `sinf`, `cosf`, `sincosf`, `atan2f` in L1 without exposing external types.

The origin-specific D65 measurement precedes its correction: historical `visualize/color.rs:1784–1794` sends `ProcessColor::Luma` directly to Oklab::from_color; palette `xyz.rs:317–334` maps linearized Luma to XYZ using D65 `(0.95047, 1.0, 1.08883)`, then `oklab.rs:262–280` maps XYZ→LMS→Oklab. On 2026-08-29, `gradient.linear(black, white, space: oklab).stops().last().first().components()` returned vanilla JSON `["100%", 0.000011444091796875, 0.00003707408905029297, "100%"]`; product baseline `e055a12f` exposed a/b `[0.0, 5.960464477539063e-8]`. White was Luma in both products. Therefore Luma→Oklab preserves encoded Luma→linear Luma→XYZ D65→Oklab with full palette constants, product/sum order, and cbrtf. Because the source uses from_color rather than unclamped, clamp resulting l to [0,1], leaving a/b unclamped (`oklab/properties.rs:8–20`). Keep direct sRGB's different matrix route. The proposed first-cause inference is the erroneous sRGB detour for Luma; failure of the exact palette route to reproduce those public components and close S06/S22 would refute it. Acceptance compares language components, not Rust structure or steps. Other Luma destinations, ICC/PDF, tolerances, and adaptive budgets are outside that correction.

On 2026-10-13, `lab/results/construction/diagnostics/cores-ci-2026-10-13/before.json.gz`, `constructors-before.json.gz`, and `evidence.json.gz` recorded HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, source/hashes/UTC/status/diffstat, upstream `9f2b6e8715237cb086899a42873660fe744622e8`, ratified binary SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Palette's libm configuration uses sinf/cosf/atan2f in Oklch/Oklab, where local conversion still used prohibited native functions. Authorize those pure symbols explicitly; implementation/tests remain with conversion, and this facade only reseals rather than duplicating algorithms.

The same HEAD/uncommitted provenance, upstream pin, and binary hash are recorded in `lab/results/construction/diagnostics/limites-cor-2026-10-13/before.json.gz`, `parents-before.json.gz`, and `evidence.json.gz`, including complete public digits. Before correction, `red.rotate(45deg)` exposed negative blue versus vanilla zero; `rgb(10%,20%,30%,40%)` exposed 10.2%/30.2% versus 10%/30%; Linear RGB accepted 256 and alpha -1% despite upstream rejection; negative Oklch chroma produced a chromatic color versus gray. Source `visualize/color.rs:1825–1895` distinguishes clamped FromColor, identity, and from_linear; palette `convert/from_into_color.rs:53–59` clamps after conversion; `encoding/srgb.rs:100–120` and `num.rs:718–722` require reciprocal multiplication on linear decode and nonfused mul_sub on encode; `oklab.rs:355–368` clamps chroma to zero before polar conversion; `hues.rs:155–157` uses sincosf through libm. Correct those internal formulas under ADR-0127 continuous flow: no public field/signature, binding, default, compatibility, or phase changes. L0 precedes observable RED tests, implementation, and revalidation. Conversion owns implementation and facade only resealing. Remaining probe differences are not normalized away or given a generic parity certificate.

## Operators and weighted reduction

For process colors, palette `macros/lighten_saturate.rs` defines:

```text
increase(c, f) = clamp(c + (1 - c) * f, 0, 1) if f >= 0
                 clamp(c + c * f, 0, 1)       if f < 0
```

`lighten(f)` applies increase in the original space: Srgb/LinearRgb r/g/b, Luma l, Oklab l, Oklch l, Hsl l, Hsv v. CMYK instead uses `clamp(u - u*f, 0, 1)` on each c/m/y/k. `darken(f)` is lighten(-f) for palette variants; CMYK uses `clamp(u + (1-u)*f, 0, 1)` per channel.

`saturate`/`desaturate` return None for Luma, surfaced by the caller as `cannot saturate/desaturate grayscale color`. Hsl/Hsv increase their own saturation; Srgb/LinearRgb/Oklab/Oklch/Cmyk convert to HSV, increase s, and return to the original space. `desaturate(f) = saturate(-f)`. Saturation's Option result supersedes the earlier infallible signature because grayscale is a language error.

`negate` defaults to Oklab: convert to the chosen space, apply `(1-l, -a, -b)` in Oklab, then return to self's original space. All eight process spaces are accepted; measured `negate(space: rgb)` gives `#00bec9`. `rotate` defaults to Oklch, adds degrees to hue in the chosen space, then returns to self's original space. Only Oklch/Hsl/Hsv support hue; other chosen spaces produce `this color space does not support hue rotation`, as measured by `rotate(90deg, space: rgb)`. Public language validation remains the stdlib owner's responsibility rather than changing the preserved domain signature here.

Binary `mix` interpolates in the chosen space, default Oklab; the result remains in that selected space, not self's original one. Source `visualize/color.rs:1185–1204` directly accumulates weight*component and divides once; repeated binary interpolation without preserved totals can change observations. Keep the public binary signature unchanged. A nonpublic weighted-pair helper converts each input once to the selected space, accumulates four components, and divides by total weight once, constructing through existing `from_vec4_in_space`. Exactly two hue-space inputs preserve the measured shortest angular path; the stdlib owner rejects N>2 in hue spaces. Expose no mix_iter or new public type.

`to_hex` returns sRGB hex including alpha when <1.0. `transparentize(f)` sets `alpha' = alpha * (1-f)`; `opacify(f)` sets `alpha' = alpha + f*(1-alpha)`. Spot-specific handling and conversion/alpha/saturation errors are bounded by the next section and stdlib owner, not silently process-converted during a language operation.

## Approved spot colorant and spot color packages

The initial spot gate recognized that public Color/ColorSpace/Value/Type changes and exhaustive consumers required `BLOCKED_ADR0127_PUBLIC_CONTRACT`; that earlier gate authorized no entity or consumer edits. `color.map` remained implementable solely in the stdlib owner as a Module of existing Srgb arrays. The later confirmations supersede the spot prohibition only for their bounded packages.

The reduced colorant package's bilateral evidence and reason for excluding Color expansion/PDF Separation belong to `00_nucleo/prompts/core/compiler/stdlib/color.md`. Its entity contract is:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct SpotColorant {
    pub name: Option<EcoString>,
    pub fallback: Color,
}
// fallback must already have been validated as a process color by caller:
SpotColorant::new(name: Option<EcoString>, fallback: Color) -> Self
```

SpotColorant is a sibling value/type, not itself a Color variant; `Type::SpotColorant`/`Value::SpotColorant` are owned by `00_nucleo/prompts/core/entities/value.md`. The stdlib caller validates fallback and emits `expected color, found spot colorant` before this entity; the constructor does not revalidate. None is an accepted absent name. Repr is `color.spot({name.repr()}, {fallback.repr()})`, following existing Option<EcoString> repr (`none` or a normally quoted string); invent no formatting convention. PartialEq derives structurally. Color itself has no Hash because f32 has no std Hash; where SpotColorant needs Hash under the existing entity contract, use the established manual `format!("{self:?}")` Hash-via-Debug pattern of `entities/elements/list_item.rs`, not a new hash mechanism. This was corrected in L0 before causal commit `3ac60a578`. Human confirmation followed on 2026-09-21; agent A `01a82593d` materialized the specified entity with manual Debug hashing from the beginning. The recorded suite was GREEN with zero regressions; this is historical evidence, not a new run.

That initial package excluded `.tint`, SpotColor, instance lighten/darken/negate/fallback, Color-consumer integration (gradients/to-hex/PDF), and Separation. The later owner-confirmed 2026-09-23 package supersedes only the relevant language-value/call/cast exclusions. Its preceding measurement at upstream `586e1bd43`, `visualize/color.rs:280–286,889–924,937–1020,1147–1245,1260–1415,2254–2397,2471–2549` showed Color::Spot carrying colorant+tint and ColorSpace::Spot identifying the complete colorant. `color.spot("Brand", red).tint(50%)` has type color and repr `color.spot("Brand", rgb("#ff4136")).tint(50%)`; equality uses name+fallback+tint, not RGB fallback. Tint 0%/100% is valid; -1%/101% produces the boundary diagnostics at `:2385–2397`. `rgb(spot.tint(50%))` projects tinted fallback without equating values; `.space()` returns the colorant with type spot, and `.components()` returns `(50%,)`. The old local eight-Copy-variant process-only enum (`entities/color.rs:31–61,356–367,616–680,1024–1047`) failed with `type spot colorant has no method tint`.

The approved data extension retains all eight process variants and uses `Value::Color` for tinted results. `SpotColorant.fallback` stays process-only even after adding Color::Spot; validate that at construction. `None` and `"all"` remain distinct names. Tint uses Ratio without premature rounding and preserves endpoints. Color loses Copy but keeps Clone/PartialEq; SpotColorant retains Clone/PartialEq/Hash. Add `ColorSpace::Spot(SpotColorant)` while preserving the eight process-space identities. ColorSpace loses Copy/Eq/Hash, retaining Clone/PartialEq because the new numeric-fallback colorant is not a map key in the measured corpus. All those public trait breaks, carriers, variants, and conversion signature changes belong to the confirmed ADR-0127 package, not a new editorial approval.

`to_space` now returns `Result<Color, ColorSpaceError>` with `ProcessToSpot { target: SpotColorant }` and `SpotToOther { source: SpotColorant, target: SpotColorant }`. Never invent spot from process or convert between distinct colorants. `to_process(&self)` explicitly applies tint by `fallback.lighten(1 - tint)` in the fallback's own process space (`visualize/color.rs:2295–2297`): 0%→white, 100%→full fallback. `to_srgb`, `to_rgba_f32`, and `to_hex` use this projection only for a consumer that cannot represent spot, without mutating the value. `space` identifies the entire colorant rather than the fallback's process space. Spot lighten/darken adjust tint; default negate inverts tint; components expose only its percentage. Mix retains spot only when all operands have the same colorant; otherwise use process Oklab by default. Stdlib owns conversion/alpha/saturation errors. Every Color/ColorSpace match must be total: no language operation may erase identity or use unreachable! for spot.

This is intentionally a split constructor: the approved language-value package (historically P1482) covers values, calls, and casts; the separately scoped render/PDF package (historically P1483) completes PDF `/Separation`, spot gradients, and process backends. Until a consumer supports spot, export may use explicit fallback projection; that does not certify PDF conformity or the entire color.spot.tint route. `00_nucleo/prompts/infra/export/stream.md` owns the same boundary. Do not promote the initial colorant-only GREEN into evidence of full downstream integration.

## Verification and historical discrepancies

Keep at least two controls per materialized process space. The original controls include rgb(255,0,0) storing Srgb(1,0,0,1) and returning `(255,0,0,255)`; luma(.5) storing Luma(.5,1) and returning `(127,127,127,255)`; LinearRgb constructor preserving f32 and its (.5,.5,.5,1) conversion differing from `(127,127,127)` due to inverse transfer; Oklab(1,0,0,1) approximately white and Oklab(0,0,0,1) approximately black; Oklch preserving (L,c,h) and (.5,0,0,1) producing gray; Hsl(0,0,.5,1) approximately `(127,127,127,255)`; CMYK all zero→white and k1→black. These historical approximate example controls do not authorize production tolerances or weakening later exact language-component expectations.

Corrected operator controls for `Color::rgb(0xFF,0x41,0x36)` are lighten(.2)→`(255,103,94,255)`, darken(.2)→`(204,52,43,255)`, default negate→`(0,75,116,255)`, rotate90→`(135,161,0,255)`, saturate(.2)→`(255,55,43,255)`, desaturate(.2)→`(255,103,94,255)`. Luma(.5).saturate(.2) returns None and the caller emits the grayscale error. Zero lighten/darken preserve identical variant/value; zero saturation/desaturation preserve identity. Retain the existing confirmed binary mix controls.

Hex/alpha controls distinguish input precision: integer-channel `Color::rgb(255,65,54)` negated in Oklab gives `"#004b74"`, rotated90 in Oklch gives `"#87a100"`, and mixed .5 with `Color::rgb(0,116,217)` in Oklab gives `"#a37095"`, versus Srgb `"#805a88"`. Named public constants retain the measured vanilla f32 literals, so `red.mix(blue, space: rgb).to-hex()` gives `"#805b87"`. Do not substitute integer constructors for those literals. `Color::rgba(255,65,54,128).transparentize(.5).to_hex()` gives `"#ff413640"`; opacify(.5) gives `"#ff4136c0"`; rgb(255,65,54).to_hex() gives `"#ff4136"`.

The historical 2026-08-13 document used c=rgb(255,65,54), d=rgb(0,116,217), then negate Oklab, rotate90 Oklch, mix Oklab, mix rgb, transparentize50, opacify50, to-hex. `/usr/local/bin/typst` identified as `e0e8ca4d`; local `target/release/typst` source was HEAD `4f64e4e69` with only prompt edits; pdftotext read output. The table reported both binaries giving respectively `#004b74`, `#87a100`, `#a37095`, `#805a88`, `#ff413640`, `#ff4136c0`, `#ff4136`. The earlier L0 erroneously used `#805b87` for both mix spaces and was corrected. However the same record's prose claims five of seven exact matches and escalates local sRGB mix `#805b88` versus oracle `#805a88`, a one-green-channel difference tentatively attributed to rounding and scoped to a separate behavior gate. **The table/prose conflict remains explicit pending reproduction**; this consolidation cannot resolve it by choosing the convenient row. Neither the tentative rounding cause nor the old low-priority label closes it.

The same historical record measured local rejection of weighted syntax `c.mix((d, 50%), space: rgb)` with exact error `color.mix: argumento 'col2' deve ser Color, recebeu array`, where upstream returned `#aa526c`. Retain that historical gap; the later weighted-reduction helper specifies its internal mechanism but does not alone prove current stdlib acceptance. On 2026-08-28, direct named-constant probes against ratified `a51e02804` gave red.mix(blue,rgb) `#805b87`, negate Oklab `#004b74`, rotate90 Oklch `#87a100`. Those named-constant observations do not invalidate the integer-input measurements or settle the contradictory local row.

The CSS parse_color refinement, originally in rules/stdlib/shapes.rs, extended the five basic names red/green/blue/black/white with yellow/cyan/magenta/orange/purple/gray/grey/silver/maroon/navy/olive/teal/lime/aqua. Preserve `gray == grey == rgb(128,128,128)` and `aqua == cyan == rgb(0,255,255)`. The old “18 colors” and “13 + 2 aliases” were chronological inventory labels, not an instruction to invent extra names. This partially closes the proposed named-color scope-out for basic CSS names; it does not replace the separately measured precise global named constants. Other names remain incremental refinements under their own owners. The old operator scope-out recorded all six operators closed (lighten/darken/mix/negate/saturate/desaturate); this does not certify all later conversions or weighted syntax. Original runtime ColorSpace absence is superseded by its approved current enum. Native PDF `/DeviceCMYK` remains its export owner's responsibility, not a universal statement that every present exporter converts CMYK to sRGB.

## Explicit Debug guard maintenance

Evidence for T276 at `01_core/src/entities/color.rs:78` precedes the prescription: `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz` preserves sources/L0 hashes, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, status, git diff HEAD --stat. The hash-reconfirmed inventory contains 28 guards across 21 owners; pinned CI requires expanding the condition or moving it into the body. This measures Rust structure, not a functional defect, upstream intent, or improved AI readability.

Unite Srgb Debug arms; inside the body keep exactly the four == comparisons identifying nominal blue and its blue output. The fallback preserves the same r/g/b/a debug_struct. Preserve signed zero/NaN, other spaces, and all color formulas. This internal ADR-0127 continuous-flow maintenance belongs solely to this consumer. Do not add a helper to hide conjunction, suppressions, or linter configuration changes. Existing functional controls run before code with unchanged expected results; actual RED is V17's 28 warnings and GREEN requires zero V17. Check success/fallback/error paths, short-circuiting/effects; reseal only affected owners. Revalidate build, complete workspace suite, all-targets Clippy with warnings and unfulfilled expectations denied, fmt, Rustdoc warnings denied, strict/reverse V1/V5/V15/V26 and SARIF without new warnings under 12 GiB, zero swap, one job. No memory/performance/global-parity claim without separate measurement.

## Exhaustive representation and identity maintenance

Evidence at `01_core/src/entities/color.rs:214` and `lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` preserves the same HEAD/uncommitted tree, UTC, sources/L0/configuration/hashes/status/diffstat. Pinned SARIF recorded eleven warnings and one V16 note across seven owners, with nonemitted fallbacks measured in the same scope. Rust patterns/enum evolution are mechanics; observable fields/results/identity/errors remain unchanged. Infer no upstream intent or AI benefit.

Color::eq keeps the nine same-space pairs and bodies first, process f32::to_bits and structural Spot. Replace catchall false with a pair whose first axis explicitly names Srgb/Luma/LinearRgb/Oklab/Oklch/Hsl/Hsv/Cmyk/Spot and second axis remains _. Do not convert spaces to compare. Enumerating the first axis forces a decision for future Color growth, including new/new. Preserve NaN/signed-zero and Spot semantics. Add cross-space inequality among nine spaces and clone-identity controls before code; these verify the Rust mechanism, not new language parity.

Internal continuous flow changes no enum definition, public field/trait/signature, default/dependency/compatibility/phase. Preserve bodies, order, guards and ownership; no helper hides the catchall, no neutrality comment acts as suppression. Before code run local functional controls and scoped V16 RED. In a disposable workspace copy add only a sentinel variant to copied Color, ColorSpace, MathStyleKind, FuncRepr, and StateUpdate, never productive definitions. Use `cargo check --offline --locked -p typst-core`, one job/shared artifact cache, recording sources/hashes/argv/UTC/diagnostics. For all 21 selected matches baseline must lack E0004 at that location and GREEN must emit E0004 there independently of other already-exhaustive matches. Compile the real code without sentinels and run behavior controls; negative probes do not prove parity. Do not edit productive source during commands.

Restricted direct/reverse reseal, no global dirty-tree repair. Revalidate build, workspace/all-targets Clippy `-D warnings`/`-D unfulfilled_lint_expectations`, fmt, full workspace suite, Rustdoc `-D warnings` and four HTML indexes, strict/reverse V1/V5/V15/V26, no-new-warning SARIF, and diff --check. Use MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1, sequential heavy tasks. Do not relax tolerances or historical fixtures. External V16 debt, including tracked nonemitted T027/T158/T160, remains open.
