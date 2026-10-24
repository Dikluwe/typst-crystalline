# L0 — stream
Hash do Código: 3922e873

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream.rs:54` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own FontScenario, PageContext, borrowed resource/font maps, constructors and internal
compatibility reexports. Preserve Type1/CIDFont/Multifont static variants, existing
visibility and supplied graphic mode. Raw context constructors initialize tags to
Disabled and UA1/oracle to false; product defaults belong to the export façade,
not these constructors. Preserve independent tagging/oracle settings. Descendants
receive this context; font/image/gradient resources are prepared outside this owner.

Own the deterministic page-local MCID prepass in with_pdf_tags. Restart at zero per
page and traverse `page.items` in the current content order. This prepass does not
traverse the page background or foreground layers. Formula, Label and TH/TD own
their MCIDs outside Artifact. Artifact suppresses descendants; UA1 boundary carriers
consume nothing, semantic owners stop further UA1 leaf collection, and real Text,
TextShaped, Glyph and Image leaves receive UA1 MCIDs outside those owners. Group/Link
recurse. Disabled tagging does not populate marked-content assignments. Expose the
same assignments to drawing and builder structure construction, not parallel rules.

Keep the existing inline regression suite owned by this consumer. Page assembly,
recursive painting, text operators and shape primitives belong only to the children
below; this façade does not own their algorithms or PDF stream compression.

The façade reexports the private marked Glyph emission variants to the draw
owner for Large-operator occurrence identity. This is internal composition of
existing PDF stages: it does not choose text from GIDs, create a new mode or
alter the ordinary Glyph emission API.

## Delegated owners

- `00_nucleo/prompts/infra/export/stream/draw.md` owns `03_infra/src/export/stream/draw.rs`.
- `00_nucleo/prompts/infra/export/stream/page.md` owns `03_infra/src/export/stream/page.rs`.
- `00_nucleo/prompts/infra/export/stream/shape.md` owns `03_infra/src/export/stream/shape.rs`.
- `00_nucleo/prompts/infra/export/stream/text.md` owns `03_infra/src/export/stream/text.rs`.

## Textual carrier composition

Source→PDF RED probes/handler inspection in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` preceded public layout_types approval 2026-09-28. Declare descendant separator, retaining static dispatch/resources. UA1 treats TextSeparator as textual Span leaf with coordinated MCID and paragraph enclosure/semantics.

`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` records eight bilateral U+002D/SHY equal-geometry controls; approved-before archives traversal inspection and layout_types owner confirmation. Declare private text_replacement; tagging descends children with no envelope-owned MCID. Transparent visual envelopes preserve text/order/coordinates/calculation, no new algorithm/advance. Pinned text-replacement owns shared claims; require text/boxes/causal/recursive/literal-hyphen controls, no PDF byte equality/new selection.

## Ratified bidi and effective face binding


`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
records unit references/1:1 owners; confirmed manifesto 1ae7513e requires None for
uncertified constructors and `..` read patterns, preserving language observations
under the pinned origin nucleus without phase/API expansion.

`identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}`
distinguishes regular GIDs from Oblique selection. Human approval of public
TextShaped.font: Option<Font> is manifesto
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`;
implementation state/inventory: identidade-fonte-shaped-implementacao-2026-10-05/.

Migrate only this owner's references: legacy constructors None, transforms preserve
Font, shaped comparison/join distinguishes face/axes. The shaper supplies the actual
Font clone even on hits without semantic/style changes to force selection. Collectors/
subset/CMap/PDF bind the same original face and style coordinates. PNG/SVG prepare
local bindings and instance once per face/request before render, keeping original
resource index separate from instanced bytes; None follows legacy resolution.
Preserve public APIs/FontKey, no fabricated fonts/unsafe/globals/new phase; private
functions may receive Font/bindings explicitly.

Association uses original face bytes plus axes, never names/slots alone. Table
lifetime is export-local; preserve supplied resource order and append missing faces.
For multifont distribute items once before per-font collectors; no cross-face
GID/CMap reuse. References never share ownership. Require RED→GREEN/bilateral
miss/hit/variable-font controls without a promised speedup. Shared invariants remain
in pinned shaped-face-identity, no second productive owner.

## Proven unused private aliases

After export-facade import removal, infra-imports-docs-ci-2026-10-13/check-initial-failed.log identifies draw_item_local reexport at stream.rs:43; source search finds only alias/comments, while draw owns its definition/calls. additional-before.json.gz preserves source/L0/state. Remove only that pub(super) reexport; retain required child aliases/PageContext/FontScenario/algorithms. conic-posicionamento-2026-10-14/green.log identifies unused dedup_key_for at stream.rs:11 after shape switched to gradient occurrence keys; facade-before.json.gz preserves prior state/source/L0. Remove only this private import. Both continuous internal changes preserve public API/default/phase and require all-target compile, unchanged suite names/assertions and lineage, no suppression.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [788]. layout_types.rs:531–539
lacks technical-root distinction; helpers.rs:253–269 reconstructs Group. Box groups
background/body; Block stays flat except clipping. Reference-frame preservation is
language, envelope representation mechanics; prior general Rotate/Box geometry
divergences remain open.

### Contract and acceptance

Adapt Group construction/patterns to approved paint_root. New technical/legacy
fixture groups are false; reconstruction preserves received value, never inferring
root from dimensions/clipping/transforms. Keep other fields/drawing/resources.
Metadata migration alone does not certify exporter pattern integration or replace
Shape.fill/stroke with FramePaint. Independently authorized prepared-transport/
native-target/parity slices (historical T1c/T2/T3) stay incomplete until integrated
verification, no production/parity claim from a successful build. L0 first,
RED→GREEN/scoped resealing; preserve source→PDF/SVG text/geometry and revalidate
suite/build/doc/check/fmt/lineage.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Private context receives occurrence-keyed tiling references; constructors default to None for nontiling fixtures. Decorative cell emission temporarily disables tags/UA1 and restores fields before the page, preserving font scenario/resource maps. build_cell_stream uses draw_item_local in Y-down; Pattern matrix owns the external inversion. Do not clone items or invent page height.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

Place build_cell_stream before tests without private signature/behavior change. No raised limits/allow; suite/renders before/after, fmt/doc/lineage.

## Solid opacity composition


`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserves source/owner/HEAD/UTC/tree. tiling-nativo-2026-10-14/final-renders and remaining-analysis.md show cell8×8/pitch4×4/alpha50%: PDF opaque red, SVG one contribution, PNG four(255,15,15). pdf_tiling.rs:184 uses BBox=size, svg/paints.rs:148 overflow visible, shape.rs:155/text.rs:275 drop alpha. Upstream also clips overlap and is not a requirement for that defect. The explicit-period source-over hypothesis must preserve no-overlap controls and analytical four-contribution composition.

Privately reexport the shape owner opacity helpers for resources/draw/text/page, without changing PageContext or external APIs. This is internal within the approved tiling package, no new public API/phase/flag/default. L0 first/RED→GREEN, analytical composition including distinct-color order, simple/nested/rotated/text/image and extraction/page-selection controls; workspace/build/fmt/Rustdoc/lineage. Variable-gradient alpha/PNG quality remain incomplete, so no production certification with these open.

## Existing scenario construction and representation

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` preserves checked prior L0/source/signatures/hashes/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted status/diffstat/UTC2026-10-15T16:39:18.873477+00:00. cidfont:224 has 9 arguments (C062), multifont:255 has 11(C064), threshold7. This is Rust structure, no functional/parity proof. Existing constructors dismantle/remake four/six FontScenario components plus page resources/mode; reuse the existing typed boundary, not a second correlated array set or invented cardinality validation.

`lab/results/construction/diagnostics/type-complexity-2026-10-15/{before.json.gz,red-clippy.log}` preserves state/source/L0/hash/UTC and RED at stream.rs:76(C061), :260(C063). Use existing export::FontKey for Multifont fonts and equivalent parameter with identical borrowed slice/lifetime/type expansion. Remove only three unused type imports. This alias correction alone did not authorize the separate arity materialization; the later nominal materialization below supersedes its pending status only for C062/C064.

`lab/results/construction/diagnostics/clippy-aridade-interna-2026-10-16/before.json.gz` records sources/callers/current contracts/spans/28-warning inventory and source hashes/HEAD/tree/UTC/diffstat. Replace private cidfont/multifont with with_font_scenario taking four borrowed resources, existing FontScenario and StreamMode(six parameters). type1 retains signature/delegates FontScenario::Type1. Preserve all PageContext fields/defaults: tiling_names None, tags Disabled, semantic_mcids empty, ua1/oracle false, supplied mode, default LookupCache and supplied index alignment/glyph reverse/bitmap/axes/face selection. Keep enum visibility/variants/borrowed data/no map/font clones, no array-validation claim/new fallback/public type. Migrate CIDFont stream fixture to the identical scenario with unchanged asserts/bytes.

Continuous flow adds no external API/default/phase/dependency/semantics; private caller L0 precedes code. Identical alias expansion must be demonstrated; a new/incompatible public type needs another gate. Narrow nominal type_complexity exceptions cover only the three justified batch items, no module/impl/crate silence/threshold increase/other correction suppressions. Six-case arity materialization permits only conditional C047/C066 expectations after removing their dead _text arguments, not the five remaining correction arities. Preserve public fields/values/PNG-SVG-PDF/precedence/diagnostics/resources/MCIDs/no globals. Keep fixtures/expectations/tolerances and existing controls without private-carrier/alias/syntax tests. Require selected lint RED→GREEN, denied unfulfilled_lint_expectations, full workspace/build/all-target Clippy/Rustdoc/fmt/strict reverse lineage, 12GiB/swap0/jobs1; no new parity/performance/AI-benefit claim.

## Existing MCID condition and exhaustive decision

stream.rs:219 historical T325 and `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz` preserve source/L0/SHA/HEAD/tree/UTC/status/diffstat; hash-reconfirmed inventory 28 guards/21 owners under pinned CI. In collect's five leaf types move ua1 && !inside_artifact from guard into body. Occurrence insertion/next increment stay conditional; preserve Disabled tags, semantic MCID owners, Artifact, recursion/order/page scope. No upstream-intent/functional-defect/AI-reading inference. Private maintenance, no new API/default/phase/suppression/threshold or hidden guard; preserve bodies/tests/real CI gates.

## Exhaustive existing FrameItem decisions


### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

Only replace collect's `_` at stream.rs:229 (historical F056) with FrameItem::Line/Shape, using the existing prefix.
Preserve complete fallback bodies/prior arms/guards/transforms/calls/returns/order.
Callbacks/effects/metadata/clipping/fonts/positions/ink/width follow current owners;
no expanded traversal/wrapper-leaf change/text-or-ActualText geometry replacement.
Tests retain fixtures/expectations/tolerances/counts/Unknown failures; V19 group
notes stay visible without pretending one arm covers all alternatives.

Continuous ADR-0127 adds no enum/field/trait method/public signature/default/
dependency/compatibility/phase and approves no historical pending proposal.
L0 first, scoped bidirectional resealing; no suppression/higher threshold or
match→if-let evasion. Before code run new FrameVisitor control/workspace suite
with unchanged expectations and confirm AST RED for all 70 family warnings.
Afterward zero scoped-family V16/no new warnings; outside V16 remains pending.
For productive core matches reached by cargo check -p typst-core, use a disposable
FrameItem sentinel variant to observe absent E0004 before/present after. That
probe covers only compiled sites, never unbuilt cfg(test) or blocked L3 sites.
For the full batch replay patterns/guards/bodies, verify enum/signatures unchanged,
real build/tests, no stub bodies.

Revalidate full workspace suite/cargo build/strict workspace-all-target Clippy
(-D warnings, -D unfulfilled_lint_expectations)/fmt/Rustdoc -D warnings/four HTML
indexes/strict V1/V5/V15/V26/reverse lineage/SARIF/diff --check. Freeze sources/L0/
configuration during runs; heavy commands sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1. Structural cleanup is not new global parity,
memory gain or measured AI benefit.

## Compact coordinate regression expectations

`lab/results/pdf-bbox-precision-2026-10-23/compact-green.log` records two
legacy byte expectations failing after the compact absolute-origin regression
turned GREEN. stream.rs:488–492 checks linked child700pt Y; :727 checks the
Type1 envelope. Keep both test names, coordinates, text and operator sequence.
Update only expected Td decimal spellings to five decimals, including the
negative wrong-Y assertion; retain the complete compact envelope assertion.
Historical byte-invariance wording is superseded only for Td precision. This
owner supplies regression expectations, not descendant formatting logic. No
new public/default/phase behavior. Require all exporter regressions and complete
final-state validation, preserving the failed candidate log.
