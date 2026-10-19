# L0 — Compilation environment contract
Hash do Código: 5abd444e

**Layer:** L1
**Exclusive consumer:** `01_core/src/contracts/world.rs`

## Evidence

`01_core/src/contracts/world.rs` declares the pure `World: Send + Sync` trait,
rooted-path methods and a duration-based `today`. Retained upstream evidence:
`lab/typst-original/crates/typst-library/src/lib.rs:92-99` for duration offsets,
and `foundations/path.rs:186-214` under that tree for captured path resolution.
These historical coordinates support the decisions, not fresh parity claims.

## Boundary and public interface

`World` declares what the pure core needs; L3 obtains it from disk, network,
memory or mocks. L1 never performs direct system I/O. This owner defines
`World`, `SysInputs` and local mock tests, not the facade or tracking adapter.

```rust
pub type SysInputs = IndexMap<EcoString, EcoString, FxBuildHasher>;
pub trait World: Send + Sync {
    fn library(&self) -> &Library;
    fn book(&self) -> &FontBook;
    fn main(&self) -> FileId;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId) -> FileResult<Bytes>;
    fn resolve_path(&self, current_file: FileId, path: &str) -> Result<RootedPath, String>;
    fn read_path(&self, path: &RootedPath) -> Result<Arc<Vec<u8>>, String>;
    fn include_path(&self, path: &RootedPath) -> Result<Source, String>;
    fn read_bytes(&self, current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String>;
    fn include_source(&self, current_file: FileId, path: &str) -> Result<Source, String>;
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String>;
    fn inputs(&self) -> SysInputs;
    fn plugin_host(&self) -> Option<Arc<dyn PluginHost>>;
    fn font(&self, index: usize) -> Option<Font>;
    fn candidates_for_char(&self, c: char) -> Vec<usize>;
    fn today(&self, offset: Option<Duration>) -> Option<Datetime>;
}
```

The excerpt omits default bodies, not their obligations:

- `source`/`file` may fail; callers must handle `FileResult`. Unavailable font
  slots return `None`; font consumers retain fallback.
- `resolve_path` performs no I/O and captures `VirtualRoot` and the originating
  file's base. Absolute paths resolve within the current project/package root;
  relative strings resolve against the originating directory. `Value::Path`
  supplies its captured identity instead of being rebound in another consumer.
- `read_path`/`include_path` delegate materialization to L3, which validates root
  containment and performs I/O/cache operations. Their defaults, and the default
  resolution method, report unsupported operations so mocks need no filesystem.
- Legacy `read_bytes`/`include_source` retain two-argument signatures and
  unsupported-operation error defaults. Supporting compatibility adapters must
  compose rooted resolution plus the corresponding rooted operation; they must
  not become a parallel normalization authority.
- `resolve_package` returns the entrypoint source or an error; the default
  reports unsupported package resolution. Cache lookup, manifests and physical
  directory conventions belong to L3.
- `inputs` defaults to an empty insertion-ordered map. Values remain strings:
  `--input n=42` exposes `"42"`, not an integer.
- `plugin_host` defaults to `None`; the capability interface belongs to
  `00_nucleo/prompts/contracts/plugin_host.md`.
- `candidates_for_char` collects `FontBook` candidates by default. Coverage is
  approximate by blocks of 256 codepoints; L3 confirms actual glyph availability.
  Overrides may calculate coverage lazily.
- `today(None)` requests the local date; `Some(offset)` requests UTC plus the
  exact duration, including sub-hour precision. `None` as the result denotes
  unavailable date information.

Preserve existing diagnostic strings, optional-service defaults, thread bounds,
pure mocks and error propagation. ADR-0005 separates this interface from tracking;
this prompt does not declare `TrackedWorld` or prescribe an unsupported
`TrackedWorld: World` supertrait. Contract/code discrepancies belong to
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`, not to
silent weakening of obligations or public API changes.
