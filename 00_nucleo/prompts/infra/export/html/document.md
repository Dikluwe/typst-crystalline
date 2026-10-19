# L0 — html/document
Hash do Código: 6f0012ea

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


## Realização com avaliação única

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
