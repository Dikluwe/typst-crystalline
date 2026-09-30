# L0 — Document state value
Hash do Código: 36fba5ee

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/state.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:27acb21a5e0b2e0cb3b65de61bba5266158f3e9828392fe92a1a3b160e9d61a4

## Contract

`State` preserves its key and initial value, construction/data access, and
equality/hash/repr consistent with language identity. It does not execute native
methods, introspection or I/O; those belong to other owners. The consumer stores
`key: EcoString` and `init: Box<Value>`, deriving `Debug`, `Clone`, `PartialEq`;
representation does not waive pinned language-semantics obligations.
