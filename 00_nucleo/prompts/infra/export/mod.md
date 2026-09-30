# L0 — infra/export/mod
Hash do Código: cf09a98c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/mod.rs:196` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Public exporter façade

Own the public PDF option enums, compatibility reexports, and the existing PDF,
PNG, SVG and HTML API surface. Preserve every current signature and parameter
order, including document IDs, timing variants and explicit tags/serialization/
pages/standard variants. Do not restore obsolete signatures without StreamMode.

StreamMode defaults to Verbose; Compact remains the explicit optional graphics
mode under ADR-0126. PdfTags defaults to Enabled. PdfSerializationMode defaults
to Compact; Pretty is an independent physical-serialization option. Conveniences
supply Enabled, Compact, no page selection and PdfStandards::default() (PDF 1.7,
no archival/accessibility profile) when their corresponding axes are omitted.
Pass resolved PageRanges and PdfStandards through without interpreting CLI flags,
feature gates, profile compatibility or upstream tagging-selection policy.

The font-bearing families use FontList/FontVariant/FontVariations identity.
Single-font entrypoints pass bytes to PdfBuilder; multifont entrypoints parse every
face and use the existing Helvetica fallback for an empty collection or any failed
face parse. Timing variants return the existing subset timing; non-timing variants
discard it. Preserve the external 16-byte document-ID option. Standard-specific
emission belongs to the builder descendants, including identification, output intent and tagged structure as well as the header.

The oracle entrypoints explicitly enable the diagnostic path. Keep their existing
single versus multifont routing, including the variable-font/merged-axis condition,
and ordinary defaults for axes not present in their signatures. Ordinary entrypoints
do not enable oracle transformations.

PNG wrappers delegate to raster. SVG and HTML use direct reexports rather than
parallel encoders; HTML remains the separate semantic target defined by ADR-0128.
Keep test-only helper reexports restricted to tests. No document layout, filesystem
routing, output validation or encoding algorithm belongs in this façade.

## Delegation

- `00_nucleo/prompts/infra/export/builder.md`: PDF orchestration.
- `00_nucleo/prompts/infra/export/stream.md`: borrowed PDF emission context.
- `00_nucleo/prompts/infra/export/fonts.md`: font/Unicode helpers.
- `00_nucleo/prompts/infra/export/font_subset.md`: subset adapter.
- `00_nucleo/prompts/infra/export/images.md`: PDF image façade.
- `00_nucleo/prompts/infra/export/gradients/mod.md`: gradient discovery.
- `00_nucleo/prompts/infra/export/render.md`: raster.
- `00_nucleo/prompts/infra/export/svg.md`: SVG.
- `00_nucleo/prompts/infra/export/html.md`: HTML.
- `00_nucleo/prompts/infra/export/oracle.md`: diagnostic transformation.
- `00_nucleo/prompts/infra/export/pdf_defaults.md`: shared constants.
- `00_nucleo/prompts/infra/export/tests/mod.md`: PDF regression aggregation.

## Verification

Check option forwarding and all convenience defaults through the existing public
families. Preserve text, geometry, profile fields, tagging relationships and exact
diagnostic messages through their owners. Complete PDF bytes, object IDs and
allocation algorithms are not language-parity requirements.
