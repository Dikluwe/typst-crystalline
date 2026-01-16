# Contract: World

> The system dependency interface for the Typst compiler.

## Purpose

`World` is the abstraction layer between the compiler and the environment. It provides access to files, fonts, and configuration without the compiler knowing the underlying implementation.

## Interface

```rust
pub trait World: Send + Sync {
    /// The main source file.
    fn main(&self) -> FileId;

    /// The root relative to which absolute paths are resolved.
    fn root(&self) -> &Path;

    /// Resolve a path to a file ID.
    fn resolve(&self, path: &Path) -> FileResult<FileId>;

    /// Read a file's content.
    fn file(&self, id: FileId) -> FileResult<Bytes>;

    /// Get a font by index.
    fn font(&self, index: usize) -> Option<Font>;

    /// Get the current date.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | `main()` always returns a valid FileId |
| **I2** | `file(id)` for any resolved ID must succeed |
| **I3** | Font indices are 0-based and contiguous |
| **I4** | Thread-safe: `Send + Sync` bound |

## Implementer Obligations

### File Resolution
```
resolve(path) → Ok(id)  ⟹  file(id) → Ok(bytes)
```
Any successfully resolved path must be readable.

### Font Enumeration
```
font(0)..font(n-1) → Some(_)
font(n) → None
```
Fonts are enumerable starting from 0.

## Consumer Obligations

- Must not assume file system semantics
- Must handle `FileResult::NotFound` gracefully
- Must not cache `today()` results across compilations

## Known Implementations

| Implementation | Crate | Environment |
|---------------|-------|-------------|
| `SystemWorld` | typst-cli | CLI/local filesystem |
| `TestWorld` | tests | Unit testing |

## Related

- [ADR-007: World Interface](../adr/ADR-007-world-interface.md)
- [typst-kit/](../../03_infra/typst-kit/)
