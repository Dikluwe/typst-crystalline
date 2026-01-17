# 🧬 Crystal Facet: grid/layouter.rs

> **Crystal Face**: The Grid Algorithm Core — Track Resolution Engine.

---

## 💎 Facet DNA

$$
\text{resolve} : (\text{Grid}, \text{Regions}) \to \text{Tracks}
$$

**layouter.rs** implements the **core grid algorithm** — resolving track sizes and positioning cells.

---

## Prescriptive Axioms

### Axiom I: Two-Phase Resolution

$$
\text{Phase}_1: \text{Columns} \to \text{widths}
$$
$$
\text{Phase}_2: \text{Rows} \to \text{heights}
$$

Track resolution occurs in **two phases**: columns first (width-constrained), then rows (height-constrained).

---

### Axiom II: Constraint Propagation

$$
\text{auto} \Rightarrow \text{measure}(\text{content})
$$

Auto-sized tracks **propagate** content constraints upward.

---

### Axiom III: Fractional Distribution

$$
\text{fr}(n) \Rightarrow \text{remaining} \times \frac{n}{\sum \text{fr}}
$$

Fractional tracks **distribute** remaining space proportionally.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE GRID ALGORITHM CORE (grid/layouter.rs)      │
├──────────────────────────────────────────────────────────┤
│  Laws: Two-phase resolution, constraint propagation      │
└──────────────────────────────────────────────────────────┘
```
