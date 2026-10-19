# Independent final structural-witness review

PASS for the selected structural obligations. This read-only audit independently
reparsed `structural-green-final/*.log`, reconstructed each archived mutation
from `original-sources.json.gz`, and checked the selected primary source spans
against the actual mutated source bytes. No Cargo or productive edit was made.

The reviewed jobs ran from 2026-10-19T13:41:13.987252+00:00 through
2026-10-19T13:44:45.844028+00:00 at HEAD `370d78355bd646d1abead5bcacef215288cc8834` with the uncommitted
candidate. Exact source/manifests and owner hashes are retained in
`structural-green-final/inputs.json.gz`; the candidate state and changed-file
list are retained in `candidate-final/state.json.gz`.
`structural-witness-review.json` records input/log/mutation hashes, exact primary
spans, selected messages, isolation arms, domains and per-job results.

| Job | Selected primary E0004 | All E0004 | Other error codes |
|---|---:|---:|---:|
| all-domains | 25 | 275 | 0 |
| new-old | 1 | 9 | 0 |
| old-new | 1 | 9 | 0 |
| new-new | 1 | 9 | 0 |
| native-new-old | 1 | 13 | 0 |
| native-old-new | 1 | 13 | 0 |
| native-new-new | 1 | 13 | 0 |

All 25 intended decisions have their own attributable primary E0004 in the
all-domains run: the 19 original batch-C decisions plus nested Type, font-name
Value, Metadata Value, selector receiver, contextual boundary and native identity.
These supplemental targets are not retroactively counted as original baseline
cases. The selected 027 span is the identity match after node_kinds, not the
carrier passthrough; 011 distinguishes its outer Content match from nested Value;
012 selects the outer display Value match.

Actual enum declarations, not stubs, receive the unit sentinel. The combined
Value/Type file has separate additions to its two enums. Each isolated pair job
restores all originals, extends only Selector or only FuncRepr and inserts exactly
the documented coverage arm into the productive equality decision. Independent
finite-domain checking verifies that new/old, old/new and new/new are separately
left uncovered for both tuple domains. Each class produces E0004 at its actual
pair match; all six isolated_axis annotations are now persisted, including the
last job.

All seven Cargo invocations exited101 as required for these negative compile
probes. There are no non-E0004 compiler errors. Other expected E0004 diagnostics
from extending the real shared enums remain in the logs (250 in the combined
run, eight in each Selector-only run, twelve in each FuncRepr-only run); none is
used to satisfy an owner obligation. Therefore “no unrelated errors” must not be
reported as “only the selected diagnostics existed.”

The frozen-input receipt is empty, every archived original Rust source matches
its recorded input hash, and current frozen source/owner files still match the
recorded hashes at review time. Exact mutation reconstruction found no extra
source edits in any job.

This proves sensitivity to future nominal variants at the specified decisions
and isolated pair axes. It does not prove correct current payload semantics,
complete runtime path coverage, language parity or closure of remaining V16 and
semantic debts. Those remain governed by the separate tests, oracle, lineage and
full validation receipts.
