# L0 — infra/plugin_host/execution
Hash do Código: 8cd6ac64

**Layer:** L3
**Exclusive consumer:** `03_infra/src/plugin_host/execution.rs`

## Current evidence

- `03_infra/src/plugin_host/execution.rs:14` — `call`.
- `03_infra/src/plugin_host/execution.rs:44` — `transition`.
- `03_infra/src/plugin_host/execution.rs:83` — `run_call`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/plugin_host/execution.rs`.

## Owned contract

- Own call/transition/run_call only. Use a fresh Store/instance per call, with per-Store CallData; restore any entry snapshot before execution. No pooling, new memoization or linker reconstruction.
- Validate exported function lazily: all parameters i32, exactly one i32 result, then matching arity. Argument lengths use the existing i32 conversion and owned buffer copies.
- Preserve exact errors: ``plugin function `{func_name}` not found``; ``plugin function `{func_name}` has a parameter that is not a 32-bit integer``; ``plugin function `{func_name}` does not return exactly one 32-bit integer``; `plugin function takes {expected} argument{s}, but {given} {verb} given` (singular s/was rules unchanged).
- Runtime instantiation errors keep runtime text. Trap has precedence (`plugin panicked: {err}`), then recorded bounds failure (`plugin tried to {kind} out of bounds: pointer {offset:#x} is out of bounds for {kind} of length {length}`), then protocol result.
- Code 0 returns output Bytes. Code 1 returns `plugin errored with: {message}` for UTF-8, otherwise `plugin errored, but did not return a valid error message`. Other codes return `plugin did not respect the protocol`. Keep read/write kind and formatting exact.
- transition runs the same validation/protocol, discards successful output, captures linear memory only after success and registers a fresh derived id sharing the base. Failure creates no derived module and never mutates the original entry. Memory operations belong to `00_nucleo/prompts/infra/plugin_host/snapshot.md`, protocol callbacks to `00_nucleo/prompts/infra/plugin_host/typst_env.md`, load errors to loading.
- Missing module/function defense is direct API behavior; do not claim a new language parity measurement or capture WASM globals.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
