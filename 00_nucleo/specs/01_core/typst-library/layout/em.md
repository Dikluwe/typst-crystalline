# 🧬 Crystal Facet: layout/em.rs

> **Crystal Face**: The Font-Relative Metric — Typographic Scaling.

---

## 💎 Facet DNA

$$
\text{Em} = k \cdot \text{font-size}
$$

**em.rs** defines the **Font-Relative Metric** — lengths that scale with font size.

---

## Prescriptive Axioms

### Axiom I: Proportional Scaling

$$
1\text{em} = \text{current font-size}
$$

Em units are **proportional** to the current font.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FONT-RELATIVE METRIC (em.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Typographic scaling unit                          │
│  Resolution: em × font-size → abs                        │
└──────────────────────────────────────────────────────────┘
```
