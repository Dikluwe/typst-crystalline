# L0 — compiler/eval/markup_stream
Hash do Código: 225aaab7



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/markup-sequence-children.toml sha256:cd963a81970f86c33abcf36ffb19ec69acfd789ab528092c8abb7a95129ebd99
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/eval/pending-text.toml sha256:66cf98676c3f6ddd3641e4e67eaed2ed2b71a06d5fd9a0be93dbc3059e506074
- 00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml sha256:2213a4793474159cfae64b699263b16621a492ad2056fbaf5d28ac7950593c6b
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:a0d3cae81e77da5875204a94bc2fbd707009f0d751cf50ed870c416a01187a22
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

## Ownership and extraction evidence

At clean f7195cdf6,2026-09-30, eval/mod.rs:1351–1650,2238–2261 contains extracted
block. Boundary evidence, no performance/fresh bilateral measurement. Layer L1,
sole consumer `01_core/src/compiler/eval/markup_stream.rs`. Coordinate markup
sequence/body eval/value display with scopes/order/spans/public contract/default/
phase/compatibility/purity/static dispatch. L1/build/crystalline-lint pass;
structural split is no speedup proof.

## Lexical insertion and alias evidence

core-second.log shows local occurrence returning Original versus controls.py
oracle Local. markup_stream.rs:178–184 inserts without local rule snapshot,
which outer recipe loses on return. Realization handles inserted Content in its
own lexical environment before append_markup_sequence; occurrence protection/
completed products stay apply-owned. Outside realizing_show_rules keep route.
Preserve known spans; detached insertion errors get real expression span, no
message lookup. core-third.log shows P790 regression from generic interception
replaying text rule. Thus delegate only internal realize_inserted_occurrences to
apply owner; literal/regex/text/Par preparation stay existing path. This supersedes
initial generic intercept_content direction only at that insertion boundary.

## Ratified semantic SmartQuote markup

### Evidence before decision

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
records historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA,
HEAD/tree/exact diffstat/UTC/sources/argv/cwd/environment/stdout-stderr. PDF uses
extracted text, HTML DOM/text/diagnostics, not PDF-byte equality. Eval converts
markup quotes to Text, losing double/enabled. Stored quotes keep English glyph
when used with German/French or enabled:false.6'/2" primes become Tekt quotes.
Native show/public name were separately fixed/passing before gate; markup still
not SmartQuote. Empty constructor has empty vanilla fields; double:false known,
omitted double Unset. enabled:false remains SmartQuote. set double:false/filtered
show add positives. Literal HTML works from Text; programmatic SmartQuote rejects.
Literal title renders quotes; script takes string/rejects Content pre-export in
both.

markup_stream.rs:79–128 picks glyph in eval; layout/smartquote.rs alternates flags
without prior text; stdlib/text/smartquote.rs:112–117 erases disabled element;
observation/elements.rs:242–250 compares existing fields. Pinned markup.rs:141–146
creates explicit double; inline/collect.rs:196–211 resolves context/style;
text/smartquote.rs:119–158 separates prime/apostrophe/nesting.

### Ratification, exclusive observation and limits

Human approved2026-10-06: double public bool→Option<bool>, enabled:Option<bool>,
pure helper/context, glyph selection eval→render. Applies only to quotes,
superseding conflicting historical clauses; other ADR/nuclei/proposals intact.
L0/pins/sole owner before materialization RED→GREEN.

Eval retains SmartQuote identity/fields; each target renders glyph. Paged owns its
unit, HTML remains semantic/no paginated quote layout under ADR-0128. One pure
L1 context selector with flow/compilation-local state; no I/O/globals/vtable/
property registry/upstream memoization copy. Tekt stack mechanics distinct from
language/field presence/glyph/style acceptance. Escaping stays HTML owners.
No new mode/flag/unsafe/watch API/math phase.

SyntaxKind::SmartQuote creates Content::smartquote(kind.double) with explicit
double, no lang/quotes/adjacency/enabled-based glyph choice. Keep Source/spans/
lexical Styles snapshots/order. Apply existing structural show boundary with
causal identity/guards/capture, never present leaf as Text to text rules. Raw
Text/code strings stay literal. Aliases/recipes use existing
realize_inserted_occurrences, no double show. Contextual decisions leave this
unit; keep byte offsets needed by other observables.

## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Exclusive Table/Cell markup insertion

At markup insertion capture/realize Table-Cell in current lexical Engine through
table_cells/apply. Stored value stays raw; each alias insertion receives actual
environment, not global definition snapshot. Keep other native/text/quote/math
paths, no repeated Cell rule/hidden-body advance.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



## Bool and Auto display mapping

geometry-fifth.json/local-bool-red.log in table-cell-realizacao-d08b3-2026-10-07
retains binary59da863f/source/HEAD/state/UTC/argv/glyphs/tests. it.breakable glyphs:
vanilla inline8.8pt mono versus Tekt 11pt text. Pinned value.rs:191–208 displays
Bool via Raw(lang:typc,block:false); markup_stream.rs:239–253 uses repr Text.
Change Bool entry only to Content::raw(repr,Some(typc),false), keeping other types.
Preserve Raw interception/show snapshot without constructor identity execution.
local-bool RED zero Raw versus true/false. Internal parity/mapping/no new type/
signature/phase/field/method/flag, RED→GREEN/recollection/suites before certification;
unmeasured Value render remains open.

Standalone-Cell after-second.json.gz html-fields with recorded HEAD/state/UTC/
binaries measures Tekt HTML X auto a text versus code(data-lang:typc) and
constant.language span #d73948. Same pinned value.rs displays Auto/Bool Raw;
markup_stream.rs:210–215 covers Bool only, layout/raw.rs:201–203 true/false only,
HTML inline lacks Raw/rejects generally. Language morphology/style, equal text
is no parity proof. Precode tests refute Auto-as-Text/HTML atomic-Raw rejection.
Map only Auto→Raw typc inline, classify auto atom constant.language, HTML atom
 auto/true/false as code+constant span through existing escape/mode owners.
No public type/signature/phase/flag/default/grammar. This unit owns markup display;
Raw paged color/HTML tags/tests remain respective owners' boundary criteria.
Other-language/arbitrary Raw HTML uncertified, no general Raw implementation or
other Value mapping authorized. RED→GREEN/product-style equality/Table-Raw-query
controls.

## Ratified lexical Source and flow append

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical9f2b6e871. Existing carrier
operations provide transparency; no observable Source element. Human confirmo
2026-10-07 final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history.

Set Text-token origin after lexical interception/before ContentBlock; initial
text origin precedes block, so [a] points at a, not delimiters. Source transparent
to text groups/markers. workspace-first.log refutes wrapping every document Text:
only markup directly inside ContentBlock gets token Source, identified from
original Span/CST, never equality/name/text. Root documents retain typographic
stream. Sole owner/static/no I/O/new signature/trait/flag/eval-layout phase;
C2/C/macro tests/recollections/build/lineage before certification.

depth-recheck.json.gz retains two aborts after Source peeling in walk.
append_markup_sequence:195–204 flattens Sequence, but Source preserve_source
retains entire tree, causing one extra layer per [X#it.body] reconstruction.
Flow-mode Source(Sequence) keeps flat concat, first origin fallback per child
without overwriting known origin; other Source retained, Styled/ShowRecipe opaque.
Transport is no semantic flow-concat boundary. Later ContentBlock-value rule below
supersedes this flattening only in preserve_source value mode, not flow mode.
Rerun ten debug controls/caret-alias-spread-body/morphology/corpus, no show-limit/
stack-size change.

## Expression-produced labels

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/labels-expressoes-2026-10-14/before/results.json.gz`
retains UTC/status/diffstat/sources/owners/commands/binary hashes, historical
vanilla9f2b6e871. alias/direct/closure/block print<alvo>/COUNT0 versus literal/pin
COUNT1. Ref with é/😀 fails pre-export; orphan lacks warning. markup_stream.rs:
100–135 associates SyntaxKind::Label only, :138–140 sends Value::Label to repr-
fallback display. Pinned markup.rs:57–81 handles Value::Label in flow. Language
association/diagnostics, not mechanics.

### Exclusive association and acceptance

After expression once, Value::Label uses literal's same private retroactive
association before display/interpolated-content realization. Preserve full name/
prior content/trailing spaces/expression-origin orphan warning. Label emits no
text. Reuse intercept_labelled/guards, no target constructor/text-rule replay.
Update text snapshots/style boundaries after association. Block-local orphan
never labels outer sibling. Change no value_to_display_content/math consumer.
Tests alias/call/closure-block return/Unicode-delimiters/once-only effect/label-
text show/spaces/warnings-spans. CLI query/ref recollection and independent PDF
reader resolves destination. Internal continuous ADR-0127/no public signature-type/
flag/phase/L0 first/RED→GREEN/restricted reseal. Initial label-forwarding scope left
target selection/repeated-label replacement-warning open; selection is completed
below only, not all association or unsourced Tekt advantage.

## Markup-tail label target selection

### Evidence before decision

labels-escopos-2026-10-14/before/results.json.gz retains same HEAD/uncommitted
state/UTC/sources-owners-binaries/commands/status/diffstat/historical pin.
Direct/block/alias Parbreak COUNT0 versus1. Set/repeated-false set/show/show-set
lets Tekt label outer Heading while pin warns orphan/COUNT0. SetPage supplies
synthetic target/suppresses warning. Let/code-block local set controls have no
outer markup boundary. markup_stream.rs:195–222 skips only space/Empty, no tail
start; :156–177 style-value diff misses redundant/false set. Pinned markup.rs:
39–56 starts lexical SetRule/ShowRule tail; par.rs:725–735 Parbreak unlabellable.

Maintain private association-tail-start index after each syntactic SetRule/
ShowRule in current stream, including emitted synthetic carriers. Infer no boundary
from style difference/function name/text; inner block set makes no outer boundary.
New tail content may be labelled. Search only tail, crossing Space/SpaceFromNewline/
Parbreak/already-ignored Empty with Source transparency. Remove/reorder/reconstruct
no crossed item; preserve indices/snapshots/styles/origins/spaces/paragraphs.
Replace only target with Label and existing interception. No target: warning at
literal/expression span, entire sequence retained.

RED→GREEN direct/interpolated Parbreak, set-show-same-false tails, orphan SetPage,
new content after boundary/let-codeblock controls/message-severity-span/separators.
CLI full query/diagnostics, continuous ADR-0127/no API-type/flag/phase/restricted
reseal/full validation. Target/boundary selection completes only its named scope;
consecutive-label replacement/warning/show-effect order still separate. Never
rename after executing recipe that should have been discarded. Explicit
interpolated Empty and Heading realization under show remain measured/pending.

## Sequence realization evidence and approval

before/expanded/controls.json.gz and candidate.log in
`lab/results/construction/diagnostics/sequence-constructor-2026-10-17` fix HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree,
UTC/diffstat/sources/binaries and vanilla pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Eight initial REDs; candidate
passed five, with show/recursion unresolved. markup_stream.rs:240–260 flattens
before selecting Sequence; realize/mod.rs:170–210 visits children before recipe;
apply.rs:140–170 delegates nonlocatable elements to fixed point. Vanilla accepts
alias and rejects a new Sequence with the same children. These are language
effects/scope/values/diagnostics.

Owner approved
`lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md`,
manifest `46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327`,
on 2026-10-17. ADR-0127 §2.3 gate covers realization order of this boundary
before children/Table/math, not a new public contract.


### Exclusive Sequence markup capture

Capture lexical Sequence boundary only with active Sequence rule. Interpolated
Sequence/Empty enters complete marked recipe before flattening append. Build
recipes per show/set tail with rules/styles/guards/file snapshot, no retroactive
prefix. Interleave style boundaries preserving nesting, text normalization/slots/
indices. Capture executes no recipe. Ratified S1–S20 require RED before code,
bilateral GREEN/regressions/no reduced mechanical-limit expectation. Valid1:1/
pins before reseal; separate build/workspace/doctests/strict Clippy-Rustdoc/fmt/
direct-reverse lineage/SARIF no new error-warning.12GiB/High 8 GiB/swap0/jobs1/
heavy sequential, no publication/release.

### ContentBlock Sequence origin

r2s-scope.json.gz retains UTC/state/binaries/current pin. fresh-empty-show has
same limit/hints, but Tekt anchors inner markup[] at2:13 versus full ContentBlock
2:12 pin. markup_stream.rs:61–71 already finds ContentBlock parent for origin,
:212–219 uses markup node.span for Sequence, expression_dispatch.rs:52–54 block
span loses to earlier canonical Source. Use parent ContentBlock span for Sequence
capture, otherwise node.span, reusing capture_origin lookup. Keep individual child
spans/canonical-first Source rule, no type/phase/API change. Full[]-location/Source
RED before code, GREEN retains limit/hints/block tests.

## Completion boundaries and unresolved evidence

r2s-scope.json.gz/text-before.json.gz preserve the incomplete morphology/textual
scope after Sequence approval. Symbol cast identity/equality measured text/true
in Tekt versus symbol/false vanilla; a public variant needs its own gate, not
Sequence authorization. Markup must preserve interpolated Sequence as immediate
ContentBlock child without Sequence rule, previously flattened. Textual matching
must measure/correct cross-scope literals C vs B in its own owner. Generic show
traces for emitted children need their diagnostic owner. Historical package
names R3-Symbol/R3-Markup/R3-Textual/R3-Diagnostics identify these completion
scopes only; they are no L0 or authorization for additional code. Do not label
the differences accepted mechanics or proven parity. Later scoped ratification
does not silently certify every recorded debt.

## Consumed slots versus explicit Empty

empty-slots-before.json.gz in the Sequence diagnostic records sources/binaries/
UTC/HEAD/working tree/diffstat/current pin. Simple body yields EBODY versus BODY;
none/[] children EEA versus EA; A→AA adds Es. markup_stream.rs:205–225 builds
tail with consumed/normalized Empty slots; textual.rs:104–120 builds packets
with them; sequences.rs:114–125 visits them as real occurrences.

Within approved capture remove only naked Content::Empty slots from markup's
Sequence-tail vector and textual's pending-packet body. Explicit Empty under
Sequence rules already has capture/Source before entering vector. Keep those
carriers, empty Text and constructor child arrays. Never filter Empty arbitrarily
in domain visitor. body/[]/none/empty-constructor/none-child/[]-child/prefix/textual
corpus must match pinned PDF, including bodyless document. L0 first, module RED
before code; independently revalidate PDF/HTML.


## Packet-root and causal EOF refinement

emission-before.json.gz confirms AA in both when Sequence emits [A]/text("A")
under A→AA. Native #text("A") still yields AAAA versus AA: separate generic
nontextual replay, incorporated into S13's scalar correction below. Generic
recipes with applicable callbacks retain barriers; do not remove them to hide
realization differences.

validation-attempt-2.json.gz/trace-recipes/trace-markup2/trace-textual2 record 11
GREEN/two RED. eof-before.json.gz measures exact sources without adding newline:
Styled with EOF gives Y in both, without EOF AB in both. Unit Y fixture lacked
EOF used in prior probe; include measured EOF, preserving both external controls
without relaxing show semantics.

Textual alias without EOF yields AAAAAAAAB versus AAB. sequences.rs:112–116 skips
every textual packet root, but Content::sequence normalizes a single item to
itself. If that item is marked ShowRecipe, skipping leaves recipe pending while
capturing children again; generic owner reruns it, repeating text rules.
trace-textual2 shows marked recipe still opaque to later coalescing.

skip_root skips only artificial raw Empty/Sequence root, never normalized
ShowRecipe/Styled/other root. Effective marked root decides before children.
Preserve strict causal styles/rules/guards/file comparison and all slot controls.
Archive temporary cfg(test) traces and remove them from code before final gates.
No new field/type/API/phase.

## Redundant scalar text transport

validation-attempt-3.json.gz/emission-after.json.gz retain working tree/UTC/
sources/argv/binaries/diffstat at current pin. Sequence [A]/text("A") under
A→AA gives AAAA versus AA. Native text("A") already does so in stack-release SHA
`2ab1495d555ff4d61e67eb14bef67597f7f668a0e3bce3a8129623011443441c`
(native-before.json.gz). textual.rs:142–161 treats Sequence marker/guard as
textual-scope distinctions, retaining generic scalar recipes applying same
literals before parent. trace-textual2/instrumentation-sources record private fields.

Sequence owner supplies crate-internal textual-transport comparison excluding
only eval.sequence.occurrence from collapsed delta and guards whose RuleId belongs
to Sequence rule in snapshot. Unknown/other-family guards remain. Filter no real
style; change no external signature.

Textual owner keeps strict TextualSnapshot::same_group. Pending textual packets
with same rules Arc/file/equivalent transport realize once. Eliminate generic
nontextual redundant transport only for raw Text (Source allowed), ordered rules
with exactly parent's RuleIds, same file/equivalent transport, and no element
rule matching Text. Then no element callback remains and parent already supplies
literal/regex; return body with Source. Never remove Styled/structural/contextual
recipe or applicable-element-rule recipe. Remove no Engine rule and run no
coalescing callback. Existing same-context IDs are identity, not names/Debug/
product text.

RED→GREEN covers [A]/text("A") emission/native scalar with pin expectations;
other-family guard/real-style controls remain opaque. Revalidate prior 13
controls without relaxation. Internal ratified single-application correction,
no new public type/field/phase or other-element policy change.

## Conditional origin capture

validation-attempt-4.json.gz/complete-workspace.log/conditional-origin-before-sources.json.gz
retain HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted state/
UTC/diffstat/sources/binaries/current pin. Core suite: 7228 passed, 24 failed,
five ignored. Two local-numbering controls lose inner elements; others fail
prefix/tail geometry after 1fr. Prior validated R1 reference had zero failures.
markup_stream.rs passes content.with_source_span(child.span()) even without
Sequence rule or capturable body. Helper returns added carrier without capture;
ContentBlock append_markup_sequence preserves Source, altering legacy style/
column boundaries needing no capture. Origin addition belongs to effective capture.

Crate-internal capture_at takes raw body and real Span, adding Source only when
apply_show_rules is active, pertinent snapshot exists and body is Empty/Sequence.
capture keeps current internal signature, delegates with detached Span;
capture_scope receives explicit Span, uses it only when creating marked recipe.
Markup/CodeBlock pass raw body and real origin separately. No pertinent rule or
uncapturable body returns exact input, preserving default scopes and avoiding
useless carrier allocations. Change no layout algorithm or 24 expectations.

Full-suite RED already observed; require GREEN for f2s, set/prefix/1fr integration
and Sequence controls before full validation. Retain []/call origins, nominal
guards, nonlocation, causal coalescing. No external Rust signature/public field/
default mode/additional phase.


## Immediate children of ContentBlock values

### Evidence before decision

sequence-r3-2026-10-18/before.json.gz/before-sources.json.gz retains UTC/HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`/uncommitted tree/diffstat/status/
sources/executable SHA. Vanilla commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`,tree
`cac836c07e4dec2a89219370428292393a3216b5`,oracle SHA
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
[#([A]+[B]) C] has Tekt four children A/B/space/C versus three Sequence(A,B)/
space/C. Aliases/two siblings/later sum/for same loss, singleton matches.
Identity Sequence show preserves matching body. append_markup_sequence:286–311
always recurses Sequence even when preserve_source identifies ContentBlock.
Pinned markup.rs:78 appends whole display value, content/mod.rs:240–250 normalizes
only zero-one collection. Language morphology, not Vec/sharing mechanics.

### Current append modes and acceptance

Internal continuous ADR-0127/no public API/field-trait/phase/mode/default. Pinned
nucleus distinguishes value collection versus flow composition. Change no
Content::sequence/concat_content/alias-Source/local rules/pre-flatten capture/
Table-Cell/style-label-snapshot indices/space policy. Lost children/singleton-plus/
visual-product regression refutes hypothesis.

When preserve_source true, append whole Content without peeling Source/Sequence.
Mode already comes from original ContentBlock CST, never element name/text.
Old flow mode keeps transparent Source/recursive Sequence with prior caller capture.
Styled/other carriers stay opaque append items. Nominal Source/Sequence complement
shares push policy, preserving item not element equivalence. No wildcard/
suppression/if-let hiding V16. Forwarding had no emitted V16 baseline, no finding-
reduction credit for explicit barrier.

Before code functional children/type/immutable alias/two siblings/singleton/later+/
for/flow RED; then GREEN/CJK/show-text-label-Source/Strong64-256 recursion/full
suite/build/strict Clippy-Rustdoc/fmt/bidirectional lineage/differential SARIF.
12GiB/High 8 GiB/swap0/jobs1/heavy sequential, no publication. New None omission/
text metadata in children/Styled repr-fields/literal-regex precedence/Symbol have
separate display-none/text-transport/reflection/textual/Symbol completion scopes;
this repair certifies none. Later consumption mask below completes only its slice.

## Consumed slots and markup None presentation

### Evidence before decision

`lab/results/construction/diagnostics/sequence-r3-2026-10-18/{before,markup-after,textual-before,transport-before,transport-before-sources}.json.gz`
retains HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree/
UTC/diffstat/sources/executables and vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Under nonmatching literal Z,
nested.children exposes packet Sequence+two Empty in Tekt, original three children
in pin; empty-child packet+Empty versus []/A. markup_stream.rs:201–249 rebuilds
slots textual.rs:103–129 zeroed; matching.rs:90–95 also zeroes consumed siblings:
private transport leaks into public morphology. Interpolated CodeBlock/None
vanishes: [#none#[]A] has two children versus three; X#none;B under Empty show gives
XB versus XEB. Historical value_to_display_content omits None. Pinned value.rs:
191–210 displays None as Content::empty; markup.rs:39–56 keeps SetRule/ShowRule
separate boundaries, :78 appends other expression values. No Rust-buffer copy or
implementation-inferred intent required.

### Exclusive mask and acceptance

Internal continuous ADR-0127, no external Rust contract/new field/mode/default/
phase. Private Vec<bool>, no fake Location/public carrier. Refuters: lost genuine
Empty/empty Text, shifted style/boundary, repeated callback/inconsistent indices.
None presentation changes only markup insertion, not generic Math/CodeSetTail
helper. SetRule/ShowRule keep own boundaries.

Ordinary expression Value::None inserts Empty at real expression span before
existing captures. SetRule/ShowRule None adds no child. Exhaustive Value match,
nominal complement forwards old helper as one V19 policy, no shared-helper/
other-type coercion change. Receive textual coordinator mask; style/Sequence folds
drain body/mask together, dropping consumed slots. New product false. Final
assembly removes only residual consumed slots. Keep old naked-Empty filtering
in Sequence-capture tail; authored Empty ContentBlock retains Source, pertinent
None captured before flow.

Functional precode RED: children/fields under nonmatching literal, []/empty Text
children, literal/let/assert/conditional None, Empty show on None, empty set/style
without artificial Empty. GREEN/bilateral corpus/R2-S/EOF/slots/styles/CJK/loops,
full build/suites/strict Clippy-Rustdoc/fmt/direct-reverse lineage/differential SARIF.
12GiB/High 8 GiB/zero swap/jobs1/heavy tasks sequential, no publication.

Markup value boundaries do not close public Styled/Styles morphology, label repr,
module/include value formation. Historical R3-Reflection/A3-include completion
packages require own evidence; precedence/guards/literal show-set belong to
textual ratification below, not this mask.

transport-after.json.gz confirms consumed Empty removal/genuine None. style-tail
still exposes synthetic Sequence(A,Space) child before Styled; rule-tail keeps
different tail form. Additional to consumption mask, not repr alone. Separate
R3-TextualGroups boundary must complete public group projection/equality/selection
across scopes while preserving authored Sequence; Reflection owns public Styles/
Styled. Do not claim those controls closed or flatten Sequence generically to hide
them. This incompleteness grants no change to other projection owners without L0.


## Causal realization evidence and ratification

`lab/results/construction/diagnostics/textual-realization-2026-10-18/{before,scope,refuters}.json.gz`
retains HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working
tree, UTC/status/diffstat/sources/argv/binary hashes and vanilla pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. before-sources.json.gz contains
complete current sources/L0. No candidate code was implemented at measurement.
matching.rs:118–150 applies reverse regex then literals in declaration order;
:194–207 exposes products to later passes. textual.rs:102–123 captures full
snapshot. realize/mod.rs:177–217 visits recipes bottom-up. apply.rs:620–648/
matching.rs:218–225 call functions without capturing children.
expression_dispatch.rs:125–129 already distinguishes insertion_rules from local
state.

Measured pairs: identical literals C/B; literal/regex tie R/L; overlap X/YB;
first effect LATER/FIRST; inverse chained function BBBB/BB; separate product
AB/X; product+neighbor X/AB; Strong hiding child gives panic HIDDEN/Q;
captured global followed by new global L/R, captured local L/L; local CJK
水\n果 gives 水果/X. Pinned foundations/selector.rs:110–125 unifies escaped
literal/regex; typst-realize/src/lib.rs:1328–1405 chooses start and carries
revocation into product. These are language/effects, not ratification of Rust
mechanics/buffer equality/PDF bytes. Scope separation and earlier child visits
are inferred causes, refutable by acceptance.json, equality/fields/style/query
and error controls.

Human confirmation of manifest
`47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`
ratified the pinned textual-recipe-frontier nucleus and this order. Within this
scope it supersedes provisional ContentBlock show execution and pending-recipe
bottom-up visits. Ordinary expression evaluation order stays. Do not reevaluate
the program, advance State/Counter, bake render into morphology or create a
public Rust contract.


## Current lexical output frames

Register lexical frame on entering stream; distinguish document root versus
ContentBlock fabrication and record local show deltas per tail. Capture recipes
without provisional textual show on values. Global inheritance participates at
insertion, local delta follows only its tail, never inferred from final text diff.
Keep authored Sequence/Source/explicit None/consumed slots/#set-#show boundaries.
Complete public Styled/Styles and multi-tail synthetic-group formation remain
separate Reflection/TextualGroups scopes.

Require acceptance.json/R2-S-R3/single-evaluation RED→GREEN, separate PDF/HTML,
build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/differential
SARIF without new error-warning.12GiB/High 8 GiB/swap0/jobs1/heavy sequential.
No RAM claim without paired experiment/suppression clearing gate/commit/push/release.

Candidate3 codeblock_set_display_raw_keeps_show_rules_once loses Raw rule on
CodeBlock-converted Bool insertion: inner packet inherited Raw but realization
starts without global environment. On deferred markup value insertion capture
insertion-frame lexical delta via capture_local_output after Sequence transport.
Root frame provides locally declared rules; bindings emit nothing. Inner packet
inherits current environment without freezing obsolete global. Preserve concat/
slots/origins/local-global distinction.

## Closed display partition and introspected content transport

### Evidence before decision

`lab/results/v16-corrections-2026-10-19/batch-c/agents/effects/probes-refined.json`
retains UTC, HEAD, exact uncommitted diffstat, source/input hashes, commands and
verified oracle identity at pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
The display-located probe queries an actual labelled Heading with a Strong body:
markup insertion retains Heading/Strong, CodeBlock set-tail wraps that Heading
in Styled, and math insertion keeps Heading under Equation. The three display
catalogue probes separately show generic Raw presentation and Module body
presentation; they do not certify the current Tekt fallback.
Pinned `typst-library/src/foundations/value.rs:192–209` preserves Content and uses
Module.content. `stdlib/foundations/query.rs:73` produces Value::LocatedContent;
`markup_stream.rs:513–565` currently sends it to repr text, destroying morphology
at markup, CodeBlock-tail and math helper calls. The existing snapshot's
`value.rs:70–72` exposes its owned content without executing callbacks.

### Exclusive correction, retained debt, and acceptance

Own one exhaustive Value presentation partition. Value::Content returns its body;
Value::LocatedContent returns the introspected body's content, preserving rich
morphology instead of formatting its representation. Display reinsertion does not
transfer query identity or mutate authoritative snapshot fields. Keep Str/Text,
Bool/Auto inline typc Raw, Symbol's existing mapping, State's existing mapping,
Counter/None omission, Version Display, Float Display and Decimal Display paths.
Int and the remaining explicitly named residual families retain the existing
nonempty repr-to-Text fallback only as compatibility pending the separate rich
value-display work. That residual is a measured debt, not a supported language
partition or parity claim: generic Raw, Module, State/Counter and Symbol
morphology remain unclosed. Do not expand their Raw/HTML/render contracts here.
Remove the nested catchall by assigning each nominal Value its visible branch at
the owner decision; keep the residual group visible to V19.

Require a failing rich introspected-content test before implementation, checking
Heading/Strong morphology and body preservation independently of repr text.
Exercise actual markup, CodeBlock tail and math caller routes with a real
LocatedContent input; preserve scalar Float/Decimal formatting, Bool/Auto Raw,
None versus explicit Empty and label association at its existing markup boundary.
No callback execution or second constructor/recipe evaluation is authorized.
A new Value variant must fail exhaustiveness at this owner decision. This is
continuous ADR-0127 internal transport/parity correction, with no new public
contract, flag, default mode or phase. Full tests/paths/lineage remain mandatory;
closing the wildcard obligation does not close the named presentation debts.
