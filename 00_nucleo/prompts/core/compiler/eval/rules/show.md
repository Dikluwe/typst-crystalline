# L0 — compiler/eval/rules/show — Show rule evaluation
Hash do Código: 08ead122

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:8388731d50379499612641a70a85695c0e7139ec605289c29161e12086abd78a
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/show.rs`

## Extraction evidence and contract

The uncommitted tree captured in
`lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz`
contains this family's routines in rules.rs. L1 passed6401 tests/two ignored
before extraction. Structural measurement, no benchmark/fresh bilateral vanilla
proof. Historical measurements/pin caveats remain in
`00_nucleo/prompts/core/compiler/eval/rules.md`.

Capture set styles/evaluate ShowRule with identity/selection/transformation/lexical
scope, preserving guards/errors/callbacks/origin/morphology.

Preserve observed signatures/identities/casts/defaults/error precedence/eval
phase. Parent keeps shared diagnostics/static bindings; selector matching,
termination and textual realization retain exclusive owners, no duplicate
algorithms. L1 suite/crystalline-lint without regression; language semantics/
syntax/morphology under ADR-0107/0108, sole ownership/valid pins ADR-0129/0130.

## Native SmartQuote show mapping

smartquote-2026-10-06/{before,fields-before}.json.gz records exact pin/HEAD/
working tree/UTC/binaries. fields-before compile-show-native yields vanilla Q,
Tekt rejects smartquote function. Add only static fn_addr_eq(native_smartquote)
mapping to existing Selector::NativeElement. No NodeKind/name dispatch/constructor
execution; alias works. Programmatic leaf/content_elem_func identity already
exists. Markup conversion and glyph/eval/layout changes are outside this mapping
slice and require their separately ratified phase owner.

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

### Exclusive Equation mapping and acceptance

Add only fn_addr_eq(native_math_equation) mapping to effective
Selector::NativeElement. Direct/alias/where use same matching owner, no NodeKind/
string dispatch/constructor execution. Homonyms/partial/closure do not become
elements by name. Internal continuous mapping correction was promoted with larger
approved insertion gate, not independently a phase change. Scoped RED→GREEN/
bilateral revalidation mandatory; only named insertion boundaries supersede old
phase restrictions, no universal parity/D07/math-label/other-debt certification.

## Existing Cell projection mapping

Before deciding, table-cell-realizacao-2026-10-07/before.json.gz retains HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/exact diffstat/
source-binary hashes/argv/cwd/results at historical pin9f2b6e871. Vanilla
table.rs:734–778 declares body/properties; local table_cell.rs:62–109 lacks getter;
content/identity.rs:107 uses table.cell; content_methods.rs:226–254 lacks real
function projection; show.rs:300–320 omits native. body/fields/func-eq/explicit
show refute parity. Language semantics/morphology, not Rust/PDF equality or
structure-inferred intent.

Add only static fn_addr_eq(native_table_cell) mapping to existing NativeElement,
never constructor execution/name classification. RED→GREEN/bilateral collection/
build/crystalline-lint without error, no public type/signature/phase change.
Lexical omitted/auto/none and position/style/padding/show realization retain
separate measured contracts. Option None proves neither omission nor explicit
none; do not certify them through this mapping.

## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Exclusive Table mapping

order/table-replacement-before-cells measures vanilla accepting show table and
hiding Cell effects; Tekt rejects at show.rs:322–331. Add fn_addr_eq(native_table)
mapping to existing NativeElement, preserving aliases/explicit where. No NodeKind/
name identity; rules/matching retain owners.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



## Approved native-transform show integration

transformacoes-2026-10-14/style-contract/results.json records pin/commands/UTC:
contextless errors, alias identity, With rejects set/getter/show, two origin sets
fold by axis. layout/tests.rs:21648 directly constructs Matrix. Approved manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`.
Accept native move/rotate/scale/skew as NativeElement by fn identity, with aliases.
Matching/filters stay selector_matching owner. With functions are not selectors,
as pin rejects. Require corpus/causal tests; construction is not layout/release.

## Ratified compact show-style producer

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

After existing capture_set_styles(set, scopes, ctx, engine)?, wrap same Styles in
Arc::new only when constructing Transformation::Style. Preserve capture/chain
swap-restoration/error propagation/to_vec/registration/order. Duplicate no fold,
extra Styles clone/callback/set evaluation. Public carrier belongs to
`00_nucleo/prompts/core/entities/show.md`; only producer migration here. Borrowing
consumers need no new algorithm.

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
requires a separate proposal. This owner only adapts its existing style producer.

## Table argument transparency and nominal barriers

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/tekt-v16-table-cells-2026-10-17/before.json.gz`
retains UTC/status/exact diffstat/sources/L0/config/hashes and initial two-owner
snapshot. contexts.json identifies ten V16/latent projections before code.
public-before.json uses real public APIs with rlib/probe/binary/SHA/argv:
Source(Cell) loses x/colspan/rowspan/kind/span in header; footer loses Source;
different levels reject only Source-wrapped members; Source footer group changes
a/b to ab/empty. snapshot-before.json measures context_floor77→0 under Source at
both restoration sites. cli-before/cli-extra-before/cli-controls-before.json
retain bilateral full binary SHA at historical pin. Footer single [a#strong[b]]
argument becomes a/b Cells versus ab/empty vanilla; show table:none wrongly
rejects. [] controls hide header/footer without Cell panic; colspan alias errors
retain message/caret. Header/footer colspan/different-level groups also diverge
in rows, explicitly not resolved here. These are semantics/morphology/origin/
errors, not Rust/bytes/intent requirements.

### Exclusive None transformation cast

Classify Value::None as Transformation::Content(Content::Empty), using existing
types. None is empty-content cast, no selector. Preserve set capture/guard/eval-
registration-ID order and other legacy type errors. Execute no Cell recipe/hidden
body to validate. show.rs:442–459 rejects None; pin accepts it, still rejects
int/auto. Preserve [] controls/prove hiding equivalence. Invalid-type localized
message remains legacy observable without integral diagnostic-parity claim.

### Acceptance and remaining scope

L0 before tests/code; continuous internal Source/language-cast correction, no new
public field/type/signature/trait/flag/default/phase/external compatibility.
Constructor sets raw Cell only, no recipe/with_defaults/placement/font/layout.
Keep Table show→defaults/placement→Cell show→body. Justify visible V19 per case,
never infer coverage from grouped body. No allow/expect/@capture/if-let hiding
new exhaustiveness. Preserve unchanged legacy errors/casts, failed guards,
callbacks/success-error restoration.

RED→GREEN covers explicit/Source Cell properties/presence/kind/origin, footer
per-argument not per-fragment casts, raw unprepared forms, Source groups/both
snapshot restorations, type/Auto/None negatives, invalid-local-key shadow and
None show transformation without admitting other values. Revalidate hidden/late
CLI effects and pinned structural observables. Relax no tests/whitespace.

Real future-variant Content/Value/TableCellKind probes in disposable workspace,
cfg(test) compiled: all18 final matches demand E0004, each counted once. Preserve
inputs/cfg(test) witness. Private Cargo target; never share mutant/measurement
artifacts with main target. Require build/full suite/fmt/all-targets Clippy
-D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/four indexes,
strict V1/V5/V15/V26/reverse/replay/full differential SARIF/diff-check.
V16 expected141→131, V19 only prescribed groups, no V20 increase; remove both
optional projections' depth. MemoryMax 12 GiB/zero swap/one job, heavy commands
sequential. No commit/push/merge/release/new CI pin.

Header/footer spanning and interlevel boundaries still need table-row-group-
contract's own measurements/L0. Group level differs from Cell kind/semantic level;
current inference is no complete contract. Repair no occupancy using Vec length
or Source/Styles as boundary marker. Preserve legacy Grid. Rich casts/paint/inset
and historical obligations are not certified by closing V16. Global objective,
math-stretch-composite and two matching obligations remain open.


## Measured native-element fallback

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, state/UTC/sources in
lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz; vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. before-pdf-probe.json.gz freezes
bilateral field/admission/callback/init controls. Prior
paridade-novo-vanilla-2026-10-17/inventario.md E02–E04 records carrier limits/
discriminating sources, no dump/Rust-shape acceptance. Language observables.

Keep NodeKind/special page-par mappings. Function fallback uses native_element_fields
and NativeElement only for already catalogued natives, including text/bibliography/
real aliases. Reject With/closures/homonyms. Existing matching/termination/scope
owners retain algorithms; no arbitrary function promotion.

L0 first/functional RED/bilateral GREEN, no public/default/phase/compatibility
change. State reader proposal gate was independent (later separately withdrawn/
replaced by ratified reader review); this repair grants it no authority. Valid1:1/
pins before reseal. Build/full workspace/strict Clippy/Rustdoc/fmt/strict-reverse
lineage, sequential MemoryMax 12 GiB/High 8 GiB/swap0/jobs1. debug0/incremental0 only
validation environment; measured consumption, no inferred product gain.

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


## Current textual show-set admission

Accept existing Transformation::Style for literal/regex selectors, capturing real
Styles with current helper. Remove artificial textual show-set rejection. Change
no selector identity/signature/general set validation or invent NodeKind. Delegate
slice/application to matcher, acceptance by colors/styles/effects.

Require acceptance.json families/R2-S-R3/single-evaluation RED→GREEN; separate
PDF/HTML, build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/
differential SARIF without new error-warning.12GiB/High 8 GiB/swap0/jobs1/heavy
sequential. No RAM gain without paired trial/suppression to clear gate/commit/
push/release.
