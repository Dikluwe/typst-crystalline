# L0 — html/inline
Hash do Código: 914edc58

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:a0d3cae81e77da5875204a94bc2fbd707009f0d751cf50ed870c416a01187a22
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/inline.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130


## Consumer evidence

`03_infra/src/export/html/inline.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Inline content dispatch

Own static Content dispatch for text, sequences, Strong/Emph, accumulated Styled
chains, linebreak, table wrappers, HtmlElem and HtmlFrame. Preserve the unsupported
content fallback `HTML export does not yet support this content`. Delegate normal body whitespace,
table serialization and context-dependent escaping to their owners.

HtmlElem preserves typed tag names and attribute order. Empty attribute values emit
only the name; nonempty values are quoted/escaped by encoding. Void tags emit no
closing tag and reject Content bodies even when empty with
"HTML void elements must not have children". Raw/title handling delegates to encoding.
For HtmlFrame, insert the resolver's validated SVG markup unescaped at its occurrence;
without a resolver retain `html.frame must be resolved by the HTML compilation pipeline`. Do not render
layout, wrap frames in p, substitute a frame tag or change style/resolver contracts.

## Verification

Check ordered/empty attributes, void-body rejection, styled chains, raw/title
delegation, unsupported variants and resolved/unresolved inline frames.


## Visual realization boundary

Serialize only the visual projection of carriers, never consumed origin or
transport DOM tags. Source.body shares the existing ShowRecipe.body transparency,
without separate origin execution/carrier publication/Location assignment.

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


Accept Content::SmartQuote through pure L1 resolve_smart_quote with active chain/
language, then escape_text in selected mode. Share SmartQuoteContext through
siblings/transparent inline wrappers using a private stateful helper; retain the
old wrapper for isolated callers. No paged quote layout; HtmlFrame retains SVG
resolver. Mixed native/markup, disabled/inherited contexts and nested tags share
flow. Raw/script remain string-only; title delegates with active chain to encoding.

## Ratified isolated-cell visual projection

Evidence: `table-cell-isolada-d08b3b-2026-10-07/complete.json.gz` retains 53 probes
from 2026-10-07T14:47:56.340144+00:00–14:48:12.375245+00:00, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/diffstat/status/hashes/
sources/argv/cwd/environment/full output; historical vanilla
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, Tekt
`6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`.
Do not count before/refuters/complete as independent rounds. Isolated-default/x,
raw-fields/x/table-style/raw-versus-prepared, nested/body/late effects, fresh-
generation/alias/two-rule/hidden cases establish defaults, once-only effects,
termination and causal-span language; upstream guard/data structure is mechanics.
The full source anchors/results/refuters and acceptance matrix remain in
`00_nucleo/prompts/infra/export/html/table.md` and the archived original of this
owner. No intent inferred or new-pin certification.

The insertion and isolated-cell packages were individually ratified on 2026-10-07:
`lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`
records `comfirmo`, manifesto
`5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`;
`lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`
records `confirmo`, materialized status, manifesto
`3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`.
Only stale pre-approval candidate gates are superseded. Public carriers/fields
remain owned by their canonical domain prompts; this owner consumes their
approved contract without sharing ownership. cell-set/style defaults require
separate measured L0; general Table equality/defaults, rich casts/paint/strokes/
insets/lines/gutters, multipage spans, PDF/UA/global nonlocatable policy stay open.
Do not normalize the existing empty-file eval warning to certify parity.

Serialize exclusively the emitted prepared isolated-Cell visual, never hidden
lexical body. Delegate children/escaping through HTML without paginated layout,
new getters or recipe execution. Require reviewed-candidate/current-L0 tests RED
before code, scoped seals, bilateral/additional controls, effects once/equality/
getters/selection/HTML/actual text-style-geometry with original messages/spans;
debug/release build and core/infra/shell suites, lint zero errors/no owner V5/V6/
V15/V26. Revalidate earlier table lexical/insertion, Raw defaults/blocks, Equation/
math/matrix/query. Preserve prior tree through snapshot/restricted diff, no commit/
reset/global resealing, no byte/whitespace normalization to close render.

## Restricted typc atom display

### Evidence

`lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/after-second.json.gz`
html-fields retains HEAD/state/UTC/binaries. Tekt emits X auto a as text;
vanilla uses code(data-lang:typc), span constant.language color #d73948.
Upstream value.rs:191–208 displays Auto/Bool as inline typc Raw;
markup_stream.rs:210–215 covers only Bool/falls back Text for Auto; raw.rs:201–203
colors only true/false; inline.rs lacks Raw and rejects it generally. This is
language morphology/style, not PDF bytes; equal text alone proves no parity.

### Contract and acceptance boundary

Before implementation tests refute Auto→Text and atomic-Raw HTML rejection.
Only map Auto→Raw(lang:typc,block:false), classify auto as constant.language and
serialize typc auto/true/false via code/constant span with existing escaping/mode.
Each owner implements only its existing responsibility: markup presentation,
paged Raw color, inline HTML tags or tests. No new public type/signature/phase/
flag/default/grammar or other Value change/general Raw HTML. Other languages/
arbitrary Raw remain uncertified. Require RED→GREEN/product-style observations
and Table/Raw/query regressions. This internal presentation repair follows
continuous ADR-0127.

## Explicit Raw eligibility guard

`lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`
retains checked sources/L0/SHA-256/HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat;
inline.rs:47 is archived T317. The hash-reconfirmed inventory has 28 guards/21
owners; pinned CI requires split/body conditions. Rust structure proves no
functional/readability/upstream-intent claim.

Raw's body decides existing eligibility: !block, lang typc and auto/true/false.
Eligible observes text once then emits code/span with existing escaping;
ineligible returns the exact existing generic diagnostic without observing text.
No general Raw implementation/helper hiding conjunction/suppression/linter config.
Continuous ADR-0127 adds no public/default/compatibility/phase. L0 first/existing
functional controls unchanged; real gate RED=28 V17, GREEN=zero. Check success/
fallback/error/short-circuit/effects, scoped resealing, build/full workspace suite/
all-target Clippy warnings/unfulfilled expectations as errors/fmt/Rustdoc warnings
as errors/strict V1/V5/V15/V26/reverse lineage/SARIF no new warnings under 12 GiB,
zero swap, one job. No unmeasured memory/performance/global-parity claim.
