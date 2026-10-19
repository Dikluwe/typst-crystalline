# L0 — Document state value
Hash do Código: bbd624bd

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/state.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:58a1e343cd346a842c0e0a2760c8b7438e89054b228d00a1b4972019320996e5

## Contract

`State` preserves its key and initial value, construction/data access, and
equality/hash/repr consistent with language identity. It does not execute native
methods, introspection or I/O; those belong to other owners. The consumer stores
`key: EcoString` and `init: Box<Value>`, deriving `Debug`, `Clone`, `PartialEq`;
representation does not waive pinned language-semantics obligations.
