# Checked HTML stdout completion — batch H, 2026-10-23

Status: corrected and fully validated, preserving the received A–G work.
The exclusive specification is [wiring/compile L0](../prompts/wiring/compile.md).

## Measured cause and contract

The received inputs exactly match G's completion: HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working tree**. The exact
`git diff HEAD --stat`, status, source/config/L0 hashes and UTC are in
[baseline](../../lab/results/html-stdout-2026-10-23/baseline.json);
[the complete source archive](../../lab/results/html-stdout-2026-10-23/baseline-source.json.gz)
also retains untracked productive inputs. Only `04_wiring/src/compile.rs` and
`00_nucleo/prompts/wiring/compile.md` change among the 2162 frozen inputs.
The [patch](../../lab/results/html-stdout-2026-10-23/change-vs-received.patch)
is relative to that received dirty state, not HEAD.

[Qualified baseline probes](../../lab/results/html-stdout-2026-10-23/measured-probe.json)
show healthy stdout and file HTML of 174 bytes (short) and 28160 bytes (long).
These are channel controls, not general HTML-parity criteria. For the short
case, both executables call write(1) after stderr warnings, receive ENOSPC from
`/dev/full`, and exit 0. The long control attempts the failed write before
warnings: Crystalline exits 2 and the oracle exits 1. Before correction,
`compile.rs:585,592` called write_all without checking flush; pinned upstream
`crates/typst-cli/src/args.rs:612–616` does likewise. The traces establish actual
write attempts; matching success statuses did not establish successful delivery.

This is a correction of the existing owned operational failure contract:
checked write/flush failures exit 2. ADR-0127 continuous flow applies to private
composition, with L0 updated and sealed before the runtime seam/tests and fix.
No public API, flag, default mode, pipeline phase or compatibility change is
introduced. The oracle's late success is measured behavior, not inferred intent
or a behavior copied as a requirement. Oracle commit remains
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.

## Correction and RED→GREEN

Both existing stdout sites use a private helper: write_all, then checked flush
on the same writer/lock. A write failure short-circuits flush; a flush failure
uses the existing exit-2 diagnostic path. Timings and successful Published are
reached only after output completion. Healthy bytes, status callback ordering,
experimental warning → domain warning → operational error, file/bundle/watch
policy and all target dispatch stay intact. PDF/PNG and SVG share these sites.

A behavior-preserving unflushed seam first produced runtime RED: three failures
(short-write flush count, flush-only error and buffered real `/dev/full`) and
one passing direct-write-error control. The correction then passed all four.
Exact commands, source archives, original errors and timestamps are in
[RED](../../lab/results/html-stdout-2026-10-23/red-tests/receipt.json) and
[GREEN](../../lab/results/html-stdout-2026-10-23/green-tests/receipt.json).
Neither the initial sandbox-denied strace attempt nor the first observer's
absent-output handling error is counted as RED; both are retained separately.

## Validation and scope

- Twelve complete sequential validation gates passed: Python CI, layout, strict
  lineage, reverse lineage, workspace build, domain/infra and wiring tests,
  doctests, all-target build, Clippy, formatting and Rustdoc. Rust totals:
  **9,853 passed, 7 existing ignored**, zero failures.
- [98 debug controls](../../lab/results/html-stdout-2026-10-23/controls.json)
  and [98 release controls](../../lab/results/html-stdout-2026-10-23/release-controls.json)
  preserve healthy HTML stdout/file bytes and warnings, compile errors, Unicode
  and semantic markup; `/dev/full` and closed pipes return 2. Failed stdout
  preserves prior timings; healthy stdout still writes valid timings. PDF, PNG
  and SVG healthy before/after payloads and failure controls also pass. Physical
  `-` is unchanged/absent throughout. These are transport-preservation controls,
  not claims of global format parity.
- [Release build](../../lab/results/html-stdout-2026-10-23/final-build/receipt.json)
  ran 2026-10-23T22:43:48.032019+00:00 through 2026-10-23T22:47:13.491383+00:00,
  with unchanged frozen inputs and the accepted G ThinLTO/codegen recipe.
  Final executable SHA-256: `a18ecc19a76863f6c07f6574c606b3c5c7d00ab99b4480a8763a2bc704c8aa32`.
  [Release traces](../../lab/results/html-stdout-2026-10-23/release-probe.json)
  confirm short and long failures now exit 2, with warnings preserved in order.
- Heavy Cargo jobs ran sequentially, offline/locked, one job, MemoryMax 12 GiB,
  MemoryHigh 8 GiB and MemorySwapMax 0. Full crystalline lint and diff checks
  pass; all 1853 font/reference assets are unchanged. Validation
  source freeze is unchanged; no reference refresh or oracle resync occurred.

The standard runtime still retries its buffered stdout during process cleanup
after the already detected flush error; the traces retain this attempt. The
private helper does not retry, and the exit remains 2. This batch does not alter
runtime cleanup, add rollback/atomic publication, certify global HTML coverage,
or close earlier documented render/resource limits. The oracle still exits 0
for short late failures. Full evidence, final dirty status/stat and source-bound
gates are in [completion](../../lab/results/html-stdout-2026-10-23/completion.json).
