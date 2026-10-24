# L0 — metrics
Hash do Código: 1c26d835

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/metrics.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/metrics.rs:22`, `01_core/src/compiler/layout/metrics.rs:27`, `01_core/src/compiler/layout/metrics.rs:37`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Historical separation evidence: cursor measures/selects word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source→PDF observations and isolated pre-export measurements are archived in `lab/results/construction/diagnostics/auditoria-quebra-paginacao-2026-09-28/`. They expose limitations, but separation preserves those exact decisions. Initial code/L0/provenance are in `lab/results/construction/diagnostics/separacao-texto-linhas-regioes-2026-09-28/`.

Bilateral probes and exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and `before/` measure U+002D versus SHY with coincident geometry in eight controls. Inspection of this consumer's FrameItem traversal is in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the public injected FontMetrics contract and its current defaults: text advance, vertical metrics/cap/text edges, ink, shaping width, math constants, glyph variants/assembly/kern and accent/correction access. Preserve Send + Sync and the existing borrowed-trait forwarding methods without adding public methods or stronger font guarantees.

The default text-width formula belongs exclusively to `00_nucleo/prompts/core/compiler/layout/text_measure.md`; this owner preserves its delegation through the public injected method. Preserve the existing complex-script predicate. FixedMetrics remains a deterministic approximation (advance 0.6 per character at size, vertical factors 0.8/1.2, default cap edge 0.7; current x-height/ascender/descender/explicit edge handling), not production font parity. line_content_right preserves its current item projections and ignores direct private HTML markers. Shared frame-helper extents belong to `00_nucleo/prompts/core/compiler/layout/helpers.md`; do not replace this trait method's own existing projection by prose.

TextSeparator does not contribute to the visual right edge; its position/advance do not alter ink metrics. Keep the public FontMetrics API and every override/borrowed forwarding method unchanged. Default text_width delegates to the private text_measure formula and supports unsized implementers. Explicit overrides remain authoritative for word/hyphen-prefix measurement; callers must not bypass them by directly invoking the default formula.

Treat TextReplacement as a transparent visual envelope in this owner's traversals; descend into children, preserving text, order, coordinates and existing calculation, without a new algorithm or advance. The text-replacement nucleus shares invariants only; this prompt owns only its declared consumer.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Preserve existing responsibilities, observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen control, never PDF bytes or a new selection policy.


## Approved font-backed underline metrics — 2026-10-22

### Evidence and public-contract gate

The fresh paired sources and exact source/font/binary/UTC/dirty-state provenance
are in `lab/results/remaining-parity-2026-10-22/underline-before-font-scoped/measurement.json.gz`
(SHA-256 `db81849f43eaa92212629aeaddce587ba5fff8465fd878a22890cd05e894a838`).
DejaVu at 11pt requires downward offset 0.21484375pt and thickness
0.4833984375pt; Nimbus at 11pt requires 1.386pt and 0.55pt. Both 17pt
witnesses preserve the actual table scaling. These measured differences
refute a universal fixed-font coefficient. Noto's numeric equality does not
waive the separately measured SVG open-line fill failure. Exact font-table
hashes and reads are retained in the sibling
`agents/inline/underline-font-table-measurement.json`; rendered pairs and
source language geometry govern, rather than a required Rust representation.

The human approved the exact additive contract and bounded integration in
`lab/results/remaining-parity-2026-10-22/agents/inline/underline-contract-proposal.md`,
SHA-256 `e7f3db142865646f13f9041f36f572582802cd302f1aebb7f8b42bb0610f2c6f`.
Approval is recorded in `lab/results/remaining-parity-2026-10-22/underline-approval.json`.
This satisfies ADR-0127 section 2.1 for exactly the method below; it supersedes
only this addition's earlier no-new-public-method restriction. No new entity,
FrameItem field, phase, dependency, mode or unrelated API is authorized.

### Exclusive contract and acceptance

Add this default method to the existing public FontMetrics trait:

```rust
fn underline_metrics(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
    let _ = (text, style);
    (size * 0.2, size * 0.06)
}
```

Return `(downward_offset_from_baseline, thickness)` in points. Real font
position may produce a signed offset; thickness is nonnegative at valid
positive text size. The supplied size governs conversion independently of
style.size. Text identifies the actual run for existing fallback selection;
empty text requests the style's primary face. No callback/content evaluation.
The default is the pinned tableless fallback, not real font evidence.

Forward the method explicitly in the existing borrowed trait-object adapter
so an injected override is preserved through isolated subframe measurement.
Do not generalize that adapter or change any other public method. Require
non-default override dispatch through direct and borrowed/dynamic providers,
exact tableless scaling, and integration RED proving the decoration consumer
actually uses the override. Existing typography/Hash/purity contracts remain.


## R8 — font-backed note marker geometry (public contract approved)

Status: **APPROVED FOR MATERIALIZATION**. The human approved exactly
FontMetrics::text_superscript_metrics on 2026-10-24. The original drafts and
explicit approval are bound by `lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/approval.json`.
This satisfies ADR-0127 for this addition; all RED→GREEN and lineage gates apply.

### Measurement before proposal

`lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/baseline.json`
and `received-source.json.gz` bind the exact dirty HEAD and all inputs. Fresh
`before/receipt.json` reproduces ten current-pin note controls; all ten fail the
actual-output 0.001 pt physical page/word-box gate in `red-geometry.json` and
`red-geometry.log` (ten assertion failures, no harness errors). The original
extracted word order remains separately available; physical matching preserves
page membership and every word occurrence without replacing these observations.

`font-tables.json` binds actual table/font hashes. DejaVu Sans at 11pt has
OS/2 superscript ySize1433/upem2048 =>7.69677734375pt and upward offset
983/upem2048 =>5.27978515625pt. Liberation Serif has ySize1331/upem2048
=>7.14892578125pt and upward offset928/upem2048 =>4.984375pt, plus a `sups`
GSUB feature requiring actual whole-run coverage verification. The current
`compiler/layout/text/style.rs:288–300` instead uses 0.65 and a positive
0.3em baseline resolved after scaling; `cursor.rs:72–75` adds that offset.
The pinned oracle reads OS/2 script metrics in
`typst-library/src/text/font/metrics.rs:67–80`, default superscript metrics
in `text/shift.rs:213–218`, and actual GSUB-or-synthesis selection in
`typst-layout/src/inline/shaping.rs:1160–1215`. Its rendering rules apply 85%
entry text size, 0.5em leading, 1em indent and 0.05em number gap in
`model/footnote.rs:334–341` and `typst-layout/src/rules.rs:401–411`.
These are font-dependent render/language observations; Rust representation and
vanilla iteration mechanics are not acceptance criteria (ADR-0107/0108).

The current public FontMetrics API exposes advances, vertical/ink edges,
underline and mathematical metrics, but no text superscript metrics or native
script-feature coverage. Mathematical MATH constants select a different
semantic source and cannot be repurposed as OS/2 text metrics. L1 must not read
font files or select arbitrary measured coefficients to bypass this boundary.

### Exact proposed additive public method

```rust
fn text_superscript_metrics(
    &self,
    text: &str,
    size: Pt,
    style: &TextStyle,
) -> (Pt, Pt, Pt, bool)
```

The tuple is `(resolved_font_size, horizontal_offset, downward_baseline_offset,
use_native_sups)`, all geometric values in points relative to the supplied outer
size. It is a pure, object-safe, provided method. No new public entity/field,
FrameItem variant, dependency, mode or phase is proposed.

- Synthetic path: resolve the selected text face's OS/2 superscript height and
  horizontal/vertical offsets, with vertical sign converted to downward baseline.
- Native path: only if the selected actual face covers the complete supplied
  run and its `sups` substitution applies to every glyph, return outer size,
  zero offsets and true. Consumers request the existing OpenType feature for
  both measured advance and paint. Presence of a feature tag alone is insufficient.
- Tableless default: `(size * 0.6, Pt(0.0), size * -0.5, false)`, matching pinned
  synthetic defaults. This approximation does not certify font parity.
- Preserve existing font-family/variant/variation/fallback policy. No MATH-table
  substitution, I/O in L1, face-name constants or global mutable cache.
- The borrowed FontMetrics implementation forwards the method to the actual
  provider, preserving overrides through `&dyn FontMetrics`.
- The method supplies automatic font/run data; existing explicit style overrides
  remain owned by their callers. Mixed fallback-face runs require actual-run
  controls and retain explicitly measured limits; this method does not certify
  arbitrary multi-face shaping.

Approval of this exact addition supersedes only the earlier no-new-public-method
clause for this method. It does not authorize unrelated script API expansion.

### Required gates after human approval

Update the owning L0s of actual callers first, then reseal only the affected
owners after V15/V26 are valid. Add module Rust tests and demonstrate genuine
RED before implementation: supplied provider override, borrowed-dynamic
forwarding, default fallback, font table scaling at11/17pt, complete native GSUB
coverage versus missing-glyph/refused coverage, and measurement/paint agreement.
Require the unchanged ten R8 integration controls plus size/font/custom-number
refuters, full PDF/SVG geometry at0.001pt, all body/page memberships and source
identity/counter/callback effects once. Note style/reservation, nesting, source-
certified splitting, call/body migration and columns remain separately owned
under their current L0s; this public method alone does not close those defects.
Preserve R1 formatted/literal brackets and R2–R7 scopes. Complete workspace,
Clippy/fmt/Rustdoc, strict/reverse lineage and full lint gates are mandatory.

R8 isolated-measurement preservation: the borrowed provider must also forward
existing text_width and advance_shaped overrides. Current impl for &dyn forwards
advance but inherits these defaults, losing actual shaping/explicit overrides
when constructing an isolated layouter. Preserve the already-public signatures;
this is an internal delegation correction required by the existing override
contract, not another public addition. Test supplied overrides through a generic
borrowed provider RED→GREEN before use in note preflight.


## R10 — resolved superscript runs (public contract approved)

### Measurement before proposal

`lab/results/construction/diagnostics/superscript-r10-2026-10-24/baseline.json`,
`received-source.json.gz`, `before/receipt.json` and `baseline-audit.json` bind
HEAD, exact received R9 inputs, dirty changed-file list, UTC, both executable
hashes and the fixed bd5ebec285505889e8ff1d4f68804ab147a699f3 oracle. Six of fifteen
script controls fail the unchanged 0.001pt physical-page/complete-word geometry
gate. Literal123\nZ paints END on the original baseline instead of after Z.
Nimbus hard-break123 uses native sups although the complete script range includes
a mandatory break and must synthesize. Mixed123☃ uses the first face's size and
baseline for both faces; the Noto Color Emoji fallback has OS/2 height1433/2048
and offset983/2048, unlike Nimbus600/1000 and350/1000. The paired SVGs show the
bitmap carrier separately; it must retain extraction and paint identity.

Current metrics.rs:61-67 returns one size/offset/native tuple and no resolved
face or source partition. text/style.rs:29-47 applies it to the entire Text.
The L1 consumer cannot recover the fallback face for correct text_edges and
per-face size/offset from that tuple. Reading fonts in L1, guessing face names,
encoding data in unrelated style fields or rescaling twice are prohibited.
Pinned inline/shaping.rs:970-1004,1090-1107,1167-1217 resolves actual face/run
script geometry, retaining outer size for each fallback; collect.rs:191-196
includes authored Linebreak in the script text. Rendered text, geometry and
morphology are language obligations; copying those internal structures is not.

### Exact additive contract approved by the human on 2026-10-24

```rust
fn text_superscript_runs(
    &self,
    text: &str,
    size: Pt,
    style: &TextStyle,
) -> Vec<(std::ops::Range<usize>, TextStyle, Pt)>
```

This is a pure, object-safe, provided FontMetrics method. Each tuple contains an
original UTF-8 byte range, the resolved text style (actual face, script size,
absolute downward baseline offset and explicit native/synthetic decision), and
a paint-only horizontal compensation in points. Ranges must form an ordered,
non-overlapping complete partition; the empty input has one0..0 range. Do not
change original text or add content/callback/font I/O to L1. A consumer validates
the complete partition before admission; an invalid provider answer preserves
the existing single-run path. Font selection and metric I/O stay in L3.

The provided default returns one complete range. It delegates the existing
text_superscript_metrics capability query, applies the already-supported explicit
synthetic size override, and clones the requested style with resolved size,
absolute baseline and superscript_native. For forced synthesis when the capability tuple is native, the provided default
uses the tableless synthetic tuple (size*0.6,0,-size*0.5,false); a real provider
overrides with actual OS/2 metrics. It does not change the old capability query
or promise font parity from that fallback. Some(false) requests synthesis for the complete
source run; it must not enable native sups again on a shortened covered prefix.
The borrowed-dynamic adapter explicitly forwards the new method.

Production providers resolve actual selected face/axes for each causal fallback
range and retain the supplied outer size for each face, without inheriting a
previous face's scale/offset. Native coverage is certified on its full causal
script text before fallback, never independently on a conveniently shortened
prefix. No change to ordinary non-script font selection, existing public methods,
entity/FrameItem fields, default mode, pipeline phase or dependency is proposed.
This supersedes only the single-face limitation for this addition once approved.

Require default/override/borrowed forwarding, valid UTF-8 partition, explicit
synthesis/native and selected-face scaling RED→GREEN; actual six regressions plus
space, warm/cold, hard/literal break, styles, sources, tracking and size controls;
complete PDF/SVG geometry and bitmap/extraction checks without weakening existing
word/page/tolerance gates; preserve R8/R9, A–H/R1–R7, full workspace, lint, reverse
lineage and resource gates. No productive implementation of this method before
human approval of this exact owner and hash.

Approval receipt: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/approval.json` binds exact draft SHA-256 `65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9`. Status: APPROVED FOR MATERIALIZATION, subject to tests and validation.

R10 measurement refinement before code: `font-tables.json` in the R10 evidence
root records actual font hashes and Nimbus OS/2 yOffset350/1000, correcting the
400/1000 numeric typo in the preserved approved draft without changing its exact
public signature or scope. Pinned inline/shaping.rs:473-510 measures line edges
at outer styles.resolve(TextElem::size), while build:350-363 paints at each
reduced glyph size/offset. Resolve run top/bottom edges from the actual face at
that outer size and carry them as existing absolute TextEdge lengths; do not
use the reduced paint size to shrink the line's typographic envelope. No new
field or public-method change follows. Require actual-face line-origin and
explicit-edge controls; a regression in R8/R9 refutes an unrestricted change.


## R10 — expanded signed-tracking, fallback-space and literal-tail refuters

Measurement before decision: candidate2-build/receipt.json/source.json.gz and
candidate2-matrix/receipt.json bind exact dirty inputs, UTC, sources and binary
f2a61237c15688e655ff7c62a117cb50a4d3a154e426bd31bed61fa999385e68
against the fixed bd5ebec2 oracle. Original15 script PDF/SVG controls and all
R8/R9 controls pass; the54 additional controls expose22 failures. At Nimbus11,
trailing literal LF loses a3.058pt following markup space; explicit DejaVu
fallback overmeasures that space by0.43858pt. At DejaVu11 tracking0.2pt, the
script's three intervals overadvance by0.18018pt. Nimbus synthetic digits need
0.12pt intervals and the following fallback glyph needs the same last-prefix
interval rather than zero. Pinned inline/shaping.rs:1269-1295 divides tracking
by outer ctx.size before each glyph's actual reduced size is applied; :473-510
retains outer-size line edges. Source sequence/space causes and actual selected
text face explain the remaining cases, without a font-name branch or looser
physical-word/page/0.001pt gate. This is private parity integration of the
approved method, not another public contract/default/phase change.

### Exclusive owning obligation

The provided resolved-run method converts requested tracking at outer size into an absolute paint tracking scaled by resolved paint_size/outer_size, preserving its sign. A zero/nonfinite outer size avoids division; existing capability and ordinary methods stay intact.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.
