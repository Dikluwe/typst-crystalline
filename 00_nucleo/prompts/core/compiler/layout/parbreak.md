# L0 — parbreak
Hash do Código: 983ccd84

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/parbreak.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/parbreak.rs:15`, `01_core/src/compiler/layout/parbreak.rs:40`, `01_core/src/compiler/layout/parbreak.rs:92`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Historical pins do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new run.

`lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/{before/results.json,initial.json}` preserves exact state/source/pinned binaries/no artificial newline for empty/Literal-only/terminal-blank-line auto-height defects. text.rs:37-86/html_frame.rs:72/line_commit.rs:280-286/parbreak.rs:40-85/page_geometry.rs:180-201 locate missing empty leaf and conflated visual/causal line. after/results.json refutes placing Parbreak marker before Literal baseline: `#text("  ")\n\n` collapses to 3pt Tekt versus 8.568pt vanilla. Admit Literal-only baseline before adding neutral marker; marker itself paints nothing/adds no width.

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/{probe.json.gz,initial.json.gz,inventory.json}` refutes isolated 456 789 resolution and preserves source/state/binaries. Human ratification of immutable API/three optional fields is manifest 1ae7513e.

The historical `smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz` probes record pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256, HEAD/working tree, diff HEAD --stat, UTC, sources, argv/cwd/environment and stdout/stderr. PDF uses extracted text; HTML uses DOM/text and diagnostics, never PDF-byte equality. Evaluation converts markup quotes to Text, losing double/enabled fields. A variable containing quotes retains English glyphs under German/French or smartquote(enabled:false). Tekt converts primes 6'/2" to quotes. Native-function show and public name were corrected separately and passed before this gate; markup still was not SmartQuote. An empty constructor has empty fields in vanilla; double:false is known, but omitted double is Unset. enabled:false remains SmartQuote in vanilla. set double:false and filtered show have new positive probes. Literal HTML works because it receives Text; programmatic SmartQuote was rejected. Literal title renders quotes; script receives a string and rejects Content before export in both binaries.

Source: `markup_stream.rs:79–128` selects glyph in eval; layout smartquote.rs alternates two flags without preceding text; `stdlib/text/smartquote.rs:112–117` deletes the disabled element; `observation/elements.rs:242–250` compares current fields. Vanilla `typst-eval/markup.rs:141–146` constructs an element with explicit double; `typst-layout/inline/collect.rs:196–211` resolves style/context during collection; `typst-library/text/smartquote.rs:119–158` distinguishes prime/apostrophe/nesting.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz`records the same HEAD, uncommitted tree/diffstat/UTC/sources/binaries/argv and historical pin. All 24 root/Box pairs after 12 prefixes differ: Linebreak/Parbreak require y22.25pt, Shape y27pt, Block/Grid/VSpace20pt y35pt, Tekt 15pt. Rotate also overlaps independently. `line_commit.rs:721–733` prepares next baseline; `shape.rs:196–206` adds below/cap; `block.rs:465` separates boundary/margin; `place.rs:91–99` recognizes still-pending line only. Pinned flow/distribute.rs:703–715 uses physical flow offset for axis-unspecified Place. Weak margins/overlap/new page/rollback/subframe leakage can refute the causal-end hypothesis.

`lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree, status/diffstat/UTC and 2,099 sources/L0s. before.json.gz contains 92 pairs; extra-before.json.gz 26; boundary-before.json.gz 10; repeat-before.json.gz 4. Oracle commit: `bd5ebec285505889e8ff1d4f68804ab147a699f3`; binary SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; initial Tekt: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`. At width 160pt/margin 8/font12, hanging10 leaves first-line x8 and moves continuations x8→18; negative-5 requires x3. RTL leaves first right edge 152 and moves continuations to 142. Tekt ignores the length. MLA defaults to 18pt (1.5em); ambient set 3em does not replace it, while show-set3em requires 36pt; IEEE/Plain do not acquire hanging indent through that show-set. bibliography.where(style:"mla") does not match in the vanilla probe: preserve selector choice rather than forcing override.

Inline-only Block/Box bodies ignore hanging10; explicit Par inside them and Block with Parbreak apply it. Simple Heading/Raw/List/Quote do not. Partial, zero, repeated and empty-scope declarations close boundaries: FIRST/MIDDLE/LAST at 5.168/27.464/49.76 versus all 5.168 in Tekt. The first lexical probe used literal brackets; it was refuted, corrected to #[...] and retained as extra-before-initial, not evidence of Styled scope. Extreme values produce extractor-clipped paint; missing pdftotext output does not mean Content deletion. A partial 24pt font does not change the shared length's 12pt em, as partial-font records.

Pinned source: `typst-realize/src/lib.rs:1061–1085,1190–1219` distinguishes fragment/Par and interruption; `typst-layout/src/flow/collect.rs:72–134` selects flow; `inline/mod.rs:174,216–221` and `collect.rs:141–147` resolve shape; `inline/line.rs:496–503` positions; `rules.rs:510–523` provides CSL inset/compensation. `typst-library/src/model/bibliography.rs:112–127,367` determines default/precedence. This measures language/geometry, requiring neither vanilla Rust structure nor inferred historical intent. Lost boundaries, run-relative em, double indent, page reset, imposed greedy fallback, lost copies, reexecuted callbacks or control regressions refute closure.

## Exclusive ownership and current contract

Own only `01_core/src/compiler/layout/parbreak.rs`. Close material or causal Literal text line, admitting a Literal-only baseline before transparent ParbreakBoundary insertion and ordinary line commit. Direct-marker-only/noncausal empty lines do not gain flow/paint/width. After closing, confirm previous baseline's physical bottom for auto-page geometry; trailing paragraph spacing appears only when later causal content occupies it.

Resolve par.spacing/par.leading from active chain/style with `00_nucleo/prompts/core/compiler/layout/vanilla_defaults.md`, apply max(spacing-leading,0) and existing pending-block-margin collapse. Preserve prev_block_below_pending/block_chain_active/prev_margin_is_parbreak and no painted boundary text.

Preserve flush-certified physical flow end while preparing spacing/block state, revalidating witness at new cursor without adding weak margin to end. This adds only necessary private state under the causal-boundary protocol, with mandatory LIFO save/restore; other suffix state stays intact.

Reset full SmartQuoteContext at logical paragraph boundary, including Paragraph caller, never physical wrap flush. Preserve margin/collapse/marker/bidi/empty-Literal/terminal-state protocols. Human 2026-10-06 ratification covers quote field changes/helper and eval→target-render choice only for SmartQuote; canonical quote/entity/eval owners retain public scope, HTML stays semantic under ADR-0128. One pure per-flow L1 helper, no I/O/globals/vtable/property registry/upstream memoization copy/new mode/unsafe/watch/math phase; unrelated ADRs/nuclei/proposals remain.

After existing causal closure, terminate hanging context even for an inkless boundary without making line/height. Not-yet-admitted CSL compensation remains pending. Ratified bidi migration retains None in uncertified constructors/`..` read patterns and existing pinned transforms without phase/API expansion.

## Acceptance and incomplete scope

Preserve current text/morphology/causal positions-pages/styles/errors and static descendant interfaces. Internal ADR-0127 corrections require L0 first/RED→GREEN/restricted owner reseal/valid graph, no new public field/signature/flag/default/phase. Empty/Literal/auto-page controls and full bilateral matrix/prior regressions/build/lint retain expected outputs.

Require occurrence/UTF8/restoration/parbreak tests and bilateral wraps for logical transport. Tracking corrections require RED→GREEN, bilateral text/position/line controls for both directions/signed values and preserved LTR/uniform/terminal controls, error-free build/lint and restricted resealing; all other recorded limits stay open.

Causal-end certified producers line/shape/Block/Align/Grid/Raw/VSpace do not complete Block/inset/fragmentation, late inline Box/Transform or Heading/List/Stack/Equation/Columns. Invalid witness keeps legacy fallback/pending rather than parity credit. Revalidate whole suite/HTML-context/PDF-SVG/build-doc-fmt-check/lineage/warning inventory. Tiling public/native scopes stay separately qualified.

This internal parity correction follows ADR-0127 continuous flow: no new public field/signature, external flag/default, phase or realization order. Initial/continuation shape supersedes only the corresponding constant-width limitation. Certified Latin RTL remains subject to the nucleus conditions, without active BidiCursor/tracking or invented BidiSpan; every other prior limit remains. Off-page values/copies require frame/SVG inspection rather than equality of clipped extraction. Do not anticipate the separate R3-T1 scope.

Require RED→GREEN before implementation and source→layout/bilateral controls for relevant signs, boundaries, variants and restoration. Executing a grouped arm does not prove every alternative: closed nominal inline/flow-context classifications are intentional and need class/variant tests. Reseal only after V15/V26 are valid; run tests, build, Clippy/Rustdoc/fmt, reverse lineage and differential Tekt against pin1279707, under 12GiB/High8GiB/swap0/jobs1. No publication.


## Remaining inline source correction — 2026-10-22

### Measurement before specification

`lab/results/remaining-parity-2026-10-22/baseline-nine/measurement.json.gz` (SHA-256 `320e226fc9eb1b606cb155c4bfcba26046c2398f6fe0339102f1fd870ebd2aa1`, UTC 2026-10-22T11:48:44.824207+00:00–2026-10-22T11:49:15.381299+00:00)
records the exact nine immutable sources, ratified oracle commit/tree/executable,
source-bound baseline product `9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`,
HEAD/dirty stat/source archive, font bytes, commands and full PDF/SVG observations.
The exact inspected L0/source hashes are preserved in
`lab/results/remaining-parity-2026-10-22/agents/inline/investigation-state.json`.
These are source-language geometry/text observations, not required execution
mechanics. Preserve all historical refusals and failed candidates as evidence;
only the explicitly measured subsets below supersede their conservative limits.

### Carry the settled inline descent into the pending paragraph boundary

`context-source-once` has identical word sequence and STATE=1, while the final
STATE baseline/autoheight are7.642578125pt too small:16pt Box height minus its
actual8.357421875pt text baseline. `line_commit.rs:911–926` records the true
physical line end, but positive weak-margin admission in `cursor.rs:131` uses
previous baseline+pending descent, with the inline descent not carried.

After actual causal line closure, retain its valid settled physical descent
in the existing pending block-boundary state before weak paragraph spacing
prepares the next unit. Derive it from the actual accepted flow end and
previous baseline, never paint, Content-kind guesses or fixture offsets.
Preserve source flow witness validity at the changed cursor, existing weak/
strong margin collapse, empty/double boundaries, terminal autoheight, ordinary
text-only paragraphs, explicit Box baselines and finite-region rollback.
No new hub state, public signature or phase is required.

Require a genuine declared-metric RED for P+40pt fractional Box[A], paragraph
break, NEXT: NEXT starts after actual Box bottom+declared paragraph spacing+
NEXT's actual top. Preserve all markers once and terminal autoheight. Add
text-only/zero-height/baseline-shift/double-break and next-block controls.
The unchanged source corpus must close the original Context geometry, without
interpreting positional error as duplicated state effects or weakening reads.
