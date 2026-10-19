# L0 — compiler/eval/show_rule_termination
Hash do Código: b5897386

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
00_nucleo/prompts/compiler/eval/rules.md, which owns innermost-first precedence,
active RuleId guards and at most one transformation per call. This module does
not inspect Engine, EvalContext, selectors, styles or document realization.

Termination must not rely on a claim that state reads in show rules are forbidden:
show rules establish style context and locatable rules may establish location
context in the language. CounterUpdate/StateUpdate remain in canonical morphology,
but their presence alone is not a proof that all contextual observations are
stable. Context-sensitive evaluation and post-show introspection require their
own coordinating contracts and revalidation. Do not change eval/layout phase to
repair such behavior inside this loop.
