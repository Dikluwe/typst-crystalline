# 🧬 Crystal Domain: layout/

> **Crystal Face**: The Spatial Element Contracts — Geometric Primitive Definitions.

---

## 💎 Domain DNA

$$
\text{LayoutElement} : \text{Attributes} \to \text{GeometricFacet}
$$

**layout/** defines **Spatial Element Contracts** — element definitions that bind to layout facets.

---

## Core Concepts

### Axiom I: Metric Types

$$
\text{Length} = \text{Abs} + \text{Em} + \text{Ratio}
$$

**Length** combines absolute, font-relative, and percentage units.

---

### Axiom II: Container Hierarchy

$$
\text{Page} \supset \text{Block} \supset \text{Box}
$$

Layout elements form a **containment hierarchy**.

---

### Axiom III: Alignment Algebra

$$
\text{Align} : (\text{x-align}, \text{y-align}) \to \text{Position}
$$

**Alignment** determines positioning within available space.

---

## Element Contracts

| Category | Elements |
|----------|----------|
| **Metrics** | Length, Abs, Em, Ratio, Fr, Angle |
| **Geometry** | Point, Size, Axes, Sides, Corners |
| **Containers** | Page, Block, Box, Stack |
| **Positioning** | Align, Place, Pad, Hide |
| **Transforms** | Move, Rotate, Scale, Skew |
| **Structure** | Columns, Grid, Repeat |
| **Frames** | Frame, Fragment, Regions |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SPATIAL ELEMENT CONTRACTS (layout/)         │
├──────────────────────────────────────────────────────────┤
│  Role: Geometric primitive definitions                   │
│                                                          │
│  Laws:                                                   │
│    ✓ Metric Types — Length = Abs + Em + Ratio            │
│    ✓ Container Hierarchy — nested containment            │
│    ✓ Alignment Algebra — positioning within space        │
└──────────────────────────────────────────────────────────┘
```
