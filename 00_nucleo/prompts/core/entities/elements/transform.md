# L0 — entities/elements/transform
Hash do Código: 2da70039

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/transform.rs`
**References:** ADR-0107/0108/0109/0127/0128/0129/0130.

## Evidence before contract

`lab/results/construction/diagnostics/transformacoes-2026-10-14/{before,results}.json.gz`,
controls/results.json, binding-controls/results.json and context-controls/results.json
preserve HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree/UTC/diffstat,
sources/commands/binary hashes and pin9f2b6e8715237cb086899a42873660fe744622e8.
The old entity stored only matrix/body (line 19), stdlib/transforms.rs:19/51/91/191
discarded arguments into matrices, and layout used a central pivot/has_orphaned
layout workaround. Upstream preserves four elements and resolves styles,
region/body/origin/reflow in layout.

Probes found missing fields/native func, rotate(0deg)==scale(100%) wrongly true,
collapsed explicit defaults, ignored/rejected origin/reflow, scale(x:150%)
wrongly scaling y (upstream 100%), and lost move percentages/em. Absent fields
must remain lexically unknown. set rotate(23deg) is valid while named angle
is not; contextual getters reflect style. Initial context controls labelled
the wrapper and cannot measure value; corrected controls relocate the label.
Pinned HTML warns and drops body; strict Tekt HTML rejects the operation.
Language observables include semantics/syntax/morphology, positions/advances
and diagnostics; enum-versus-struct and measurement-step count are mechanics.
Refuters include collapsed presence, baked style, wrong origin, lost errors,
reflow=false changing advance or dropped body.

## Authoritative typed contract

This replaces the earlier matrix/body carrier; exact public types/fields:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TransformElem {
    pub operation: TransformOperation,
    pub body: Content,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformScale {
    Auto,
    Ratio(Ratio),
    Length(Length),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformOperation {
    Matrix(TransformMatrix),
    Move {
        dx: Option<Rel<Length>>,
        dy: Option<Rel<Length>>,
    },
    Rotate {
        angle: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Scale {
        x: Option<TransformScale>,
        y: Option<TransformScale>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
    Skew {
        ax: Option<Angle>,
        ay: Option<Angle>,
        origin: Option<Align2D>,
        reflow: Option<bool>,
    },
}
```

Use existing Content/Rel/geometric types, not another units/alignment family.
Option means explicit presence, not resolved default; Some(Auto) differs from
omission. Body is language-required and always appears in fields, even empty.
No redundant matrix beside operation.

## Projection, preservation and compatibility

Native name/function derive from the variant, never coefficients. Lexical
order: Move dx,dy,body; Rotate angle,origin,reflow,body; Scale x,y,origin,reflow,body;
Skew ax,ay,origin,reflow,body. Omit only None fields. get_field yields explicitly
known values; method classification distinguishes declared/omitted from
undeclared. Layout defaults never become explicit fields. Language identity
and equality distinguish operation and presence, not only the resulting
matrix; conservative observation must not reuse unlike operations/presences.

plain_text delegates to body; is_empty keeps Element's false default.
Clone, equality-compatible manual Hash, maps and reconstruction preserve the
whole operation. Layout stays in
`00_nucleo/prompts/core/compiler/layout/transform.md`, without reverse imports
or entity rendering logic. These fields do not make the family locatable.

Public matrix is removed; external reads/literals migrate to operation.
Generic use is TransformOperation::Matrix(matrix). Existing
Content::transform(matrix,body) remains and constructs Matrix with prior generic
semantics, without native move/rotate/scale/skew identity/properties or a new
public constructor. Native constructors create typed variants directly.

## Ratification

The human approved the typed-family package (X1–X5 implementation/tests, no
publication), manifest `960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`.
Receipt: `lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/approval-receipt.json`.
This receipt applies to the specified public compatibility migration.

## Acceptance

Require identity/presence (omission/zero/false/auto), partial axes, units and
maps RED→GREEN. Cover constructors,set/show,contextual getters,serialization,
layout and target controls; green build alone does not certify migration.
No I/O/globals/vtable/entity PropMap or inference from rendering.
Hash cannot use raw Debug: equality ignores signed zero and transparent body
wrappers. Canonicalize parameter zeros and conservatively project body text;
nontext body collisions are allowed but never imply equality. Refute with
-0/+0 and Source controls.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/transform.md` preserves the original evidence and chronology.
