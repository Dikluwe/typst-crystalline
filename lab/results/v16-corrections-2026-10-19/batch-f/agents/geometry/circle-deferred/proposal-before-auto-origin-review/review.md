# Circle radius and constructor identity — review candidate

Status: proposal only. No active L0 or productive source is changed. V16-065 remains open. This is a bounded constructor-identity and explicit-radius correction, not complete Circle parity.

## Evidence before decision

The source and owner hashes are frozen in `source-manifest.json`. The current source has one ShapeKind::Ellipse for both constructors (`stdlib/shapes/ellipse.rs:58,130`), converts circle Length to its absolute scalar (`:92-104`), and exposes that carrier through derived ShapeElem PartialEq (`entities/elements/shape.rs:21-29`). `entities/content/equality.rs:93` delegates to that relation; `morphology.rs:185` preserves Shape. The private language comparison in `compiler/eval/operators/equality.rs:150-172` strips only specified style details and then compares projected Content. These observations establish a causal risk, not upstream intent.

The actual baseline equality receipt `../../baseline-equality/receipt.json` binds eleven language probes to the frozen baseline product build and pinned oracle. Circle 2pt equals the deliberately supported number radii 2 and 2.0 locally. Circle 2pt differs from ellipse 4pt and circle 2em differs from ellipse 4em in both languages. Circle 2em wrongly equals circle 0pt and circle 1em locally. The rejected candidate preserved full Length and passed actual 2em geometry at font sizes 10pt and 20pt, but failed two language equality tests in the central candidate runtime: numeric-radius compatibility broke, and Circle became equal to Ellipse. `../deferral.json` binds that failed log, candidate state and archived patch. The entire original owner pair was restored, rather than dropping the acceptance conditions.

Pinned source `lab/typst-original/crates/typst-library/src/visualize/shape.rs:222-304` declares distinct Ellipse/Circle language elements and converts Smart<Length> radius to twice its full relative-length value. `typst-layout/src/shapes.rs:438-456` lays out Circle separately. Tekt does not need to copy these Rust types or layout strategy. Existing Tekt `compiler/layout/shape.rs:35-49,171-190` already resolves full Length against the active font; generic Auto/default geometry is separately incomplete.

## Proposed public contract

Add exactly this public enum and field in the existing ShapeElem owner:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeOrigin {
    Geometry,
    Circle,
}

pub struct ShapeElem {
    pub origin: ShapeOrigin, // new; all existing fields retain their types
    // kind, width, height, fill, stroke, fill_rule remain unchanged
}
```

`Geometry` means the established generic Shape construction contract. It is not a claim that Rect, Square, Polygon and Curve identities are complete. `Circle` records the semantic constructor family independently of its rendered ellipse geometry. This is a typed domain distinction, not a hidden style marker, special floating-point value, alternate Value variant or identity inferred from dimensions. ShapeKind remains unchanged. No new Content variant, trait method, native signature, public constructor signature, I/O or phase is introduced.

Existing `Content::shape` and `shape_with_radius` keep their signatures and initialize Geometry. Existing direct ShapeElem literals likewise initialize Geometry. Native Circle constructs a ShapeElem with Circle using the current public entity fields, without mutating a shared Arc. Native Ellipse continues the current Geometry route. Cloning and terminal maps retain the origin. Derived ShapeElem PartialEq and its existing structural hash include origin; conservative ObservationEq explicitly compares it. Morphology and the language comparison algorithm need no new special case: they preserve this semantic distinction through their existing projections. Content's compact hash may still collide for unequal shapes; do not claim hash uniqueness or rewrite hashing as part of this correction.

This adds a required public struct field and a public enum, so external Rust struct literals must migrate. ADR-0127 §2.1 approval is required. The existing shape owner also shows `fill: Option<Color>` in its code block while productive source uses Option<Paint>; reconcile that stale documentation with the actual retained field type, without changing paint behavior.

## Radius conversion and compatibility

For an explicitly supplied Float or Int radius, convert the existing point-number extension to Length::pt, then double it. For an explicit Length, double both absolute and em components without resolving a font in eval. Both routes store the same canonical `Value::Length` diameter in width and height. Thus circle(radius:2), circle(radius:2.0) and circle(radius:2pt) remain equal and keep the same geometry. Same-circle aliases remain equal; different radii and mixed absolute/em values retain their distinction. Circle and generic Ellipse are unequal regardless of equal final dimensions or Value representation.

Retain explicit Auto and omission as the current bounded unresolved behavior: omission keeps absent dimensions; explicit Auto keeps the established zero-diameter path for now, explicitly documented as debt rather than correct Circle semantics. Do not hardcode 30pt or infer full default/body sizing from the oracle. Numeric extensions remain accepted, including their existing negative/nonfinite behavior unless a separate measured decision is approved. Length sign/nonfinite cases need preservation controls and must not be silently clamped.

Reject only the measured malformed supplied radius classes using the frozen oracle type diagnostic and causal value span; the current pinned upstream admits Auto but does not admit Ratio/Relative as radius. Preserve name validation, radius-before-paint order, fill/stroke/default absence rules, and all unrelated constructor arguments. Keep the Value decision nominally exhaustive, retaining Auto as its own explicit debt branch. Do not enumerate invalid values merely to hide V16 while preserving malformed-to-zero behavior.

The approval scope explicitly includes correcting accidental language equalities caused by the old shared scalar carrier: Circle 2pt currently equals generic Ellipse with numeric 4×4 dimensions locally, and em radius values collapse together. Those equalities change to false. This is not deletion of the numeric radius extension. Circle/ellipse function identity, fields, selector/show/set support, body/inset/outset, width/height syntax and complete default sizing remain separate measured work; this proposal must not claim them complete or change their current paths.

## Acceptance and refutation

Before implementation, freeze a fresh baseline and place the archived six Circle tests back under the current owner with corrected explicit scope. Add meaningful language comparisons for numeric/Length equivalence, Circle/Ellipse distinction with pt/em/numeric dimensions, same/different mixed radius, aliases, arrays/dictionaries/membership, Source and ordinary semantic wrappers. Preserve existing zero/negative/paint/stroke/default controls. Any previously matching language case regressing outside the explicitly approved accidental equalities refutes the implementation.

Actual layout must measure explicit 2em diameter 40pt at 10pt text and 80pt at 20pt text, plus unchanged 2pt diameter 4pt, mixed absolute/em scaling, reuse under two occurrence font sizes, and nesting in existing Block/Box routes. Use actual logical geometry and paint bounds, not only stored width/height. The existing 182-case replay and equality corpus remain intact; only approved case-specific expectations may change. Auto/default cases stay visibly failing parity obligations, not normalized away. Pinned oracle identity, source/build hashes, HEAD/dirty state and UTC bind each measurement.

Require RED before code, all focused/full suites, build, strict Clippy/Rustdoc, formatting, path checks, direct/reverse lineage, V15/V26 and exact V16/V19 delta. A real future Value variant must force E0004 at the repaired radius decision; no suppression or nominalization-only closure. The field migration must compile every direct ShapeElem literal and conservative observer. No acceptance depends on Rust equality alone or equal output bytes.

## Decision boundary

Approval would authorize the exact ShapeOrigin field/type, mechanical call-site migrations, canonical explicit-radius conversion, malformed diagnostics and the listed accidental equality corrections. It would not authorize implementation of default Circle sizing or its other missing language surfaces. Until approval and successful validation, this is a proposal and V16-065 remains unresolved. Candidate owner deltas and the exact impact graph are alongside this document.
