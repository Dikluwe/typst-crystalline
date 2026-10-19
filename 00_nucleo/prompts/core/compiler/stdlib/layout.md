# L0 — stdlib/layout — layout façade and alignment
Hash do Código: 23c8b00d

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/layout.rs`

## Evidence before specification

The behavior-extraction snapshot `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` records an uncommitted 2966-line module and L1 suite result of6401 passed, two ignored. This is structural evidence, not a benchmark or new parity result. The hub retains native_align, alignment extraction, static bindings, tests and reexports. Child owners under `00_nucleo/prompts/core/compiler/stdlib/layout/` implement place/grid/page/casting/pad/hide/spacing/block/stack/box_elem/repeat/columns/measure/stroke/pagebreak. The catalog below specifies their composition boundaries without duplicating implementation ownership. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Fractional Box width was measured at HEAD 363883a89 with pinned Sizing source and 1fr/1:2/zero/negative probes, then approved on 2026-09-15. Only fractional width was approved; height fr remains excluded. Place clearance probes on 100×100 pages showed omitted clearance equals 1.5em, while zero changes pagination. Pinned place.rs:139–144 and explicit approval `2026-09-01T08:46:11-03:00` support the default. Controls at 10pt/20pt require 15pt/30pt rather than a baked 16.5pt. Flush probes distinguish the constructor marker from actual draining and partially applied Place arguments.

Block's relative-height audit at extract_length:861–874/native:1535–1539 refutes Ratio→zero. The later public Rel width/BlockHeight Fraction proposal has no explicit ratification receipt located, as tracked by the Block entity owner. Preserve its full specified carrier and approval-provenance gap; do not infer approval from code/extraction. Old absent-page/preparatory canvas/geometry stages are superseded by PageRun and constructor confirmations in `lab/history/construction/records/00_nucleo/materialization/typst-passo-1140.19.md` §15 and `typst-passo-1140.26.md` post-gate, dated 2026-08-24. The typed Str/Func/None numbering public/phase approval is separately recorded; eval cannot execute its callback.

Measure interception probes at `2026-09-24T14:12:42-03:00` against historical 9f2b6e871 distinguish real isolated layout from type estimation or a height alias: a 10×20 rectangle gives width 10/height 20/baseline 20; Box baseline 3 gives height 20/baseline 17; empty content is zero. Preserve HTML invariants. Stroke's clean baseline `c9d0162163f80f105f022ff9081f1e079f63f266`, UTC2026-09-25T10:54:15–10:57:42Z, binary hashes/commands in the archived owner and pinned stroke.rs:227–243,397–439 measure positional identity/exclusivity/presence. Historical `typst-passo-1499.md:3` explicitly confirms the positional slice on 2026-09-25; the named-Paint amendment has its own measured scope in `00_nucleo/prompts/core/compiler/stdlib/layout/stroke.md`.

The imports-ci-2026-10-13/before.json.gz receipt preserves dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/state/source/owners and production-versus-test uses. stroke-paint-2026-10-14 implementation-private-module/facade-before records E0603 requiring a typed cast bridge, not another algorithm or API. The archived owner retains the complete chronological probes and old scope-outs.

## Exclusive façade and alignment contract

Preserve the standard native ABI `(ctx,args,world,current_file) -> SourceResult<Value>`, including unused injected parameters. No direct I/O, mutable global state or dynamic registry. Align accepts an Align or legacy Str positional, defaults left when alignment is omitted and body is identified, requires Content/Str body, accepts no named arguments and returns Content::align. Keep legacy-string support and the exact body error `align() exige um bloco de conteúdo`. Parsing alignment alone does not certify layout. All other formulas belong to their child owners.

Keep static child bindings and existing public paths/function identities. Reexports is_horizontal_page_binding, is_positional_paper, validate_flush_args and is_valid_miter_limit are cfg(test)-only; child functions and productive uses remain unchanged. The positional_stroke pub(super) reexport is productive, has no cfg(test), retains the child signature/logic and adds no copied cast.

## Current spacing, containers and flow boundary criteria

Place accepts Align/legacy Str alignment (default top-left), Content/Str body, Length/legacy Float/Int dx and dy (default 0), scope column (default) or parent (requires float:true), boolean float (default false) and nonnegative Length clearance. Omitted clearance is declarative 1.5em, equal to explicit 1.5em; explicit zero is valid. Layout resolves the effective occurrence font; eval never bakes points. Preserve exact `scope \"parent\" requer float: true`. The constructor records data; float fitting/reservation/causal flow and advanced geometry belong to `00_nucleo/prompts/core/compiler/layout/place.md` and related owners. Their current certified slices supersede the old blanket float-stub description. native_flush takes no positional/named arguments and produces Content::flush, repr flush(); partial Place.flush remains unbound by PlaceArgs. The constructor does not drain/layout/read config; the element/layout owner defines effects.

Grid accepts Length/Fraction/Auto column/row arrays or a positive integer count converted to that many Auto tracks, default [Auto]; variadic Content cells; nonnegative Length gutter; top-left Align; uniform inset 0; optional Header/Footer; and its represented Stroke/Color fill. Per-side inset, rich paint/stroke and multiregion gaps retain the child owner's actual limits. Structural Table/Grid catalogs remain separately owned.

Pad takes Content/Str and Length side/x/y/rest options, with side>axis>rest precedence and missing/None resolving to zero. Negative padding remains deliberately rejected. Hide takes Content/Str, no named arguments, and produces Content::Hide: evaluation effects/counters/labels remain while layout is invisible. Stack takes ttb(default)/ltr/rtl/btt direction, nonnegative Length spacing(default 0), variadic Content/Str and the existing invalid-child errors. Repeat takes Content/Str, nonnegative Length gap(default 0), boolean justify(default true), producing Content::Repeat. Its owner defines runtime copies, available-space floor, body-once behavior and incomplete flow semantics.

H accepts Length/legacy point number or nonnegative Fraction, yielding Absolute/FractionalSpacing; weak defaults false. V has the analogous Length path but rejects Fraction with exact `v() espera amount como length, recebeu fraction`. Omitted weak means false with weak_explicit=false; explicit false retains explicit=true. Presence alone has no layout metric. Generic-length Ratio truncation and the current Relative cast remain RM1 debts, not desired percentage semantics or implicit extra points. Weak collapse and fraction resolution remain layout-owner responsibilities.

Colbreak takes no positional arguments and optional boolean weak(default false), retaining presence. Pagebreak adds optional to:even/odd and repr order weak then to. Current column/break effects belong to certified owners; old transparent/pagebreak-stub descriptions do not override them. Columns requires exactly two positionals: integer count>=1 and Content/Str body, with optional nonnegative Length gutter and existing errors. No universal multiregion behavior is claimed.

Linebreak takes no positional arguments and only boolean justify(default false), retaining explicit false presence. Markup backslash produces the same element with omitted false. The registry value is a Func, not Type, and its native belongs to spacing/flow rather than text-font construction. The separate layout owner defines justify:true's visual effect; constructor transport does not certify it.

Block takes optional Content/Str body(default Empty) and width Auto when omitted. Its relative-height correction maps Length/Float/Int to absolute, Ratio to relative and Relative to the full value; preserve mixed relative values with negative absolute components until effective-region validation, while purely absolute negative height rejects. The full specified later carrier uses Rel width and BlockHeight Relative/Fractional, accepting Length/Float/Int/Ratio/Relative plus height fr only. Fraction width and negative/nonfinite height fraction reject. Eval does not resolve regions or fr into points. The public-expansion approval gap above remains; this consolidation authorizes no new materialization.

Block inset/outset accept uniform Length/legacy Relative absolute part or per-side dictionary left/right/top/bottom/x/y/rest, default zero; unresolved relative components remain carrier debt. Radius is uniform Length or corner dictionary. Defaults remain breakable:true, clip:false, Color fill, represented rich Stroke, optional Length spacing/above/below and sticky:false. Explicit None fill/stroke at the call site means absence; the required Stroke helper still rejects None. Do not substitute zero-width hairlines or change the extractor globally. Shared-cell presence rules do not automatically expand Grid/Block.

Box takes optional Content/Str body(default Empty), Length/Auto dimensions, Length baseline(default 0, negative allowed), and the same bounded inset/outset/radius/clip/fill/stroke rules. Fraction width occupies width_fraction only; Length occupies width only. Preserve zero/negative fr without normalization and reject height fr. Logical dimensions/paint/clip/baseline composition and fractional completion belong to `00_nucleo/prompts/core/compiler/layout/boxed.md`; parsing proves none of them complete. Complex fill, relative inset/outset and per-side rich-paint limitations remain explicit.

## Page, contextual helpers and Stroke boundary criteria

Page is a constructor distinct from set-rule handling and the old SetPage-only rejection. It accepts optional positional/named paper, required positional Content body, and the 18 named properties paper,width,height,flipped,margin,bleed,binding,columns,fill,numbering,supplement,number-align,header,header-ascent,footer,footer-descent,background,foreground in their typed set-rule forms. Preserve absence/None/unknown/invalid/missing/excess errors; no accepted argument may be ignored. Produce PageRunArc, isolating body between two page boundaries/flush markers with empty-body handling and config restoration; never return Content::SetPage alone. Global/std use the same Func; set page/show page warnings remain unchanged. Numbering Str/Func/None maps through Numbering and Option<Option<Numbering>> (outer omission, inner None off), without callback execution or baked defaults. Child owners define exact run/config/realization; old staging did not authorize an incomplete callable.

Measure takes Content/Str body and no named arguments (width/height overrides excluded), sharing extract_measure_body. Direct/global/std interception checks the real native_measure pointer, never spelling; shadowed measure remains untouched. Outside context preserve exact `measure() can only be used inside context`. Inject Engine/Styles into an isolated Layouter with existing FixedMetrics/NullImageSizer and unbounded width/height None, composing actual items rather than guessing by type. Return an ordered dictionary width,height,baseline as Lengths. Baseline is causal top-to-composed-baseline, distinct from height; empty content returns all zero. FixedMetrics' monospace 0.6×size per codepoint is an explicit engine limitation, not real shaping parity. First-class indirect native fallback fails loudly instead of returning zero. Runtime query/counter/label effects and named size overrides remain uncertified. Layout(func) similarly uses native-pointer interception and an eval-owned Content::Layout region-Size callback; the indirect stub fails loudly. The façade adds no callback business logic or name-based shadow capture.

Global Stroke is callable Type(Stroke), with the same native constructor. Its ratified positional branch accepts at most one Stroke/Length/Color/Gradient/Tiling/Dict and is exclusive with every named option and a second positional. Stroke preserves explicit/Auto/overhang; Length sets only thickness, including zero/negative (not a render certificate); Paint sets only paint. Dict permits only paint/thickness/cap/join/dash/miter-limit, leaves unspecified slots Auto and rejects unknown/invalid values; overhang is not a Dict field.

Existing named defaults are black Paint, positive 1pt thickness, overhang:true, cap:butt, join:miter, dash:None and positive finite miter-limit 4. Preserve nine dash presets, Length/dot arrays and dictionary array/phase; dots remain LineWidth until resolved. The later named-Paint cast accepts Color/Gradient/Tiling/Auto exactly as its child amendment specifies. Nonpositive named thickness remains separate debt. None paint rejects with the measured union rather than meaning absence. Cast changes do not alter rendering or inheritance. Optional Grid/Table hline/vline Stroke carrier gaps need their own entity/renderer gates; zero PDF hairline is not invisibility.

## Acceptance and remaining scope

Preserve every stated value/default/presence/function identity/diagnostic/span/validation order and each child 1:1 L0. Source→language controls cover Align legacy/default/body; Place clearance at 10/20pt/zero, Parent+float, zero-argument flush and With namespace; Grid tracks/gutter/types; Pad precedence/negative; Hide effects; Stack order; H fractions/weak presence/negative and V fraction rejection; Linebreak justify presence; Columns counts; Block relative height/fractions/inset/None paint; Box baseline/width fractions and pending geometry.

Page controls cover paper/body, isolation, empty/multipage/config restore, every named field and numbering-function delta without eval callbacks. Measure controls cover actual dictionary/baseline/context/shadow/std/indirect failure. Stroke controls cover identity/slots/shorthand/exclusivity/Dict/Auto/overhang/rich fields. A mirrored formula or private flag is no substitute for actual effect.

Require meaningful RED→GREEN and bilateral probes, valid V15/V26 before restricted owner resealing, build/full L1/workspace/all-target checks, strict Clippy/Rustdoc/fmt and strict/reverse lineage. Preserve negative cases, tolerances and test inventory. No new public signature/default/phase/compatibility is authorized beyond explicit receipts. Block's public approval gap, RM1 relative casts, fractional Box render, Repeat flow, generic measure runtime queries, rich side paint and diagnostic debts remain pending. Existing certificates prove only their measured regions/render scopes, not universal behavior or performance; do not resurrect superseded stubs or declare all scopes closed.

The complete original is archived at `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/stdlib/layout.md`.


## Closed decision coverage

### Measurement before decision

At HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, the source/owner hashes
and exact inspection state are in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/read-only-audit.json`.
The baseline binary build and UTC-stamped bilateral commands/output are retained
in `lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/baseline-enabled-probes.json`;
the separate HTML textual projection receipt is `baseline-html-projection.json`
in that directory. The initial feature-disabled HTML run is setup evidence only.
The oracle is pinned to commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
These observations do not infer upstream intent or certify unrelated parity.

`layout.rs:117–127` selects the first Align or legacy Str and ignores other
values; native_align separately selects the Content body. Probes show accepted
legacy/extra arguments and general fields differ from the oracle. The earlier
Content/Str body description is not evidence that native_align accepts Str body:
the observed Content-only search remains an explicit unresolved contract gap.

### Decision and acceptance

Close extract_alignment over Value, preserving first-match and supplied default.
Every non-Align/non-Str variant is search-neutral, including the required Content
body; do not turn this helper into an argument validator. Tests must discriminate
body-only/default, earlier neutral candidates, competing Align/Str ordering and
legacy conversion. Existing native-align tests and source probes retain body and
errors. Future variants require a local exhaustiveness decision. This internal
ADR-0127 correction adds no acceptance of new public arguments and does not close
legacy-string, extra-argument, body-Str or field-reflection obligations.

The current fallible Grid/Table track conversion is owned by the grid and table
prompts. `lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/baseline-eval/receipt.json`
and pinned upstream `layout/grid/mod.rs:455–463` support rejecting malformed
non-dimensional track entries rather than silently dropping them. Preserve
columns-before-rows validation and named-value spans; the facade retains static
delegation only. Ratio/Relative omission or truncation remains the explicitly
recorded carrier debt and is not replaced by rejection or claimed resolved.
