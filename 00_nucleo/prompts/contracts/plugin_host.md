# L0 — WASM plugin host boundary
Hash do Código: 93f5264e

**Layer:** L1
**Exclusive consumer:** `01_core/src/contracts/plugin_host.rs`

## Evidence and boundary

The consumer declares the opaque module handle, domain error and host trait.
Retained upstream transition evidence is
`lab/typst-original/crates/typst-library/src/foundations/plugin.rs:328-352`;
linear-memory snapshot evidence is `plugin.rs:177-182,420-426,522-545` in
the same source. These are historical source coordinates, not fresh measurements
against the current upstream pin.

Runtime machinery belongs to `00_nucleo/prompts/infra/plugin_host.md`;
no `wasmi` type crosses into L1. Environment injection belongs to
`00_nucleo/prompts/contracts/world.md`, not this trait.

## Public interface

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PluginModuleId(pub u64);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginError { pub message: EcoString }
pub trait PluginHost: Send + Sync {
    fn load(&self, bytes: &[u8]) -> Result<PluginModuleId, PluginError>;
    fn exports(&self, module: PluginModuleId) -> Result<Vec<EcoString>, PluginError>;
    fn call(&self, module: PluginModuleId, func_name: &str, args: &[Bytes])
        -> Result<Bytes, PluginError>;
    fn transition(&self, module: PluginModuleId, func_name: &str, args: &[Bytes])
        -> Result<PluginModuleId, PluginError>;
}
```

`Bytes` is the pure entity type. `PluginError::new(impl Into<EcoString>)`
retains its message; `Display` writes it verbatim and the type implements
`std::error::Error`. Interior runtime mutation belongs to L3; preserve
`Send + Sync` and the shared `&self` interface.

## Behavior

- `load` compiles and validates the module and requires exported `memory`.
  A successfully loaded handle cannot later fail merely because the module
  is invalid or lacks memory.
- `exports` lists function exports in the host's stable order; upstream
  enumeration evidence is `foundations/plugin.rs:366-380` in the source above.
- `call` receives one buffer per argument. The WASM ABI passes lengths as
  `i32` and obtains bytes through `typst_env`. Each call starts from initial
  module memory; plugin-author purity is assumed, not enforced.
- `transition` performs a mutable call and derives a module from resulting
  linear-memory pages/data, not WASM globals. Calls and export inspection of
  the derived handle observe the mutation; the original remains unchanged.
  Distinct derivations retain distinct identity.
- Errors use the exact observable language messages specified by the L3 host
  owner. L1/L2 must not rewrite, prefix or translate them.

Message parity is observable behavior (ADR-0107); the opaque `u64` handle and
trait/runtime representation are mechanics, not upstream structure to copy.
Verify object safety, handle copy/equality/hash, verbatim error formatting,
signatures and the absence of runtime types/direct I/O in L1.
