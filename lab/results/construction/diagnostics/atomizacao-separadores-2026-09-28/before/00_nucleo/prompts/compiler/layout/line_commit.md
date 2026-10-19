# L0 — line commit
Hash do Código: b99c576e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/line_commit.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/line_commit.rs:20`, `01_core/src/compiler/layout/line_commit.rs:66`, `01_core/src/compiler/layout/line_commit.rs:189`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

The initial-block no-typography regressions reach the empty-line flush in
`01_core/src/compiler/layout/line_commit.rs:358`: text edges are requested even
when material-line classification is false. The intermediate failure is preserved
in `00_nucleo/diagnosticos/font-demand-geometry-study/green.stdout`.
The empty flush still owns draining/resetting state; eliminating an unused font
query does not authorize skipping that protocol.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own expansion of horizontal fractions/repeats, justification and physical line closure, including baseline/leading, causal decorations and line numbering. The hub retains buffers and checkpoints. Delegate region progress to `00_nucleo/prompts/compiler/layout/cursor.md`, repeat multiplication to `00_nucleo/prompts/compiler/layout/repeat.md`, and callback matching/final marginal-width adjustment to `00_nucleo/prompts/compiler/layout/par_line_callbacks.md`. Do not move page assembly or Sequence preflight here.

For a finite line, sum weights without requiring positive individual weights or a positive sum. Add provisional natural fractional-box widths back to distributable remaining space. A share is `max(weight / total * remaining, 0)` only when the ratio and remaining are finite; otherwise zero. Replace provisional occupancy by share-minus-natural-width; h(fr) has natural width zero. Associate a repeat with its actual boundary, shift its suffix once, insert copies before that suffix and repair later indices by the actual insertion. Infinite width/orphan state clears pending work defensively without looping.

Justification expands fractions first, then distributes only over opportunities before material content ends, preserving the present RTL behavior and link/semantic bounds. No opportunity is invented after the last material item.

Consume the validated-prefix one-shot arm at the first ordinary flush; explicit certified closure clears the arm and uses the same closing path. Empty/direct-marker-only lines drain carriers but add no typography or page. Material lines combine actual text and inline extents. Distinct text sizes, or multiple text items differing from the active outer style, select outer next-ascent/leading while retaining the current maximum/ascent/descent. Correct a positive difference from the assumed ascent by translating the entire line and collectors together.

A certified larger singleton uses the outer next-ascent/leading; a certified smaller singleton keeps its own baseline/ascent/next-ascent and only substitutes outer leading. Ordinary singleton behavior is unchanged. When float reservation makes ink exceed the effective region, reject the hub transaction or request ordinary Cursor transition; never migrate only current_line. A terminal fractional line whose real ink fits does not create an empty page merely because a hypothetical next baseline does not.

Number material non-subframe lines only after stabilizing baseline and causal page. Preserve document/page scope, none/absent neutrality, pattern formatting and Func request/realized-Content consumption without Func execution. Missing resolution records a request rather than inserting a provisional number. The last physical column uses the end margin; earlier columns retain the configured margin. Keep region-local greatest-width fixups, clearance, horizontal alignment and the existing overlay suppression/edge compensation. Under a segmented pattern context, buffer baseline/page/pattern/chain candidates without immediately incrementing or formatting; callbacks stay on their existing request path. Columns owns consolidation and shared-style capture.

Drain the line in order into emitted items, clear justification opportunities, record causal baseline/advance/descent, reset cursor_x to line start and request any remaining necessary transition. Preserve typed style, visual position, content order and diagnostic behavior; Rust buffer shape and exported PDF bytes are not language-parity gates.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

When the expanded pending line has no material items, use neutral local text-edge
values without consulting FontMetrics. Continue normal drain/reset and marker
transport. Material lines retain current and next style-edge measurements and
baseline/leading/overflow behavior. Initial-block tests in the image/shape owners
exercise this shared path with an injected metrics provider that rejects font queries.
