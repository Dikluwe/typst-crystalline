# L0 — compiler/eval/show_rule_termination
Hash do Código: f77bb6d1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/show_rule_termination.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/show_rule_termination.rs:28` — `type ShowRuleLoopResult`.
- `01_core/src/compiler/eval/show_rule_termination.rs:30` — `fn run_show_rule_loop`.
- `01_core/src/compiler/eval/show_rule_termination.rs:97` — `fn show_rule_cycle_error`.
- `01_core/src/compiler/eval/show_rule_termination.rs:113` — `fn show_rule_depth_exceeded_error`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/docs/content/reference/language/context.typ:13`.

## Contract

Own the pure one-node termination strategy, not selector matching or rule
evaluation. Keep ShowRuleLoopResult = (Content, bool) and
run_show_rule_loop<F>(initial, apply_one_step, max_depth, full_error), where the
callback returns SourceResult<Option<(Content, RuleId)>>. The bool reports whether
any rule was applied. Propagate callback errors unchanged.

No match ends successfully. A transformation with the same morph_canon is a
morphological fixed point. Otherwise retain canonical forms and RuleId transitions;
a repeated form establishes a cycle and identifies the participating rules.
The depth limit is a backstop, not the sole termination mechanism. Preserve
`show rule cycle detected` and its rule-identifying hint separately from
`maximum show rule depth exceeded`, whose hints are exactly
`maybe a show rule matches its own output` and
`maybe there are too deeply nested elements`. full_error adds only the existing
cyclic/nonconvergent classification hint and associated history; fixed-point and
cycle detection are independent of that flag.

The one-step callback is supplied by
00_nucleo/prompts/core/compiler/eval/rules.md, which owns innermost-first precedence,
active RuleId guards and at most one transformation per call. This module does
not inspect Engine, EvalContext, selectors, styles or document realization.

Termination must not rely on a claim that state reads in show rules are forbidden:
show rules establish style context and locatable rules may establish location
context in the language. CounterUpdate/StateUpdate remain in canonical morphology,
but their presence alone is not a proof that all contextual observations are
stable. Context-sensitive evaluation and post-show introspection require their
own coordinating contracts and revalidation. Do not change eval/layout phase to
repair such behavior inside this loop.

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

Keep the loop's private type/signature. For prepared Cell, standalone or
positioned, return one callback step and the applied indicator to realization,
which owns guards/depth/product visits. morph_canon equality does not terminate
a new Cell generation. This module uses no Engine/context/styles and applies no
recipes itself. Other elements retain fixed-point/cycle/depth behavior. Do not
replace this nominal exception with a global nonlocatable policy.

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
