# L0 — math/layout/tests
Hash do Código: c3b4f99a

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
