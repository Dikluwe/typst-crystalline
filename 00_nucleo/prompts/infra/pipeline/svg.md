# L0 — infra/pipeline/svg
Hash do Código: 4a73b491

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/svg.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:199640afc0a7243ca509288840d9c9e0c431f850f67919148fc8dac3b1cb33a0
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417
- 00_nucleo/prompts/_nuclei/export/svg-output-files.toml sha256:c113eedd71c8ccb4bedb949e5765dadcc14cc8435a5a5485d6e2ff14eb02a570
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
```

## Current evidence

- `03_infra/src/pipeline/svg.rs:12` — `compile_to_svg_string`.
- `03_infra/src/pipeline/svg.rs:21` — `compile_to_svg_string_full_error`.
- `03_infra/src/pipeline/svg.rs:33` — `compile_to_svg_string_with_timings_full_error`.
- `03_infra/src/pipeline/svg.rs:47` — `compile_to_svg_string_with_timings_full_error_and_features`.
- `03_infra/src/pipeline/svg.rs:59` — `compile_to_svg_string_with_timings_full_error_and_features_and_pretty`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/svg.rs`.

## Owned contract

- Own SVG facades, resolved pretty, selected-page destination context and glyph-font associations. Compile through the common paged path; export the first page with existing empty-page fallback.
- Resolve pretty as explicit caller then document.svg.pretty then its default. Decision-free wrappers pass None and boolean compatibility wrappers pass Some. The exporter receives only final SvgOptions; do not teach it CLI/document precedence.
- Derive destinations only from labels on the selected page with corresponding extracted positions. Do not invent missing positions or move maps into Page; cross-page unresolved links remain Unknown without dangling fragments. Exporter owns ID spelling.
- Associate each normalized GlyphFontRequest with the complete FontKey resolved through FallbackFontMetrics. Distinguish base character, requested family, variant, variations and relevant math state independently of slot/address/traversal occurrence. Conflicting results invalidate an association; missing entries remain Unknown, never default slot zero.
- Collect associations across background/body/foreground and Group/Link/Semantic. Without fonts do not fabricate associations, reread sources or repeat layout. Inline `html.frame` SVG uses the HTML owner's resolved option, not format.svg.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Transparent text identity and export font bindings

### Evidence

`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
and before/ retain exact state/bilateral probes for U+002D versus SHY at matching
geometry across eight controls. approved-before/ contains this consumer's
FrameItem traversal audit; the owner confirmed the carrier at
`00_nucleo/prompts/core/entities/layout_types.md`. Source identity is observable,
not full-PDF byte identity. `identidade-fonte-shaped-2026-10-05/probe.txt` is the
measured basis for original bound export fonts.

### Contract and acceptance

Treat TextReplacement as visually transparent in this owner's traversals, descending
into children with unchanged text/order/coordinates/calculation, no new algorithm
or advance. Consume pinned shared invariants without shared productive ownership.
Require text/bbox/causal/recursive-transport/literal-hyphen controls, no PDF-byte
or new break-selection claim.

Consume private resolve_export_fonts returning bound original fonts under the
existing instancing gate. Instancing stays at the exporter in the same render/
export phase, retaining diagnostics/API/page selection. The old helper name did
not authorize already-instanced bytes.

## Test-module placement

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`
retains sources/L0/hashes/HEAD/uncommitted tree/UTC/diffstat for svg.rs:160.
Inspection preceded decision; selected Clippy must fail before code. Internal
representation/observation proves no new intent/parity. Move only #[cfg(test)]
p1462_tests to the consumer's end, preserving names/bodies/attributes/super import/
APIs and productive helper order, without extraction/new dispatch.
Continuous ADR-0127: no public/default/phase contract or lint suppression.
L0 first/scoped resealing, exact inventory reduction/no new warnings, selected
Clippy RED→GREEN/unfulfilled_lint_expectations as error; workspace suite/build/
fmt/Rustdoc/lineage under 12 GiB, zero swap, one job. No derive/Entry/module-
placement mirror tests. Any historical bbox control still requires actual
observation in its own scope; other controls preserve fixtures/expectations.

## Exhaustive decisions for existing FrameItem variants

### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC,
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked first. classification.json records 70 cases across 26
owners using pinned CI AST. This consumer's historical F066 is svg.rs:197 collect.
Rust pattern/exhaustivity is mechanics; semantics/morphology/geometry/effects/
diagnostics remain. No new architecture/upstream-intent interpretation.

### Exclusive contract and acceptance

Only replace collect's `_` pattern with nominal group
FrameItem::TextSeparator/Text/TextShaped/Line/Image/Shape using the existing Rust
prefix. Preserve fallback body/prior arms/guards/transforms/calls/returns/order
verbatim. Keep callbacks/effects/metadata/clipping/fonts/positions/ink/width under
their owners. No traversal expansion, wrapper→leaf reinterpretation or text/
ActualText replacement for geometry. Test fixtures/expectations/tolerances/counts/
Unknown failures remain; no missing-observation acceptance. Nominal alternatives
share the prior body; V19 alternative notes stay visible without pretending one
arm's coverage proves every alternative.

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

## Ratified complete SVG output collection (2026-10-22)

### Evidence and ratification

`lab/results/remaining-parity-2026-10-22/svg-outputs-before/receipt.json.gz`
records exact UTC, HEAD/uncommitted diffstat, fixed executable/build/source archive,
font bytes and actual CLI results for the unchanged two-page finite source and
a separately pinned12-page source. `qualification.json` preserves strict1e-8 SVG
geometry, glyph/paint and complete destination-name comparisons: all10cases RED,
including selected page2 whose filename alone had concealed first-page content.
The human approved the exact additive proposal SHA-256
`d2a1ddfd71d9f741253b00b20c23084aae474e4c9cb979d70f3daa37b40d943d`;
`lab/results/remaining-parity-2026-10-22/svg-pages-approval.json` records the gate.
This is the existing paged compile/export phase, without source reconstruction,
new flag, second evaluation or PNG/bundle behavior change.

Add exactly public `compile_to_svg_output_files(world: &dyn World, source: &Source,
full_error: bool, features: Features, pretty_override: Option<bool>,
pages_override: Option<PageRanges>, output: &Path)` returning
`(Result<Vec<(PathBuf, Vec<u8>)>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings)`.
The bytes are UTF-8 SVG; this facade writes neither files nor stdout. Compile once
through the existing common paged pipeline and bind actual fonts once. Export
every selected actual page with its own destination and glyph-font contexts.
Explicit typed page override selects physical indices in ascending order; None
selects all. Preserve pretty override/document/default precedence and original
warning order. Old first-page APIs stay compatible and independent.

A path containing {p}, {0p} or {n} is indexable: replace every {p} by the original
physical index, {0p}/{n} by that index padded to total document decimal width, and
{t} by original total count. A lone {t} is not indexable and remains literal.
Preserve ordinary non-UTF-8 single paths. Multiple selected pages to '-' fail
with `cannot export multiple images to stdout`; multiple pages without an
indexable path fail with `cannot export multiple images without a page number
template ({p}, {0p}) in the output path`. Errors have detached operational spans
and produce no partial collection. An empty selection returns an empty vector.

The complete output collection must satisfy the measured RED gates before
any candidate certification. A legacy first-page adapter cannot satisfy this
facade's complete-output contract.
Tests must cover actual two-page output/selected page2, all template forms and
padding/repetition, plain/stdout rejection, empty selection, pretty and legacy
compatibility. The unchanged CLI full-page comparison remains required.


### Private naming of the unchanged output result

Measured before this readability correction: `lab/results/remaining-parity-2026-10-22/preflight-clippy-final/receipt.json.gz`
(2026-10-22T14:37:23.430220+00:00 through 2026-10-22T14:37:43.562476+00:00, HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`,
exact dirty-file stat and inputs in that receipt) records Clippy
`type_complexity` on the nested output return type. Its complete log SHA-256
is `aa2fbd49b9e4eb8bc46cf5d8ae7c9dc19b2ee273ccc71bb91a40561acfd8ce0e`. The public API and its previously measured behavior
are not the defect.

Use a module-private `SvgOutputFilesResult` alias for exactly
`Result<Vec<(PathBuf, Vec<u8>)>, Vec<SourceDiagnostic>>` in the return tuple.
This is a transparent alias, not a new public type, opaque wrapper or changed
function contract. Preserve the exact expanded public type specified above,
all callers, output selection, diagnostics and ownership. Do not add a lint
allow/expect or change the Clippy policy.
