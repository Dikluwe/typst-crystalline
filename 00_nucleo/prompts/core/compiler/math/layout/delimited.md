# L0 — math/layout/delimited
Hash do Código: 75bc4a27


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/delimited.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Consumer evidence

`01_core/src/compiler/math/layout/delimited.rs:21` locates `fn positional_lr_delimiters(body: &Content) -> Option<(char, Content, char)>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Fenced groups and lr/mid

Own mechanical MathDelimited composition plus public MathLr/MathMid layout.
For a balanced group, target height is
`2*max(body.ascent-axis,body.descent+axis)`.
The binomial's line=false fraction stack is unbalanced and targets body.height.
Stretch delimiters using their own axis-centered geometry and horizontally
concatenate opening/body/closing without moving the body's baseline or
axis-centering the complete group again.

MathLr retains public identity and body. Resolve size by explicit >
math.lr.size > 100%, with vertical shortfall 0.1em. For a group with fences,
measure the natural body and derive its relative target from the same
extent as an implicit fenced group: normally the axis-balanced
`2*max(body.ascent-axis, body.descent+axis)`, or the raw height for the
barless binomial exception above. Apply the effective percentage
or absolute length to that extent before stretch selection; an explicit
`lr` wrapper at 100% must not select a smaller delimiter variant than the
equivalent implicit group. Absolute length resolves against causal font size.
A body without fences remains visible; an item-only group preserves its
existing semantics. Nondefault nested sizes must not be flattened or baked
into a MathDelimited carrier. Only an implicit inner lr whose effective size
is 100% may be absorbed according to the approved rule.

Select first/last eligible fences by positional role, including inverted
pairs such as U+005D ... U+005B. Role changes do not replace original chars,
base_char, glyph variant, advance or order. Reuse the normal group's target.

MathMid classifies actual body items as Relation. Outside lr it keeps natural
geometry; inside, use the nearest lr's vertical target, including size and
shortfall, without crossing an inner lr boundary or independently selecting
a target. Preserve public body/identity.

Acceptance discriminates natural/balanced/binomial targets, same neighboring
baseline, 100%/150%/absolute/mixed/cascaded sizes, nested lr, mid scope and
normal/inverted fence variants. Current implementation breadth alone does
not demonstrate full language parity or retroactively prove a historical gate.

## Test-only legacy wrapper boundary

### Evidence before decision

compiler/math/layout/delimited.rs:54 contains the historical Clippy inventory
item. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
retains sources/owners/references/UTC/status/diffstat. Call-site and productive-path
research precedes classification, without new parity/performance evidence.

### Contract and acceptance

Compile internal lr_target_pt only under `#[cfg(test)]`, preserving signature/body
for existing tests. Only test helpers p1436_target_pt and p1437_mid_target_pt use this shortfall calculation. lr_requested_target_pt and productive selection remain unchanged.

Preserve productive routes, test names/bodies, external APIs, defaults and phase.
This ADR-0127 internal correction requires all workspace-target build/check,
core suite, test inventory, Clippy and lineage; remove no expectation to pass.


## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## Issue 8973 — measured lr sizing and linebreak correction (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8973-2026-10-31/before-state.json`,
before-sources.json, before/receipt.json and before-relative/receipt.json
bind dirty HEAD, exact diffstat, UTC, sources and freshly built product bytes.
The oracle bytes are verified against SHA-256
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34,
commit 64c5e8c02fe0394d913ead36e42ef8c1334d2107. The Tekt baseline warns
that set lr.size is ignored; even without set, its two fences enclose the
whole two-row stack rather than their own first/last segments. The pinned
oracle itself wrongly uses inherited size over explicit size in multiline;
that defect is not an acceptance oracle. Unset explicit-size controls,
unequal-height rows and percentage/mixed probes distinguish the healthy
sizing contract from that defect. Source: rules.rs:200-216 omits lr.size;
math_set.rs:170-195 drops it; delimited.rs:193-205 pairs across Linebreak.
Pinned math/lr.rs:25-28 defines size relative to wrapped content;
math/ir/multiline.rs:58-111 splits rows/cells; fenced.rs:63-100 takes the
maximum natural fragment extent, not the complete stacked height. Explicit
precedence is already ratified in the owning delimited L0 and MathLrElem
approval of 2026-09-17. Geometry, row semantics, casts, diagnostics and
morphological presence are language obligations; upstream IR/cache/storage
and output bytes are not. New measurements that lose a row, leak size,
resize non-boundary delimiters or change authored content refute closure.

### Classification

This is an internal parity correction under ADR-0127 continuous flow. Keep
all existing public entities, fields, signatures, defaults, phases and
compatibility. In particular preserve MathLrElem's original body, size and
size_explicit; no eval-to-layout migration or new public contract is needed.
L0 and seals precede code; preserve meaningful RED→GREEN and valid V15/V26.

### Exclusive correction

Own private, transient block-math lr row/cell views. Split only transparent
MathSequence/Source and MathLr at their existing Linebreak/AlignPoint markers;
retain original authored nodes, first source origins and every marker/order.
Do not cross Styled, MathStyled, fractions, attachments or other box scopes.
Nested lr retains its own effective size; carry outer sizes only across their
own fragments. Extract only the original outer first/last eligible fences;
interior row edges must not become invented outer fences. Resolve explicit
size above lexical math.lr.size above 100%, using the maximum natural
axis-balanced per-fragment extent (barless fraction exception retained).
Resolve absolute/mixed lengths against the causal font and clamp as before.

Transient fragments may use existing mechanical MathDelimited with the
existing zero-character absent-fence convention, wrapped in existing MathLr
with the resolved absolute target. These private views are never evaluated,
queried, serialized or substituted into authored content; original explicit
presence never changes. Sized composition skips absent fences instead of
emitting a NUL glyph. No new entity/field or metadata-sentinel surface.
Sizing-only passes use an isolated private MathLayouter and fresh private
MathCallbackPassState, with the same metrics/constants/lexical styles,
equation and region; their transcript is discarded and the accepted pass's
occurrence, consumption and nearest-mid state remain untouched. The existing
callback owner specifies that cancel angles change drawn lines, not MathBox
dimensions, so unresolved probe angles cannot choose a different lr target.
Only actual layout contributes callback occurrences. Absolute sizes require
no speculative body measurement. Existing nonbreaking lr and mid behavior,
shortfall, fence characters/classes, glyph identity and font selection stay.

Tests must first refute one-line EquationExtent and same-baseline endpoint
painting for a two-line lr, then require first/last segment placement,
explicit precedence, shared relative sizing, prefix/suffix flow, nested sizes,
mid, actual glyph variants, Source transparency and unchanged authored
body/size/presence. No extra terminal empty row or crossed opaque scope.
Recollect official, single-line, unequal-row, percentage/mixed and nested
render probes. No parity claim for previously uncertified general math
layout/wrapping or inline math containing an explicit linebreak.


## 8973 recheck at 3c6b0ce7 — direct nested Equation and shared boundaries

### Fresh measurement before decision

The clean HEAD is3c6b0ce7b7632cc74b004cea6681e4746e63d28d on Tekt.
`lab/results/construction/diagnostics/issue-8973-recheck-3c6b0ce7-2026-10-31/`
contains before.json.gz, before-receipt.json and oracle-receipt.json with exact
source/L0/configuration bytes, HEAD/status/diffstat/UTC, binary hashes, commands
and independent SVG raster geometry. The original issue literal is healthy in
Tekt: explicit3em retains73.25pt ink height with set1em; pinned vanilla shrinks
to29.25pt. Therefore its inherited-size regression must not be copied.
A separate Tekt failure exists: lr whose complete body is an Equation cuts
single-row ink to2.75pt and loses multiline text. Healthy unset vanilla and
unwrapped-body controls retain33pt single-row or73.25pt two-row ink. Both inline
and block Equation wrappers fail in Tekt. Consecutive complete lr wrappers also
ignore an outer absolute3em on a single-row paired fence, while healthy vanilla
uses33pt. Genuine nested groups inside an outer bracket pair are separate.

Pinned `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:859-869`
explicitly projects the complete Equation body; :891-925 retargets the boundary
fences of a single fenced component with the surrounding lr size.
Current delimited.rs:489-532 treats Equation as an ordinary external box and
returns a nested lr's unchanged box for non-leaf bodies. The former protocol's
lr_multiline_body also cannot see markers behind that complete Equation.
Physical content, rows, fence size and authored body/presence are language;
private projection/IR/glyph IDs/byte equality are mechanics. The inference that
a complete shared-boundary wrapper may reuse the outer absolute target is
refuted by resizing independent inner fences, lost rows, changed no-wrap peers,
source/callback effects or modified authored content.

### Classification and bounded exclusive correction

Continuous internal parity correction under ADR-0127: no public field, trait,
signature, default mode, compatibility break or eval/layout phase movement.
Keep the original MathLrElem and EquationElem unchanged for eval, queries,
source identity, hashes/repr and callbacks. No native-constructor unwrapping.

Provide a private transient lr layout-body view used before both the multiline
marker decision and ordinary lr composition. Project only a COMPLETE direct
Equation body, including its Source origin, through the existing math-default
transform. Do not project Equation descendants inside genuine groups, fractions,
Styled/MathStyled, attachments or other opaque bodies. Its block flag does not
change the surrounding mathematical context. This narrowly supersedes the old
blanket Equation opacity only at this complete lr-body boundary.

For an outer absolute target only, consecutive complete lr wrappers whose
body has a certified paired boundary (existing MathDelimited or the current
positional fence classifier) may expose that same body to the outer size.
One-item MathSequence transport may expose only such a complete wrapper or
Equation. Preserve all genuinely interior lr groups and nearest-mid scopes;
never cross a non-paired body such as a bare mid wrapper. This refines the old
nondefault-nesting clause only for redundant wrappers around the exact same
boundary fences and an absolute outer target. Relative consecutive-wrapper
retargeting remains outside this bounded repair and keeps its old route.

Return a temporary MathLrElem/view with the original outer size and explicit
presence, preserving source spans and every original row/align marker. No
source reevaluation, reparented Location, new carrier, public API or new phase.
Reuse existing row splitting, sizing probes and accepted callback pass exactly.

### Acceptance and refuters

Functional module RED must show lost row/fences for an Equation body and ignored
outer absolute size for a complete paired lr body. GREEN requires inline/block
Equation parity with the same unwrapped body, two real EquationExtent rows,
explicit3em over inherited1em, unchanged authored body/size/presence and normal
nonbreaking/relative/mixed controls. Genuine interior2em groups, mid scopes and
source-wrapped bodies remain intact. Recollect the literal and nested cases in
PDF, SVG and raster against healthy controls; the known multiline vanilla
precedence defect is not the oracle. Preserve existing callback/cancel/source
regressions; add the direct Equation callback control without extra evaluation.
Build and tests remain in the exclusive tmpfs target with MemoryMax12GiB,
MemoryHigh8GiB, MemorySwapMax0 and one job. V15/V26 and restricted seals precede
code; require full lint/reverse lineage, build and proportional tests. No commit
or push. Other math breadth, inline linebreak layout and relative redundant-lr
retargeting are not certified by these absolute-target fixtures.
