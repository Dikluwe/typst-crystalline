# L0 — Object-safe DynElement bridge
Hash do Código: 75bc04e6

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/dynamic.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:eddb0b1b680907d9db343169884ebcc6e668d4d46a7e94146c6ddc018d4fe4f7

## Evidence and contract

The consumer defines `DynElement` and its blanket for `Element + Send + Sync
+ 'static`. Users implement the native trait, not a second manual bridge.

- Provide object-safe text, emptiness, map, field, kind, payload and settable bridges.
- `dyn_kind` delegates to the stable Element name. `dyn_eq` downcasts and compares
  structurally; distinct concrete types are unequal.
- `Arc<dyn DynElement>` retains O(1) cloning. Hashing stays content-Debug based,
  with no `dyn_hash` method.
- Native dispatch remains static; only `Content::Dynamic` uses the bridge.

Verify object safety, native blanket bridging, equality, maps and hub dispatch.
The native trait belongs to `00_nucleo/prompts/core/entities/elements/mod.md`.
