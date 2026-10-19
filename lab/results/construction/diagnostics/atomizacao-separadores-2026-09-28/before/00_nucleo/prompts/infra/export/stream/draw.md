# L0 — stream/draw
Hash do Código: 5ebc84c1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/draw.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/draw.rs:26` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Recursive painting

Own exhaustive static draw_item_top/draw_item_local dispatch and recursive handling
of Text/TextShaped/Glyph, Line/Image/Shape/Group/Link/Semantic. Preserve paint order.
Top-level uses page_height - y; top-level Link children use that same convention.
Group applies position/matrix and clip once, then uses local drawing. Local text
uses the existing reflection envelope q / 1 0 0 -1 0 0 cm / Q with (x,-y), restoring
the text helpers' coordinate precondition; line/shape/image keep their own conventions.
Images compose resolved size/position with EXIF 1-8 once at the existing precision.

With tags disabled, Semantic is visually transparent. With tags enabled, draw
Formula/Lbl/TH/TD from supplied MCIDs and balance BDC/EMC; alt is never painted.
Artifact Other uses BMC. Header emits `/Artifact << /Attached [/Top] /Subtype /Header
/Type /Pagination >> BDC`; Background emits its existing Type Background property
list `/Artifact << /Type /Background >> BDC`. Other artifact variants retain BMC. These branches do not select properties
by PDF version and consume no MCID. Do not manufacture a StructElem here. In UA1, meaningful leaves
use Span/Figure while geometric decorations use Artifact Layout. Boundary carriers
paint no empty content. Recurse through each child exactly once.
Delegate text and geometry to their children; resource IDs and ParentTree belong
outside this owner. Unsupported artifact-version/AT behavior remains unproven.

## Verification

Check top/local coordinate composition, nested reflection, clips, asymmetric EXIF
images, balanced semantic/artifact envelopes and UA1 boundary/leaf behavior.
