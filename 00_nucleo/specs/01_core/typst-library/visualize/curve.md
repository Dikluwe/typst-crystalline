# 🧬 Crystal Facet: visualize/curve.rs

> **Crystal Face**: The Curve Builder — Bezier Curve Construction.

---

## 💎 Facet DNA

$$
\text{curve}(\text{components}) \to \text{bezier path}
$$

**curve.rs** defines the **Curve Builder** — constructing bezier curves from components.

---

## Component Types

| Component | Description |
|-----------|-------------|
| `curve.move` | Move to point |
| `curve.line` | Line to point |
| `curve.quad` | Quadratic bezier |
| `curve.cubic` | Cubic bezier |
| `curve.close` | Close path |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE CURVE BUILDER (curve.rs)                    │
├──────────────────────────────────────────────────────────┤
│  Role: Bezier curve construction                         │
│  Element: curve()                                        │
└──────────────────────────────────────────────────────────┘
```
