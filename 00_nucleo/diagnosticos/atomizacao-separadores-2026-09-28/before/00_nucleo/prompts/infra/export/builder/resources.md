# L0 — builder/resources
Hash do Código: aae8dcfe

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/resources.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/resources.rs:46` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Resource aggregation

Own page ColorSpace entries, spot-colorant collection and Separation dictionaries,
gradient-object emission and image-XObject aggregation. Consume image and gradient
payloads from their specialized owners; do not redefine decoding, ICC profile
selection, gradient sampling or conic mathematics here.

Verbose resources include /c0 referencing the shared sRGB ICC profile, even when
there is no RGB JPEG. Compact retains its existing resource policy. An RGB JPEG
requires the shared profile; grayscale and CMYK keep their own image paths.
Collect selected-page spot colorants in encounter order across background, body,
foreground and nested content, including text, line, shape fill/stroke and gradient
stops. Identity is the complete colorant, not only its name. Emit deterministic
resource names by delegating complete-colorant naming to
`00_nucleo/prompts/infra/export/stream/shape.md`, and correctly escaped PDF names,
including None and All. Separation
alternate spaces and Type2 tint functions preserve the declared fallback: RGB uses
the shared ICC when available, and CMYK starts at zero ink. Do not replace /c0.

Emit supplied gradient and image objects at their allocated IDs with resolvable
references. Preserve spot-gradient Separation resources and the existing axial
Extend and stop-function routing without taking ownership of the gradient model.
Merge prepared bitmap /ImN entries into page /XObject dictionaries alongside normal
images; preserve both when the page uses both. Excluded pages must not introduce
page-local spot inks into an included page's resource dictionary.

## Verification

Check RGB JPEG, grayscale/CMYK, verbose/compact, repeated and same-name distinct
spot colorants, gradient roles and mixed bitmap/image resource closure.
