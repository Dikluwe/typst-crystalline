# Contract: Exporter

> Interface for output format exporters.

## Purpose

Exporters transform laid-out frames into output formats (PDF, SVG, PNG). Each exporter operates independently on the same input.

## Common Pattern

```rust
pub fn export(
    document: &Document,
    options: &Options,
) -> Result<Vec<u8>, Error>;
```

## Exporters

### PDF (`typst-pdf`)

```rust
pub fn pdf(
    document: &Document,
    options: &PdfOptions,
) -> Result<Vec<u8>, Vec<EcoString>>;
```

| Option | Type | Description |
|--------|------|-------------|
| `timestamp` | `Option<Datetime>` | Creation timestamp |
| `standards` | `PdfStandards` | PDF/A compliance |

### SVG (`typst-svg`)

```rust
pub fn svg(frame: &Frame) -> String;
pub fn svg_merged(frames: &[Frame], gap: Abs) -> String;
```

| Variant | Description |
|---------|-------------|
| Single frame | One SVG per page |
| Merged | All pages in one SVG |

### PNG (`typst-render`)

```rust
pub fn render(frame: &Frame, pixel_per_pt: f32) -> Pixmap;
```

| Option | Type | Description |
|--------|------|-------------|
| `pixel_per_pt` | `f32` | Resolution scale |

## Invariants

| Invariant | Description |
|-----------|-------------|
| **I1** | Exporters are pure: same input → same output |
| **I2** | Frame order preserved in multi-page output |
| **I3** | Errors collected, not thrown immediately |

## Related

- [ADR-014: Multiple Exporters](../adr/ADR-014-multiple-exporters.md)
- [typst-pdf/](../../03_infra/typst-pdf/)
- [typst-svg/](../../03_infra/typst-svg/)
