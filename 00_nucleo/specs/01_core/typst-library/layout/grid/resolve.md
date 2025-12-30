# 🧬 Crystal Facet: layout/grid/resolve.rs

> **Crystal Face**: The Track Resolution — Column/Row Metric Calculation.

---

## 💎 Facet DNA

$$
\text{resolve} : \text{TrackSpec}^* \to \text{Abs}^*
$$

**resolve.rs** defines **Track Resolution** — calculating absolute column/row sizes from specifications.

---

## Prescriptive Axioms

### Axiom I: Track Specification

$$
\text{Track} \in \{\text{Abs}, \text{Fr}, \text{Auto}\}
$$

Tracks are specified as **absolute**, **fractional**, or **automatic**.

---

### Axiom II: Resolution Order

$$
\text{Abs} \to \text{Auto} \to \text{Fr}
$$

Resolution follows **priority order**: absolutes first, then auto-sized, then fractional.

---

### Axiom III: Fractional Distribution

$$
\text{Fr}_i = \frac{f_i}{\sum_j f_j} \cdot \text{remaining}
$$

Remaining space is **distributed** proportionally to fractional tracks.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TRACK RESOLUTION (resolve.rs)               │
├──────────────────────────────────────────────────────────┤
│  Role: Column/row metric calculation                     │
│                                                          │
│  Laws:                                                   │
│    ✓ Track Specification — abs/fr/auto                   │
│    ✓ Resolution Order — abs → auto → fr                  │
│    ✓ Fractional Distribution — proportional share        │
└──────────────────────────────────────────────────────────┘
```
