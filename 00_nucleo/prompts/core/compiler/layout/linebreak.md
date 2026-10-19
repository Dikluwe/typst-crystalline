# L0 — linebreak
Hash do Código: eae927ee

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/linebreak.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/linebreak.rs:16`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/probe.json.gz` refutes resolving 456 789 in isolation; source measurement/affected locations are in inventory and initial.json.gz. The human ratified immutable API/three optional fields in manifest 1ae7513e. This is internal parity correction without a phase change.

`lab/results/construction/diagnostics/block-align-2026-10-06/linebreak-baseline.json.gz` shows an initial authorial backslash followed by ABC at y20 in Tekt versus 35.507448 vanilla, two at y51.014828; ABC\DEF matches. After nested Align, explicit break opens an empty paragraph line: its absence explains the remaining 15.5074pt nested-control delta rather than new misalignment. `linebreak.rs:20-25` establishes causality only with a first item; `line_commit.rs:278` correctly refuses uncaused technical flush.

The historical `smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz` probes record pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256, HEAD/working tree, diff HEAD --stat, UTC, sources, argv/cwd/environment and stdout/stderr. PDF uses extracted text; HTML uses DOM/text and diagnostics, never PDF-byte equality. Evaluation converts markup quotes to Text, losing double/enabled fields. A variable containing quotes retains English glyphs under German/French or smartquote(enabled:false). Tekt converts primes 6'/2" to quotes. Native-function show and public name were corrected separately and passed before this gate; markup still was not SmartQuote. An empty constructor has empty fields in vanilla; double:false is known, but omitted double is Unset. enabled:false remains SmartQuote in vanilla. set double:false and filtered show have new positive probes. Literal HTML works because it receives Text; programmatic SmartQuote was rejected. Literal title renders quotes; script receives a string and rejects Content before export in both binaries.

Source: `markup_stream.rs:79–128` selects glyph in eval; layout smartquote.rs alternates two flags without preceding text; `stdlib/text/smartquote.rs:112–117` deletes the disabled element; `observation/elements.rs:242–250` compares current fields. Vanilla `typst-eval/markup.rs:141–146` constructs an element with explicit double; `typst-layout/inline/collect.rs:196–211` resolves style/context during collection; `typst-library/text/smartquote.rs:119–158` distinguishes prime/apostrophe/nesting.

`links-captura-2026-10-14/first-green.log` records three causal regressions, with source in first-candidate.json.gz. The empty child position in ExplicitLinebreakBoundary uses current_line.first() at `linebreak.rs:38-43`; a leading private boundary yields(0,0) while actual text remains at correct baseline.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Append the existing transparent ExplicitLinebreakBoundary at the first pending item's position when a first item exists. If `justify` is true, request justification through `00_nucleo/prompts/core/compiler/layout/line_commit.md`; otherwise do not request it. Close the current line through the ordinary commit hook. The boundary does not paint text, and the adapter does not define the justification formula.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

This unit consumes its boundary in private Cursor/Arc-input/exact-span transport. Prepare the whole textual paragraph before emission with already-certified lexical base; preserve visit-order offsets rather than string matching. Sequence/Par scope, nested Sequence inheritance, checkpoint cursor/current-span preservation and subframe isolation/restoration remain their owning units' boundaries (`00_nucleo/prompts/core/compiler/layout/bidi_origin.md`). Text supplies subspans for words/Literal spaces; Space consumes its segment even weak; Word emits a span only for exact untransformed text. This owner consumes Linebreak boundary once.

Initial certified scope is incremental Simple outside cells/columns/subframes, without tracking/hyphenation/justification/smallcaps; Text has no newline/tab/language segmentation. Refused regions carry None. Prior global preparation stays for paragraphs needing no bidi context. Certified RTL-base/RTL-character/opposite-override subset uses incremental without changing permitted breaks; explicitly selected Optimized is untouched. The bidi_origin owner prepares/validates; this owner coordinates only its role.

With no causal line before an authorial Linebreak, admit baseline through existing helper and append empty Text with current style/bidi None. Do not invoke Text layout or consume another bidi span. Append the existing transparent boundary and close through ordinary commit. Keep justify, run position/direction and single origin consumption. Technical compiler flushes stay no-op; SpaceFromNewline must not become an explicit break.

For boundary position, choose the first item that is not a direct private marker, preserving empty-line admission, TextSeparator, every existing material wrapper, justify, bidi consumption and flush. Never change test expectation to accept metadata origin.

Human ratification on 2026-10-06 preserves semantic SmartQuote identity/fields through eval and selects glyph at each target's render composition. The approved gate changes public double bool→Option<bool>, adds enabled:Option<bool>, exposes a pure helper/context and moves eval→render glyph choice only for quotes; unrelated ADRs/nuclei/proposals remain. Paged quote rendering belongs to `00_nucleo/prompts/core/compiler/layout/smartquote.md`; HTML remains semantic under ADR-0128 and owns escaping. One pure L1 contextual helper has per-flow/compilation state, no I/O/globals/vtable/property registry/copied upstream memoization. Stack/state is mechanics; language/known-versus-omitted fields/glyph/style are acceptance. No new modes/flags/unsafe/watch API/math phase.

This owner observes logical newline in quote context without erasing nesting or inventing painted text. Preserve marker, justification, cursor and flush.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require occurrence/UTF8/restoration/parbreak-origin tests and bilateral wraps. Empty-line controls cover one/two initial explicit breaks versus material-line advance, integration after Align and prior corpus. Semantic-quote ratification authorizes RED→GREEN with exclusive ownership/pinned nuclei and supersedes historical conflicts only within quotes. Preserve exact target/field/helper boundaries above.
