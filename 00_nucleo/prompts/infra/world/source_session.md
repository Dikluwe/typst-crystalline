# L0 — infra/system-world/source_session
Hash do Código: d619ab61

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:c7f9b3d18b112b1d725e082b7da75b886bf7120346ffbca911c236ea65c52986

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/source_session.rs`

## Evidence before decision

`lab/results/construction/diagnostics/incremental-prioridade6-2026-10-08/{before.json.gz,measurement.json.gz,control-measurement.json.gz,count.json}`
retains tree/HEAD/UTC/commands/releases. Source::new rebuilds CST; a fresh World
rebuilds main/import Sources per edit. Minimal import already consumes eval_ms;
do not attribute all of it to parsing. Existing owners define causal Source/id/
path identity; reuse is mechanics preserving language observables. Cost/diagnostic/
effect/stale source/geometry/identity changes refute benefit. No upstream intent
is inferred.

source_session.rs:24 declares public no-argument new. The field-reassign-2026-10-15/
clippy-inventory.json reports new_without_default; clippy-default-2026-10-16 retains
source/L0/hashes. new creates an instance-local empty Mutex<Vec<Entry>> without
parsing/I/O. Default delegating to it is public Rust construction, no new language
parity measurement. The owner approved manifesto
`11f3e138f5ba006d04ed4dd1e829e27c004778e691f4a1df63127b1644146d03`
on 2026-10-16, receipt
`lab/history/construction/records/00_nucleo/materialization/clippy-default-2026-10-16/approval-receipt.json`.
This satisfies the earlier pending gate only for the three explicit Default impls
and D1–D5 controls; all other contracts remain.

## Ratified session contract

Own opaque public SourceSession with new() without I/O, private fields/internal
comparison/retention methods. Retain only immutable markup Source by canonical
physical path and current FileId; validate full text, never Source Eq/hash as proof.
Do not assign ids. Bounds: at most eight LRU entries, 1 MiB total text, 512 KiB per
entry; oversize parses normally without retention. Instance Mutex in L3, no globals/
unsafe/disk/clock. Parse using L1 Source::new. Concurrent calls may duplicate work;
never return Source with unvalidated text/id/path. No other products/read errors
retained. CST errors/warnings follow normal eval reemission without Sink caching.

Public `impl std::default::Default for SourceSession` has `fn default() -> Self`
returning Self::new(). Keep new/visibility/fields/values/other methods/semantics;
no other trait/mode/flag/global state/constructor I/O/phase change. Existing callers
need no migration; new callers may use explicit Default/bounds. This additive
contract's future removal would break consumers. Approval narrowly supersedes
generic API bans for its specified impls, not arbitrary API work.

## Acceptance and limits

Local tests cover real-World reuse, refusal by text/id/path, bounds/eviction,
transients/error/recovery/concurrency. Text budget is not an RSS bound; measure
RSS. Without proven product gain remove productive connection and retain negative
evidence. Require RED→GREEN, production counts separate from timing, three paired
CLI rounds against the archived prior release, startup/RSS and invalidation/
identity/effects/Unicode/diagnostic/assets/fonts/geometry controls. Owner-only
resealing, relevant builds/suites and lint zero errors; no full-incrementality claim.

Before Default implementation, bound-based usage tests fail due to missing impl;
after delegation they pass. Check fresh-context/session values/isolation, lazy
resources and unchanged controls; workspace suite/all-target Clippy/build/Rustdoc/
fmt/lineage, scoped resealing. No default-value changes or new_without_default
suppression. Original package provenance is archived in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/world/source_session.md`.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.


## Approved World metadata integration controls

The approved metadata proposal is pinned in `lab/results/world-metadata-errors-2026-10-22/approval.json`. Healthy dependency results and directory/path lookup fixtures must explicitly require successful Result acquisition while retaining all existing content, identity, source-session and dependency assertions. Test registry and slot poison without catching production panics or reusing poisoned state; assert exact existing FileError errors, unknown-id versus poison distinction, no mutation, and no silent root/empty-graph fallback. Record an actual runtime RED before changing implementations. No other public API, phase, flag or diagnostic policy change is authorized.
