# ADR-001: Compilation Phases

## Status
**Accepted**

## Context

Typst needs to transform source code into final documents (PDF, SVG, HTML). This transformation is complex and involves multiple responsibilities: syntactic analysis, code execution, visual positioning, and serialization to output formats.

A monolithic design would make maintenance, testing, and adding new export formats difficult.

## Decision

The compilation process was divided into **4 distinct sequential phases**:

```
┌─────────┐    ┌────────────┐    ┌────────┐    ┌────────┐
│ Parsing │ →  │ Evaluation │ →  │ Layout │ →  │ Export │
└─────────┘    └────────────┘    └────────┘    └────────┘
     ↓              ↓                ↓             ↓
 SyntaxNode      Content          Frame[]        PDF/SVG
```

### Phase 1: Parsing (`typst-syntax`)
- **Input**: `&str` (source code)
- **Output**: `SyntaxNode` (CST)
- **Characteristic**: Pure function, never fails

### Phase 2: Evaluation (`typst-eval`)
- **Input**: `Source` + `World`
- **Output**: `Module` (Content + Scope)
- **Characteristic**: Resolves imports, executes code

### Phase 3: Layout (`typst-layout`)
- **Input**: `Content`
- **Output**: `Frame[]` (one per page)
- **Characteristic**: Positioning and page breaking

### Phase 4: Export (`typst-pdf`, `typst-svg`, etc.)
- **Input**: `Frame[]`
- **Output**: Final format bytes
- **Characteristic**: Serialization to different formats

## Consequences

### Positive
- **Separation of concerns**: Each phase has a clear scope
- **Testability**: Phases can be tested in isolation
- **Extensibility**: New exporters don't affect other phases
- **Incrementality**: Memoization can be applied per phase

### Negative
- **Passing overhead**: Data is transformed between each phase
- **Type complexity**: Multiple intermediate types (SyntaxNode → Content → Frame)

## References
- [legacy-docs/dev/architecture.md](file:///home/dikluwe/.gemini/antigravity/scratch/typst-crystalline/00_nucleo/legacy-docs/dev/architecture.md)
