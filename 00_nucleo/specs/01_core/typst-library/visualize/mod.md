# 🧬 Crystal Domain: visualize/

> **Crystal Face**: The Visual Primitive Contracts — Stroke, Fill, and Color.

---

## 💎 Domain DNA

$$
\text{Visualize} : \text{Primitives} + \text{Paint} \to \text{Rendering}
$$

**visualize/** defines **Visual Primitive Contracts** — colors, strokes, fills, and geometric primitives.

---

## Core Concepts

### Axiom I: Color Space Algebra

$$
\text{Color} \in \{\text{RGB}, \text{CMYK}, \text{HSL}, \text{HSV}, \text{Oklab}, \text{Oklch}, \text{Luma}\}
$$

Colors exist in multiple **color spaces** with defined conversions.

---

### Axiom II: Paint Hierarchy

$$
\text{Paint} = \text{Color} \mid \text{Gradient} \mid \text{Tiling}
$$

**Paint** is a polymorphic fill source.

---

### Axiom III: Stroke-Fill Duality

$$
\text{Shape} = (\text{Stroke}^?, \text{Fill}^?, \text{Geometry})
$$

Every shape has optional **stroke** (boundary) and **fill** (interior).

---

## Element Contracts

| Category | Elements |
|----------|----------|
| **Colors** | Color, RGB, CMYK, HSL, Oklab, Luma |
| **Paints** | Paint, Gradient (linear, radial, conic), Tiling |
| **Strokes** | Stroke (width, cap, join, dash) |
| **Shapes** | Line, Rect, Circle, Ellipse, Polygon, Path |
| **Curves** | Curve (Bezier segments) |
| **Images** | Image (raster, vector) |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE VISUAL PRIMITIVE CONTRACTS (visualize/)     │
├──────────────────────────────────────────────────────────┤
│  Role: Stroke, fill, and color contracts                │
│                                                          │
│  Laws:                                                   │
│    ✓ Color Space Algebra — multiple spaces, conversions  │
│    ✓ Paint Hierarchy — color/gradient/tiling             │
│    ✓ Stroke-Fill Duality — boundary vs interior          │
└──────────────────────────────────────────────────────────┘
```
