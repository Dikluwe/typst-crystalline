# 🧬 Crystal Facet: layout/transform.rs

> **Crystal Face**: The Affine Operators — Geometric Transformation.

---

## 💎 Facet DNA

$$
\text{Transform} : \text{Frame} \to \text{Frame}'
$$

**transform.rs** defines **Affine Operators** — geometric transformations on laid-out content.

---

## Element Contracts

| Element | Transformation |
|---------|----------------|
| `rotate` | Angular rotation |
| `scale` | Size scaling |
| `move` | Translation |
| `skew` | Shear distortion |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE AFFINE OPERATORS (transform.rs)             │
├──────────────────────────────────────────────────────────┤
│  Role: Geometric transformation                          │
│  Operations: rotate, scale, move, skew                   │
└──────────────────────────────────────────────────────────┘
```
