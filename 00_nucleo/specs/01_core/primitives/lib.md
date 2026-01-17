# 🧬 Crystal Facet: typst-library

> **Crystal Face**: The Standard Library — Semantic Contract Repository.

---

## 💎 Facet DNA

$$
\text{Library} : \text{Semantic Contracts for Document Elements}
$$

**typst-library** is the **Semantic Contract Repository** — defining the identity, attributes, and behavioral contracts of all document elements.

---

## Core Architecture

### The World Trait

$$
\text{World} : \text{Environment} \to \text{Resources}
$$

The World trait defines the **compilation environment contract**:
- **library**: Standard library definition
- **main**: Entry source file
- **source**: Source file resolution
- **file**: Binary file resolution
- **font**: Font enumeration

---

## Domain Organization

| Domain | Role |
|--------|------|
| `foundations/` | Core type contracts (Value, Content, Styles) |
| `introspection/` | Query and state contracts |
| `layout/` | Layout element definitions |
| `model/` | Document structure elements |
| `text/` | Typography and font contracts |
| `visualize/` | Visual primitive contracts |
| `math/` | Mathematical element contracts |
| `loading/` | External resource contracts |
| `pdf/` | Export-specific contracts |

---

## Crystal Purity

| Aspect | Status |
|--------|--------|
| I/O | **Impure** — World trait requires I/O |
| State | **Contains state definitions** |
| Role | **Contract definition**, not execution |

---

## Dependencies

| Crate | Role |
|-------|------|
| `typst-syntax` | Source and span types |
| `typst-utils` | Utility types |
| `typst-macros` | Derive macros |
| `comemo` | Memoization |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE STANDARD LIBRARY (typst-library)            │
├──────────────────────────────────────────────────────────┤
│  Role: Semantic contract repository                      │
│                                                          │
│  Domains:                                                │
│    foundations/ — Core types and values                  │
│    introspection/ — Query and state                      │
│    layout/ — Element definitions                         │
│    model/ — Document structure                           │
│    text/ — Typography                                    │
│    visualize/ — Visual primitives                        │
│    math/ — Mathematical elements                         │
│    loading/ — External resources                         │
│    pdf/ — Export specifics                              │
└──────────────────────────────────────────────────────────┘
```
