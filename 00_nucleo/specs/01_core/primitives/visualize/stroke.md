# 🧬 Crystal Facet: visualize/stroke.rs

> **Crystal Face**: The Stroke Style — Line Rendering.

---

## 💎 Facet DNA

$$
\text{Stroke} = \text{paint} + \text{thickness} + \text{cap} + \text{join} + \text{dash}
$$

**stroke.rs** defines the **Stroke Style** — configuration for line rendering.

---

## Stroke Properties

| Property | Values |
|----------|--------|
| **paint** | Color, Gradient, Pattern |
| **thickness** | Length |
| **cap** | butt, round, square |
| **join** | miter, round, bevel |
| **dash** | Pattern array |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE STROKE STYLE (stroke.rs)                    │
├──────────────────────────────────────────────────────────┤
│  Role: Line rendering                                    │
│  Properties: paint, thickness, cap, join, dash           │
└──────────────────────────────────────────────────────────┘
```
