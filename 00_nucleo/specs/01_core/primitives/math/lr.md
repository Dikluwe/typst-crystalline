# 🧬 Crystal Facet: math/lr.rs

> **Crystal Face**: The Delimiter Scaling — Matched Bracket Pairs.

---

## 💎 Facet DNA

$$
\text{lr}(\text{left}, \text{content}, \text{right})
$$

**lr.rs** defines **Delimiter Scaling** — matched bracket pairs with automatic size adjustment.

---

## Prescriptive Axioms

### Axiom I: Size Adaptation

$$
\text{size}(\text{delimiters}) = \text{max height of content}
$$

Delimiters **scale** to match content height.

---

## Element Contracts

| Element | Role |
|---------|------|
| `lr` | General left-right pair |
| `abs` | Absolute value `|x|` |
| `norm` | Norm `‖x‖` |
| `floor` / `ceil` | Rounding brackets |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DELIMITER SCALING (lr.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Matched bracket pairs                             │
│  Elements: lr(), abs(), norm(), floor(), ceil()          │
└──────────────────────────────────────────────────────────┘
```
