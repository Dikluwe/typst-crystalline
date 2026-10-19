# L0 — html/tests
Hash do Código: dbf5cea8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml sha256:a0d3cae81e77da5875204a94bc2fbd707009f0d751cf50ed870c416a01187a22
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

## Consumer evidence

`03_infra/src/export/html/tests.rs:13` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Regression suite ownership

Own the existing cfg(test) HTML suite and its fixtures/assertions. Cover serialization
modes, ordered DOM/attributes, void/presence errors, styled raw/title, whitespace,
explicit roots, ruby, tables/spans, protected pretty contexts and frame resolution.
Exercise public paths and private helpers through existing restricted access.
Expected behavior comes from the productive owners, not a second normative encoding
contract here. Byte assertions are encoder regressions, not universal vanilla parity.
Do not add network, layout or I/O merely to reorganize this suite.


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


RED→GREEN covers markup-like/native quotes without Paged, language after capture,
disabled/nesting/span/title/style-separated siblings/boundaries. Observe DOM text/
per-mode escaping while retaining raw/style/string rejection, headers and HtmlFrame.
Existing lang=en is no bilateral-equality baseline; unsupported other Content stays
visible, no hidden guard.

## Cell presence, realization and visual controls



### Evidence

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
diffstat/status/sources/binaries/argv/results at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`; oracle SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, Tekt SHA
`7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.
Vanilla grid/resolve.rs:204–278 resolves fields, :1500–1530 ordered conflicts,
:1665–1703 empties, :1945–1988 breakable. Tekt grid_placement.rs:91–112 prioritizes
explicit cells and :262–270 knows only GridCell; realize/mod.rs:160–200 visits
bottom-up; table.rs:127–152 unwraps without attributes. Order probes observe zero
cell effects under table replacement and no hidden-heading effects; aliases
observe LOCAL a/OUTER LOCAL a/NEW a. Unknown fields/hidden effects/wrong order
refute the language hypothesis; no cache/pagination equivalence or Rust/byte
requirement is inferred.

`table-cell-isolada-d08b3b-2026-10-07/complete.json.gz` contains 53 probes from
2026-10-07T14:47:56.340144+00:00 through 14:48:12.375245+00:00, same HEAD/dirty
tree with diffstat/status/hashes/sources/argv/cwd/environment/full outputs. Same
vanilla pin/SHA; Tekt SHA
`6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`.
Do not count before/refuters/complete as independent extra rounds. Vanilla isolated
defaults expose x/y/inset/align/fill/breakable auto, spans 1, empty stroke dict;
Tekt omits them and emits two metadata versus one. isolated-x works only upstream;
raw-fields is only body in both and raw-x rejects in both. Table style is not
isolated Cell default. raw-vs-prepared is false upstream/true Tekt; nested/body/
late-rule controls refute once-only effects/lexical origin. fresh-isolated/table/
fresh-block/fresh-styled/multi-fresh incorrectly succeed in Tekt versus
`maximum show rule depth exceeded` with the new-constructor span. identity/body/
alias/fresh-conditional/hidden remain valid; multi-identity/multi-return-table show
returning it must permit earlier unconsumed recipes. Cell query is nonlocatable
in both, location none, no fabricated location. cell-set is a separate excluded
negative.

Upstream table.rs:734–778 declares defaults/properties; realize/lib.rs:367–386
prepares/guards arguments, :418–429 depth-visits product, :491–505 distinguishes
per-element guards, :571–586 materializes properties. Tekt table_cell.rs:111–143
projects lexical/positioned plan only; realize/table_cells.rs:110–129 reapplies raw
Cell, :280–304 terminates whole snapshot; show_rule_termination.rs:65–70 treats same
form as fixed point; rules/apply.rs:139–166 globally revokes callback rule. Fields/
effects/order/termination/causal errors are language; upstream guards/materialization
structure is mechanics. Nominal choice is an inference refutable by alias/two-rule/
new-generation/hidden-content controls, not upstream intent.

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

Lexical-presence evidence in table-cell-realizacao-2026-10-07/{before.json.gz,
presence-refuters.json} retains HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/diffstat/UTC/binaries/
argv and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Upstream
model/table.rs:734–778 Smart/Option contrasts with local table_cell.rs:40–51
Option-only values collapsing x:auto/omitted and fill:none/stroke:none. Refuters
allow auto in x/y/fill/align/inset/breakable, reject it in colspan/rowspan/stroke;
none only in fill/stroke. These are language observables, no Rust-structure mandate.
Public per-field/new-type changes approved on 2026-10-07, receipt
table-cell-presenca-aprovacao-2026-10-07.json, no render-phase/default change.

Fixtures specify presence matching explicit arguments, keeping all render
observations/assertions/tolerances without fabricated known defaults. Adapt
constructors to approved raw/default fields and isolated literals standalone=None
without changing old controls. Add only behavior-owned RED→GREEN for fields/order/
effects/aliases/hidden content/visual/clone/maps/representation/HTML/actual geometry,
never implementation-mirror tests or extra productive behavior.
Lexical acceptance additionally retains body/presence/methods/JSON/clone/map/
observation/spanned rejections, suites/build/lint/exact provenance/refuters and
existing table/raw/smartquote/matrix/query collections named in the archive.



The ratified packages require only their reviewed candidates/current owning L0,
with tests real RED
before implementation, owner-only seals, bilateral collections/additional reviews,
effects once/equality/getters/selection, HTML and actual text/style/geometry,
unaltered messages/spans. Build debug/release and appropriate core/infra/shell
suites, lint without errors/owner V5/V6/V15/V26; preserve prior tree by snapshot/
restricted diff, no commit/reset/global resealing. Revalidate prior table lexical/
realization, Raw defaults/blocks, Equation/math/matrix/query controls identified in
the archived contract. Do not use byte equality or whitespace normalization to
close render, change legacy extra casts/syntax/flags/global nonlocatable policy.

The insertion package was explicitly incomplete for isolated intrinsic defaults
(raw-cell-no-placement/outside-fields) and fresh show-cell-reconstruct generation;
these are completed only by the separate isolated-default approval. That package
completes only intrinsic defaults and
alias/new-show-generation distinction; cell-set/style defaults require a third
measured L0/gate with no promised schedule/implementation. Generic Table defaults/
equality/fields, rich function styles/paints/strokes/insets/lines/gutters,
multipage spans/PDF-UA/global nonlocatable policy remain open. Do not correct or
normalize the existing empty-file eval warning to assert parity. Historical
regression collections retain table lexical/raw/smartquote/matrix/query obligations
without claiming those earlier runs certify the new upstream pin. Full original:
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/export/html/table.md`.
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
