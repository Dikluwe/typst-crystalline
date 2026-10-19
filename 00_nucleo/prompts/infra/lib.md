# L0 — Infrastructure crate root
Hash do Código: 9b5648a3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L3
**Exclusive consumer:** `03_infra/src/lib.rs`
**ADRs:** ADR-0129 and current crystalline topology

## Evidence and responsibility

`03_infra/src/lib.rs` declares the static crate topology, including private
bidi_context and test-only integration_tests/pdf_snapshot_tests. The original
construction/bidi clauses remain archived at
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/lib.md`.
Only this crate root is owned here.

## Contract and structural acceptance

Expose the existing L3 modules:

```text
embedded_fonts, export, fallback_fonts, font_metrics, font_variant, fontdb,
fonts, image_sizer, layout, layout_bidi, measurements, package_downloader,
pipeline, plugin_host, project_init, query_helpers, runtime_info, shaper,
watch, world
```

Keep bidi_context private. Include integration_tests and pdf_snapshot_tests as
private modules only under #[cfg(test)], each independently owned. L3 materializes
I/O and integrates L1 contracts; the root declares topology without implementing
child behavior or reexporting their items. User-facing diagnostics belong to L2
under ADR-0050. Require exactly the productive source modules, test-only suites
only in test builds and no business/formatting logic in the hub.

Pipeline/export/fonts/world and other child behavior belong to their own owners;
E2E tests belong to `00_nucleo/prompts/infra/integration_tests.md`, PDF snapshots
to `00_nucleo/prompts/infra/pdf_snapshot_tests.md`. Fixtures/individual tests/future
API decisions are outside this owner; any future pub mod change requires fresh
ADR-0127 classification.

## Full-paragraph bidi context

### Evidence

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{initial,probe}.json.gz`
retains bilateral cases/Unicode refutation and sources/state/binaries.
`layout_bidi.rs:73–82,227–240` redetects base/invents separators;
`shaper/bidi.rs:34` redetects a fragment, losing the human-ratified complete input.
This internal parity correction adds no phase or external public signature.

### Boundary contract and acceptance

For certified spans, use the compilation-local registry of auxiliary strings/maps
by input Arc, then normally borrowed BidiInfo views; no unsafe/self-reference.
Virtual LRE/RLE/PDF is inserted only when segment direction differs from base.
Retain real spaces, nonjustified Linebreak LF and author controls; justify=true
is outside certification. Resolve once per input per registry; paged pipeline
shares registry/views between reorder/shape. Existing public wrappers may create
a local equivalent registry.

Reorder queries the actual line interval, exact segment/range intersections and
end-of-line levels, never midpoints or invented spaces. Split mixed Text at original
boundaries preserving style/subspan. Nontext items and unknown sources gain no
certificate. Preserve the L1-accepted physical anchor and actual width/advance;
certified context cannot use suffix/coalescing fallback.

Shaping queries full-input runs for each original span without fragment direction
redetection, preserving sequential subrun offsets and exact TextShaped subspans.
The existing cache key already distinguishes direction; add no context to
TextStyle. Reconciliation uses input base when present, not any RTL run; retain
prior None fallback. Require RED→GREEN for `456 789` refutation, overrides/neutrals,
no auxiliary controls in extracted text and bilateral measurements. Registry
implementation is exclusively owned by `00_nucleo/prompts/infra/bidi_context.md`;
reordering and shaping remain with their own canonical owners.
