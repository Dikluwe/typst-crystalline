# 🧬 Crystal Facet: foundations/calc.rs

> **Crystal Face**: The Calculation Module — Mathematical Functions.

---

## 💎 Facet DNA

$$
\text{calc} : \text{Num}^* \to \text{Num}
$$

**calc.rs** defines the **Calculation Module** — mathematical functions exposed to Typst.

---

## Function Categories

| Category | Functions |
|----------|-----------|
| **Basic** | abs, min, max, clamp |
| **Powers** | pow, sqrt, exp, ln, log |
| **Trigonometry** | sin, cos, tan, asin, acos, atan |
| **Rounding** | floor, ceil, round, trunc, fract |
| **Special** | fact, perm, binom, gcd, lcm |
| **Constants** | pi, e, inf, nan |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE CALCULATION MODULE (calc.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Mathematical functions                            │
│                                                          │
│  Access: calc.sin(x), calc.sqrt(x), etc.                 │
└──────────────────────────────────────────────────────────┘
```
