# L0 — infra/shaper
Hash do Código: 1d7e398c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper.rs`

## Current evidence

- `03_infra/src/shaper.rs:1` — lineage and facade module declarations/reexports.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper.rs`.

## Owned contract

- Own only private module declarations and facade reexports. Preserve public shape_document/fix_line_positions and the existing crate-private FaceCache/CachedFace/shaped_width import paths.
- A crate-private reexport may expose reconciliation with borrowed metrics to the paged pipeline, while the public entry point and pipeline phase stay unchanged. `00_nucleo/prompts/infra/shaper/reconciliation.md` owns that implementation.
- The paged pipeline owner places the pass: `00_nucleo/prompts/infra/pipeline/paged.md`. Text traversal and fallback are governed by `00_nucleo/prompts/infra/shaper/traversal.md`; glyph materialization is governed by `00_nucleo/prompts/infra/shaper/shaping.md`. This facade does not repeat those contracts.
- Traversal, bidi, font selection, shaping, caching and reconciliation are owned independently below. Keep static delegation and existing visibility; do not add a registry, dynamic dispatcher, public contract or L1 font-byte access.

## Child owners

- `00_nucleo/prompts/infra/shaper/bidi.md` owns only `03_infra/src/shaper/bidi.rs`.
- `00_nucleo/prompts/infra/shaper/cache.md` owns only `03_infra/src/shaper/cache.rs`.
- `00_nucleo/prompts/infra/shaper/font_selection.md` owns only `03_infra/src/shaper/font_selection.rs`.
- `00_nucleo/prompts/infra/shaper/reconciliation.md` owns only `03_infra/src/shaper/reconciliation.rs`.
- `00_nucleo/prompts/infra/shaper/shaping.md` owns only `03_infra/src/shaper/shaping.rs`.
- `00_nucleo/prompts/infra/shaper/tests.md` owns only `03_infra/src/shaper/tests.rs`.
- `00_nucleo/prompts/infra/shaper/traversal.md` owns only `03_infra/src/shaper/traversal.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Author SHY and tracking boundaries

### Evidence

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/{initial.json,before/resultados.json}`
retains state/binaries/commands and 30 bilateral probes. At 300pt,
repre<U+00AD>sentation extracts two SHYs in Tekt versus one in pinned vanilla;
spaces control extracts six versus one. shaping.rs:313–325 carries rustybuzz SHY;
export/fonts.rs:318–328 maps its substring to a space GID; builder/fonts.rs:523–535
keeps one identity per GID. Prior stream associates CID 0001 with 00AD then reuses
it for later space. This is observable text, not byte equality. Missing that
association or pre-shaper duplication would refute the causal hypothesis; no
historical intent inferred.

`tracking-residual-2026-09-29/{initial.json,results.json,summary.json,causal-checks.json}`
retains 36 inputs/108 PDFs, pinned vanilla/sources/UTC/HEAD/full working tree.
Isolated word accumulates +0.052pt at 0.2pt tracking; boundaries lose one interval
at adjacency, two with a space, four with three literal spaces. ±1.2pt removes
quantization but not boundary loss. shaping.rs:225–232,319–326 rounds to i32;
text.rs:49–83 splits words; space.rs:15–33 uses nominal advance only.
correcao-tracking-2026-09-29/initial.json retains pre-materialization state.

### Facade contract and acceptance

Reuse approved TextReplacement for author U+00AD identity, not appearance/GID/
position. Internal continuous ADR-0127 adds no public field/signature/flag/phase,
break-selection or ink change. Narrow-width break-selection differences remain
outside SHY policy closure. Declare only private text_identity child, owned by
`00_nucleo/prompts/infra/shaper/text_identity.md`; no identity policy/reexport change
in this facade.

Declare crate-private tracking child for exporter use without public reexport,
owned by `00_nucleo/prompts/infra/shaper/tracking.md`; no formula duplication.
Internal parity correction supersedes only older tracking-limit preservation,
leaving other render policies. Require RED→GREEN boundary/selection and independent
Tc/TJ interpretation, bilateral ±/zero/repeated/adjacent/wrap tracking controls,
build/lint zero errors and scoped resealing. No public/default/phase changes.

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
