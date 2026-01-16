# ADR-009: Realization vs Layout

## Status
**Accepted**

## Context

Typst supports **show rules** that transform content before layout:

```typst
#show heading: it => text(blue)[#it.body]
```

This transformation must happen before layout, but after evaluation. A clear separation was needed.

## Decision

Split the layout phase into two sub-phases:

1. **Realization** (`typst-realize`): Apply show rules, transform Content into layoutable elements
2. **Layout** (`typst-layout`): Position realized elements into Frames

### Realization Process

```
Content → [show rules] → Realized Elements
   ↓                           ↓
 Raw AST elements      Ready for layout
```

### Key Properties
- **Shallow**: Only realizes top-level elements lazily
- **Recursive**: Show rules can produce content that needs realization
- **May evaluate**: Show rule closures trigger evaluation

## Consequences

### Positive
- **Separation of concerns**: Style transformation vs positioning
- **Composable rules**: Show rules can nest arbitrarily
- **Lazy**: Deep content realized only when layouted

### Negative
- **Complexity**: Two-phase process harder to understand
- **Interleaving**: Realization can trigger evaluation

## References
- [typst-realize/](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/01_core/typst-realize/)
