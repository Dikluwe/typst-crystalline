# L0 — compiler/stdlib/registry/microbenchmarks
Hash do Código: bc076d40

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/microbenchmarks.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own an ignored cfg(test) Criterion runner under the private registry; do not
expose factories/helpers or production APIs. Require release profile. Measure
Scope construction/destruction across Html/A11yExtras/Bundle combinations,
empty and scaled inputs, present/absent global lookup, calc lookup, and calls
through pre-resolved calc.abs/upper pointers for valid and invalid type/arity/
named arguments. Keep fixtures, native resolution, World and EvalContext outside
timed sections. World performs no I/O and fails source/file requests. Exclude
parser, eval dispatch, cache, layout and export from these benchmark names.
Include output/diagnostic construction and destruction. Use black_box and fixed
semantic assertions before timing. Use 30 samples, 150 ms warmup and 300 ms
measurement windows. Reject debug execution. Controls verify feature gates and
nonempty diagnostics on invalid calls; results do not establish vanilla parity
or improvements without a comparable repeated baseline.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Executable profile precondition

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
registry/microbenchmarks.rs:203–206 is the measured fragment. Clippy inventory and
owner reading distinguish effectless code, deliberately invalid data and empty
checks. `lab/results/construction/diagnostics/controle-fluxo-ci-2026-10-13/before.json.gz`
and companion evidence retain sources/L0/UTC/status/diffstat/commands. This is
neither new parity evidence nor inferred upstream intent.

### Contract and acceptance

Preserve exactly the guard rejecting explicit debug runner execution.
The cfg!(debug_assertions) assertion is an intentional profile precondition,
not a product behavior test. Use expect(clippy::assertions_on_constants) only
on that guard with justification; suppress neither module nor other warnings.
Run the selected ignored runner and confirm debug fails with
`run with cargo test --release` before Criterion. Semantic fixture assertions
and release measurements stay intact. Preserve public APIs/defaults/phase and
language expectations; validate the appropriate suite, Clippy, build and lineage.
This internal correction follows ADR-0127 continuous flow.
