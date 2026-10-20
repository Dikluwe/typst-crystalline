# Figure/Grid implementation — pending validation and decision

**Batch F remains open. This draft does not certify complete Figure/Grid parity or completion of the twelve validation gates.** Results below belong to their linked frozen source records; later edits require new receipts.

The [approval receipt](approval.json) authorizes the exact [reviewed Figure/Grid package](../figure-kind/review.md): typed nominal kinds, lexical/resolved supplement fields, Grid child order, reached-occurrence preparation, numbering-only automatic actions, complete demanded counter histories, Location-based caption/reference reads, the additive bundle-target facade and the declared compatibility changes. It does not authorize the separately proposed reference-production phase. Owner paths are listed in [scope.json](scope.json); [scoped resealing](reseal-1/receipt.json.gz) records no changes outside its scope.

## Identity and provenance

The branch is `codex/tekt-mature-layout`, baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`. Implementation measurements use an **uncommitted working tree**: each test/build receipt retains its exact input hashes, changed-file stat, command and UTC interval. A HEAD alone is not the measured source state.

- [Baseline identity](baseline.json): accepted prior Circle binary `e114a4889a17292590a6a3c2ed0feeafd13dff970215740624fdf9aae0e8ef78`, input-equivalent to baseline HEAD; it was not rebuilt at that HEAD. The [baseline input manifest](baseline-inputs.json.gz) retains this qualification.
- Oracle: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Source/build provenance is retained in the [ratified resync report](../../construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md).
- [Candidate build 3](candidate-build-3/receipt.json): workspace build passed at 2026-10-20T00:22:40–00:22:52Z with `changed_inputs=[]`; executable SHA-256 `ed530741f7e48a6bceb13e5cdd82edd2c040fc7cbb887c3ffb122ea4a2753307`. The following CLI receipts bind this executable, not any later build.

## Measured evidence available

| Gate or control | Observed result | Frozen evidence |
|---|---|---|
| Initial functional RED | 9 failures, 2 controls passed | [RED log](red/tests.log), [state](red/state.json.gz) |
| Captionless automatic payload RED | 1 failure | [RED log](red-payload/tests.log), [state](red-payload/state.json.gz) |
| GREEN9 filtered core tests | 188 passed, none failed/ignored | [log](integration-tests-9-core/tests.log), [state](integration-tests-9-core/state.json.gz) |
| GREEN9 filtered infra tests | 13 passed, none failed/ignored | [log](integration-tests-9-infra/tests.log), [state](integration-tests-9-infra/state.json.gz) |
| Original frozen acceptance policies | 84/84 passed | [candidate replay](candidate-replay-3/receipt.json) |
| Hidden ancestors, own replacement and first-label controls | All four results match the pinned oracle | [candidate](ownership-candidate-3/receipt.json), [oracle](ownership-oracle-2/receipt.json) |
| Disposable real-enum extension probes | All eight selected decisions reject missing variants with E0004; the deliberately mutated workspace exits 101 | [structural receipt](structural-candidate/receipt.json) |
| Full differential lint | 20 → 19 V16; no new error/warning; all 1,649 outside-scope diagnostic objects identical | [frozen lint](lint-final-1/lint-state.json.gz), [independent complete note review](agents/counters/lint-final-review.md) |
| Additional rendered/target controls | 9/11 passed; two failures remain visible | [candidate controls](candidate-target-controls-3/receipt.json) |

The warning corrections retain each actual transaction and publish only the retained candidate through the existing Sink policy. Distinct warning origins remain distinct; discarded validation work cannot consume later retained diagnostics. [Pinned warning-origin evidence](agents/counters/warning-origin-oracle/receipt.json) and the [warn-then-panic differential](agents/counters/warning-error-oracle/candidate-selected-receipt.json) preserve the exact causal controls. Test counts above describe filtered suites, not a complete workspace test run. Structural probes do not prove runtime coverage of every grouped alternative; all V19/V20 notes remain visible.

## Work still required

1. **Rich reference supplements require a mandatory new decision.** The [frozen proposal and gate](reference-phase-gate.json) request three exact additive transport functions and admission of emitted numeric Figure references to the existing contextual boundary (ADR-0127 items 1 and 3). Nested supplement effects/occurrences at references remain missing; no Ref phase implementation has started. The [proposal](agents/realize/ref-phase-proposal.md) includes forward references, fresh versus aliased occurrences, cycles, diagnostic retention, labels and target controls. Approval and independent RED→GREEN acceptance are required before closure.
2. **Source `SetPage` header admission remains open.** The failing control emits global count 1 where the pinned oracle emits 2. Approved static `PageRun` content has separate passing bilateral evidence; it does not certify configured or repeated physical-page headers. See the [carrier boundary](static-running-boundary.md), [native PageRun evidence](native-page-running/receipt.json) and unchanged failing control above. No new `SetPage` phase or layout callback workaround is implemented.
3. **The second additional-control failure is under correction:** the candidate renders `raw table` rather than oracle `rawtable` for the inherited-kind HTML control. The measured ShowRecipe/Figure-original projection gap and its subsequent repair need a new frozen build/replay; the 9/11 receipt is not relabeled as passing.
4. **Final validation is incomplete.** The orchestrator must supply the remaining workspace/all-target tests, Clippy, Rustdoc, formatting, paths and strict forward/reverse lineage receipts against the final source state. The nineteen remaining global V16 errors are not waived or suppressed. Neither the current implementation nor this draft closes F.
