# L0 — infra/pipeline/pdf
Hash do Código: 0f32a119

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/pdf.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
```

## Current evidence

- `03_infra/src/pipeline/pdf.rs:180` — `resolve_pdf_pages_and_tags`.
- `03_infra/src/pipeline/pdf.rs:243` — `resolve_pdf_pages_tags_and_standards`.
- `03_infra/src/pipeline/pdf.rs:288` — `require_ua1_document_title`.
- `03_infra/src/pipeline/pdf.rs:308` — `require_ua1_image_alt`.
- `03_infra/src/pipeline/pdf.rs:351` — `compile_to_pdf_bytes_impl`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/pdf.rs`.

## Owned contract

- Own PDF compilation facades, resolved document options, PDF-only validations and font-aware export dispatch. Obtain a complete paged document from its owner; preserve raw warnings on both success and failure.
- Preserve every compatibility wrapper's explicit/default choices, DocumentID transport, oracle path, timings and public import paths. Some DocumentID fixes the document id, not the instance id; None leaves existing random behavior. Empty executable-only content preserves the existing empty-output path.
- Resolve pages from caller then document then all pages; resolve standards from caller then document then PdfStandards default. Resolve pretty from explicit caller then document. StreamMode (verbose production default versus explicit compact), serialization, tagging, features and DocumentID are independent axes.
- Without page selection, explicit tagged decisions win; documentary Auto enables tagging. With pages, effective Explicit(true) fails with `cannot enable tagged PDF and export a page range`; Auto disables tagging and emits `exporting a page range disables PDF tagging` with hints `the resulting PDF will be inaccessible` and ``set `pdf(tagged: false)` to silence this warning``; explicit false stays silent.
- CLI-only rejections and the separate CLI pages warning belong to `00_nucleo/prompts/wiring/compile.md`, not this module. Do not collapse caller absence early or duplicate those messages.
- After pages/tagging resolution, an accessible archival profile (A1a/A2a/A3a) or Ua1 with Disabled tags fails with ``cannot set `pdf(tagged: false)` when exporting a {name} document``. Add `using `pdf(pages: ...)` implies `pdf(tagged: false)` only when pages caused disabling and false was not explicitly selected.
- Pass resolved pages/tags/standards/serialization unchanged to all font-aware exporter branches. Standard-name/version resolution is not proof of full PDF/A or PDF/UA structural conformity; do not add a conformity engine or version-specific internal rewrites.
- Before export validate attachments in final order: duplicate derived virtual paths fail with `attempted to attach file {path} twice`. Direct infallible exporters require a validated document; they do not replace this language error channel.
- Only under Ua1, absent document title fails with `PDF/UA-1 error: missing document title` and hint ``set the title with `set document(title: [...])```. This checks None versus Some, not a new empty-title policy or combined-validator prefix.
- Only under Ua1, real images without alt outside `pdf.artifact` fail with `PDF/UA-1 error: missing alt text` and hint `make sure your images and equations have alt text`. Some empty text is still supplied; keep available origin spans and existing detached-span limitation.
- Resolve/instantiate ordered complete font keys through the resource owner, passing the World without requesting its catalog in advance. Dispatch no resolved fonts to fallback export, one to single-font export and many to multifont export without dropping explicit variations. Preserve ordinary and oracle behavior, not total PDF-byte identity as a language criterion.

The resource owner's private matching-axis predicate governs the early instancer dependency check. Unknown-axis-only requests must not produce a missing-tool diagnostic; retain dependency errors for actual font axes and preserve export dispatch/options.

When a real-axis request lacks the instancer dependency, delegate its gate/diagnostic to the private helper governed by `00_nucleo/prompts/infra/pipeline/resources.md`, preserving message/span/precedence. That owner records causal evidence; zero/one/many-font dispatch remains unchanged.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Text identity and shared PDF pre-export validation

`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` records eight U+002D/SHY equal-geometry controls and approved-before traversal audit; layout_types owner confirmed TextReplacement. Treat it as transparent visual children preserving text/order/coordinates/calculation, no new algorithm/advance; pinned shared claims confer no shared ownership. Require text/boxes/causal/recursive/literal-hyphen controls, not PDF bytes/new selection.

`lab/results/construction/diagnostics/auditoria-composicao-construcao-2026-09-29/probe.json` measures same body with format.pdf(standard:"ua-1"): standalone fails `PDF/UA-1 error: missing document title`, bundle emits pdfuaid:part=1 without title. pdf.rs:404–426 validates title/alt/duplicate attachments; bundle.rs:463–513 omits them. The lost-composition-invariant hypothesis is refuted if same body/effective standard differs between routes; XMP confirms effective standard, not full UA conformance/byte parity.

Own one pipeline-private helper for existing paged validations in order: UA1 title, UA1 alt, then duplicate attachment paths in original order, with exact messages/hints/spans/conditions/precedence. Both routes call it after standards resolution before fonts/export. Bundle delegation does not transfer PDF ownership. No new public API/mode/phase/conformity rule. Require both routes' missing-title diagnostic, supplied-title success, reached missing-alt/duplicate diagnostics, optional non-UA title; internal parity continuous flow with RED→GREEN/revalidation.

## Plural font-buffer transfer

pdf.rs:436–437 resolves an owned font table then temporarily borrows it for bind_document; font_binding.rs:127–148 returns Cow::Borrowed if no face must be appended, and into_owned clones the table/each Vec<u8>. Binding semantics remain; copying is mechanics. font-primary-reuse-2026-10-10 records variant-memory provenance; attributing peak to copies is an RSS/allocation-refutable inference.

pdf-font-buffer-reuse-2026-10-10/all-tables-measurements.json.gz (original measurements.json.gz) records release0ef5e759, local RSS41122→42356KiB(+2.84% paired) while saving only337547 requested bytes/three allocations; 800 variants save240.6MiB requested/RSS525072→292954KiB/time15.16%, complete state/UTC in receipt. Single-font amplification remains unattributed; fragmentation is a hypothesis, not solely reverted style capture.

Keep old binding+into_owned for empty/single tables. For supplied.len()>1 keep the owned table alive during binding: Borrowed transfers the original complete table without cloning; Owned uses binder's complete result and drops original. This threshold matches existing single/plural dispatch, never fixture/text/family/weight/size. A private helper may expose transfer to tests. Preserve all bytes/order/keys/appended faces/fallback/validation/ordinary-oracle dispatch, no API/phase/instancing/font/diagnostic change. Require buffer-transfer/missing-face RED→GREEN, unchanged language/geometry/raster, local single RSS control/unchanged single allocations/plural savings, separate requested-byte/allocation/RSS measurements and final-release output comparison. No pointer language identity or unmeasured RSS promise; continuous flow.

## Public compatibility wrappers and private request

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` preserves checked source/prior L0/signatures/hashes/HEAD/tree/status/diffstat/UTC2026-10-15T16:39:18.873477+00:00. pdf.rs:121(C101,8), :376(C102,12), :548(C103,9), :578(C104,10), :614(C105,8), :641(C106,9), :669(C107,10), :702(C108,8), :733(C109,9), :760(C110,10), :793(C111,8), :878(C112,8), :945(C113,8), threshold7. Arity measures Rust structure, not a functional defect/new parity.

Keep every public signature/parameter order/return and None/Some distinction. Explicit tags/serialization become Some(C101/C112/C113). Timing pages wrapper passes standard None(C103); complete ordinary timing passes oracle=false(C104); timing pretty-only pages None(C105). Nontiming pages standard None(C106), complete ordinary local Timings/oracle=false(C107), pretty-only pages None(C108). Oracle pages standard None(C109), complete oracle local Timings/oracle=true(C110), oracle pretty-only pages None(C111). Never resolve absent standards/options before documentary precedence or broaden oracle emission. Preserve raw errors/warnings/timings and zero/one/many font routes.

clippy-locais-2026-10-16/clippy-inventory.json records 79 warnings; clippy-excecoes-2026-10-16/before.json.gz preserves sources/spans/reasons/signatures/hashes/HEAD/tree/UTC/diffstat. Only direct C101, C103–C113 expectations are materialized, superseding earlier audit-only pending status; C102 requires the private repair below and gains no direct annotation. Keep signature/body/calls/cfg/order/other attributes/semantics; exact literals:

- `#[expect(clippy::too_many_arguments, reason="C101: Wrapper público converte PdfTags/serialization explícitos em overrides Some, preservando features, ID, modo e retorno de tempos")]`
- `#[expect(clippy::too_many_arguments, reason="C103: Wrapper público com tempos conserva pages e fornece standard None ao mais completo")]`
- `#[expect(clippy::too_many_arguments, reason="C104: Wrapper público completo com tempos fornece as opções ordinárias e oracle=false ao núcleo")]`
- `#[expect(clippy::too_many_arguments, reason="C105: Wrapper público com tempos e pretty sem pages delega pages None")]`
- `#[expect(clippy::too_many_arguments, reason="C106: Wrapper público sem tempos com pages delega standard None")]`
- `#[expect(clippy::too_many_arguments, reason="C107: Wrapper público ordinário completo sem tempos cria Timings local e passa oracle=false")]`
- `#[expect(clippy::too_many_arguments, reason="C108: Wrapper público sem tempos e sem pages fornece ausência de seleção")]`
- `#[expect(clippy::too_many_arguments, reason="C109: Wrapper público oracle com pages fornece standard None; a rota diagnóstica é independente da comum")]`
- `#[expect(clippy::too_many_arguments, reason="C110: Wrapper público oracle completo fornece oracle=true e Timings local")]`
- `#[expect(clippy::too_many_arguments, reason="C111: Wrapper público oracle sem pages fornece pages None")]`
- `#[expect(clippy::too_many_arguments, reason="C112: Wrapper público sem tempos transforma tags/serialization explícitos em Some para o núcleo com precedência documental")]`
- `#[expect(clippy::too_many_arguments, reason="C113: Wrapper público oracle com serialization conserva as conversões explícitas e a seleção da rota diagnóstica, sem antecipar resolução documental")]`
- `#[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")]`

Apply only the named function/lint;  no module/impl/crate/global application, threshold
increase or other lint. This narrowly supersedes generic suppression bans only
for named item/lint. If no longer over threshold remove/review the expectation;
never silence unfulfilled_lint_expectations. Conditional exceptions requiring
dead-argument removal still require that correction; pending correction cases
(including historical C047/C066) gain no annotation here.



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

`lab/results/construction/diagnostics/clippy-aridade-interna-2026-10-16/before.json.gz` records audited source/callers/contracts/spans/28-warning inventory/HEAD/tree/UTC/diffstat. Materialize C102 as strictly private PdfCompileRequest, no Default derive, with unchanged types for full_error/document_id/stream_mode/pdf_tagged_override/features/pretty_override/pages_override/standard_override/oracle. compile_to_pdf_bytes_impl takes World, Source, request, &mut Timings; destructure once at entry, retain the rest of the body literally. Three direct facades build named fields from identical values, pure-expression order, oracle false/false/true, and None/Some until document resolution. No public facade change.

This six-case internal slice permits conditional residual C047/C066 expectations only after their _text removal; five other correction arities remain visible. No new external API/default/phase/dependency/semantics/global mutation. L0/private callers before code, scoped lineage. Preserve fields/values/PNG-SVG-PDF/precedence/diagnostics/resources/MCIDs, existing fixtures/expectations/tolerances; no carrier-mirror tests. Real selected RED→GREEN/denied expectations, workspace/build/all-target Clippy/Rustdoc/fmt/strict reverse lineage, 12GiB/swap0/jobs1.

## Exhaustive existing FrameItem decisions


### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

Only replace missing's `_` at pdf.rs:330 (historical F065) with FrameItem::TextSeparator/Text/TextShaped/Line/Glyph/Shape. Restricted/guarded earlier arms do not cover their whole variant: retain failing-condition cases in fallback, specifically Semantic kind typst_core::entities::layout_types::SemanticKind::Artifact(_).
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

## PDF export warning integration

Destructure every approved `PdfExport` result at the existing render junction,
append its diagnostics to the pipeline warning vector in export occurrence order,
and preserve the public pipeline tuple signatures. Malformed images remain
errors at the existing validation gate. Preserve PDF bytes, subset/render timing,
all zero/one/many-font and oracle routes, resolved options and diagnostic content.
L4 remains the publication owner; this pipeline performs no direct stderr write.
