# L0 — introspect/walk
Hash do Código: 84685146

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


## Ownership and evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/introspect/walk.rs`. At clean state
`df72a6c4a701917ab56a7c6169764a92ea775693`,
`01_core/src/compiler/introspect.rs:1897–2485` contains the transferred behavior.
Source/state are recorded in
`lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`.
Separate behavior without API change or performance claim.

## Causal walk and identity contract

Traverse Content with exhaustive static match. Allocate Location, capture
payload/snapshot before Start, descend once according to each variant, and emit
End at the same Location after recursion. Delegate Heading/Label to their
existing modules. Execute no callbacks or I/O. Language invariants, causal order
and parity boundaries remain in
`00_nucleo/prompts/core/compiler/introspect.md`; create no second interpretation.
Use static L1 calls, no vtable/I/O/mutable global state. Preserve introspection
and L1 suites, build and crystalline-lint with coherent hashes. Future semantic
changes need language measurement and L0 before code.

Located registers at its explicit Location, retaining body snapshot. Realized
visits origin then visual product; aliases of registered occurrences do not
duplicate tags/counters, subject to the consumed-Table refinement below. Pending
ShowRecipe projects body for initial introspection. Locator observes explicit
Locations to avoid ordinary-expansion collisions. Never deduplicate by text,
morphology or address. Origin registration accepts a Location prepared before
the recipe; unprepared occurrences retain ordinary allocation. Exhaustive
carrier dispatch precedes native-element traversal; do not duplicate carrier
logic in native arms.

### Ordering evidence and reservation

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
CounterRegistry::value_at/StateRegistry::value_at compare Location.as_u128;
before/after queries and counter events do likewise. Realization prepares
identity before transformation, so the integer no longer orders origin/product.
The CLI probe and single_evaluation_counter_follows_causal_order sentinel retain
evidence in the implementation diagnostic. Apply the pinned realization-order
nucleus; retain historical fallback only without recorded order.

Record every Tag::Start's causal ordinal before populating registries/temporal
history. Explicit identity is not ordinal. Reserve explicit Locations before
allocating ordinary occurrences introduced by contextual/outline expansion;
reservation emits no tag or causal-order change. Reservation is a pure internal
service reused by layout for allocation synchronized with walk; do not duplicate
its implementation.

## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Exclusive prepared-Table traversal

Resolved Table visits the complete plan; Complete Cell visits visual. Do not
index hidden raw body or register Cell as locatable. Table/products retain
Located/causal order; generated Heading becomes queryable, hidden Heading does
not. Padding metadata occurs once in row-major order, not per physical repeat.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



### Measured indexed-view refinement

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{after-fourth,refuters-fourth}.json.gz`,
nested-reuse-red.log and effect-order-red.json retain sources/binaries/HEAD/
state/UTC/results. after-fourth/resolved-rowspan and rowspan-fixed observe b,a,c
index order although origin plan is a,b,c. effect-order-red.json measures a's
SPAN panic first in both: recipes remain row-major; metadata index follows
rowspan end (end-row,col). Vanilla grid/layouter.rs:1755–1787 emits deferred
rowspans. Only the prepared indexed view sorts by (y+rowspan-1,x); plan and HTML
remain row-major. refuters-fourth/reused-prepared-table measures missing metadata:
walk.rs:88–104 skips Located Table when original occupied its Location.

Register consumed Table origin shallowly, preserving payload/counter without
visiting hidden caption/children. Emitted Table aliases at registered Location
still visit prepared visual; children's Location guards prevent duplicate
contributions. Do not generalize to other elements; Cell remains nonlocatable.
This supersedes only the measured projection/guard, no new public contract or
phase. Refuters are differing tests/recollections, repeated effects, visible
hidden source or cache/geometry regressions.

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

Index only effectively emitted visual of standalone prepared Cell, never lexical
body in parallel; Cell remains nonlocatable. Positioned Cell keeps existing
logical-plan order. Emitted recipe/body metadata occurs once in current causal
context.

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

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`
records HEAD/working tree/UTC/sources/argv/binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized
branches provide corresponding transparent operations; Source must not leak as
an observable element. Human `confirmo`, 2026-10-07, is in final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
its nested contract.md is the preapproval candidate.

Walk Source body once under the same chain/label without its own tag; Location
reservation also visits body once. Keep exclusive ownership/static dispatch;
no I/O/new signature/trait/flag/eval↔layout phase. Require C2/C/macro tests,
collections, builds and lineage before certification.

## Borrowed Location reservation

### Evidence before decision

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks-deep.json.gz`
records binary/HEAD/working tree/command/sources/UTC and stacks sampled every
64 allocations. local-set-1600 has 450 TableElem::map_content samples under
reserve_locations during layout, versus 151 without set. walk.rs:27–36 uses
map_content for inspection and discards its product. reserve-red-gate.json.gz
records 1,029,471 allocation requests with prior-binary provenance. Refutable
inference: borrowing common variants removes copies without changing reserved
Locations; allocation gate and paired output/time comparison test this.

### Exclusive reservation and acceptance

Directly visit Source, Located, Realized, ShowRecipe, Styled, Sequence, Table,
TableCell along the same children as map_content. Resolved Table visits cells/
caption; raw Table visits children/header/footer/caption. TableCell visits
standalone/resolved visual when present, otherwise body. Realized visits visual
and origin; Located observes its Location after body. Other variants retain
map_content visitor fallback, including recursive Realized origin traversal.
Create no Content/API; change no causal emission/index order, Locator/phase/
default. Verify largest nested explicit Location stays reserved, language outputs/
errors equivalent, measured copy/allocation reduction and paired time comparison.
ADR-0127 continuous flow.

## Measured arity boundary and nominal lint exception

### Structural evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
captured at `2026-10-15T16:39:18.873477+00:00`. Full sources, prior L0,
signatures/hashes/UTC/status/git diff HEAD --stat are in
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
Individual classification and owner clauses read before deciding are in
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Clippy spans were reconfirmed against inventory sources. This measures Rust
structure, not fresh parity or a functional defect: C007 at
`01_core/src/compiler/introspect/walk.rs:91`, walk, has eight arguments; C008
at :121, walk_at, has ten; both exceed threshold seven.

### Exclusive prescription and scope

walk's pure traversal receives mutable compilation accumulators and lexical
context by borrowing. Auto-label counter, Locator, tags and introspector have
distinct roles, with no global state or new decision owner. walk_at adds
forced_location/shallow_table as branch-local causal data; do not persist them
globally or merge them into a semantic snapshot. Preserve signatures/order/
semantics. Exact machine metadata:

- `#[expect(clippy::too_many_arguments, reason="C007: Travessia pura recebe os acumuladores mutáveis da compilação e o contexto lexical por empréstimo")]`
- `#[expect(clippy::too_many_arguments, reason="C008: A recursão adiciona forced_location e shallow_table à mesma travessia")]`

These Portuguese reasons are literal machine metadata.

The initial audit prescribed an internal maintenance target and did not
materialize it. Preserve current public signatures and this owner's language
decisions; the maintenance judgment is supported by the stated evidence, while
arity alone proves no functional defect. Confirm the warning with workspace/
all-targets Clippy and no `unfulfilled_lint_expectations` after annotation;
preserve owner calls and functional controls. Do not raise the global threshold
or silence other lints.

Only the named function may carry this exact lint expectation. Never apply it
to a module/impl/crate/global configuration. A residual exception for dead-
argument corrections depends on that correction. Remove/review the expectation
if arity falls below threshold; never suppress unfulfilled expectations.
This specific permission supersedes generic suppression prohibitions only for
the justified nominal item/lint. All other corrections remain required.

The original audit updated only L0 and its lineage seal; code attributes and
signature/body changes remained pending materialization. Corrections require
owner language/integration tests and before/after call controls, without relaxed
fixtures/tolerances. Preserve layers/phase/defaults/static dispatch and no mutable
global state. Private signature corrections also migrate callers and their L0
before code. New public/compatibility requirements need a concrete ADR-0127
proposal and confirmation; this exception grants neither.

### Materialization evidence and acceptance

The validated inventory
`lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json`
retains 79 warnings. Source/span/current-reason/signature/hash reconfirmation is
in `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`,
with HEAD, uncommitted working tree, UTC and git diff HEAD --stat. Concrete
functions/data were read before deciding. Arity/type heuristics are Rust
mechanics, not new parity or measured cognitive benefit.

The 2026-10-16 materialization covers only C007/C008's justified direct expectations,
reconfirmed against current spans and individual clauses. Use the nominal
function attribute, preserving signature/body/calls/cfg/unrelated attributes.
Correction entries, including conditional C047/C066, remain pending and receive
no annotation in that batch.

ADR-0127 continuous flow adds lint metadata only: no public contract, executable
body, cfg/default/phase/dependency/architecture change. Only explicit nominal
exceptions supersede generic suppression prohibitions. L0 precedes code,
restricted resealing and proof that removing added attributes plus lineage
header recovers the prior source after rustfmt normalization. Measure RED first;
then check all-targets inventory: exactly 46 arities and five test warnings
satisfied by 49 expectations, no new warning, unfulfilled expectations as errors.
The 11 arities requiring correction remain visible. Validate build/fmt/strict-
and reverse-lineage/Tekt without new errors or warnings, existing Counter tests
and coverage under 12 GiB/zero swap/one job. The earlier full suite is evidence
for the same executable code; add no tests mirroring lint attributes and require
no fresh performance comparison for metadata.


## Ratified compact carrier emission

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/before.json.gz`
retains UTC/sources/L0/hashes/status/git diff HEAD --stat. Existing owners were
read before proposing. probe-details.json retains rustc 1.92/argv/rlib/binary/
probe SHA-256/output; probe-result.json records 12 GiB/zero swap/one job/events.
This synthetic carrier/register-copy probe is not compile/layout/RSS benchmarking
or a real workload distribution. Candidates mirror real domain types; inline
uses productive Tag/ShowRule.

Tag is 416 bytes, boxed/Arc Start 48. For 4,096 Start/End pairs with heap-free
Heading, requested bytes/allocations are inline 3,407,872/1, Box 2,031,616/4,097,
Arc 2,097,152/4,097. Cloning boxed Tag allocates per Start; inspected productive
fixpoint.rs:86–136 and entrypoints.rs:44–66 use temporary Vec<Tag>/borrowed reads,
without cloning the vector. Walk:325–356 populates the index by reference before
moving info to Start. Synthetic clone frequency is not assumed productive.

Transformation is 328 and ShowRule 376 bytes; Box/Arc<Styles> candidates are 120
and 168. rules/show.rs:464–470 copies to_vec before every registration;
apply.rs:108–112,349–352 filters/clones rules. Across 128 registrations of simple
100% show-set without retained old snapshots, requested bytes/allocations are
inline 12,276,960/384, Box 8,194,560/8,640, Arc 5,530,624/512. With retained
snapshots, requested peaks are 3,203,856, 4,141,744 and 1,477,808. A custom-channel
control copying 4,096 rules requests 4,107 inline, 8,203 Box and 11 Arc
allocations, including Vec growth. apply/intercept/outline/table_cells borrow
styles; application still clones a delta when needed.

Mandatory counterexample: building one simple 100% show-set collection of 4,096
rules requests 1,540,096 bytes inline, 2,031,616 Box and 2,097,152 Arc. Arc may
retain excess custom-channel capacity; the rich 128-registration control without
old snapshots ends with 78,064 live bytes inline and 156,432 Arc. Claim no
universal gain or ignore atomic/indirection cost. This is ADR-0030/0107 storage
mechanics, not language parity or measured AI benefit. Real relevant memory/time
regression refutes the favorable copy-cycle inference; retain those controls.

### Ratification and exclusive responsibility

The human approved manifest
`282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`
records approval. The former ADR-0127 public-payload/compatibility proposal gate
is satisfied; this contract, including T1–T6 and no release, is in force.

Migrate only existing Tag::Start(loc, info) emission to
Tag::Start(loc, Box::new(info)). Preserve info construction/values, index
population before moving info, loc/End/hash, walk order and effects/counters.
There is one allocation per Start; do not clone info or move population, change
deduplication/reservation/snapshot/recursion. Public form belongs to
`00_nucleo/prompts/core/entities/tag.md`; this owner preserves its own behavior.

### Acceptance and owner boundaries

Ratified L0 precedes tests and real RED; reseal strictly changed owners. No
large_enum_variant suppression, threshold/dependency/flag/default/phase change.

- T1: construction Box<ElementInfo>/Arc<Styles> and relative budgets (Tag at most
  three Location; Transformation at most Content plus alignment) fail before
  and pass after without freezing a universal ABI. Those payloads stay owned by
  their entity prompts.
- T2: preserve Start/End/Location/payload/label, value equality/structural hashes,
  independent Box clone and mutation isolation; hashes are not persistence format.
- T3: existing introspection/convergence/nesting/heading/label/state/counter/query/
  metadata tests retain expectations/errors.
- T4: Transformation::Style clone shares Arc; Arc::make_mut on a copy preserves
  original styles including custom; Func/Content/Str variants remain unchanged.
  This is a dependent entity boundary, not implementation in convergence.
- T5: show-set/func, precedence/scope/labels/outline/table.cell/textual/regex
  controls preserve semantics/morphology/errors with no extra execution.
- T6: full workspace/build/all-targets Clippy with prior expectations/lints as
  errors, rustdoc -D warnings with four indexes, fmt/diff/strict/reverse lineage
  and no Tekt increase, under 12 GiB/zero swap/one job. Record HEAD/working tree/
  hashes/diffstat/UTC and distinguish requested sizes/bytes from RSS.

No release. Any signature/field/phase change beyond these ratified payloads
requires a separate proposal. This owner preserves its existing emission behavior apart from the ratified boxing.


## OutlineEntry alignment and exhaustive walk

### Later OutlineEntry evidence and supersession

`lab/results/construction/diagnostics/tekt-v16-introspect-walk-2026-10-16/before.json.gz`
retains the same HEAD/uncommitted state, UTC/exact diffstat/source hashes/binaries.
measurement-before.json/measurement-fields-before.json at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`: direct entry appears once, synthesized
once, mixed twice; Tekt direct query loses it (0 vs1), mixed loses direct (1 vs2),
and direct body() errors `cannot outline heading`. outline_entry.rs:61 accepts
only raw Heading in body; :99 supplies NativeElement; extract_payload.rs:148
drops OutlineEntry; walk.rs:805 treats it terminal; outline_entries.rs:38–61
synthesizes post-walk. map_content visits element/fill (traversal.rs:221), used
by realized.rs:139–281 to certify origins: test a Located referenced Heading
without confusing it with a rendered descendant. Pinned outline.rs:472–481
declares OutlineEntry Locatable. This measures language semantics/morphology/
errors, not Rust/byte/PDF mechanics. Wrong query/fields/body, phantom headings,
duplicates or synthesis/order/label regressions refute the correction.

The later decision supersedes only this owner's earlier pending OutlineEntry
alignment caveat. Historical investigation remains in the archived original;
Dynamic still depends on its own kind/payload boundary. No universal proof of
all variants follows from scoped alignment.

### Exclusive nominal barriers and direct entries

Close only the six W1–W6 wildcards listed in scope.json with flat nominal Content
complements. Preserve {}, false/None bodies, recursion/effects/early returns/
signatures/short-circuit/order. W6 is the old_reserve regression oracle, not
production; close it too. No semantic Table-rule change, except the later
measured transparent-root repair below. OutlineEntry remains terminal: element/
fill are semantic fields, not children to index as Heading or execute effects.
Extractor's own NativeElement path registers direct entry, native query and
labels; invent no payload/walker table. Existing post-walk synthesis remains,
with direct/generated/mixed identity/count controls preventing duplication.
No phase/API migration. This supersedes only earlier pending OutlineEntry caveats.

## Acceptance and limits

Source controls cover producer/nonproducer wrappers, Located/Source, no duplicate
collection, fallbacks/predicates. Confirm transparency RED before implementation;
existing preservation/single_evaluation_ controls pass before/after. Real workspace
copies with separate Value/Content sentinels retain evidence of absent E0004
barriers before and require E0004 at six selected points after, with sources/
other errors recorded. Restricted resealing only. Validate build/full workspace,
all-targets Clippy warnings/unfulfilled expectations as errors, fmt, strict
Rustdoc, V1/V5/V15/V26/reverse and SARIF removing only selected V16, allowing only
prescribed equivalent V19. Uniform groups share bodies without duplication/
suppression; structural barriers are not proof of execution of every alternative
or a defect in every fallback.

For OutlineEntry, L0 first and local tests/real functional RED before code.
Preserve old controls/CLI measurement; repeat direct/synthesized/mixed body()/field
fixtures at language level. Separate copied Content sentinel must add E0004 at
W1–W6 after, not before, without other errors/stubs. Keep logs/inputs. Reseal only
five affected pairs after valid V15/V26. Require workspace build/fmt/all-targets
Clippy -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings, full
workspace suite, strict/reverse lineage, diff and differential SARIF: six V16
removed (239→233), six new informational V19; extractor complement loses one
alternative and two needs_fresh_index groups move OutlineEntry false→true.
No new V20 or other diagnostic change after line mapping. V19 counts alternatives,
not execution coverage. No suppression/global configuration change. Under
12 GiB/zero swap/one job, heavy commands sequentially; record state/UTC/hashes.
ADR-0127 internal parity flow adds no API/field/trait/default/phase/compatibility.
No publication/commit/push. Other V16/general objectives remain open.

### Source traversal-context refinement

source-carrier-before.json.gz preserves first implementation and alias RED:
native query returns Locations 1/2 instead of explicit 0. Seven controls pass,
one fails in attempt-source-carrier-validation-controls.log. Source's walk_at
branch called walk, resetting forced_location=None/shallow_table=false: lost
causal-wrapper context, not two language occurrences.

Delegate Source body through walk_at with the same accumulators/chain/label and
received forced_location/shallow_table. Observe/allocate no Location for wrapper;
change no other carrier/signature. Before repair add Source(Table) shallow/
forced-Location control: one Table Start/End, no body Heading. Alias
Located(Source(entry)) and shallow control pass after; retain reservations and
W1–W6 bodies. This restores the pinned origin nucleus in the same phase, no
public/default change. Initial seven-failure RED predates corrections; this
refinement keeps its independent RED without rewriting original evidence.

## Direct-entry kind indexing and synthesized causal order

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
order-before.json.gz/before.json.gz retain state/UTC/hashes/diffstat.
trace-runtime-result.json/trace-runtime.log show direct entry in pure/runtime/L3
NativeElement index; bare outline.entry becomes Kind(OutlineEntry), per
foundations/selector.rs:113. Terminal walk did not fill that kind.
measurement-order-before.json at historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8` records BARE [Heading] versus
[Heading,Direct], FILTERED [Direct,Heading] versus [Heading,Direct], STATE 2
versus1 and COUNTER ((2,),) versus ((1,),) at synthesized entry Location.
These are language observables, not required Rust/byte equality. Post-walk
synthesis allocates after updates although causal position lies inside earlier
outline. realization-order already forbids integer identity as order. Wrong
order/temporal prefix, duplication or entry data refutes correction.

### Exclusive responsibility and acceptance

In terminal OutlineEntry branch register emitted_loc in OutlineEntry kind_index
when present. Payload/Start/End remain existing native path; duplicate no action/
tag and visit no element/fill. Preserve own identity/prepared aliases. Cover
Kind and NativeElement queries without changing language selector to evade index.

Before change test bare own/alias entry indexing, three-entry order (two outlines
with direct entry between), where/no phantom Heading, and State/counter prefix
at synthesized Location. Then contract-test two internal helpers before their
implementation: unknown anchor, duplicates/repetition, nonmonotonic IDs, dense
append. Repeat nine CLI probes without relaxed outcomes (six original plus
order/state/counter). Require full suite/build/fmt/strict Clippy/Rustdoc,
V1/V5/V15/V26/reverse/diff and SARIF: six V16 removed, seven expected V19,
three existing messages updated, no new V20. Use 12 GiB/zero swap/one job,
heavy tasks sequential. Internal/parity ADR-0127, crate-private helpers and
nine L0/consumer pairs; no external signature/deliberate default/new phase.
Restricted reseal, no commit/push/release. Other V16 debt remains open.

## Transparent consumed-Table root

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree.
`lab/results/construction/diagnostics/tekt-v16-eval-math-2026-10-17/before.json.gz`
retains UTC/sources/L0/hashes/config/status/exact git diff HEAD --stat. Pinned
parser AST is in contexts.json/all-contexts.json/wildcard-inventory.json;
current definitions by hash. Prior suite passed the same executable state,
verified by hashes before deciding. Infer neither upstream intent nor AI benefit
from lint.

source-table-before.json records external probe linked to real lib via public
introspect: Realized(Table(Heading hidden), Text replacement) returns zero
Headings; Source-wrapped origin returns one hidden Heading. source-origin
requires transparency/single contribution; consumed Table's shallow contract
forbids hidden children/caption indexing. Observable difference is query(heading),
not Rust equality. Six earlier tekt-v16-eval-helpers CLI probes passed and were
insufficient to refute this public-contract carrier defect. Causal inference:
table_root recurses Located/ShowRecipe but classifies Source false, selecting
deep walk. Refuters: wrong index, lost visual, identity/order/counter or duplicate
effects after normalization. First external attempt used crate-private helper
and failed E0624: probe setup error, never functional RED. Later public
construction measures defect with recipe/rlib/binary/output/UTC preserved.

### Exclusive repair and acceptance

In Realized's local table_root helper replace only `match c` with
`match c.source_body()`. Borrow-unwrap Source before classifying Table/Located/
ShowRecipe; preserve all nominal arms/bodies/order. Do not classify every Source
as Table, remove stored carrier, move phase, traverse hidden semantic bodies,
change reservation/forced_location/shallow_table/own tag/signatures/payloads/
queries/synthesis or other elements. Source(Heading) remains non-Table with
original/visual traversal. This supersedes earlier table_root exclusion only
for the existing transparency repair, no public/default change.

Before repair add four pure public-introspection tests: consumed raw Table keeps
own tag/no hidden Heading; single/nested Source in both Located orders and inside
ShowRecipe keeps one Table/no hidden Heading; visual Heading stays queryable and
explicit Table Location stays; non-Table Source(Heading) keeps origin/visual.
Real RED before scrutinee change, GREEN after without changed expectations.
Preserve prior Source-context/other controls. Private same-phase traversal
selection restores ratified contract: continuous ADR-0127. Reseal only two pairs
after V15/V26 valid. Repeat public probe with 0/0, six CLI probes and workspace/
Clippy/Rustdoc/build/fmt/lineage/reverse/SARIF gates. No new V20/diagnostic;
12 GiB/zero swap/one job, no publication.

### ShowRecipe context refinement

attempt-recipe/validation-controls.log records four controls passing, one failing.
Source single/nested and both Located orders stop publishing hidden; ShowRecipe
(Source(Table)) still publishes hidden Heading. Earlier root diagnosis omitted
ShowRecipe context: walk_at called walk, resetting forced_location=None and
shallow_table=false. Preserve pre-refinement state/UTC/hashes/failure and original
RED. Before code add recipe-plain to carrier matrix and
Located(ShowRecipe(Source(Table))) with visual Heading: Table keeps explicit
Location/uniqueness/no hidden Heading, shown visual queryable once. Confirm
additional RED without weakening controls.

Replace only ShowRecipe delegation from walk to walk_at, passing same
accumulators/lang/chain/label_from_parent/received forced_location/shallow_table.
Observe/allocate no carrier Location, run no callback/recipe, do not substitute
captured styles for chain or change reservation/signature. Body projects once
in same phase. Private prepared-identity context repair matches existing Source
rule; no public API/default/compatibility change. This complements table_root's
c.source_body, not replaces it.

Replay permits these two private expressions/new controls only. Six selected
tests must pass: five from this owner, one eval owner. Workspace suite grows
by six from entry state, preserving old cases and seven ignored tests. Retain
build/fmt/Clippy/Rustdoc/lineage/reverse/SARIF and 12 GiB.

## Heading callback evidence and responsibility

The 2026-10-17 evidence records HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c` and working-tree/UTC/hash provenance
in `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz`.
`before-pdf-probe.json.gz` records `heading-supplement-panic` succeeding in Tekt
and producing `A1_HEADING` in vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Both accept
`heading-supplement-valid`, but only vanilla uses `Custom` in the reference.
Upstream `model/heading.rs:249–289` synthesizes supplement per occurrence;
`model/reference.rs:393–415` applies the function and display.
`Set.rs:286–320` ignores supplement and clears numbering;
`snapshot_fields` at `heading.rs:55–68` fixes translation. These are observable
callbacks/values, not required upstream mechanics.

The initial custom-supplement capture proposed a temporary Styled with collapsed
causal delta in canonical content, retaining fields, to restore callback styles
before removing that wrapper on publication. It applied only to custom occurrences
and changed no original tree/tags/Locations; walk remained Engine-free. The later
ratified before-query contract below supersedes this transport mechanism because
raw-constructor func/query observed the temporary wrapper. Do not retain that
obsolete competing mechanism.

ADR-0127 internal parity flow adds no public API/field or eval/layout transfer.
Existing post-walk Engine owns callback application; L0 first, RED/bilateral
controls, no encoder/walk/layout callbacks or universal-field parity claim.

## Ratified Heading finalization before queries

### Evidence and approval

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
`final-freeze.json.gz` and `target-controls.json.gz` in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
record UTC/diffstat/sources/argv. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`.
`heading-raw-constructor-func` passes on baseline/vanilla and fails on the
candidate because transport `Styled` changes `func`. Query also observes a
temporary causal wrapper; HTML ignores a panic callback. Sources:
`walk.rs:930–945`, `pipeline/paged.rs:419–427`, `pipeline/html.rs:101–106`
prepare queries through the pure walk. These are language values/errors, not a
requirement to copy vanilla's structure or algorithm. The owner confirmed A1-H2
on 2026-10-17 in
`lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`,
SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

### Exclusive snapshot capture and acceptance

Keep semantic Content intact in canonical entry; never create Styled to transport
Heading causality. For custom heading.supplement attach causal styles to the
existing snapshot's private header. Preserve full map/order/label-last/Location;
no second store or walk callback. Pure entrypoint projects Heading as Heading.

Require H1–H8 in the ratified review, with bilateral RED before code. No public
contract beyond the Heading facade is authorized by H2; A1-S2 separately covers
five State helpers. No new public fields. Ordinary `Styled` and other callbacks
retain their boundaries. Require workspace/doctests/build/Clippy/Rustdoc, fmt,
V1/V5/V15/V26/reverse lineage and no V16 increase; use
12GiB/High 8 GiB/swap0/jobs1 with heavy commands sequential. RAM gain requires
measurement, not intention.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Effective Figure is_counted is numbering-present independently of caption, including Func and captionless occurrences; None disables. Preserve one action per prepared Location, Source/Located/Realized original identity and existing semantic indexes. Tag presence never creates implicit demand; discovery belongs to actual-visual figure_demands.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


### Effective Figure style carrier identity

Current `walk.rs:1338` delegates Styled to walk, clearing forced_location and shallow_table despite Source/ShowRecipe already preserving those traversal contexts. Approved final preparation transports matched show-set numbering inside Located(Styled(Figure)), so that reset would allocate a second identity and break alias/action conservation. Before code, require carrier identity regression. Delegate Styled body with pushed chain through walk_at and the received forced_location/shallow_table, preserving all accumulators, outline-prefix recording, label and order. No wrapper Location/action, new phase or shadow state.


### Static running Figure actions

`figure-kind-implementation/target-controls-oracle/receipt.json` freezes the current-pin header/body control: rendered header Figure 1, body Figure 2, global final (2,). Current PageRun walk visits only body while actual Paged layout renders static header/footer/background/foreground. Index static Content in observed causal order background → header → body → footer → foreground in the same walk and effective chain, preserving explicit Locations and one action per occurrence. Never execute Func marginals, recurse page supplement metadata, invent actions for hidden originals or add duplicate per-page source actions. Running Content is indexed once under its prepared identity; actual physical repeats remain the marginal owner's mechanics. This is required approved Figure action/demand coherence, not a new Grid phase or blanket Page metadata traversal.

Later `figure-kind-implementation/marginal-order-oracle/receipt.json` refines the header control: background/header precede body; footer/foreground follow it. Pinned pages/finalize.rs:56–72 agrees. Preserve that exact action and demand ordering, not merely the same final scalar. Static one-page controls certify this scope; repeated-page and dynamic marginal realization retain their separately visible boundaries.


### Hidden ancestor Figure eligibility

Actual RED in integration-tests-4 and current-pin ownership-oracle-2 require no Figure query/action membership for an unprepared descendant behind an ancestor replacement, while an own-hidden prepared Figure remains indexed once. Scope a private candidate original_walk_depth around Realized.original traversal, restoring it before visual traversal. At depth>0, return without indexing/recursing only a Figure with resolved_kind None. Keep prepared originals, other original element behavior, synthetic standalone Figures at depth0, existing Table shallow policy, pure walk signatures and all action/Location order unchanged. The transient depth is zero after completed walk; existing helper recursion carries the shared candidate, avoiding broad signature/module changes. No ignored warning, shadow action or global mutable state.


### Emitted prepared caption supplement child

`figure-kind-implementation/candidate-replay-1/receipt.json` records four required-closure callback-metadata cases returning [] instead of table/sequence/heading/first-body values. The callback result is rich Content and layout/figure emits it between body and caption; the old walker/collector omitted that actual visual child. For a prepared Figure with a present caption and a currently emitted numeric prefix (default or Str numbering), traverse resolved_supplement Content after body and before caption in canonical walk and demand discovery. Do not traverse lexical aliases or invoke callbacks. No-caption, numbering None and the explicit Func visual-prefix debt emit no supplement child and retain no traversal/demand from it. The Figure's own preparation callback/error semantics are unchanged. Preserve exact typed keys, occurrence Locations and selected-content alias conservation; no new Ref synthesis or physical SetPage admission phase is authorized.


## Approved reference contribution integration

Measured existing boundary before decision: `01_core/src/compiler/introspect/walk.rs:356 and :1051` retains the old generic transport or legacy assertion behavior. Source inspection establishes ownership; the independently frozen actual L3 INNER_PREPARATION RED establishes the language omission.

The human approved frozen proposal SHA1578cbea7378b739a7c1b0809d3437c0165c2de6d288e40b2db75c9ce7e38ff2, recorded in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/reference-phase-approval.json`, including the separate no-child clarification. Actual L3 explicit/callback INNER_PREPARATION regressions failed on the unchanged pre-implementation source (`reference-phase-red-frozen`), authorizing RED-to-GREEN for this exact phase. Existing unrelated obligations remain.

Index the prepared Figure original root only (through Source/Located/Styled wrappers), then traverse actual visual Figure children even when the root Location is already indexed. Do not walk retained raw body/supplement templates for actions. Recognize the exact prepared-reference child facade before Styled traversal; visit that one selected child under the existing lexical chain, omitting the token-only carrier delta. Recognize generated reference ContextBlock before generic payload publication: retain its real allocator/forced Location and context_block_locations entry, but publish no semantic element/query/tag/action. Ordinary source ContextBlock remains unchanged. Preserve original Ref label morphology rather than promoting internal transport or first inserted Figure to the language label target.

No additional public contract beyond the exact three approved transport functions, new target inference, second arena, shared result/effect cache, fake observation, weakened diagnostic or unrelated generic visitor change is authorized. Preserve five-attempt stabilization, canonical paths and scoped lineage; validate reached and hidden effects, forward/repeated references, aliases, missing/unnumbered/non-Figure/bibliography/page controls and mixed target behavior.

Private root traversal may distinguish ordinary indexing, original-root-only indexing and already-indexed visual-child traversal explicitly. This prevents either duplicate automatic root actions or skipped emitted Figure children while retaining existing Table visual handling.


### Ordinary Block original root versus actual children

Measurement before decision: ordinary-block-hidden-heading/receipt.json under agents/counters binds pinned oracle and frozen build8. A real source show block:none leaves child Heading query count0 in oracle but1 in product; identity Block shows INNER in both yet counts1 versus2. The captionless Block integration independently retained an unexecuted raw body Context as a second semantic Context through Realized.original. Current walk's original-root-only classification and return cover Table/Figure, while ordinary Block originals recurse raw bodies. Read current realization-order and content/source-origin Nuclei: identity is not order, source is transparent and contributes no extra occurrence.

Classify a Realized ordinary Block original as root-only, preserving any root snapshot/label/action if that root has a semantic payload, then visit its actual visual children normally. Extend the existing explicit OriginalOnly return to Block; do not reclassify arbitrary containers as empty or change generic alias policy. A registered Located visual Block whose canonical original at that Location is also Block must still visit actual Block children through ChildrenOnly, preserving one root. Restrict this same-type transport case; do not bind Figure Location to arbitrary Block/replacement products or reopen all aliases. Existing Table/Figure behavior and causal order remain unchanged.

The separate labeled Block probe records oracle [1,1] versus build8 [2,0]: child count duplication belongs to this correction; absence of Block label query membership is a pre-existing nonlocatable-label projection gap, retained as open rather than silently expanding Label ownership. Require hidden/identity/identity-labeled child language regressions plus existing reached Context membership, positions, counters and source wrappers. No callbacks, public field/API, new phase, cache, suppression or global registry relaxation.
