# L0 — infra/shaper/reconciliation
Hash do Código: 1a264f74

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/reconciliation.rs`

## Current evidence

- `03_infra/src/shaper/reconciliation.rs:28` — `fix_line_positions`.
- `03_infra/src/shaper/reconciliation.rs:98` — `reconcile_rtl_positions`.
- `03_infra/src/shaper/reconciliation.rs:149` — `fix_line_positions_page`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/reconciliation.rs`.

## Owned contract

- Own fix_line_positions and helpers. Adjust only top-level page.items under the existing nonrecursive policy, grouping baselines within 0.5pt and ordering by physical x; preserve y.
- Skip the entire line if it contains math TextShaped or Glyph.
- LTR anchors its first item and accumulates real-minus-estimated advances while preserving layout gaps. Nontext widths remain stable under the existing supported cases.
- RTL captures `gap[i]=x[i+1]-(x[i]+w_est[i])` and `right_edge=x[last]+w_est[last]`; reconstruct `x_real[last]=right_edge-w_real[last]`, then `x_real[i]=x_real[i+1]-gap[i]-w_real[i]`. Preserve normal/justified gaps and the right edge; do not replace them with fixture constants or recompute layout decisions.
- Permit an internal helper to borrow `FallbackFontMetrics` already warmed by the same World and compilation. Keep the existing public `fix_line_positions(world, doc)` signature and its fresh-metrics behavior for all other callers. Width equations, item order and positions remain identical; only the cache lifetime changes.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Approved textual carriers and tracking

Source→PDF RED probes in
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and this
consumer's handler inspection precede separator changes; public layout_types
approval was received on 2026-09-28 with materialization provenance. The
`identidade-hifen-discricionario-2026-09-28/{initial.json,before/}` probes preserve
state and eight U+002D/SHY controls at equal geometry; FrameItem traversal audit
is in approved-before/. Owner confirmed layout_types. Text/bbox/causal/recursive/
literal-hyphen controls, not whole PDF bytes/new break selection, define acceptance.
Pinned nuclei provide shared invariants without ownership sharing.

`tracking-residual-2026-09-29/{initial.json,results.json,summary.json,causal-checks.json}`
retains 36 inputs/108 PDFs against historical vanilla/sources/UTC/HEAD/full tree.
At tracking 0.2pt isolated word accumulates +0.052pt, while boundaries lose one
interval at adjacency, two with one space, four with three literal spaces.
±1.2pt removes quantization, not boundary loss. shaping.rs:225–232,319–326 rounds
i32, text.rs:49–83 splits words, space.rs:15–33 adds nominal advance.
correcao-tracking-2026-09-29/initial.json retains pre-materialization state. Internal
ADR-0127 correction supersedes only earlier preservation of these tracking limits,
not other rendering policy or public/flag/phase contracts.

## Exclusive adjustment contract

Move TextSeparator carriers with shaping deltas, treating advance as already
applied width (estimated==real), never applying again or inventing ink. TextReplacement
x/y and estimated/actual width come from visual children, moved by the same delta
without changing author text. LTR shift uses visual envelope widths exactly as
text leaves, never replacement-string estimate.

Use tracking::width for actual TextShaped leaf widths including TextReplacement
and existing RTL calculation. Replace duplicate LTR formula with
estimated_and_real_width. Separators remain estimated==real since L1 supplied
their intervals; do not add boundaries twice. Require RED→GREEN boundaries/
selection/independent Tc-TJ, ±/zero/repeated/adjacent/wrap bilateral controls,
build/lint zero errors and scoped seals. Existing text/geometry/adjacency behavior
and pinned invariants remain.

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
## Exhaustive existing FrameItem decisions

### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

Only replace fix_line_positions_page's `_` at :210 (historical F069) with
FrameItem::TextReplacement/TextSeparator/Semantic/Text/Line/Image/Shape/Group/Link.
Only replace its `_other` at :224 (F070) with
FrameItem::TextReplacement/TextSeparator/Semantic/Line/Glyph/Image/Shape/Group/Link.
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
