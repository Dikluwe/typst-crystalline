# ADR-014: Multiple Export Formats

## Status
**Accepted**

## Context

Typst documents need to be exported to various formats:
- PDF (print, archival)
- SVG (scalable graphics)
- PNG (raster images)
- HTML (web, future)

Each format has different requirements and capabilities.

## Decision

Implement **separate exporter crates** for each output format, all consuming the same `Frame[]` output from layout.

### Architecture

```
typst-layout → Frame[]
                 ↓
    ┌────────────┼────────────┐
    ↓            ↓            ↓
typst-pdf   typst-svg   typst-render
    ↓            ↓            ↓
   PDF          SVG          PNG
```

### Exporter Crates

| Crate | Output | Notes |
|-------|--------|-------|
| `typst-pdf` | PDF | Full feature support |
| `typst-svg` | SVG | Per-frame export |
| `typst-render` | Pixel buffer | Via tiny-skia |
| `typst-html` | HTML | Different pipeline (from Content) |

## Consequences

### Positive
- **Independence**: Exporters evolve separately
- **Optional**: Users include only needed exporters
- **Unified input**: All use same Frame structure

### Negative
- **Duplication**: Some rendering logic repeated
- **HTML exception**: Starts from Content, not Frames

## References
- [03_infra/](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/03_infra/)
