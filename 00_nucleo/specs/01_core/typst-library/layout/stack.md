# 🧬 Crystal Facet: layout/stack.rs

> **Crystal Face**: The Linear Accumulator — Axial Stacking.

---

## 💎 Facet DNA

$$
\text{Stack} : \text{children} \xrightarrow{\text{axis}} \text{linear arrangement}
$$

**stack.rs** defines the **Linear Accumulator** — stacking content along an axis.

---

## Prescriptive Axioms

### Axiom I: Axial Progression

$$
\text{position}_n = \sum_{i=1}^{n-1} (\text{size}_i + \text{gap})
$$

Elements are positioned by **cumulative size**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE LINEAR ACCUMULATOR (stack.rs)               │
├──────────────────────────────────────────────────────────┤
│  Role: Axial stacking                                    │
│  Direction: horizontal or vertical                       │
└──────────────────────────────────────────────────────────┘
```
