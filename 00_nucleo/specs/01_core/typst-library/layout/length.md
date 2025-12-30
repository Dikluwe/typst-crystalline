# 🧬 Crystal Facet: layout/length.rs

> **Crystal Face**: The Composite Metric — Absolute + Relative Length.

---

## 💎 Facet DNA

$$
\text{Length} = \text{Abs} + \text{Em} \cdot \text{font-size}
$$

**length.rs** defines the **Composite Metric** — lengths composed of absolute and font-relative components.

---

## Prescriptive Axioms

### Axiom I: Additive Composition

$$
\text{Length} = a_{pt} + b_{em}
$$

Lengths are **sums** of absolute (pt) and relative (em) components.

---

### Axiom II: Context Resolution

$$
\text{resolve}(L, \text{font-size}) \to \text{Abs}
$$

Lengths **resolve** to absolute values given font context.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE COMPOSITE METRIC (length.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Absolute + relative length                        │
│  Resolution: L → abs given font-size                     │
└──────────────────────────────────────────────────────────┘
```
