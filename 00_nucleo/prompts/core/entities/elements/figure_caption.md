# Prompt L0 — `entities/elements/figure_caption` — `FigureCaptionElem`
Hash do Código: af02d023

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:b57c37587932a65caedce0b0df5691463ad336b90126a978ea88a7c369432738

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/figure_caption.rs`

## Evidence and ratified scope

The owner confirmed construction-only scope on 2026-09-21 after L0-only commit
3ac60a578; receipt is in
`lab/history/construction/records/00_nucleo/materialization/typst-passo-1473.md` §8.
Historical upstream 586e1bd43fae6c9a973218163d3165c53ab8d16d,
model/figure.rs:511-591, and the figure_image construction owner establish
the phase boundary: public figure.caption construction, without synthesis
or figure layout in this slice.

## Contract

FigureCaptionElem derives Debug/Clone/PartialEq/Hash and has public
position:Option<CaptionPosition>, separator:Option<Content>, body:Content.
CaptionPosition is closed Top/Bottom; never substitute a broader generic
alignment enum. Casts reject every other string with
`expected `top` or `bottom`, found <valor>` (historical value placeholder).
Position's language default is bottom, settable but without layout effect in
this slice. Separator's language default is Auto by language/region; None
means not explicitly supplied, not a resolved default.

Content::FigureCaption carries Arc. Preserve the documented constructor
Content::figure_caption(body:Content,position:Option<CaptionPosition>,
separator:Option<Content>) and its payload. The historical arrow names
FigureCaptionElem despite the Content constructor naming; that annotation
mismatch is retained as a specification clarification, not a new return type.
plain_text/is_empty delegate to body; content/text maps recurse into body and
Some separator. Equality/hash include all three fields.

## Existing-field reflection: evidence and contract

caption-shape-before.json and caption-after.json in
`lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17`
preserve source/UTC/binaries, validation-result.json HEAD/tree, and pin
9f2b6e8715237cb086899a42873660fe744622e8. Existing projection yielded only
func:figure.caption in CLI, omitting body/getters/fields; json.encode had the
same defect and kind metadata rejected the valid caption function. Upstream
model/figure.rs:510-594 declares Locatable and public position/separator/body
plus synthesized kind/supplement/numbering/counter and internal figure_location.
This correction does not synthesize those additional fields.

Override only existing Element::get_field: body is always known; Some Top/
Bottom yields vertical alignment; Some separator yields original Content;
None stays unknown with no baked default. Content delegates to this owner.
No struct, trait method, constructor, synthesis or phase change.

## Acceptance and excluded work

Preserve local plain_text/is_empty, position/separator traversal, content/text
mapping and equality/hash checks. The reflection correction follows ADR-0127
continuous internal/parity flow: L0 before real getter/metadata/encoder/CLI
RED→GREEN; external corpus keeps body/name expectations. Revalidate all package
gates under 12GiB/no swap/one job; reseal only owners after V15/V26.

Synthesis of kind/supplement/numbering/counter/figure_location, realize() text
combining supplement and Counter::display_at, regional separator defaults,
and upstream's `not known at this point` for unsynthesized fields remain out
of scope. Here absent synthesized data is an absent field, an intentional
mechanical difference. Reopening requires its own measurement/ADR-0127 gate.
FG1/SY1 remain incomplete per the package's follow-ups.md. No pending API is
authorized by field reflection.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/figure_caption.md` preserves the original chronology and evidence.
