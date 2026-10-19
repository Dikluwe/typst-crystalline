# L0 — infra/export/mod
Hash do Código: c092762c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

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
- `00_nucleo/prompts/infra/export/subset.md`: subset adapter.
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

## Private composition and native tiling

export/mod.rs:45, 55, 56, 57, 61, 63, 64, 65, 70, 71, 74, 75, 76 and `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz` preserve source/owner/spans/original compiler output/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted status/diffstat/UTC. Distinguish productive/test/comment use. Keep six gradient helpers and process_png_for_pdf under cfg(test) for descendant super use; remove unused bitmap/stream imports and RectKey/PdfImagePayload facade types, never definitions/child reexports/builder-qualified paths. No public signature/default/phase/formula/acceptance/suppression change. Validate all targets/infra test names/execution/build/workspace Clippy/Rustdoc/lineage, scoped resealing; no import/comment mirror tests.

conic-posicionamento-2026-10-14/first-validation/followup-clippy-inventory.json records dedup_key_for unused at mod.rs:54 after stream's last alias changed to occurrence lookup; snapshot-before.json.gz preserves source/L0. Remove only this private import; preserve definition/API/required aliases, all-target/lineage validation without suppression.

conic.rs:142–143,241–242 fixes center/radius; :271–272 pairs CMYK stops by circular index, ignoring offsets/final sample; resources.rs:508–552 fixes eight-bit coordinates/unit Decode. conic-posicionamento-2026-10-14 before/after/refuters prove white corners, ignored center and ineffective CMYK offsets although sample() responds correctly. conic-geometria-amostragem-2026-10-14/before.json.gz preserves sources/L0/HEAD/tree/diffstat/UTC/binaries. Bilateral corpus covers center/axis-diagonal colors, square/rectangle/rotation. Pinned typst-pdf/src/paint.rs:274–299 and pixels agree left-origin/clockwise; old local code differs and observed vanilla is no uniform fallback. PDF Reference1.3 table4.31(PDFp250) permits16-bit coordinates and independent-axis Decode, a format contract rather than upstream equality.

The conic geometry/sampling contract is governed by `00_nucleo/prompts/infra/export/gradients/conic.md`. Its source/raster/mesh probes precede this facade's decision. Restrict this facade to private imports/reexports: productive mesh emitter/domain, historical wrappers/helpers cfg(test). Preserve public PDF/PNG/SVG/HTML/defaults, no new algorithm. Formula/parity continuous flow requires L0 before RED/code, unchanged functional expectations; obsolete mechanical assertions use the independent verifier. Full corpus/qpdf/raster/regression/strict reverse lineage; regenerate a conic reference binary only after harness proof and the corpus protocol, never to absorb a failure.

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

Declare private tiling for shared prepared-paint geometry and private pdf_tiling for PatternType1 emission/registration under its canonical owner. No public export signature/default/serialization/tag/page-selection change.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Exact public compatibility exceptions

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` retains checked source/prior L0/signatures/hashes/HEAD/uncommitted status/diffstat/UTC2026-10-15T16:39:18.873477+00:00. Four public eight-input facades exceed threshold7: mod.rs:446 single-font (C074), :555 single-font timing(C075), :672 multifont(C082), :810 multifont timing(C089). They preserve independent tags/serialization/pages/standard, full variant/axis identity and existing subset timing. No argument reordering, collapsing serialization with StreamMode or tags with standards, new option resolution/algorithm; future body deduplication is a separate audit.

clippy-locais-2026-10-16/clippy-inventory.json records 79 warnings; `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz` preserves sources/spans/reasons/signatures/hashes/HEAD/tree/UTC/diffstat. Its follow-up materializes only these direct nominal expectations, superseding the old audit-only pending state. Keep executable signature/body/calls/cfg/order/other attributes/semantics; attach only the exact corresponding literal to its named function:

- `#[expect(clippy::too_many_arguments, reason="C074: Fachada pública single-font com oito eixos já publicados, incluindo tags, serialização, pages e standard independentes")]`
- `#[expect(clippy::too_many_arguments, reason="C075: Variante pública single-font com tempos conserva os mesmos oito inputs e devolve subset timing")]`
- `#[expect(clippy::too_many_arguments, reason="C082: Fachada pública multifont preserva a coleção completa com variantes/eixos e opções PDF explícitas")]`
- `#[expect(clippy::too_many_arguments, reason="C089: Variante pública multifont com tempos preserva os mesmos eixos e o retorno de timing")]`
- `#[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")]`

No module/impl/crate/global annotation, other lint or threshold increase. If no longer over threshold remove/review expectation, never silence unfulfilled_lint_expectations. This narrowly supersedes generic suppression bans only for named item/lint. Correction cases, including conditional C047/C066, gain no annotation and require their own repair.



Continuous ADR-0127 changes only lint metadata, no public contract/executable body/
cfg/default/phase/dependency/architecture. L0 first/scoped resealing. Removing added
attributes and lineage header must recover prior rustfmt-normalized source.
Measure real RED, then all-target inventory: historical batch exactly 46 arities
plus five test warnings satisfied by 49 expectations, no new warnings and
unfulfilled_lint_expectations as error; its 11 correction arities remain visible.
Keep owner functional/call controls without fixture/tolerance relaxation; private
signature corrections migrate callers/L0 before code and new public/compatibility
needs another ADR-0127 gate. Validate build/fmt/strict/reverse lineage/Tekt no new
errors/warnings, existing Counter/coverage tests under 12 GiB, zero swap, one job.
Prior full suite is evidence for identical executable code, not a fresh run;
no lint-attribute mirror tests or new performance comparison for metadata.

## Transparent aliases and preexisting defaults

`lab/results/construction/diagnostics/type-complexity-2026-10-15/{before.json.gz,red-clippy.log}` preserves source/L0/hashes/HEAD/tree/UTC/diffstat and RED at mod.rs:589,598,613,630,651,674,727,736,751,768,789,812,868,885,991 (C076–C081,C083–C088,C090–C092). Use the existing reexported public FontKey in all15 repeated multifont/PNG signatures. Its RHS remains ((FontList, FontVariant, FontVariations), Vec<u8>), with same visibility/reexports; expanded public types/parameter order/lifetimes/defaults/returns must be identical. tests/mod.rs:31–32 imports FontVariant/FontVariations itself, validation-initial/validation-clippy.log identifies redundant parent imports: remove those two, keep FontList cfg(test) for descendants. Record expanded before/after signatures and existing-caller compilation. No new entity/public type/incompatibility; otherwise another gate. Nominal type_complexity exceptions cover only the three justified batch items, no module/impl/crate suppression/raised threshold/other repair suppression.

mod.rs:201,224 and `lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz` preserve consumer audit/source/L0/hashes/HEAD/tree/UTC/diffstat. Derive existing public Default impls with #[default] on StreamMode::Verbose and PdfTags::Enabled, preserving every other derive/variant/value and PdfSerializationMode. ADR-0126 product default does not change. Continuous flow, L0 first/scoped lineage, no new public/default/phase/suppression, selected Clippy RED→GREEN/denied expectations/exact inventory/no new warnings, unchanged suite fixtures/expectations, workspace/build/fmt/Rustdoc/lineage under12GiB/swap0/jobs1. No tests mirroring derive/Entry/module placement; bbox requires real observation. No new upstream-intent/parity/AI benefit from Rust representation.

## Owned test module declarations

This facade declares separately owned line_cap_tests and delegates its test bodies; no ownership absorption.
