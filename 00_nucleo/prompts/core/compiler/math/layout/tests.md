# L0 — math/layout/tests
Hash do Código: df2f0b5f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:b1f5e8d937f80e207e7cb5f1cd7ee03f3a3eebc89af5bfc0e13ebd95c615ecd0
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/tests.rs:16` locates `fn default_style() -> TextStyle`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Test-only ownership

Own only `01_core/src/compiler/math/layout/tests.rs` and its cfg(test)
integration. Test obligations do not authorize productive geometry.
Exercise dispatch, styles, baseline/extents, spacing and feature regressions
with injected metrics and attributable oracles. Record formula/observable
provenance for new cases; fixture coordinates and bytes are not contracts.

Root fixtures preserve index_explicit: square-root controls use None/false,
explicit indices use true, without changing independent geometry oracles.

Fraction coverage distinguishes vertical/skewed/horizontal, explicit versus
omitted style, style-chain precedence, slash associativity, both
deparenthesization bits and the barless binomial. Observe axis, operand order,
separation and bar/slash presence rather than freezing font names, private
frames or operation order. Independent feature suites keep their own owners
under `00_nucleo/prompts/core/compiler/math/layout/`.

Acceptance observes morphology and attributable font-derived geometry;
historical RED/GREEN reports or aggregate test totals are not fresh parity
evidence.

For `lr` around a fraction, compare the same fenced body with and without
the explicit wrapper using injected OpenType-like vertical variants. The
fraction's unbalanced raw height and axis-balanced target must straddle a
variant threshold, so a test fails if explicit `lr` uses raw height while
implicit fences use the balanced extent. Observe selected glyph advance and
declared box extents for both parentheses and bars; the pinned vanilla
source's `FencedItem::relative_to_from_fragments` supplies the independent
axis-balancing rule, and bilateral `sec_22.typ` checks its product effect.

For Display Large operators, verify that a multi-part assembly carries one
source-character TextReplacement around its visual parts; the wrapper does
not alter part positions, width or extents. Preserve base italics-correction
behavior when scripts consume the wrapped visual glyphs. Verify that a
sufficient base glyph remains
unchanged before consulting MATH constructions. Exercise the case where the font's largest
vertical variant remains below DisplayOperatorMinHeight and has no assembly:
retain that largest variant, including its native horizontal advance and
ink-derived extents, rather than reverting to the base glyph. Also cover the
assembly path when the font supplies one. Attribute expected selection to the
active font's OpenType MATH construction and the pinned vanilla algorithm.

## Extractable identity verification

For cases/accents/spreaders, L1 verifies TextReplacement retains the language
character and visual pieces; probe PDF extraction in local examples after
RED→GREEN. Extractor-only spacing such as .typ/sec_27.typ does not authorize
whitespace in authored trees without a language cause. Two nested combining
accents require base+inner mark+outer mark with one final envelope.
For hat(mu), Greek MathText reaches accent as mathematical 𝜇; multicharacter text
does not become mathematical letters by approximation. PDF probes preserve
base+mark composition and compare raster against qualified vanilla.

## Ratified bidi carrier adaptation

Before decision, `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
identifies this unit's references/exclusive owners. Manifest 1ae7513e was confirmed;
adapt uncertified constructors with None and read patterns with `..`, preserving
language observables. Transformations follow the pinned origin nucleus without
phase/API expansion.

## Test quality maintenance

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/testes-ci-2026-10-13/before.json.gz`
retains sources/hashes/UTC/status/diffstat. Input inventory
matematica-ci-2026-10-13/clippy-inventory.json is retained in that batch's
evidence.json.gz. tests.rs:8238 uses hypot in a size-12 FixedMetrics geometry
oracle; :9417/:9473/:9506/:9546 drops MathLayouter without Drop. Other findings:
equality iter().any, uneven hex groups, cal_L and last on a double-ended iterator.
This measurement precedes decision under ADR-0108.

### Exclusive responsibility and acceptance

Keep all inputs/tolerances. For the small finite fixture use Pythagorean
sqrt(width²+height²); this is no authorization for arbitrary productive geometry.
Remove explicit drops of borrowers without destructors; compiler borrow checks
validate ending the borrow before consuming pass. Use contains/next_back/
starts_with for the diagnosed equivalences; normalize hex grouping without value
changes and rename cal_L to cal_l. Remove no layout execution, change no callback
order/count, and retain every test/observable. ADR-0127 internal test repair uses
Clippy -D warnings as quality RED; rerun cases/lineage without inventing functional
RED→GREEN for already-passing behavior.

## Nominal FrameItem observer complements

### Evidence before decision

Sites: F043 tests.rs:3361 script_y_offset; F044 :4226 visit; F045 :4240 visit;
F046 :4963 collect_dot_ys; F047 :6054 p963_y; F048 :6516 p971_x; F049 :6647 visit.
AST spans/patterns/guards/bodies/complements were checked before code.
`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC/sources/L0/config/hashes/status/git diff HEAD --stat. classification.json
records 70 cases/complements in 26 owners from the pinned CI parser. Relevant
source/L0 were read together; no new architecture/upstream intention follows.
Rust exhaustiveness is mechanics; preserve language/geometry/effects/diagnostics.

### Exclusive prescription

Replace only each `_` with its exact nominal group below using FrameItem in
scope. Keep fallback bodies, prior arms/guards/transformations/calls/returns/order.
Guarded or restricted fields do not cover a whole variant: their failing cases
remain in the complement. The observed guard counts remain part of the site audit.

| Site | Function | Nominal fallback group | Recorded guards |
| --- | --- | --- | --- |
| F043 | script_y_offset | `TextReplacement`, `TextSeparator`, `Semantic`, `TextShaped`, `Line`, `Image`, `Shape`, `Group`, `Link` | 0 |
| F044 | visit | `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | 1 |
| F045 | visit | `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | 1 |
| F046 | collect_dot_ys | `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | 1 |
| F047 | p963_y | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | 2 |
| F048 | p971_x | `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | 3 |
| F049 | visit | `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link` | 1 |

Preserve callbacks/effects/metadata/clipping/fonts/positions/ink/width under their
current owners. Do not expand traversals, turn wrappers into leaves or substitute
text/ActualText for geometry. Test helpers retain fixtures/expected results/
tolerances/counts/Unknown failures; missing observation is not acceptance.
Nominal groups share the previous body; keep V19 notes without equating arm
coverage with every alternative.

### Acceptance and limits

ADR-0127 internal flow adds no enum/field/public trait method/signature/default/
dependency/compatibility/phase. Do not materialize unrelated pending proposals.
L0 first and restricted bilateral resealing. No suppressions, threshold changes
or match-to-if-let rewrites to hide V16.

Before code run the new FrameVisitor control and workspace suite on unchanged
expectations and confirm 70-family-warning RED by AST inspection. Afterwards
require zero selected-family V16 and no new warning; external V16 stays open.
For productive-core matches reached by cargo check -p typst-core, a disposable
copy with sentinel FrameItem measures absent E0004 before/present after. Claim
coverage only for compiled sites, not inaccessible cfg(test)/L3. Replay checks
all patterns/guards/bodies, unchanged enum/signatures and real builds/tests;
no stubbed bodies.

Validate full workspace/build, workspace/all-targets Clippy -D warnings
-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes,
strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/
configuration during heavy sequential commands under MemoryMax 12 GiB,
MemorySwapMax0/CARGO_BUILD_JOBS1. Structural sanitation proves neither global
parity nor RAM/AI-reading benefit.

## Ratified complete text size

### Evidence before decision

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`,
before-measurement.json.gz, morph-before-measurement.json.gz and audit.json
preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC, status/diffstat, sources/L0/binaries/argv and historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. named-em measures A at 0pt versus
24pt; affine 3pt versus 27pt; nested over 18pt measures 0pt versus 36pt. All six
Size/Color/body permutations are accepted by the pin and rejected by Tekt.
Positional set 18pt is ignored (12 versus 18pt). Pinned text/mod.rs:290,303,
924–930,1136–1142 and args.rs:178–247 establish typed search, named preference,
remaining body and composition. style_chain.rs:513–517 and constructor.rs:123
discard em; collapse at style_chain.rs:394–395,419–422 uses top-wins where size
needs a fold. Size/scope is observable language; Rust representation is an
internal choice. Infer no historical intent. Restoration, Raw, heading, math,
capture/show, duplicates/spans, zero/negative or metric failures can refute it.
The equality probe refuted comparing declared values: the pin returns true for
text(size:2em)[A] == text(size:24pt)[A]. Pinned StyledElem::eq at
content/mod.rs:779–782 compares child. That equality defect requires independent
correction; a complete size type does not close that axis.

### Ratification and limits

The human approved the complete-size package and its two public contracts in
manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`.
Require implementation and S1–S16 tests under 12 GiB/zero swap; no release/merge/
push. Reseal only after implementation and a valid graph. RG1 remains independent.

### Exclusive test-adapter responsibility

Only fixtures/adapters change Style::Size(Pt) to Size(absolute Length), and
StyleDelta.size expectations to Option<Length>. Preserve existing language
observations/functional contracts; never replace correct geometry with snapshots
of the new representation.

## Measured Raw morphology inside math

### Measurement before correction

`lab/results/remaining-parity-2026-10-22/agents/html-length/math-length-before/receipt.json.gz`
SHA-256 `d94a39c8bd3fb37880866dbd1e4b6ee9004fc77eab39d0bcdf7bb78090a165bc`
retains UTC `2026-10-22T13:18:01.270679+00:00`–`2026-10-22T13:18:04.556564+00:00`,
exact coordination HEAD/dirty stat, both immutable sources, font bytes, commands,
cached product's separate verified archived build/source and ratified oracle pins.
Length interpolation and explicit inline typc Raw inside an Equation already
compile with equal nonspace text but differ in numeric paint and SVG geometry:
maximum glyph-origin delta `1.1204531249999832pt`, product/oracle root heights
`68.57484375pt`/`67.4765625pt`. These are preexisting debts in that cached source;
they do not establish a new regression caused by the display adapter. All
source/binary/archive/font/worker guards pass.

Pinned `lab/typst-original/crates/typst-library/src/text/raw.rs:657–669` gives
Raw its Mono font and 0.8 relative size; `typst-layout/src/rules.rs:657–680`
realizes Raw lines to styled text. `typst-library/src/math/ir/resolve.rs:127–150`
realizes content before math resolution and `typst-layout/src/math/text.rs:17–39`
lays out actual text into a frame. The current pure math catch-all flattens Raw
to plain_text, losing its language morphology. The existing external bridge
already preserves actual text/glyph baselines; only baseline-less frames use
axis centering. No new layout formula follows from the measured delta.

### Exclusive obligation

Test actual Raw layout with injected metrics: numeric typc paint, existing Mono
font and 0.8 size, upright glyphs, actual baseline zero, and extents measured
from that same run. Cover direct Raw and recursive Sequence/custom-only Styled
wrappers. Independently retain plain Text and mathematical identifier baseline,
style and geometry behavior; preservation controls may be GREEN before the fix.
Do not derive expected positions from a proposed new offset formula or mistake
unit tests for full bilateral paginated parity. Productive dispatch policy is
owned exclusively by math/layout/mod.md; external_bridge remains unchanged.


## Raw is normal text inside an equation

### Measurement before correction

`lab/results/remaining-parity-2026-10-22/candidate-2/qualification/math-length-2/receipt.json.gz`
retains the two immutable Length/explicit-typc-Raw sources, successful byte/font/
source/build guards, UTC 2026-10-22T13:44:23.723338+00:00–13:44:26.943012+00:00,
HEAD/complete uncommitted changed-file list and source-bound candidate
`feb025296531c92350e29bff032f655cc2fa47542cf8ab4aca0e239108bc4e8f`.
The ratified oracle remains bd5ebec285505889e8ff1d4f68804ab147a699f3 with executable
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
Raw morphology/paint now agrees, but actual glyph origins and page height do not.
`agents/inline/math-raw-face-cause.json` under that result root (SHA-256
`6de15bd9fafd430fda8f795e339c295a1710c4f0ee68d5669cc373fcbe4df264`)
binds the actual two font bytes/tables and candidate receipt. At the actual 8.8pt,
NewCM's `12pt` advance/ascent/descent are 17.116/5.8608/1.7072pt; Mono's are
21.1921875/6.53125/1.83046875pt. Those differences exactly explain the measured
centering, baseline and bottom error. This refutes a fitted axis/baseline offset.

Pinned raw.rs:657–669 installs ordinary Mono/relative-size text defaults;
rules.rs:657–680 realizes Raw lines to text; math/ir/resolve.rs:127–150 and
math/text.rs:17–39 resolve and lay out that actual textual content. Current
Raw inherits `math=true` from its outer Equation. The metric provider appends
MATH fallback and resolution.rs:176–184 then prefers a MATH face over explicit
Mono, while the renderer draws Mono glyphs. The classification, not the bridge
geometry formula, is the missing boundary. This is language typography/geometry;
no copied upstream buffer or flag representation is required.

### Exclusive test responsibility

Add a metric provider whose declared normal-text and mathematical glyph metrics
differ, modeling the measured real-face distinction. Compare the Raw MathBox's
advance and ink extents with the independently declared normal-text run at its
actual 0.8 size, while retaining its baseline, numeric paint and Mono morphology.
Use two enclosing sizes and more than one Raw lexeme. Preserve a mathematical
identifier's separate glyph metrics before and after Raw layout, so a global
math-metric disable cannot pass. Keep all existing FixedMetrics Raw and ordinary
Text/MathIdent controls unchanged; they previously masked this real-font cause.
These tests authorize no productive geometry formula or bridge change. The
exclusive correction belongs to core/compiler/layout/raw.md.
