# Interface Contracts

> Formal interface specifications for the Typst compiler.

## Purpose

Contracts define the behavioral guarantees of key interfaces. They specify:
- **Invariants**: Properties that always hold
- **Obligations**: What implementers must provide
- **Usage**: How consumers should interact

## Index

### Core Types
| Contract | Description |
|----------|-------------|
| [Content](content.md) | Type-erased content container |
| [Value](value.md) | Runtime value representation |
| [Span](span.md) | Source location tracking |

### Element System
| Contract | Description |
|----------|-------------|
| [Element](element.md) | Element type descriptor |
| [NativeElement](native-element.md) | Trait for element structs |
| [Packed\<T\>](packed.md) | Type-safe content wrapper |

### Compilation
| Contract | Description |
|----------|-------------|
| [World](world.md) | System dependency interface |
| [StyleChain](stylechain.md) | Style inheritance |
| [Layout](layout.md) | Layout trait |
| [Exporter](exporter.md) | Output format exporters |

## Contract Format

Each contract follows this structure:

```markdown
# Contract: Name

## Purpose
Why this interface exists.

## Interface
The Rust signature.

## Invariants
Properties that always hold.

## Obligations
What implementers must do.

## Related
Links to ADRs and specs.
```
