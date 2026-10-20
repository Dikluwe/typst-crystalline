# L0 — entities/content/traversal
Hash do Código: 3909498f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/traversal.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own bottom-up content transformation and text transformation through
`map_content` and `map_text`. Traversal remains exhaustive, static,
order-preserving, and pure. Variant-specific recursion delegates to the owning
element. Replacement nodes are not recursively reprocessed unless the
existing contract already requires it.

## Acceptance

Existing traversal tests pass unchanged and the module includes local
tests.


## Carrier traversal boundaries

map_content visits ShowRecipe.body retaining snapshot. Generic visual traversal
recurses carrier descendants without replaying a transported root occurrence:
Located transforms descendants with immediate-body-root callback suppressed,
while descendants still receive callbacks. Realized transforms visual only,
consumed original immutable; never transform origin as emitted sibling.
map_text uses only visual, preserving original/identity snapshots. Earlier
recipe-application terminal wording for prepared carriers did not forbid explicit
context/style consumers traversing descendants; this contract separates root
suppression from descendant recursion.

## Evidence and ratification

Historical measurement at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
uncommitted working tree, is preserved with UTC/diff/state/sources/binaries in
`lab/results/construction/diagnostics/math-bare-2026-10-06/{before,additional,refuters}.json.gz`
and `lab/results/construction/diagnostics/math-bare-medicao-2026-10-06.md`.
Its vanilla pin is `9f2b6e8715237cb086899a42873660fe744622e8`, not the current
upstream target. Bare frac, mat, sqrt, lr and op produce one Equation in that
vanilla snapshot and zero in Tekt; math.bold([x]) and ordinary text produce zero
in both. nested/nested-equation produce one in both; reuse produces two in
vanilla. block-style/local-block/number-style locate insertion scope;
show-generated/context-generated expose the same gaps. Explicit Equation
preserves fraction rendering; vanilla rejects it.body with
`maximum show rule depth exceeded` while accepting it; a later show affects only
the second insertion. Refuters cover a raw frac value and original label versus
explicit Equation.

`lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334` transforms
Mathy before show outside math. Historical local `compiler/layout/equation.rs:680–689`
projects bare math to text; `compiler/realize/mod.rs:155–199` prepares show/identity
without math normalization; `entities/content/traversal.rs:14–108` descends
bottom-up into math bodies; `rules/apply.rs:564–607` handles only existing
locatables. These are language semantics, morphology and render observations;
they do not require vtables, buffers, Rust equality or equal PDF bytes.

The human-confirmation gate for the math-bare package is satisfied by
`lab/history/construction/records/00_nucleo/materialization/math-bare-aprovacao-2026-10-06.json`
(manifest `149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6`),
as retained by `00_nucleo/prompts/core/compiler/realize/mod.md`. The historical
proposal-only heading is superseded. Ratification changes phase restrictions
only at the named insertion boundaries; it does not certify general parity,
D07, math labels or other debt.

### Exclusive private preorder traversal

Preserve public map_content/map_text behavior completely. Add crate-private
preorder helper with typed FnMut(&Content)->SourceResult<Option<Content>>.
Some replacement terminal: no child visit/product reprocessing. None recurses
static/exhaustive/current child order. No public signature/container catalogue
in compiler/vtable/registry/name dispatcher. Prepared Located remains one
occurrence; Realized origin never transformed in visual traversal. This owner
decides no Mathy/blockness/style/rule semantics; those remain
`00_nucleo/prompts/core/compiler/realize/bare_math.md`. Test preorder/replacement
boundary/errors/public before-after behavior including Located/Realized.

Inspection traversal.rs:30–289/Element::map_content proves existing element child
delegation bottom-up. Thus private preorder keeps its own static enumeration
inside this same owner, copying noncontent fields rather than changing public
Element methods or duplicating compiler catalogue. Legacy Dynamic trait carrier
opaque here; public map_content retains existing dynamic delegation. No certified
math-inside-Dynamic. Internal elements with private prefix/fields use existing
remapped/with_body. Ratified package requires scoped RED→GREEN/bilateral
revalidation, superseding phase limits only at named boundaries, no universal
parity/D07/math-label/debt certification.

## Ratified lexical TableCell presence

### Evidence before decision

`lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz`
and presence-refuters.json retain HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree/diffstat/UTC/binaries/argv
against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Upstream
model/table.rs:734–778 uses Smart/Option; local table_cell.rs:40–51 keeps only
value Option. x:auto/omitted collapse; fill:none/stroke:none lose presence.
Refuters: auto is accepted for x/y/fill/align/inset/breakable and rejected for
colspan/rowspan/stroke; none is accepted only for fill/stroke. These are language
observables, not required upstream representation.

### Exclusive responsibility and acceptance

Cell reconstruction preserves presence; body/text transforms set no properties.
Existing traversal/phase stays.

Require RED→GREEN for body/presence/methods/JSON,
clone/map/observation and span-anchored negatives, build/suites/lint, D08A/D05/D07
and new refuters with exact provenance. The human approved new public fields/
types on 2026-10-07; receipt
`lab/history/construction/records/00_nucleo/materialization/table-cell-presenca-aprovacao-2026-10-07.json`.
This changes no render phase/default. Position/style/padding/show realization is
the separately ratified insertion package, not implicit scope of lexical presence.


## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Exclusive prepared visual mapping

Prepared Table/Cell has one visual view: map_content/map_text/preorder transform
cells/visual, retaining raw origins/plans/properties/stage. Raw keeps prior descent;
realization hooks delimit pending Table/Cell. No domain render/Engine or source+
visual sibling emission.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



## Ratified prepared-Cell occurrence termination

### Evidence before decision

`lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz`
contains 53 probes collected 2026-10-07T14:47:56.340144+00:00 through
14:48:12.375245+00:00. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted
working tree, diffstat/status/hashes/sources/argv/cwd/environment and full outputs
are recorded. Historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`,
binary SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
Tekt `6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`.
Do not count before/refuters/complete as additional independent rounds.

isolated-default exposes vanilla x/y/inset/align/fill/breakable auto, spans 1 and
empty stroke dict; Tekt omits defaults and emits two metadata versus one.
isolated-x fails only in Tekt; raw-fields remains body-only in both and raw-x
fails in both. isolated-table-style proves Table defaults are not standalone
Cell defaults. raw-vs-prepared is false in vanilla, true in Tekt.
nested-effects/body-effects/late-rule refute once-only execution/lexical origin.
Tekt wrongly accepts fresh-isolated/table, fresh-block, fresh-styled and
multi-fresh; vanilla rejects `maximum show rule depth exceeded` at the new
constructor. identity/body/alias and fresh-conditional/hidden remain valid.
multi-identity/multi-return-table show that returning it must permit an earlier
unconsumed recipe. isolated-query remains `cell is not locatable` and location
none in both, without false Location. cell-set is separately excluded.

Upstream model/table.rs:734–778 declares defaults/properties;
typst-realize/src/lib.rs:367–386 prepares/guards arguments, :418–429 visits
products with depth, :491–505 distinguishes per-element guards, :571–586
materializes properties. Local table_cell.rs:111–143 projects only lexical or
positioned plans; realize/table_cells.rs:110–129 reapplies raw Cell unprepared;
:280–304 terminates the whole snapshot; show_rule_termination.rs:65–70 treats
equal form as fixed point; rules/apply.rs:139–166 revokes globally during callback.
Fields/effects/order/termination/causal error are language; upstream guard/storage
structure is mechanics. Infer no upstream intent. Aliases/two rules/new
generations/hidden content can refute the nominal policy.

### Ratification and exclusive responsibility

The human confirmed manifest
`3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`;
`lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-aprovacao-2026-10-07.json`
records 2026-10-07T14:53:19.127601+00:00 and materialization completed
2026-10-07T15:21:50.581533+00:00 with documented scope limits. The original public
field/type gate is satisfied by this receipt, not by the earlier Table package.
Historical candidate-only prose remains archived rather than a fresh stop gate.

Reconstruction retains standalone/stage/guards. Maps/preorder visit only prepared
visual and preserve lexical source; raw visits body. No hidden-body double walk,
preparation or recipe in entities.

### Acceptance and incomplete scope

Install only ratified candidates with language tests and valid RED before code;
reseal only changed owners. Require the collections/additional bilateral reviews,
once-only effects, equality/getters/selection, HTML product and real text/styles/
geometry, with causal messages/spans and no erased differences. Validate debug/
release builds and pertinent core/infra/shell suites; crystalline-lint without
errors and no owner V5/V6/V15/V26. Revalidate D08A/B2/B3-A, Raw defaults/blocks,
Equation/math/matrix/query. Preserve the previous tree through snapshot/restricted
diff, without commit/reset/global resealing.

The scope completes intrinsic standalone-Cell defaults and distinguishes alias
from a new show generation, not all Cell behavior. set table.cell/style defaults
remain the separately measured/specifiable Cell-style package; no deadline or
implementation promise follows from that scope-out. General Table defaults/
equality, rich casts/paint/strokes/insets, rows/gutters/multipage spans, PDF/UA
and general nonlocatable policy remain open. Do not fix/normalize the preexisting
empty-file eval warning merely to claim parity.


## Ratified Source transparency

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical9f2b6e871. Existing carrier
operations provide transparency, no observable Source. Human confirmo2026-10-07,
final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md preapproval history. Source retains span/visits body only,
no wrapper callback. map_content_pre/map_text same, no separate-origin emission.
Sole owner/static/no I/O/new signature/trait/flag/eval-layout phase;
C2/C/macro tests/recollections/build/lineage before certification.

## Change-aware generic reconstruction

Before deciding,
`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/change-aware-red-gate.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/
status/diffstat/UTC/script-fixtures-interposer-CPU-commands/release SHA
`29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`:
local-set-1600 glibc483823 exceeds pre-fixed470000 gate. traversal.rs:25–55,
235–264 rebuilds sequences/carriers/styles/Table/Cell even all callbacks None;
table.rs:85–118/table_cell.rs:308–344 clone metadata/visual contributions. Pinned
realize lib.rs:242–291,353–410 visits borrowed/materializes only changed. Cost
hypothesis, no required vtable/Rust equality under ADR-0107/0026/0109.

### Exclusive bottom-up absence proof

Keep public map_content signature/exhaustive match; internally propagate
Option<Content>: None certifies no reconstruction, Some materialized result.
Callback Some always replacement even equal; no PartialEq/address/hash inference
or double callback. Callback receives post-child node in same order, errors stop
same descent/later siblings, replacement never reprocessed.

First proof variants Source/Located/Realized/ShowRecipe/Sequence/MathSequence/
Styled/Par/SmallCaps/Table/TableCell and classified terminals. Source retains
with_source_span normalization including nested/detached origin. Located immediate
root suppression; Realized visual only. Unmigrated variants conservatively Some
through existing delegation. Table/Cell logic stays respective entity owners.
Crate-private map_content_if_changed/slice helpers allocate only on first changed
child, cloning unchanged prefix/siblings then. No public field/trait/signature/
phase. Tests reuse parents-siblings/allocation/bottom-up once/equal replacement/
error skipping later siblings/Source-Located-Realized/prepared views. Paired time/
language-render-query qualification versus old release/pinned vanilla. Initial
slice promises no all-container/map_text/map_content_pre reuse; later extension
below covers named subset only.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/content/traversal.rs:55` preserves traversal but materializes the element even when
descendants do not change. `01_core/src/entities/content/traversal.rs:111–192`
unconditionally wraps results from these owners in Some. This is mechanical
cost; language behavior and element semantic identity remain preserved.
Refutable inference: propagating explicit absence of change reduces allocations
and temporary retention. Probes and timing controls may refute that gain.

Internal refinement under ADR-0127: provide only a crate-private
map_content_if_changed helper for elements already owned by this consumer.
Return None only when no actually visited child was replaced; childless
terminals return None. Transform children once in the existing map_content
order and stop at the first error. A callback Some, even containing equal
content, always means change. Preserve every field, value, explicit presence,
omitted/none/present slot, flag, span, and metadata item. Do not use equality,
hash, or address as a certificate. Allocate vectors/rows only after the first
changed child/row; unchanged siblings retain shared content. Public signatures
and trait methods remain unchanged, with internal adaptation to this proof;
map_content_pre and map_text retain their previous contracts. This refinement
does not expand any ratification of historically proposed contracts.

Specific logic remains with this owner; Content's exhaustive match only
delegates statically. No vtable, new enum/fields, global state, I/O, or new
phase. Acceptance requires RED→GREEN for reuse, order, errors, equal-value
replacement, optional fields/slots, and metadata; core suite, builds and lint;
text/morphology/render/query qualification against the previous release and
pinned vanilla; and paired allocation/timing measurements with provenance.
Do not infer a universal improvement from isolated cases.

In this dispatcher extend proof to Block/Boxed/Strong/Emph/Equation/all current Math family, retaining Source/Located/Realized/ShowRecipe and approved rules. No element logic body moves into dispatcher.

The shared proof of absence of change follows the pinned nucleus claims.
This prompt and its sole consumer still own the algorithm, transformed fields,
visit order, and their specific preservation obligations.


## Approved typed-transform preservation

traversal.rs:906 rebuilds matrix/body; transformacoes-2026-10-14 corpus measures
lost identity/presence. Approved manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`.
Copy whole operation/transform only body, no parameter normalization/default baking/
variant reduction to Matrix. Tests maps with zero/false/auto/style-dependent values.

## Explicit Source guard

T283 traversal.rs:83 after current source/L0 reading, SHA/HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/exact
diffstat in tekt-guards-2026-10-16/before.json.gz. Hash-reconfirmed28 guards/
21 owners, pinned CI split/move condition into body. Rust structure, no functional
failure/AI-reading/upstream-intent claim.

Merge Source-mapping None arms, deciding same span conjunction in body. Keep
no-clone None when origin preservation suffices, other cases with_source_span,
callback order/count. No eager rebuild/equality/additional visit. Internal
continuous ADR-0127/no public/default/compatibility/phase. No conjunction-hiding
helper/suppression/config change. Old functional controls unchanged precode;
real RED28 V17/GREEN0. Check success/fallback/error/short-circuit/effects,
restricted owner reseal/build/full suite/all-targets Clippy warnings-unfulfilled
as errors/fmt/Rustdoc warnings as errors/strict V1/V5/V15/V26/reverse/SARIF no
new warning,12GiB/zero swap/one job. Separate measurement for resource/global
parity claim.

## Static bibliography coverage

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree.
`lab/results/construction/diagnostics/tekt-v16-bibliography-static-2026-10-17/before.json.gz`
records UTC/status/diff HEAD --stat, prior texts and 2130 checked hashes.
bibliography.rs:21–79 enumerates only some containers, omitting Heading, Repeat,
Columns and raw TableCell; :87–266 repeats the partial catalogue.
traversal.rs:337–1105 already has complete static preorder enumeration with
terminal replacement; :89–107 distinguishes Source/Located/Realized. The generic
TableCell mapper chooses standalone.visual, resolved.visual or body; visual()
has a different priority clarified below. introspect/walk.rs:1206–1268 confirms
grid/header/footer/cell bodies, Quote attribution, Bibliography title and Cite
supplement; :1396 establishes Document/Asset as separate resources. These are
internal content losses, not grounds to copy upstream equality/bytes/mechanics.

Hypothesis: ignored static contents lose keys and Ref→Cite conversion. Confirm
functional RED before code; existing preservation refutes the hypothesis, so do
not change expectations to force classification. Internal carriers use the
preceding metamorphic transparency proof. Infer no universal parity and do not
repair math/HTML walker limits here.

### Exclusive borrowed static visitor

Add only pub(crate) visit_content_static with borrowed FnMut(&Content)->bool:
false prunes subgraph, true visits static children. No clone/collection allocations
for navigation. Source/ShowRecipe/Located/Realized transparent to callback, body/
visual only, never Realized origin/captured snapshots. Exhaustive Content children
match, no terminal wildcard; future variant requires decision.

Use public mapper's navigable Content fields: simple bodies/sequences, optional
Figure/Quote/caption/footnote/bibliography, math including present slots/matrices,
HTML body, PageRun body/static marginals-supplement-background-foreground.
Raw Table children/header/footer/caption, prepared cells/caption. Cell exclusively
existing visual(), active resolved-first not generic mapping's standalone-first.
Outline title/realized_body, OutlineEntry element/fill/prefix. Document body/title
navigable only if callback does not prune. Ref/SetPage/Value/callbacks/states/
resources/Dynamic/ContextBlock terminal in static view. No Func/dyn_map_content.
Change no public maps/preorder/Element method/public type-trait-field.

Local tests order/pruning/origin transparency/prepared exclusive view/no Dynamic
effects. Structural read only, no occurrence/Location/rule. Future variant E0004
in children enumeration. Never discover containers via Debug/name strings.

### Acceptance and recorded refinements

Internal continuous ADR-0127, L0 before tests/code/restricted two-owner reseal after
V15/V26. No public field-signature/flag/default/phase/dependency/publication or
V16/V19/V20 suppression. Justify nominal terminal group as leaf/opaque data-
callback/no active static Content. Build/RED→GREEN/prior carrier controls/full
workspace/fmt/all-targets Clippy -D warnings/-D unfulfilled_lint_expectations/
strict Rustdoc-four indexes/V1/V5/V15/V26/reverse/diff --check/source freeze.
Heavy sequential MemoryMax12GiB/High10GiB/swap0/jobs1, exact state/argv/hashes/UTC.
Completes bibliography's earlier static-container debt, no global/front closure.

### Recorded RED and borrowed navigation

red-result.json/red-state.json.gz record execution from
`2026-10-17T20:02:52.979326+00:00` to `2026-10-17T20:06:42.623098+00:00`:
zero passed, two failed, no compile error and unchanged sources during measurement.
All 18 positions lose key/conversion. OutlineEntry.prefix navigation uses its
owner's new internal prefix_ref accessor because prefix() clones; change no
public interface.

The nominal 38-terminal visit group is Empty, Text, Space, SpaceFromNewline,
Parbreak, Raw, MathIdent, MathText, MathPrimes, MathAlignPoint, Linebreak, Ref,
CounterDisplay, CounterUpdate, Image, Shape, Curve, SetPage, Flush, Divider,
Asset, SmartQuote, PdfAttach, HSpace, VSpace, Pagebreak, Colbreak, GridHLine,
GridVLine, TableHLine, TableVLine, Metadata, State, StateUpdate, StateDisplay,
CounterDisplayCallback, ContextBlock, Dynamic. These have no navigable static
Content in this view. Ref keeps supplement opaque as in current map_content;
values/states are data, Dynamic/ContextBlock are not evaluated. Keep visible,
justified V19; new variants require an exhaustive-match decision.

### Selected active Cell visual correction

selected-visual-attempt records the first GREEN: five passed and coexistence
failed collection (resolved versus expected standalone). That expectation
confused generic mapper priority with active view. Re-read source:
TableCellElem::visual, table_cell.rs:261–268, chooses resolved, then standalone,
then body; finish_cell_product, realize/table_cells.rs:1065–1080, publishes
realized body in resolved when present. The entity's span-correction contract
states coexistence: standalone preserves observed properties while resolved
keeps the active plan. The mapper's standalone-priority clause is a different
operation and does not redefine visual(). The earlier attribution of that
priority to visual was incorrect.

Test resolved-only, standalone-only and coexistence separately. Active is
resolved when present, otherwise standalone; retain raw origin and inactive
carrier. Record fresh RED for Ref still in active visual, changing no old mapper
test or its contract in this scope. Conversion has a specific TableCell branch:
copy only the active carrier, convert only its visual (or raw when unprepared),
preserving fields/presence/stage/guards/props and the other carrier. Collection
uses existing visual(). Change no visual()/phase/API. Generic mappers' mismatch
with active view remains for a separate audit; the earlier fixture does not
authorize changing product behavior.

### Uniform static-visitor groups

v19-prescription-attempt/architecture-delta.json records four new visitor notes,
not only initially predicted terminal group: incomplete count prediction, not
code change/functional failure. All four already prescribed:

- Source/ShowRecipe/Located/Realized (4): unreachable children-match body because
  transparent if-let returns earlier. Test four dispatches, never execute impossible
  body for fake coverage.
- Sequence/MathSequence (2): same Arc<[Content]>, ordered children.
- Par/SmallCaps (2): same Box<Content>, body only.
- Listed38 terminals: no static descent.

Keep four notes/no suppression/body duplication. Old partial collector's
ContextBlock/Dynamic note disappears. Expected differential one V16 removed/one
old V19 removed/four new V19 (net+3), V20 unchanged. Group execution not every-
alternative proof. Prior carrier controls, public Sequence-Par probes and extra
MathSequence-SmallCaps structural tests, no claim introspection emits every math
event.

## Reconstruction stack isolation

### Preorder evidence and contract

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree;
state-stack-2026-10-17/before.json.gz/stress-candidate.json.gz retain source/
UTC/diffstat/args/limits/binary hashes. GDB debug-root.log repeated map_content_pre
frames104664 bytes apart; traversal.rs:670–1440 keeps many reconstruction locals
in same frame. Real Strong64 still aborts after concat fix; vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3` produces X. Removing artificial
Sequence nesting necessary, insufficient for legitimate depth. No Tekt-release
inference from debug.

Isolate each recursive map_content_pre arm in private static helper frame taking
typed FnOnce→SourceResult<Content>, #[inline(never)] preventing recombined locals.
No dyn/vtable/global cache/thread/stacker/unsafe/new stack config. Same exhaustive
nominal alternatives/children/order, no shallow visit/pruning/flattened wrappers/
Cell-view change. Source no extra callback. Callback preorder once, Some terminal,
None recurses/errors stop same later siblings. Keep data/spans/snapshots/slots/
prepared views/documented limits. Helper only bounds temporary native storage,
no own transform. Public methods/map_content-map_text/static visitor unchanged.
Internal continuous ADR-0127/no public API-trait-shape/phase/default.

Precode local RED RUST_MIN_STACK=2097152 only test process:64 Strong reconstruct,
order/preorder/terminal substitution-error. Same GREEN/old suite; original CLI
64×2048 and Strong64 standard stack, post-fix GDB/RAM-RSS. Strict workspace/build/
Clippy-Rustdoc/V1/V5/V15/V26/reverse/no new Tekt error/fmt-diff/source freeze under
12GiB/High8GiB/swap0/jobs1. No unlimited-depth claim: other recursion/destruction/
eq/render/real-node costs need own measurement.

### Bottom-up continuation

stress-expanded.json.gz: Strong256 vanilla rejects depth, Tekt aborts.
expanded-trace.log now map_content_inner via StrongElem::map_content_if_changed/
wrap_page_columns. Fixed preorder lets program advance; same shared-local frame
problem remains bottom-up. Do not demand upstream negative succeeds or copy
numeric limit without product decision. Existing Tekt depth policy unchanged:
require no SIGABRT, report resulting value/diagnostic independently.

Same per-recursive-arm static isolation in map_content_inner with private FnOnce
→SourceResult<Option<Content>>. Preserve None absence proof/Some even equal/
bottom-up order/transform_root/Located immediate-root suppression/Source-Realized-
ShowRecipe/Dynamic delegation. No early/repeated transform/unchanged alias rebuild/
changed child selection/equality proof. Pipeline/current limits/map_text/static
visitor unchanged. Precode local RED128 Strong/test stack2MiB: None reuses root,
equal leaf replacement rebuilds path, order/content unchanged. Same GREEN/old
regressions/CLI Strong256, upstream negative distinct from six positives. Same
continuous gate/resource requirements, no public/default change.

### Observed result and unresolved operational-limit contract

stress-both-mappers.json.gz: six positives finish standard stack; Strong256 Tekt X
versus vanilla show-depth rejection. Mappers do not own show/layout limits and
must not introduce quota to imitate abort/count. world_types.rs retains guards;
`00_nucleo/prompts/core/entities/world_types.md` was read and lacked Route/guard
specification, documentary gap for its own audit, no code-derived ratification
here. ADR-0041 records show cap64; this reading found no complete Strong256
divergence contract. No-abort proof only for slice, operational-limit divergence
recorded, not certified rejection parity/unlimited-depth safety.

## Closed storage-qualification fixture decisions

### Evidence before decision

Source inspection at HEAD `059d1a0ff` on 2026-10-19:
`01_core/src/entities/content/traversal.rs:2150–2240` constructs the bounded
container fixtures. `01_core/src/entities/content/traversal.rs:2242–2287` compares
their same-kind Arc payloads and returns false for all other pairs. The no-op and
changed-parent tests at `:2291–2343`, and unchanged-child checks at `:2414–2415`,
use this helper as a representation qualification. These are local mechanical
reuse controls, not language identity, universal Content equality, or proof of
productive workload frequency. Existing language/field/effect gates remain.

### Decision and acceptance

Keep every existing same-kind Arc::ptr_eq decision. Explicitly close both Content
axes at this exact helper. Distinguish nominal unqualified left-hand classes from
qualified left-hand classes; pairs left after qualified same-kind comparisons
are either unlike variants or have an unqualified right-hand class, and return
false. No wildcard may absorb a future variant on either axis. Do not substitute
Rust PartialEq, payload hashes, inline addresses, or generic identity for the
qualified Arc comparisons. The fixture roster and the positive match arms must
agree; omission of a fixture's input/input case refutes helper completeness.

Require every fixture's self and clone comparisons to succeed, every unlike
fixture pair to fail in both orders, and unqualified Text and Sequence clones to
remain false even when Sequence shares an Arc. Retain no-op sharing, all existing
field/slot/order/error assertions, and changed-parent/unchanged-child controls.
Add a per-fixture changed-child control: containers with seed descendants receive
new parent storage while Strong siblings retain storage; the two childless
fixtures retain storage when no callback replaces them. Never weaken a failed
assertion to make the helper agree with a rebuild.

Run these preservation controls before implementation. In a disposable real
workspace add a Content sentinel: unlike the old wildcard, the repaired helper
must fail at this decision for new/old, old/new and new/new pairs. Isolate both
axes if rustc's first witness alone does not demonstrate them. Preserve exact
source state, UTC, commands and diagnostic locations. This test-only internal
classification uses ADR-0127 continuous flow after sealing; productive traversal,
public API, pipeline phase, defaults and nucleus claims remain unchanged.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Preserve generic visual visitor boundaries and all existing transformation order. Every Figure reconstruction copies lexical kind, resolved_kind, supplement and resolved_supplement. Both maps recurse body/caption and exactly one selected supplement Content: prepared resolved Content if resolved_supplement is Some, otherwise lexical FigureSupplement::Content; keep lexical origin unchanged when prepared. Functions/spans/None/Auto are preserved and never invoked. Grid reconstructions copy source_order; maps retain their current child order, once per actual child. This is not the specialized semantic kind visitor.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.

## Borrowed nearest-origin preorder coordination

The actual nested missing-Ref error has no span after generic Block traversal:
map_content_pre's Source arm descends before callback and therefore recipes' root
Source entry cannot observe nested lexical origins. Preserve the original method's
Source-transparent callback contract. Add only crate-internal
map_content_pre_with_origin(callback(&Content, Span)); both front doors delegate
one private recursive implementation with the existing one static Content
reconstruction enumeration. A Source changes the closest proven Span passed to
its body; Source itself never reaches callback. Clone/rebuild preserves first
origin and preorder Some pruning. The old method forwards ignoring Span, with
identical callback count/order, reconstruction and opacity. No second compiler
container catalogue, public API, Nucleus change, dynamic execution or entity-to-
compiler dependency. Tests prove nested Source chooses closest span, old callbacks
stay Source-transparent/once and replacement prunes identically.
