# ADR-011: Content as Central Type

## Status
**Accepted**

## Context

Typst is a typesetting system. The language needs to represent typographic content (text, paragraphs, equations, tables) in a way that can be:
- Composed (joining contents)
- Transformed (applying styles)
- Laid out (positioning on pages)

A central type was needed to unify all forms of content.

## Decision

Create **`Content`** as the main language type, representing typographic content.

### Structure

```
Content
├── elem: Element (vtable with element type)
├── span: Span (source code location)
└── data: <element-specific type>
```

### Characteristics

| Property | Description |
|----------|-------------|
| **Type-erased** | Any element can be Content |
| **Composable** | Sequences via `+` |
| **Stylable** | Show/set rules apply transformations |
| **Labelable** | Labels for cross-references |

### Elements

Each type of content is an "element":

```typst
#heading(level: 1)[Title]   // HeadingElem
#text(red)[Red]             // TextElem  
$x^2$                       // EquationElem
```

## Consequences

### Positive
- **Unification**: All content is treated uniformly
- **Composability**: `content + content` works naturally
- **Extensibility**: New elements are easily added
- **Type-safe internally**: `Packed<T>` guarantees types at compile time

### Negative
- **Type-erasure overhead**: Type information at runtime
- **vtable complexity**: Custom implementation needed

## References
- [specs/foundations/content/](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/specs/01_core/typst-library/foundations/content/)
