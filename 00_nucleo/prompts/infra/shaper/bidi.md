# L0 — infra/shaper/bidi
Hash do Código: 79a1edf7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/bidi.rs`

## Current evidence

- `03_infra/src/shaper/bidi.rs:11` — `BidiRun`.
- `03_infra/src/shaper/bidi.rs:30` — `bidi_runs`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/bidi.rs`.

## Owned contract

- Own BidiRun and bidi_runs only: logical paragraph order, visual run order, original substrings, direction and absolute byte_start.
- Use unicode-bidi with automatic base direction when unresolved, or an already-resolved horizontal base when supplied, and visual_runs for every paragraph. Retain empty paragraphs and omit trailing class-B separators from shaping ranges.
- Empty input yields no runs. UTF-8 clusters refer to the original string. Font selection, run shaping and position adjustment belong to other owners; no vertical-text policy is added.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

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

## Already-resolved horizontal direction — #8990 residual (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8990-residuals-2026-10-31/`
before-state.json and before receipts bind HEAD 3c6b0ce7, concurrent dirty inputs,
UTC, pinned oracle and RAM binary. Explicit text(dir:ltr)[אבג.] and
text(dir:rtl)[p.v.] inside math.op are accepted yet paint the unqualified
first-strong order. Candidate-1-state.json, green-first.log and after receipts
prove that merely forwarding the property into TextStyle does not correct
visible order. shaping.rs:189,326 calls bidi_runs with no direction even when
TextStyle.dir is already Some. Existing font_metrics/caches.rs width keys
already distinguish dir. Pinned text/mod.rs:517-543 documents explicit base
direction; this is observable bidi semantics/geometry, not a requirement to
copy upstream IR, font IDs or byte output.

### Exclusive correction and boundaries
Own a sibling-private optional-direction helper returning the same BidiRun
shape. Retain bidi_runs(text) as the unchanged automatic-base wrapper. Map only
resolved horizontal LTR/RTL to Unicode levels; absent/vertical input keeps
the existing automatic path. Preserve every original substring/byte_start,
paragraph boundary, visual run order, empty behavior and class-B trimming.
Do not derive direction from language, inspect fonts, inject author controls,
add public contracts or change any product default or pipeline phase.
Inline tests must demonstrate that resolved LTR retains the Hebrew final
point on the LTR side and resolved RTL moves the Latin final point to its RTL
side, with None and the existing wrapper unchanged.

This is internal completion of an already declared direction, not authorization
for the pending automatic-language or unqualified-math default changes.
L0 precedes code, followed by RED→GREEN and bilateral observable checks,
RAM-only builds/tests and paired lineage with a valid V15/V26 graph.
