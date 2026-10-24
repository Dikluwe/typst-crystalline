# L0 — core/compiler/math/layout/mod
Hash do Código: 76c13a35




**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/mod.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/layout/typc-length-display.toml sha256:37f7c44553aaf173c60a6d9a525e196042143ccb52ec617a29d8bc62ea7dfeea
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:b1f5e8d937f80e207e7cb5f1cd7ee03f3a3eebc89af5bfc0e13ebd95c615ecd0
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


## Consumer evidence

`01_core/src/compiler/math/layout/mod.rs:24` locates `mod accent;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Core ownership

Own only `01_core/src/compiler/math/layout/mod.rs`: MathLayouter, MathBox,
equation entrypoints, static dispatch and coordination of recursive transformations,
sequence/grid composition and external-content bridging through their owning
children. Feature geometry belongs to its child owner, not to entities. External content bridging belongs to `00_nucleo/prompts/core/compiler/math/layout/external_bridge.md`; sequence flattening and horizontal composition belong to `00_nucleo/prompts/core/compiler/math/layout/sequence_composition.md`. Math grid measurement, placement and baselines belong to `00_nucleo/prompts/core/compiler/math/layout/grid_composition.md`. Recursive glyph/default style transformation belongs to `00_nucleo/prompts/core/compiler/math/layout/style_transform.md`. Keep closed static dispatch and exhaustive
FrameItem handling; no registry, vtable or reverse entities-to-compiler import.

Child owners are `00_nucleo/prompts/core/compiler/math/layout/accent.md`,
`00_nucleo/prompts/core/compiler/math/layout/assembly.md`,
`00_nucleo/prompts/core/compiler/math/layout/attach.md`,
`00_nucleo/prompts/core/compiler/math/layout/callbacks.md`,
`00_nucleo/prompts/core/compiler/math/layout/cancel.md`,
`00_nucleo/prompts/core/compiler/math/layout/cases.md`,
`00_nucleo/prompts/core/compiler/math/layout/delimited.md`,
`00_nucleo/prompts/core/compiler/math/layout/frac.md`,
`00_nucleo/prompts/core/compiler/math/layout/large_operator.md`,
`00_nucleo/prompts/core/compiler/math/layout/matrix.md`,
`00_nucleo/prompts/core/compiler/math/layout/op.md`,
`00_nucleo/prompts/core/compiler/math/layout/root.md`,
`00_nucleo/prompts/core/compiler/math/layout/spacing.md`,
`00_nucleo/prompts/core/compiler/math/layout/stretch.md`,
`00_nucleo/prompts/core/compiler/math/layout/stretchy.md`,
`00_nucleo/prompts/core/compiler/math/layout/underline.md`,
`00_nucleo/prompts/core/compiler/math/layout/underover.md` and
`00_nucleo/prompts/core/compiler/math/layout/vec.md`.
Their test consumers have separate owners.

## Pure context and equation entrypoints

MathLayouter is generic over injected FontMetrics; obtain MathConstants from
`metrics.math_constants(style)`, using the active face rather than an
unconditional fallback. Preserve `new(metrics, block, style)` and
`new_with_context(metrics, block, style, equation, region_height,
lexical_styles, callback_pass)`. The contextual constructor supplies the
equation Location, effective region height, causal StyleChain and validated
callback-pass state. The context-free constructor remains available for
existing callers; it is not a substitute for productive contextual layout.

No Func/Engine/World execution occurs here. Each equation's local cancellation
occurrence starts at zero; reserve/increment it once at MathCancel entry,
before descending into its body, including Auto and explicit-angle elements.
Pass that occurrence to cancel; cross lines share it.
Transient math.mid target state is local to the nearest lr scope.

`layout_equation` returns baseline-relative FrameItems.
`layout_equation_measured` returns items and EquationExtent from the same
layout result: width, ascent, descent, line_count and first/last baseline.
Do not introduce a second measuring algorithm or serialize a provisional
callback result as successful compilation. Pipeline realization belongs to
its existing infrastructure owner.

## Baseline and composition

MathBox contains width, ascent, descent and items. Its local y=0 is its own
baseline; y increases downwards. Horizontal concatenation changes x only,
takes maximum ascent/descent and does not top-align siblings. Missing gaps
in hconcat_spaced behave as zero. Offset every supported FrameItem consistently,
including lines, shapes, groups and shaped text.

For axis centering, `shift = axis - (ascent-descent)/2`; add shift to ascent,
subtract it from descent and move items by -shift. Grid content is centered
before merging already-centered delimiters. Fractions, roots and fenced groups
use their child-specific baseline contracts, not blanket axis centering.

Grid delimiter target is `(ascent+descent)*1.1*upem/font_size`, or zero for
nonpositive font size. The 1.1 ratio and delimiter shortfall are separate.
Shared grid helpers measure cells, allocate column widths and position rows
with explicit column/row gaps and alignment-boundary flags. At an ampersand
boundary, class spacing is accounted for in the left cell rather than adding
a normal column gap. Alternating subcolumns align right then left; grids
without alignment points are centered unless a feature specifies alignment.
Row ascent/descent are separately maximized with the synthetic-parenthesis
floor. Accumulate row descent without duplicating a row's ascent.
Multiline equations derive leading from paragraph style rather than
unconditionally using MathConstants.math_leading; retain first/last baselines
for the existing equation integration owner.

Numerator descent by level is Display→Text ×1, Text→Script ×script,
Script→ScriptScript ×sscript/script, ScriptScript→ScriptScript ×1.
Numerators inherit cramped; denominators force cramped. These are discrete
levels, not unconditional multiplication by the script factor.

## Style and morphology preservation

Apply mathematical default glyph transformation once at equation entry.
Single-character eligible Latin/lowercase Greek leaves receive mathematical
italic codepoints; uppercase Greek stays upright. Preserve MathStyled wrappers
for their own handler. Resolve nested setters per independent glyph, size,
bold, italic and cramped axes: the innermost specified setter wins on the same
axis; different axes compose. Explicit false is not absence. Size wrappers
survive glyph transformations; competing size setters are not multiplied.
Bold on a default-italic variable yields bold italic; explicit upright disables
italic. MathOp text is not rewritten into a glyph variant.

Recursive transformations preserve public identity and all causal fields:
attachment slot states; root index and index_explicit; fraction line, optional
style and both deparenthesization bits; limits inline/inline_explicit; accent
mark, size, dotless and presence; cases children, optional delimiter sides,
reverse, gap and presence; matrix rows, alignment, augment, gaps, delimiters
and presence; lr/mid/stretch bodies and explicit fields; every public
under/over spreader identity and optional annotation. Transform children
without baking inherited properties into explicit fields.
Omitted, explicit none, zero, auto and present empty Content are not interchangeable.
Preserve markup containers and semantic styles while descending; do not
flatten a mathematical subtree through plain_text as a transformation.

Dispatch fractions by explicit style > validated math.frac.style > Vertical;
line=false always selects the barless vertical binomial stack. Invalid
validated cascade values fail closed rather than silently choosing a default.
Forward complete accent/cases/matrix/cancel carriers to their geometry owners.
Root index_explicit is geometrically inert. Limits wrappers lay out their body
transparently; attach owns the limit placement decision. Public spreader
lowering is a local layout view, never a rewrite of the public carrier.

## Sequences, text and spacing

Flatten MathSequence. Keep Content::Sequence atomic only when all children are
Text/MathText/MathIdent leaves; open technical sequences with HSpace or
composite math nodes. Weak HSpace collapses at run edges but remains between
material items. Absolute space resolves against current font size; fractional
space is not fabricated as an absolute advance.

A punctuation MathText with terminal whitespace and a subsequent material
sibling contributes its boundary once, through a transient layout/spacing
view; retain the original carrier and preserve terminal whitespace when no
material successor exists. Do not reintroduce normalized glue via Text fallback.

Direct Content::Text is a TextItem: preserve math=true and math_text_item=true
through item/style merges; do not apply glyph-fragment italics correction or
script substitution to it. These flags do not reset size, cramped, font,
fallback, language, direction or features. Other math leaves are not TextItems.
Subtract terminal text glue only for direct Text immediately followed by
MathDelimited. Atomic markup and other boundaries retain their full advance.
Spacing rules belong to the spacing owner; pass active text-space metrics and
suppress class gaps when math_script or Script/ScriptScript size is active.
For a direct single-leaf MathClassOverride, retain glyph-fragment italics
correction unless the glyph has an extensible variant/assembly; do not repeat
that correction for nested composite frames.

Large operators in Display use the applicable display-operator minimum height.
Keep the base glyph when it already reaches that height. Otherwise choose the
first sufficient vertical variant; when none suffices, use a vertical assembly
if the active font supplies one, or its largest variant if it does not. Use the
base glyph only when no construction is available. The target has no delimiter
shortfall. The selected glyph's real ink is centered on the axis in both
inline and display modes, without changing operator identity or horizontal
advance semantics.

## External content boundary

Route Equation/Boxed/Align/Pad/Block and recursive content requiring normal
layout through the existing pure sub-layout bridge. Typed text deltas
(size, fill, weight, font, tracking, leading, lang, bold, italic) also require
that bridge; custom-only style transport and plain markup remain on the
baseline-aligned text path. Preserve lexical/effective style as far as the
existing bridge represents it, without claiming unsupported contextual access.

Use the first declared text/glyph baseline where present; only baseline-less
frames use axis-centered extents. Translate embedded items into local
baseline space; do not invent a top-relative Group wrapper or discard shapes.
If external layout emits nothing but plain_text is nonempty, preserve the
existing textual fallback. Region constraints, introspection, clipping and
metric approximations of this bridge remain documented limitations in the
diagnostic, not new language defaults.

## Acceptance

Observe language morphology, style precedence, baseline/extents and
font-derived geometry with attributable metrics. Preserve complete carriers
through both transformations, including empty-present attachments and optional
fields. Do not accept fixture coordinates, serialized-byte equality,
implementation similarity or historical test totals as parity evidence.

## Ratified textual transport

### Evidence before decision

Source→PDF RED probes under
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and this
consumer's FrameItem handlers were inspected before separator transport. The
public `00_nucleo/prompts/core/entities/layout_types.md` contract was approved
2026-09-28 and recorded in materialization provenance.
Bilateral probes/state in
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
and before/ measure U+002D versus SHY with equal geometry in eight controls.
This consumer's FrameItem path is archived in approved-before/; the owner
confirmed layout_types.

### Exclusive responsibility and acceptance

Transport TextSeparator position, consumed advance, style and origin without
adding ink/baseline/advance; preserve child order. Vertical scans derive neither
baseline nor ink from separators. TextReplacement is a transparent visual
envelope: descend through children, keeping text/order/coordinates and existing
calculation without new algorithm/advance. Pinned text-replacement claims share
invariants, not ownership. Acceptance uses text/boxes, causal tests, recursive
transport, literal-hyphen/adjacency controls, not PDF bytes or new selection.

## Accent base default style

The 2026-09-30 .typ/sec_12.typ probe shows hat(mu) retaining μ without math
italics while the same symbol outside accent becomes 𝜇 in pinned vanilla and
elsewhere in the equation. Inspection found apply_math_default's MathAccent arm
excluded all MathText despite its single-eligible-character mapping restriction.
Apply the same default math italics to accent bases, retaining explicit
MathStyled/multicharacter text. Change no mark/geometry/phase. Transformation
ownership remains `00_nucleo/prompts/core/compiler/math/layout/style_transform.md`.

## Ratified bidi and shaped-face transport

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
identifies references/exclusive owners; manifest 1ae7513e was confirmed. Adapt
uncertified constructors with None and read patterns with `..`, preserving
language observables under the pinned origin nucleus without phase/API expansion.

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}`
distinguishes regular GIDs from Oblique selection. The human confirmed public
TextShaped.font:Option<Font> in layout_types, SHA
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`.
Implementation snapshot/inventory is under
`lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

This owner migrates only its references: legacy constructors use None;
transformations retain Font; shaped comparison/join distinguishes face/axes.
The shaper provides an effective-Font clone also on cache hits without changing
semantics/style to force selection. Downstream owners
`00_nucleo/prompts/infra/shaper.md`, `00_nucleo/prompts/infra/export/fonts.md` and
`00_nucleo/prompts/infra/export/svg.md` retain these integration obligations:
collectors/subsets/CMaps/PDF bind the same original face at style axes; PNG/SVG
prepare local bindings and instantiate once per face/request before render,
keeping original resource index distinct from instantiated bytes. None retains
legacy resolution. Public APIs/FontKey remain; no artificial font/unsafe/global/
phase. Private helpers may receive Font/bindings explicitly.

Associate original face bytes and requested axes, never names/slots alone.
Export-local tables retain supplied resource order and append absent faces.
For multifont, distribute items once among bindings before per-font collectors;
never reuse GIDs/CMaps across faces. These are cross-owner boundary criteria,
not productive ownership here. Require RED→GREEN/bilateral miss/hit/VF controls,
without a time-gain promise. Shared claims remain in shaped-face-identity;
this owner materializes only its own references.

## Ratified transparent Source dispatch

Prior evidence:
`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`,
with HEAD/working tree/UTC/sources/argv/binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized
branches have the transparent operation; Source must not leak as an observable
element. Human approval `confirmo`, 2026-10-07, is in final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
the nested contract.md is the preapproval candidate.
workspace-seventh/p966 refutes raw-text Source fallback: math dispatch inspects
body, retaining identifiers/math italics/scripts and external/baseline paths.
Preserve realization phase/exclusive ownership/static dispatch, with no I/O/new
signature/trait/flag/eval↔layout change. C2/C/macro collections/tests, builds and
lineage remain required.

## Test-module conditional integration

Before decision, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree.
Duplicate cfg(test) pairs: math_limits_layout_tests.rs:5/parent:1714;
math_root_layout_tests.rs:5/:1717; math_frac_layout_tests.rs:5/:1720;
math_accent_layout_tests.rs:5/:1723; math_cases_layout_tests.rs:5/:1726;
math_matrix_layout_tests.rs:5/:1729. Sources/L0/UTC/status/diffstat are in
`lab/results/construction/diagnostics/cfg-test-ci-2026-10-13/before.json.gz`
and refinement-before.json.gz. Pinned Clippy reproduces duplicates. Removing
inner cfg preserves Rust tests, but the pinned architecture linter does not
propagate parent cfg and flags helpers V2.
Remove only outer cfg(test) on those declarations. Keep paths/names/visibility/
path attributes. Each file's inner `#![cfg(test)]` excludes its complete module/
imports/helpers from production. Test consumers/L0 stay intact. Add no dependency/
logic/API/default mode/phase; validate identical test list/core suite/non-test
build/Clippy/architecture. The six separately owned modules delegate their
bodies without this hub absorbing test ownership.

## Approved paint-root transport and incomplete exporter integration

### Evidence before decision

The human approved T1–T3 manifest
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`,
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Approved bilateral Parent/Solid evidence is not replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC,
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
before-source/syntax-nodes records this owner's constructors/destructuring
[536,543]. layout_types.rs:531–539 lacks root versus technical group distinction;
helpers.rs:253–269 reconstructs Group and must retain the field. Box combines
background/body; Block keeps body flat except clip. Paint reference is language,
envelope mechanics. General Rotate/Box geometry discrepancies remain open.

### Exclusive responsibility and acceptance

Adapt Group construction/patterns for approved paint_root. New technical groups/
legacy fixtures are false; reconstruction carries the received value without
inferring root from size/clip/transform. Preserve other fields/drawing/resources.
Exporters still do not use metadata for patterns: T2 remains uncertified here.
Destructure/recompose retains paint_root exactly; never reset true to false in
projection/marker removal/rebasing.
L0 first/RED→GREEN/restricted resealing; retain geometry/text in source→PDF/SVG
controls and suite/build/doc/check/fmt/lineage. This does not substitute FramePaint
for Shape.fill/stroke or implement PDF/SVG/raster patterns. Authorized T1c/T2/T3
remain open; build does not establish tiling parity/production readiness.

## Nominal test visual-item projection

### Evidence before decision

F040 is compiler/math/layout/mod.rs:302 (p1441_overline_geometry). AST spans/patterns/guards/
bodies/complements were checked before code.
`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC/sources/L0/config/hashes/status/git diff HEAD --stat. classification.json
records 70 cases/complements in 26 owners using the pinned CI parser. Pertinent
L0/source were read together, without new architecture or inferred upstream
intent. Rust exhaustiveness is mechanics; preserve product semantics/morphology/
geometry/effects/diagnostics.

### Exclusive prescription

For F040 p1441_overline_geometry, replace only `_` with nominal TextReplacement/TextSeparator/
Semantic/Line/Glyph/Image/Shape/Group/Link alternatives using the in-scope FrameItem
prefix. Preserve the complete fallback body, prior arms, guards, transformations,
calls, returns and order.

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

## Nominal Source-transparent core projections

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz`
retains UTC, sources/L0/hashes/configuration, status and exact git diff HEAD --stat;
initial-ten-owners.json.gz preserves the first snapshot. contexts.json,
all-contexts.json and wildcard-inventory.json retain AST/spans from the pinned
CI parser. The previous suite passed on the same input code, reconfirmed by hash.

source-probe-before.json measures inline limits at width 7.2pt versus 12.912pt
with Source. source-family-before.json preserves the first 12 pairs (11
differences). source-family-supplement.json and analysis preserve 23 attributable
pairs (19 differences, 4 equal controls): limits, kerns, Op.text, accents/dotless,
supports, matrix alignment, spacing, sequences, stretch and class.
source-lines-before.json measures the same equation at two lines versus one and
last baseline 16.2 versus 0 under Source despite equal extents. Geometry,
extractable identity and equation metadata are observables; bytes/pointers/Rust
equality are not. The content-source-origin nucleus already requires transparency;
inspection finds helpers classifying wrappers instead of their bodies. Infer no
historical intent or global parity. The supplemental analyzer's 24-versus-23
count error was corrected from valid original stdout without rerunning or
counting the analyzer error as functional RED.

### Exclusive prescription

Declare only private source_transparency_tests with inner `#![cfg(test)]` in
its exclusive consumer. Normalize transformed.source_body() in
layout_equation_measured for line_count/first/last baseline, node/next in
should_tighten_text_frame and content in single_math_leaf_char.
flatten_math_sequence_nodes classifies nodes/children by Source.body, preserving
atomic markup Sequence and opening only already-eligible technical sequences.
Derived fragments retain own origin or the container's first proven origin via
source_span/with_source_span. Do not clone already-originated fragments or add
reoriginating work without span. Preserve once-only glyph transformation,
baselines/callbacks/context/author tree/API/phases.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

| Site | Function | Scrutinee | Exclude unconditionally | Guarded complement |
| --- | --- | --- | --- | --- |
| L17 mod.rs:320 | p1441_overline_geometry | FrameItem | Line | none |
| L18 mod.rs:629 | needs_external_layout | Content | Equation, Boxed, Align, Pad, Block, Sequence, Styled | none |
| L19 mod.rs:936 | layout_equation_measured | Content | MathSequence | none |
| L20 mod.rs:1284 | single_math_leaf_char | Content | Text, MathText, MathIdent | none |
| L21 mod.rs:1431 | x_of | FrameItem | none | Text |

Content complements keep prior bodies outside the sought property, expanding
only specified Source transparency. Item collectors seek named species and
retain presence/count assertions; None is not layout success. Guarded variants
remain for rejected conditions. needs_external_layout's open-fallback comment
must describe its nominal complement, keeping false and all other comments.

### Acceptance and limits

L0 before code. This ADR-0127 internal parity correction adds no public field/
signature/trait, dependency, product flag/default, compatibility break or phase.
V19 groups share their previous body with per-case justification; notes remain
visible and do not prove execution coverage per alternative. No allow/expect,
threshold increase, @ capture or hidden match in if-let/matches!. Only named
transparency/shallow restructuring changes functional code; do not change
formulas/constants or relax expectations. Require functional Source RED and
structural warning/probe RED before repair; afterwards regression GREEN, zero
emitted-site V16 and future sentinel review of all 36 sites (including new
Content-under-Option projection) using copied Content/Value/MathStyleKind/FrameItem
in a disposable real workspace compiling cfg(test). Current scrutinees/functions/
composition change only as specified; revalidate AST/replay, signatures and other
owners.

Gates: full workspace suite, build, fmt, workspace/all-targets Clippy -D warnings
-D unfulfilled_lint_expectations, Rustdoc -D warnings with four indexes,
strict/reverse V1/V5/V15/V26, full differential SARIF and diff-check. Expected V16
170→152; new V19 only from prescribed groups/new observer; no V20 growth.
Other violations remain pending. Freeze sources during gates; MemoryMax 12 GiB,
zero swap, one Cargo job, heavy commands sequentially. No commit/push/release or
CI pin change. Lint closure does not prove RAM gain, AI-reading benefit or
complete parity.

## Source grid marker refinement

source-markers-before.json confirms Source(Linebreak) turns two lines into one
(a/b sharing baseline); Source(MathAlignPoint) keeps two but displaces 1/2 from
333/4's column. Preserve RED before repair. needs_grid_layout/partition_grid
classify via node.source_body(), retaining separators/empty lines/cells/order.
partition_grid content retains original clone/origin; use flat nominal complement
if removing its old normalized binding, without @/suppression. Origin never
creates Location. The regression owner adds two public display tests for Source
Linebreak/MathAlignPoint with identical baselines/positions. Previous 27 controls
stay, total 29 and expected RED 5 passing/24 failing. No contract/phase/default/
public API change.


## Planned direct Equation source bridge

### Measurement before correction

`lab/results/set-page-header-implementation-2026-10-21/route-probes11/receipt.json`
records the source-bound diagnostic11 product and the pinned vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Actual public syntax
`math.equation(metadata("equation-child"))` and
`math.equation(figure([EQUATION CHILD], caption: none))` both succeed in the
oracle and fail source-plan verification in the product. Ordinary `$a + b$`
succeeds with equal typed observations in both. Exact before source/L0 bytes,
hashes, UTC and working-tree provenance are archived in
`agents/layout/equation-bridge-before.json.gz` under that implementation root.

The source plan already declares EquationBody and recursively plans the direct
body according to the existing semantic partition. The equation painter calls
MathLayouter without that cursor; its external bridge creates an isolated
unplanned Layouter. Therefore required direct-body facts cannot be admitted by
actual painting. This is a missing internal handoff within the human-approved
physical-page phase, not authorization to open terminal MathSequence subtrees.

### Exclusive correction

Keep existing public constructors and measured signatures unchanged. Provide
compiler-private math-default and style-projection helpers for the owning parent
painter and a measured entry consuming its already painted opaque external frame.
Apply math-default once before painting and use the same existing MathBox-to-extent
composition afterwards. The no-plan/no-required-facts path remains unchanged,
including terminal MathSequence coverage and callback occurrence rules. No new
MathLayouter field, public API or pipeline stage is introduced.

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

Classify Raw as requiring actual normal layout in needs_external_layout,
including existing recursive Sequence/Styled wrappers. Delegate through the
unchanged pure external bridge so the existing Raw owner applies its defaults,
lexical overrides, token paint and real text baseline. Preserve the existing
Text/MathIdent paths and P994 baseline protections; do not flatten Raw to text,
introduce a source fixture special case, axis-offset patch, new helper/API,
phase or altered bridge. Existing bridge limits remain explicit. Meaningful
causal font/size/paint/baseline RED tests must precede the dispatch correction;
complete bilateral geometry and paint of both frozen sources is required for
closure. Internal parity correction follows ADR-0127 continuous flow.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

## Measured R3 geometry correction — 2026-10-23

### Measurement before decision

`lab/results/math-geometry-r3-2026-10-23/{baseline.json,baseline-source.json.gz,before/receipt.json,reductions/receipt.json}` retain the received R2 state, UTC, dirty HEAD/stat, full source map, exact inputs, binary hashes and commands. Oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Seven of 19 render probes fail 0.001 pt: five at 0.92119 pt vertical, unicode at 2.136005 pt, and selected-multichar at 0.492 pt. Isolated alpha without show has the same vertical error; adjacent ordinary text refutes a show-specific cause. Markup base and variation form both select NewCMMath rather than oracle FreeMono. Symbol arrow has omitted per-letter italic correction; sum happens to match.

Pinned `math/mod.rs:49–102` admits the normalized inline frame; `math/ir/resolve.rs:326–358` resolves each Symbol grapheme to a glyph; `inline/shaping.rs:929–936` uses ordinary fallback without inferring math from a font table. Current `equation.rs:49–52` installs the outer text strut before measurement, `math/layout/mod.rs:1354–1373` treats an entire Symbol as one glyph run, and `shaper/font_selection.rs:66–70` injects math fallback for ordinary DejaVu Sans because it has MATH. Coordinates, selected visual glyphs and source identity are observable output; storage/algorithm are mechanics. These are internal parity corrections under continuous ADR-0127, with no public contract/default/phase/compatibility change. Refutation requires bilateral PDF and SVG coordinates, adjacent-text, ordinary base/variation and grapheme controls; a passing glyph count alone is insufficient.

### Exclusive obligation
Keep MathIdent's existing lexical behavior. Dispatch native Symbol to the existing sequence owner for grapheme-wise composition, preserving the complete authored payload and style transformation. Each grapheme gets the existing glyph geometry/italic correction and class/spacing/large-operator logic; never split a variation selector or combining sequence from its base. No public identity rewrite, name lookup, added field or shaping/export compensation. Test per-grapheme advances and positions with deliberately nonadditive injected glyph metrics, plus selector/combining cluster and unchanged MathIdent/Text controls.

Require actual RED→GREEN, scoped resealing with valid V15/V26, full workspace validation, unchanged 0.001 pt bounds and preservation of received changes.
