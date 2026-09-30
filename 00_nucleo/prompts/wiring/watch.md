# L0 — wiring/watch
Hash do Código: 4189d4ae

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/watch.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/watch.rs:14` — `run_watch`.
- `04_wiring/src/watch.rs:113` — `write_watch_block`.
- `04_wiring/src/watch.rs:118` — `emit_watch_block`.
- `04_wiring/src/watch.rs:124` — `watch_line_at`.
- `04_wiring/src/watch.rs:128` — `watch_block_at`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/watch.rs`.

## Owned contract

- Own run_watch and local status/write/time/cycle helpers. Reuse run_compile_observed, L3 watch/server operations and the L4 eviction wrapper (`00_nucleo/prompts/wiring/eviction.md`); no local polling, fingerprinting or HTTP algorithm.
- Reject stdout output. Serve semantic Html/Bundle unless no_serve, preserving port/no_reload/URL/PID staging and the existing event loop. Never infer HTML from an asset extension.
- Capture Instant before compilation; report status before diagnostics. Map elapsed/warnings/exit into typed WatchStatus with local time or UTC fallback. Compiling uses one LF, other blocks two; write_all then flush with existing propagated errors/stderr expect.
- Preserve dependency fallback then arm, successful publish/publish_directory or failed abandon/abandon_directory. Publication failure exits 2; HTTP state changes only on successful publish and the last disk/HTTP artifact survives errors.
- Keep evict(10) and wait(100ms) call placement. Published stays private in the parent. Local helper tests characterize status/order, not a new watcher or timing policy.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
