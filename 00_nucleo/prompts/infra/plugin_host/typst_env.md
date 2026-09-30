# L0 — infra/plugin_host/typst_env
Hash do Código: e2dae107

**Layer:** L3
**Exclusive consumer:** `03_infra/src/plugin_host/typst_env.rs`

## Current evidence

- `03_infra/src/plugin_host/typst_env.rs:9` — `CallData`.
- `03_infra/src/plugin_host/typst_env.rs:16` — `MemoryError`.
- `03_infra/src/plugin_host/typst_env.rs:23` — `linker`.
- `03_infra/src/plugin_host/typst_env.rs:47` — `wasm_minimal_protocol_write_args_to_buffer`.
- `03_infra/src/plugin_host/typst_env.rs:68` — `wasm_minimal_protocol_send_result_to_host`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/plugin_host/typst_env.rs`.

## Owned contract

- Own CallData/MemoryError, linker construction and the two imports under typst_env: wasm_minimal_protocol_write_args_to_buffer(Caller<CallData>,u32) and wasm_minimal_protocol_send_result_to_host(Caller<CallData>,u32,u32).
- Preserve import names, signatures, registration order and internal expect messages. CallData is local to its Store with argument copies, output and the existing memory-error slot.
- write_args consumes arguments, writes consecutively from ptr and records the first failed write's offset/length, stopping immediately. send_result consumes output, resizes to len, reads from ptr and records read failure without returning the buffer; success restores output.
- No new imports, argument validation, bounds policy or size limits are authorized. Execution owns precedence and public error formatting, not this callback module.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
