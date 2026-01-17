# 🧬 Crystal Facet: foundations/float.rs

> **Crystal Face**: The Float Type — IEEE 754 Floating Point.

---

## 💎 Facet DNA

$$
\text{Float} \in \mathbb{R}_{64}
$$

**float.rs** defines the **Float Type** — 64-bit IEEE 754 floating point numbers.

---

## Method Contracts

| Method | Contract |
|--------|----------|
| `is-nan` | NaN test |
| `is-infinite` | Infinity test |
| `signum` | Sign: -1.0, 0.0, 1.0 |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FLOAT TYPE (float.rs)                       │
├──────────────────────────────────────────────────────────┤
│  Role: IEEE 754 floating point                           │
│  Special values: NaN, +∞, -∞                             │
└──────────────────────────────────────────────────────────┘
```
