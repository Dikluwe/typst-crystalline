# L0 — html/document
Hash do Código: 3531eac9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/document.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130


## Consumer evidence

`03_infra/src/export/html/document.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Document composition

Own export_html_with_frame_resolver, automatic doctype/envelope/metadata,
explicit_document_root and block_sequence with the initial StyleChain. A sole
explicit html root follows the doctype directly without automatic lang/head/metadata;
a sole body retains its attributes inside the automatic html/head envelope. html/body
with other relevant nodes returns `` `<{}>` element must be the only element in the document ``
with the offending tag substituted. Ignore only
the existing introspective/empty-space nodes in root classification; head/title are
ordinary nodes, not extracted metadata.

Buffer top-level paragraph-groupable content according to tags; flush at parbreaks,
block/non-groupable nodes, tables and HtmlFrame. Preserve trimming and paragraph
boundaries through whitespace helpers; do not insert paragraphs inside normal
HtmlElem bodies. Emit compact markup first and call pretty only afterwards.
Inline frames remain block-level without a paragraph wrapper; their markup belongs
to inline and their rendering to the external resolver.

## Verification

Check explicit html/body exclusivity, head as ordinary content, attributes/metadata,
mixed paragraph boundaries, tables/frames and equivalent compact/pretty document DOM.


## Visual blocks, styled headings and explicit roots

Before block/root classification visually project internal carriers, preserving
existing tags/grouping. Never serialize consumed origins; semantic indexing is
separate. `paridade-dominios-2026-10-06/measurement.json.gz` html-headings/html-show
baseline observes unsupported Tekt versus h2/h3 in historical vanilla 9f2b6e871.
document.rs:47–87,145–158 classifies only direct Heading with no offset; Styled
falls into inline, which rejects blocks. Upstream HTML rules.rs:223–260 maps level
1→h2 and ≥6→div role=heading aria-level=level+1. No intent is inferred.

Classify block boundaries through Styled/Sequence retaining lexical chain.
Convert styled blocks to their DOM blocks without wrapping headings in p or
splitting inline spans of a paragraph. Centralize Heading emission here with
level offset/semantic level-6 fallback. Emit visual only; project carriers on entry.
Preserve html/body root classification/exclusivity diagnostic. Numbering/deep-heading
warnings remain uncertified. Internal ADR-0127 adds no signature/phase/default/
unsafe. RED→GREEN styled/Sequence headings with preceding/following paragraphs,
per-level tags and unfragmented inline Styled, bilateral DOM/L3 suite.

Intermediate html-headings/measurement.json.gz explicit-body/html under Styled
produced duplicate roots versus the oracle's single explicit root. The prior
direct-only explicit_document_root hid it until styled-block support. Classify
roots through Styled/Sequence, transporting chain to inline with each relevant node.
No automatic body for an explicit root, no other relevant siblings and no treating
HtmlElem children as document roots. Keep exclusivity diagnostic; RED→GREEN styled
body/root exclusivity. workspace-seventh/p1421 contextual HTML separately refutes
tag loss under Source: root/block-inline/document/Sequence project Source.body,
retaining tags/style/encoding/SmartQuote, without fabricated paragraph/container.

## Semantic quotes with late target resolution

### Evidence and approval

`lab/results/construction/diagnostics/smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz`
retains historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256,
HEAD/tree/diffstat/UTC/sources/argv/cwd/environment/stdout/stderr. PDF uses text,
HTML DOM/text/diagnostics, not PDF bytes. Eval converted markup to Text, losing
double/enabled; variables retained English glyphs under German/French/disabled
contexts, primes 6'/2" became quotes. Native show/public name already had separate
passing repairs while markup remained non-SmartQuote. Vanilla empty constructor
has no fields, omitted double is Unset versus known false, enabled:false retains
SmartQuote. Positive set double:false/filtered-show controls supplement literal
HTML Text success/programmatic rejection. Literal title quotes render; script
requires string/rejects Content before export in both.
Local markup_stream.rs:79–128 selects eval glyph, layout smartquote.rs toggles two
flags without preceding text, stdlib/text/smartquote.rs:112–117 removes disabled,
observation/elements.rs:242–250 compares current fields. Vanilla markup.rs:141–146
sets explicit double, inline/collect.rs:196–211 resolves styles/context,
text/smartquote.rs:119–158 distinguishes prime/apostrophe/nesting.
Human approval on 2026-10-06 ratified public double bool→Option<bool>,
enabled:Option<bool>, pure helper/context and eval→render glyph choice. Supersede
only conflicting SmartQuote clauses, never other ADRs/nuclei/proposals.

### Owner boundary and acceptance

Domain preserves quote identity/fields in eval, each target resolves glyph at
render composition. HTML stays semantic without paginated quote layout (ADR-0128).
One pure L1 contextual helper uses per-flow/compilation state, no I/O/globals/vtable/
property registry/upstream memoization copy. Stack/state mechanics are Tekt choices;
language/known-versus-omitted fields/glyph/style are acceptance. Escaping remains
with its owners, no new modes/flags/unsafe/watch API/math phase.


Share quote context while append_sequence/append_block accumulate a paragraph,
not per leaf. Reset on logical flush/block boundary; Heading and Par body own
flows. Pass private stateful inline context with public APIs unchanged, preserving
envelope/root/exclusivity/visual carriers/style. Existing auto-root lang=en versus
specified locale remains outside certification; glyph/body equality is no HTML-
byte equivalence claim.

## Nonvisual introspection markers

### Evidence

`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/{candidate-probe/target-controls/state-extended.json.gz,candidate-probes-resources.json}`
retains HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree/UTC/
sources/argv/diffstat. State reader-init/unused/at-prefix pass PDF but HTML rejects
markers unsupported; current pin `bd5ebec285505889e8ff1d4f68804ab147a699f3` accepts.
Upstream styles.rs:1049–1050 registers StateUpdate/Metadata empty for both targets;
local document.rs:28–35 already projects carriers before blocks, and Label has
body at construction.rs:391–398 that cannot disappear. This is observable content/
error, not upstream-mechanics or inferred-intent requirement.

### Contract and acceptance

Within this visual projection StateUpdate/Metadata/legacy State initializer
contribute Empty; Label contributes transformed body. Earlier semantic index keeps
actions/values/labels. No exporter callback evaluation or hiding unresolved
ContextBlock/StateDisplay. Preserve Label body/Heading boundaries/whitespace.
Ref links/anchors and HTML math remain uncertified. Internal continuous ADR-0127,
no public API/flag/phase/default: owner RED→GREEN/bilateral PDF-HTML; markers among
text, labeled heading, metadata, legacy State and still-unsupported-content error.

## Figure show-product paragraph morphology (2026-10-19)

### Measurement before decision

`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/realize/html-figure-morphology/receipt.json`
binds baseline HEAD 731f0ffdf4c7c152f590551e72ff63f7bd85ef85, dirty-state diffstat,
UTC, exact sources/argv, pinned oracle commit bd5ebec285505889e8ff1d4f68804ab147a699f3
and executable SHA 9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9,
and candidate-build-2 receipt SHA plus binary
12265c0a05442725b28274a3d11f89ee8c93578d4dc308bcd224178249e0396a.
Document.rs:31 projects every Realized directly to visual before paragraph
classification. Oracle `A figure B figure C` with a Figure show returning X gives
five paragraphs; candidate gives one. Adjacent Figures give two versus one.
Strong replacement retains strong markup within separate paragraphs. Heading
replacement stays inline in both. Alignment overrides do not remove Figure
boundaries. None/empty Figure products still separate surrounding A/B into two
paragraphs without an empty paragraph. A div replacement remains a div between
neighbor paragraphs; two-paragraph replacement retains both. This is measured
language morphology under ADR-0107, not a text-normalization or byte requirement.

### Responsibility and acceptance

Before discarding a Realized original Figure boundary, retain its measured actual
paragraph separation around the emitted visual, through Source/Located/Styled
original carriers. Serialize only visual content; do not render/index original
Figure body or infer support for implicit Figure HTML body/caption rendering.
Preserve rich inline products, adjacent products, surrounding inline paragraphs,
empty replacement separation, existing block visual products and heading/nonFigure
replacement inline behavior. Keep explicit roots, lexical styles, smart quotes,
markers, encoding and existing unsupported-content errors. No new public API,
phase, mode or generic entity change. Functional figure_ RED precedes sourcefix;
tests observe paragraph/tag sequence and rich payload rather than flat text only.

The observed RED8 paragraph failures authorize a bounded visual projection:
recognize Figure originals through existing Source/Located/Styled wrappers and
place existing Parbreak boundaries before/after that emitted visual contribution
before ordinary document grouping. These boundaries encode the measured paragraph
separation even for Empty; they are not new content fields, synthesized original
body rendering or a text-space deletion. Existing block/div/multiparagraph visual
contributions continue through existing block grouping without a forced p wrapper.
NonFigure Realized visual projection stays unchanged.

## Captured Figure product origin transport

GREEN9 unit controls passed, but real candidate-target-controls-3 still merges
html-kind-inherited-auto into one paragraph. Its exact source/HTML/argv and
candidate identity remain in that receipt. Causal source evidence:
compiler/realize/recipes.rs:486 delegates completed replacement to
compiler/eval/rules/textual.rs:114–139, which wraps the prepared Located original
in ShowRecipeContent.body for causal revisitation. document figure_origin only
unwrapped Source/Located/Styled, so actual captured Figure originals failed the
classifier while synthetic transport tests passed. ShowRecipe is data transport
here; reading its body executes no rule and cannot render the consumed original.

Add a separate functional figure_ RED with the actual ShowRecipe(original)
carrier, retaining the existing passing direct-origin/heading controls. Recognize
ShowRecipe.body in the same borrowed original-root classification. Preserve all
capture styles/rules/guards/file metadata untouched, never execute its callback or
collapse it to infer identity. No new owner/API/phase, no Ref/SetPage expansion.


## Located-root visual classification

### Evidence before decision

At baseline4382891, `03_infra/src/export/html/document.rs:28–46` uses
bottom-up map_content. The contract in
`00_nucleo/prompts/core/entities/content/traversal.md` suppresses a Located node's
immediate-body callback and never revisits replacements. Thus Located(State),
Located(Metadata), Located(Label(marker)) and a suppressed Realized root can
miss the promised visual-root decision. Source hashes and measurements are in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/audit.json`.
Direct and contextual Metadata language probes succeed; they do not establish
external failure or language reachability of every synthetic Located shape.
The exact local exporter regressions are preserved in the focused RED receipt
`lab/results/v16-corrections-2026-10-19/batch-g/focused-red-infra-3/receipt.json`.

### Contract and acceptance

Classify the current visual root after peeling only existing causal carriers and
Label roots. Preserve Realized Figure paragraph boundaries and use only its
visual, never its original. Empty only State, StateUpdate and Metadata. Leave
unresolved ContextBlock/StateDisplay and unsupported semantic nodes to the
renderer diagnostic. Generic bottom-up child traversal, root suppression and
replacement boundaries remain unchanged. Ordinary processed variants explicitly
retain their content; this projection adds no supported semantic element and
executes no transported recipe or callback.

Verify direct/Located/double-Located/Source/Styled marker chains and Label bodies
among text, Heading and explicit html/body roots. Preserve source/visual
separation, all Figure product paragraphs/rich payloads and unsupported-producer
errors. No global transparency, public API, target/default/phase or semantic
indexing change is admitted. The internal correction follows ADR-0127 continuous
flow; helper-level tests certify the local projection contract, not complete
HTML language reachability.


## Standalone Cell block-flow composition

### Evidence before decision

The current-pin source `bundle-cell-heading` in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/residual-container-probes/receipt.json`
requires body/h2 for an isolated table.cell heading. The exact final-candidate
attempt in `lab/results/v16-corrections-2026-10-19/batch-g/html-candidate/receipt.json`
binds source SHA712a1e5d9c534085224efa8bba378dc9faf05bcf7cb2ec569c6dcd12d56f3cd2,
product SHA185e63b30415aaec44cca9115781a662077aa1edffab6aabe97166d7a82b1620,
HEAD4382891f0d5f0839669597cb5c90a98f24cfb8bd, dirty inputs, UTC and argv.
It emits body/p/h2 while exit and diagnostic payloads match. This is measured
language morphology, not byte equivalence or a comparator-normalization issue.

`03_infra/src/export/html/document.rs:253–320,357–419` classifies direct blocks,
Source, Styled and Sequences but leaves a Cell to the paragraph fallback.
`03_infra/src/export/html/inline.rs:110–115` then emits that Cell's standalone
visual or body. The selected visual already contains the projected h2, so the
outer paragraph survives incorrectly. `03_infra/src/export/html/table.rs:44–50`
has a separate row-cell selection and owns td/th boundaries. The prior public
preparation assertion checked the inner h2 substring; the frozen DOM policy also
checks its actual outer form. Source hashes and the bounded causal reading are
retained in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/html/cell-block-flow-plan.json`.

### Contract and acceptance

In document block-presence detection, direct body composition and sequence
accumulation, inspect the same borrowed Cell child as the ordinary inline
renderer: standalone.visual when present, otherwise body. Its resolved snapshot
is not the standalone-inline selection. Delegate that chosen child through the
existing composition flow, inheriting the current StyleChain and paragraph quote
state. A block child flushes adjacent paragraphs; plain inline or rich inline
children remain in their surrounding paragraph. Preserve trimming, block resets,
rich markup and all unselected Cell fields/snapshots.

Do not peel Cells in project_visual_root or generic bottom-up traversal. A Table
still reaches its dedicated renderer with nominal cells and exact td/th, span and
resolved-row metadata intact. Do not change explicit html/body root recognition,
support for semantic kinds, renderer dispatch, public API, target/default or
pipeline phase. This is an ADR-0127 continuous correction inside existing HTML
composition, not a new Cell mode or global transparency rule.

Require direct isolated Cell h2 output without p/h2, and a mixed sequence with
neighbor paragraphs and a rich heading child. Preserve a single paragraph for
plain/Strong Cell content among inline text, and preserve a Table row's td/th
and colspan/rowspan boundaries around a projected heading. The frozen standalone
Cell language source must pass its original semantic DOM policy; all preceding
tests, warning/error policies and unsupported-content diagnostics remain binding.
