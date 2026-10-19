# V16 batch A correction evidence

Implementation checkpoint: `841c9855f8ad6695f4e140d1ee4a33e4f4a46689` on
`codex/tekt-mature-layout`. Status: batch A validated; 14 findings closed, 91 remain open. `scope.json` lists the
14 selected findings and their 11 exclusive productive/L0 owners. This receipt
is diagnostic evidence, not an L0 specification.

## Inputs and independent gates

- `baseline-state.json.gz`: original checkpoint, snapshot identity and hashes.
- `baseline.sarif.gz`: full pinned-linter baseline, 105 V16 errors; unchanged
  linter SHA-256 `421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`.
- `snapshot-citation-completion.json`: five cited oracle source blobs added to
  the disposable snapshot so citation validation has the same inputs as the
  real repository. The incomplete attempt is retained separately.
- `baseline-tests/`: original core/infra/watch tests before new controls.
- `preimplementation-final.patch.gz`: exact L0/header/new-test changes before productive
  implementation, with checkpoint and digest in `preimplementation-final.json`; the earlier snapshot is retained.
- `preservation-before*`: all characterization attempts, including failed ones;
  each has its own frozen-input state, exact commands and logs.
- `structural-baseline/`: complete copied workspace compilation with isolated
  enum sentinels or selected positive-arm omissions. Every mutation is retained
  with compiler JSON and target attribution. Baseline selected sites lack E0004.
- `reseal-receipts.json`: restricted metadata-only seals after ownership/nucleus
  validation. The worker refuses changes outside the eleven selected pairs.

Workers are in `../workers/`. Cargo uses the locked offline dependencies, one job,
12 GiB maximum memory, 8 GiB high threshold, and zero swap. The `memory.*` fields
in runner receipts come from the actual worker cgroup. UTC, HEAD, uncommitted
`git diff HEAD --stat`, and input digests identify each measured state. No probe
sentinel is introduced into productive code.

## Failed characterization attempts and unresolved parser intent

The first preservation attempt failed to compile four new assertions because
unqualified `None` resolved to the AST's tuple constructor. Qualifying those
expected option values changed no expectation or productive behavior.

The second attempt compiled and found a new assertion based on the old parser
comment to be unsupported: for `a^b'_c^d`, the top attachment was an inner node,
not the expected leaf `d`. Source tracing shows that the chained underscore
empties the stop set and parses its RHS at precedence 2. The pinned oracle source
has the same loop and the same comment; this is source evidence, not a bilateral
language execution or proof of intent.

The next run retained the long input with full structural characterization:
base `a`, top `b'`, bottom `c^d`, plus the minimal `a^b'_c` prime-boundary control.
It passed against the original productive bodies (see
`preservation-before-retry2/new-core-tests.log`). The test explicitly labels the
long grouping as observed behavior. This preserves the internal classification
scope; it does not ratify the grouping as the intended language contract.
The discrepancy between the parser comment and this observed tree remains an
open diagnostic obligation. Resolve it through current-pin language evidence
and the existing parser owner before any semantic correction. The productive
comment and parser precedence/chaining algorithm are unchanged by this batch.

The third attempt passed all core controls, then exposed two calls in new infra
tests to a nonexistent `Gradient::sample` method. The tests now destructure their
known linear fixture and call the existing `Linear::sample` API, preserving all
expected colors, alpha, endpoints and spot identity assertions.

`preservation-before-retry3/` is the accepted complete preimplementation run:
all seven jobs passed, inputs stayed frozen, and the worker reported no OOM.
The broader `v16_` filters intentionally include existing controls. These logs
are not a claim that every executed test was newly added.

## Structural proof interpretation

The core experiments add a sentinel to copied SyntaxKind, Value, ColorSpace,
NodeKind, Pattern, or Expr. Pattern and its nested Expr are separate obligations.
All actual manifests, modules, dependencies and function bodies are retained.
Other E0004 diagnostics are expected but do not count as target proof.

For infra/watch, changing an upstream enum can prevent dependency metadata from
being emitted before the consumer is checked. The owning L0s therefore specify
an attributable paired omission alternative: remove the same unique positive
arm in original and repaired copies with unchanged real dependencies. The
wildcard original still compiles; the nominal repaired decision must emit E0004
at that exact match. Runtime preservation controls remain mandatory and separate.
No stub, substitute function, dependency change, target exclusion, severity
change, or suppression is involved.

## Additional evidence and cache boundary

The first structural GREEN run attributes E0004 to all 15 selected sites across
14 findings, versus no selected-site E0004 in the structural baseline. Pattern
and Expr have separate mutations and selected matches. Compiler output for the
NodeKind tuple initially chose a left-axis witness, which by itself did not
exhibit each required pair class separately. `node-axis-proof/` adds six runs:
original/repaired versions each isolate new/old, old/new and new/new. The repaired
compiler witnesses name all three old variants in each mixed direction and the
single new/new pair, at the selected match. Source hashes, injected isolating
arms, unchanged dependencies, freeze records and full diagnostics are retained.
These probes never change the productive enum or weaken the owning contract.

The additional metadata test puts every continuation kind at collect_top's root,
with meaningful-neighbor/MCID controls; valid nested-table integration is retained.
`metadata-control/` compiles a complete current workspace copy with only the
selected nominal match reverted to its original wildcard. All three metadata
controls pass there. The first copy lacked corpus files used by unrelated
compile-time includes; `metadata-control-incomplete-fixtures/` retains that
inconclusive failure. The successful repeat copies and hashes the real corpus.
No fake fixture or new semantics were introduced.

The immediately following current-workspace metadata command reused the copied
workspace's test artifact without recompilation. Its exit zero is **not** accepted
as independent candidate-build evidence. `cache-boundary.json` records the issue.
The final `validation-run/` first cleans only the four workspace packages in
`lab/target/product`, then performs the existing integrated build/tests/lineage
workflow against the actual checkout. Cached third-party dependencies are not
substitute project builds; the oracle binary is untouched. Final acceptance must
use those fresh-build receipts. Functional controls and structural mutant failures
remain separate kinds of evidence.

## Reproduction boundaries

The baseline snapshot is disposable. Recreate it from the recorded checkpoint
with `git archive`, including the four layer directories, `00_nucleo/prompts`,
`lab/benches`, root Cargo/Crystalline/Clippy/Rustfmt/toolchain manifests and
`.cargo/config.toml`. Restore the five exact source blobs listed in
`snapshot-citation-completion.json` at that same checkpoint, and verify the
recorded hashes before using the snapshot. The preimplementation patch retains
the L0 and test additions separately from productive corrections.

Job JSON files preserve the exact argument arrays. Run their workers under the
recorded systemd resource limits. Use new output directories to retain prior
receipts; `node_axes.py` explicitly refuses to overwrite an existing result.
After any copied-workspace experiment sharing `lab/target/product`, clean the
four local workspace packages before treating a subsequent checkout build as
fresh evidence. Final integrated validation already follows this boundary.
The pinned oracle and linter identities must match their recorded hashes.

## Final acceptance

The clean checkout validation completed at 2026-10-19T11:09:36Z: all 12 integrated
gates passed, with no change in the 2,130 frozen inputs. All 31 newly added test
cases are observed passing in the full domain/infra log. The actual cgroup peak
was 6,455,119,872 bytes under its 12 GiB cap; swap remained disabled and no OOM
event occurred. This is a resource receipt, not a performance-improvement claim.

`closure-ledger.json` closes the selected 14 IDs against final source/L0 hashes.
The full pinned linter still exits 1 for the other 91 V16 errors. Strict lineage
and all integrated gates pass; no overall zero-violation claim is made. The
current concise report is `00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md`.
