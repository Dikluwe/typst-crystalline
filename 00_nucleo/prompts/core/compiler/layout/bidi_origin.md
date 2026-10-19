# L0 — Logical text origin for bidi
Hash do Código: c9aa0b18

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:5c733e4756935ef5e4841991852a5f60f0f00e2b76c988c6af340b52cc0ce977
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425


**Layer:** L1.
**Exclusive consumer:** `01_core/src/compiler/layout/bidi_origin.rs`.

## Evidence before specification

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/probe.json.gz` records the refutation of per-fragment resolution with equal base. `paragraph.rs:15–51` already certifies integral direction; `text.rs:66` splits words and `space.rs:31` emits separators. `initial.json.gz` preserves source, state and binaries. The public contract was ratified in `bidi-contexto-aprovacao-2026-10-05.json`.

`lab/results/construction/diagnostics/bidi-contexto-implementacao-2026-10-05/verification.json.gz` preserves the first candidate: RTL mixed/LTR numeric still match the incorrect baseline. `pipeline-context.log` records full-pipeline RED: every leaf lacks a span; `realize/mod.rs:38–44` inserts Located around occurrences. Direct evaluation had not exercised that envelope. Preserve first-candidate artifacts. Vanilla `collect.rs:191–194` requires LF in the default case; numeric-explicit measurement is in the implementation receipt.

Typed Styled evidence: `word_layout.rs:89–121` emits Shape for highlight; `text/style.rs` decodes typed tracking and baseline, so rejecting only chain.custom would miss typed deltas. `style_scope.rs:42–69` handles pdf.marker.tag=label with an inline subframe and outline.link.destination with Link. Current reorder works on direct line leaves.

`focused-candidate-verification.json.gz` and `candidate-source.json.gz` under `lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06/` preserve a candidate with correct width/anchor but incorrect ABC/123/Arabic RTL order. `bidi_origin.rs:111-117` rejects all cells/subframes, preventing integral input from reaching L3 reorder. `cell_body.rs:223-235` treats Text/TextShaped, but not TextSeparator, as text for baseline rebasing, introducing a top-edge delta within the same line. The two internal causes can be refuted by order/baseline still differing after limited certification and consistent recomposition.

The source-origin baseline `lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv and binaries at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. The source has corresponding transparent operations in ShowRecipe/Located/Realized branches; Source must not leak as an observable element. Human approval `confirmo` was received on 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`. These historical measurements do not certify the current ratified reference without a new run.

## Ownership and certified scope

Own a private layout descendant helper, without expanding public API. A Clone cursor contains Arc input and the next index; next returns a complete Text/Space span in order, while Linebreak advances without a span. Prepare through integral structural traversal: Styled updates chain, Strong/Emph preserve order. Text keeps original EcoString and effective horizontal Dir; Space/Newline retain origin; Linebreak retains boundary. Unsupported rejects the entire preparation. Do not discover origin through string search.

Restrict certification to a simple incremental region: no columns, generic subframes, justification except the specifically permitted isolated-cell capability, hyphenation, smallcaps, tracking or optimized selection; Text has no newline/tab, words require no segmentation, and no transformation may mismatch the text. Activate only when base RTL, run RTL or run differs from base; pure LTR stays on the previous planning path. Base comes from certified lexical direction, never the first character. Cursor is not global; checkpoint clones it and subframes suspend it. The exact fragment occupies a private Layouter field only during emission, then is restored.

Integral refusal forbids recertifying a partial child. A private Sequence/Par scope guard prevents reentry; subframes suspend guard together with cursor. Located is transparent only for certification of its complete textual body: do not ignore opaque bodies or change Location registration. Lexical direction and collection traverse the same envelope. Linebreak justify=true remains uncertified because the ratified unit carrier does not encode that distinction.

Styled certification also inspects typed deltas. Tracking, highlight, subscript/superscript and shifted baseline produce paint/positions outside the known reconstruction and reject the entire input, even only within a wrapper. Font/size/bold/italic/font-variation without text transformation remain allowed. Par line numbering and pending floats reject certification because they add material/change region. Internal pdf.marker.tag=label and outline.link.destination channels likewise reject integral certification; do not presume their leaves transparent.

Permit a cell region only with existing cell_bidi_allowed and regions.cell; generic subframes remain rejected. Do not change collect, grammar, chain, lexical base, ranges or transform guards. Certify the complete original cell input, without authored virtual text. The cell correction is internal parity work with no public contract, phase, product flag or new unsafe.

Certification traverses Source without changing logical text, direction, complete ranges, chain or order; an origin wrapper does not cause Unsupported. Preserve exclusive ownership/static dispatch, without I/O or a new signature/trait/flag or eval↔layout phase.

## Acceptance and limits

Require RED→GREEN for traversal of two equal words with distinct UTF8 offsets, causal Space, override and unsupported None; real-source Sequence/Par, wraps and checkpoint tests; zero I/O and unsafe. Verify None for sequences with opaque objects and separated leaves. Full-pipeline tests require spans in every text leaf and RTL order with an initial Latin fragment. Direct Styled Tracking/Highlight/Superscript tests must first be RED. Source-origin tests and C2/C/macro collections, builds and lineage are required before certification. Preserve all restrictions above rather than claiming general bidi parity.
