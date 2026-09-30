# L0 — compiler/stdlib/registry/microbenchmarks
Hash do Código: ec3fcf9e

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
