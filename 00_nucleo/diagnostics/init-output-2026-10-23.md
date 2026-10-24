# Checked init success output — 2026-10-23

Status: batch B / D2 complete on the source-bound debug candidate. This is batch B / D2
of the [handoff](vanilla-divergence-handoff-2026-10-23.md). Batch A changes and all
other pre-existing working-tree changes are preserved.

## Measured baseline and cause

[Baseline](../../lab/results/init-output-2026-10-23/baseline.json) and
[source archive](../../lab/results/init-output-2026-10-23/baseline-source.json.gz)
record HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working
tree**, branch, full dirty stat/status, source/L0/config hashes and executable
identities. All inputs match the completed batch A receipt before this task.
HEAD alone does not identify the measured candidate.

[Before receipt](../../lab/results/init-output-2026-10-23/before/receipt.json)
records 28 real local-template process probes, exact UTC intervals, commands,
isolated data/cache/destination paths, output bytes and package/project content.
A fresh destination is used each time. Only the preserved local package fixture
`@local/stderr-probe:0.1.0` is copied; registry/network acquisition is unnecessary.
The frozen Tekt baseline is batch A's debug executable; the oracle remains commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.

At baseline, `04_wiring/src/init.rs` used two unchecked success `println!` calls. On full or
closed stdout, the process exits101 after creating the complete project. A panic
message is captured when stderr is healthy; failed stderr makes that message
unobservable. Initialization-error stderr failures already produce operational
exit2 through checked diagnostics. The main owner already maps private I/O errors
to2. L3 publishes project files before returning its successful InitResult.

Pinned vanilla uses stderr for success, so failed stdout alone leaves its success
exit0 unchanged. A failed used stderr produces exit101 or SIGPIPE in those oracle
controls. Local oracle source uses checked summary writes at
`crates/typst-cli/src/init.rs:106` but calls
`print_error(...).expect(...)` at `crates/typst-cli/src/main.rs:59`; a checked
first write does not establish a panic-free process boundary. This difference does not certify checked stdout in vanilla or authorize
moving Tekt's existing success channel.

## Correction and scope

Only `04_wiring/src/init.rs` and `04_wiring/tests/cli.rs`, with their exclusive L0
owners, change among productive inputs. L0 updates precede tests/implementation
and resealing under ADR-0127 continuous flow. Existing CLI-observables Nucleus
pins and the main dispatcher remain unchanged.

After successful L3 initialization, hold one stdout lock, write the existing two
lines in order, and explicitly flush. Propagate the original I/O error through
the existing private result and operational exit2. Do not retry initialization,
roll back created files, redirect success to stderr or retry the failed sink.
Validation, resolution, download, staging/publication and cleanup stay in L3.
Healthy success remains stdout/exit0; a healthy initialization failure preserves
its error stderr bytes/exit1. A failed unused stderr does not fail a successful
stdout notification.

## Tests and results

[RED](../../lab/results/init-output-2026-10-23/red.log) has three actual process
failures (full stdout, both full, deterministically closed descriptor) and two
passing healthy/error controls. Each failure occurs after asserting that the
created project bytes survived. This is semantic RED, not compilation failure.

[Writer tests](../../lab/results/init-output-2026-10-23/green-writer.log) cover
short writes, first/second-line write failures without retry or flush, WriteZero,
and flush-only failure after both lines. The injected original error kind/message
is preserved. [Process tests](../../lab/results/init-output-2026-10-23/green-process.log)
cover real notification failures, exact success bytes/order and unused-stderr
failure, plus healthy and failed initialization-error stderr. All nine new tests
pass.

[After receipt](../../lab/results/init-output-2026-10-23/after/receipt.json)
retains all 14 Tekt cases and 14 oracle controls. Every used failed Tekt sink
produces operational exit2 without panic; healthy success/error and unused-sink
controls retain their original outcomes. Project and package file bytes match
before/after in every case. Failed channels are explicitly marked uncaptured;
no claim about invisible output bytes is made.

[Batch A preservation](../../lab/results/init-output-2026-10-23/layout-preservation/receipt.json)
passes eight real-font bilateral page/text controls: direct weak breaks, hidden
content, zero spacing, strong/weak parity, terminal metadata page/coordinates,
PageRun terminal metadata, multipage and Context. The final validator also checks
that every productive input outside the two changed consumers and two L0 owners
remains byte-identical to the task baseline.

[Final source/build/check receipt](../../lab/results/init-output-2026-10-23/final-validation.json)
and [source archive](../../lab/results/init-output-2026-10-23/source-final.json.gz)
bind every result to exact source inputs, UTC, dirty stat and commands. The
measured debug executable is frozen as
[`tekt-debug`](../../lab/results/init-output-2026-10-23/tekt-debug).
[Resource/toolchain evidence](../../lab/results/init-output-2026-10-23/environment.json)
retains one-job execution, 12 GiB maximum, 8 GiB high and zero swap; heavy commands
are sequential. The complete Rust workspace and doctest run has 9,834 passing tests,
zero failures and seven existing ignored tests. Nine tests are new in this batch.
Full workspace and all-targets builds, strict Clippy (`-D warnings -D
unfulfilled_lint_expectations`), rustdoc (`-D warnings`), formatting, Python CI
checks, path/layout checks, strict forward ownership/Nucleus/Hash checks and
reverse lineage all pass. Full crystalline-lint has zero errors and warnings;
existing informational observations remain. Productive inputs stayed unchanged
through the validation. No warning, assertion, threshold or original failure was
suppressed. No commit or push was made.

The [completion receipt](../../lab/results/init-output-2026-10-23/completion.json)
binds these outcomes, the frozen executable hash, complete measured source state
and [patch against the original dirty baseline](../../lab/results/init-output-2026-10-23/change-vs-dirty-baseline.patch).
The prior batch A executable and evidence remain untouched. The next handoff
batch is C / D3 (prose continuation).

The existing stdout/stderr channel difference from vanilla remains explicit.
This correction does not close batches C–H, footnote marker morphology, general
stdout panic candidates or general CLI robustness.
