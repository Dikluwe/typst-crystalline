# L0 — smartquote
Hash do Código: 9d2dd992

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/smartquote.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/smartquote.rs:18`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The historical `smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz` probes record pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256, HEAD/working tree, diff HEAD --stat, UTC, sources, argv/cwd/environment and stdout/stderr. PDF uses extracted text; HTML uses DOM/text and diagnostics, never PDF-byte equality. Evaluation converts markup quotes to Text, losing double/enabled fields. A variable containing quotes retains English glyphs under German/French or smartquote(enabled:false). Tekt converts primes 6'/2" to quotes. Native-function show and public name were corrected separately and passed before this gate; markup still was not SmartQuote. An empty constructor has empty fields in vanilla; double:false is known, but omitted double is Unset. enabled:false remains SmartQuote in vanilla. set double:false and filtered show have new positive probes. Literal HTML works because it receives Text; programmatic SmartQuote was rejected. Literal title renders quotes; script receives a string and rejects Content before export in both binaries.

Source: `markup_stream.rs:79–128` selects glyph in eval; layout smartquote.rs alternates two flags without preceding text; `stdlib/text/smartquote.rs:112–117` deletes the disabled element; `observation/elements.rs:242–250` compares current fields. Vanilla `typst-eval/markup.rs:141–146` constructs an element with explicit double; `typst-layout/inline/collect.rs:196–211` resolves style/context during collection; `typst-library/text/smartquote.rs:119–158` distinguishes prime/apostrophe/nesting.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve alternative and custom quotes with element values ahead of the active chain, then use the language owner's existing pair resolver with active language and single/double selection. Preserve existing handling of parsed inherited auto/custom quote values; do not invent a local localization catalog. Resolve through the shared pure SmartQuoteContext helper and emit the selected glyph through layout_word under the effective text style. Clone and restore the entire private context in the hub checkpoint. Explicit/automatic quote policy is not a new show-context evaluator.

The human ratified semantic quotes and late resolution on 2026-10-06, authorizing RED→GREEN with exclusive ownership and the pinned nuclei. Ratification supersedes conflicting historical clauses only for SmartQuote; unrelated ADRs/nuclei/proposals remain intact. The approved gate changes public double bool→Option<bool>, adds enabled:Option<bool>, exposes a pure helper/context and moves glyph selection eval→render. These approved cross-owner obligations are boundary criteria: semantic identity/fields remain in `00_nucleo/prompts/core/entities/elements/smartquote.md`, `00_nucleo/prompts/core/compiler/stdlib/text/smartquote.md` and `00_nucleo/prompts/core/compiler/eval/markup_stream.md`; this consumer owns paged rendering.

Preserve SmartQuote identity/fields in eval and resolve glyph only in each target's render composition. Paged uses this unit; HTML remains semantic and must not invoke paginated layout for quotes (ADR-0128). Contextual selection is one pure L1 helper, with per-flow/compilation state, no I/O, globals, vtable, property registry or copied upstream memoization algorithm. Stack/state is chosen Tekt mechanics; language, known/omitted fields, glyph and style are acceptance. HTML escaping remains with its owners. No new modes/flags, unsafe, watch API or math phase.

Resolve glyph through `resolve_smart_quote` with chain and effective language, following `text::effective_style` precedence; emit with active style/fonts and restore outer style. Use Layouter context shared with text; do not alternate isolated booleans or rewrap the element. Preserve inheritance/overrides, disabled behavior, French NBSP and nested-context/prime/apostrophe glyphs. No I/O, new Engine/show evaluation or new logic in the hub arm.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

The 2026-10-06 human confirmation covers the stated public field/helper and eval→render changes only. Apply RED→GREEN with the pinned semantics; preserve all quote field, glyph, style, inheritance and target boundaries above. This prose does not authorize unrelated unresolved proposals.
