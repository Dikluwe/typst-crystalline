# ADR-007: World Interface for System Dependencies

## Status
**Accepted**

## Context

The Typst compiler needs to access external resources:
- Source files (imports)
- Images and data
- Fonts
- Configuration

However, Typst is deployed in different environments:
- CLI (direct filesystem access)
- Web App (files in memory/server)
- IDE (virtual file system)

An abstraction was needed to allow the compiler to work in any environment.

## Decision

Create the **`World`** trait as the interface for all system dependencies.

### Interface

```rust
pub trait World {
    /// Returns the main source file.
    fn main(&self) -> FileId;
    
    /// Resolves a path to a FileId.
    fn resolve(&self, path: &Path) -> FileResult<FileId>;
    
    /// Reads file contents.
    fn file(&self, id: FileId) -> FileResult<Bytes>;
    
    /// Returns available fonts.
    fn font(&self, index: usize) -> Option<Font>;
    
    /// Returns current date/time.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}
```

### Implementations

| Environment | Implementation |
|-------------|----------------|
| CLI | `SystemWorld` (typst-cli) |
| Web | `WebWorld` (closed source) |
| Tests | `TestWorld` |

## Consequences

### Positive
- **Portability**: Same compiler in any environment
- **Testability**: Easy to mock filesystem
- **Purity**: Compiler core doesn't do direct I/O
- **Flexibility**: Environments can implement cache, virtual FS, etc.

### Negative
- **Indirection**: Every file read goes through the trait
- **Complexity**: Implementers need to manage FileId

## References
- [typst-kit/](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/03_infra/typst-kit/)
