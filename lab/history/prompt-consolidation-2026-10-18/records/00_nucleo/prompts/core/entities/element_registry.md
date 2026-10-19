# L0 — Injected element registry
Hash do Código: 7607893d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/element_registry.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

## Evidence and contract

The consumer maps names to `ElementCtor` without global state. Constructors are
`Arc<dyn Fn(&[Value]) -> SourceResult<Content> + Send + Sync>`.

- Register/replace constructors by name, query names and clone a constructor O(1).
- Construct known elements; unknown names produce a source diagnostic, not panic.
  Preserve its observable message and detached span.
- Stay pure and injected, without global `static`, `OnceLock` or I/O.
- The guard sweeps all names, testing construction, dynamic dispatch and field
  round-trips. Verify empty construction, lookup and unknown-name errors too.

Evaluation integration and chain behavior remain with evaluator/style owners.
