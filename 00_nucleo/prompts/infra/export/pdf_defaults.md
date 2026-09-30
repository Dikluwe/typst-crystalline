# L0 — infra/export/pdf_defaults
Hash do Código: 61654d67

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/pdf_defaults.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/pdf_defaults.rs:` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Canonical export constants

Own the shared constants consumed by PDF and raster geometry/text emitters. Do not
redeclare the child emission algorithms or calibrate domain geometry to serialized
bytes.

- BEZIER_CIRCLE_KAPPA = 0.552_284_749_831 approximates a circular quarter arc by
  cubic Bézier, with mathematical value 4*(sqrt(2)-1)/3. Preserve this geometric
  coefficient; ISO 32000-1:2008 and the PostScript language reference provide context.
- FAUX_BOLD_K = 0.04 is the stroke-to-font-size ratio for synthetic bold used by
  the text emitter. Upstream PDF text rendering supplies architectural evidence,
  not a claim that every upstream mechanical choice is normative.
- A4_DEFAULT_WIDTH/HEIGHT derive from Paper::A4.width_pt()/height_pt() in
  `00_nucleo/prompts/entities/page_geometry.md`, using millimetres × 72 / 25.4.
  Rounded 595.28/841.89 are serialization examples, not production constants.
- A4_FALLBACK_WIDTH_ROUNDED = 595.0 and A4_FALLBACK_HEIGHT_ROUNDED = 842.0
  remain the compatibility values for missing-page metadata/coordinate fallbacks.

## Verification

Check canonical paper derivation, coefficient and fallback preservation.
Serialization rounding does not redefine physical A4 or language parity.
