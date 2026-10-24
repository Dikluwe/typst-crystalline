# L0 — title
Hash do Código: 02fd00e9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/title.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/title.rs:18`, `01_core/src/compiler/layout/title.rs:22`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Temporarily install the current title TextStyle: bold, non-italic, size `1.7 * outer_size`, with the remaining fields initialized by TextStyle defaults. Close preceding horizontally pending text, dispatch the title body and close its line, then restore the outer style. Do not silently change default-field initialization to full style inheritance.

Isolate body as an inline hanging fragment after existing prior closure; restore context after flush. Preserve original TextStyle initialization and scale.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Route this feature's actual source children through closed typed plan edges `TitleBody`, retaining their field/index and original source authority. Generated markers, punctuation and decorations remain explicitly unplanned; do not turn equal-looking generated content into source children. Preserve all existing geometry, style, hiding, count, spacing, diagnostics and restoration formulas. Hidden bodies retain causal admissions even though their paint is removed; no execution is introduced. Source-route coverage is checked independently from paint.
