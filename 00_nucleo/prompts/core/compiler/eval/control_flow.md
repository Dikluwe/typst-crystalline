# L0 — compiler/eval/control_flow
Hash do Código: eeac1ec0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/control_flow.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/control_flow.rs:20` — `fn eval_conditional`.
- `01_core/src/compiler/eval/control_flow.rs:48` — `fn eval_while`.
- `01_core/src/compiler/eval/control_flow.rs:101` — `fn eval_for`.
- `01_core/src/compiler/eval/control_flow.rs:137` — `fn run_for_loop`.

## Contract

Own conditional, while and for evaluation. Conditions require boolean values;
preserve condition/type errors and expression spans. Loops use the explicit
EvalContext iteration limit, lexical scopes and pattern destructuring. Join
iteration values through 00_nucleo/prompts/core/compiler/eval/operators/join.md.
Consume Break and Continue at the loop that owns them and propagate Return
unchanged, including value, span and conditional flag. Preserve iterable order,
the existing accepted iterable domains, first failure and evaluation order.
Do not introduce global loop state or perform I/O.

## Explicit optional absence and closed-domain preservation — 2026-10-23

### Evidence before decision

The exact source and owner bytes, HEAD, dirty-tree stat and initial nine V16
findings are preserved in
`lab/results/v16-option-exhaustiveness-2026-10-23/before-state.json`,
`before/` and `before-v16.sarif`. The parser source
`tekt-linter/03_infra/rs_parser.rs:1605–1621` classifies every bare identifier
pattern as a catch-all, including the Option unit variant `None`. The Rust
compiler resolves that name to the unit variant; qualification makes that
existing nominal meaning explicit to both tools. These are mechanical
coverage observations, not a newly discovered runtime failure or new vanilla
parity claim.

### Exclusive correction and acceptance

`control_flow.rs:74–80,172–178` already enumerates Break, Continue, Return and absence. Qualify only the two absence patterns as `Option::None`; retain each body, loop exit, join, span, conditional bit and restored outer flow. Future FlowEvent variants must remain compile-time exhaustiveness errors.

Use ADR-0127 continuous internal flow: no public contract, default, phase or
compatibility changes. Add no lint exception, allow/expect attribute, severity
change, suppression or match-to-if rewrite. Preserve all existing assertions
and pending limitations. The initial V16 gate is structural RED; preservation
tests may already pass before implementation and must not be misreported as
behavioral RED. Require the final unfiltered linter, tests, strict Clippy,
rustdoc, formatting, canonical paths and both lineage directions.
