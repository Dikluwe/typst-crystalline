# 🧬 Crystal Facet: grid/rowspans.rs

> **Crystal Face**: The Span Resolver — Tension Distribution Geometry.

---

## 💎 Facet DNA

$$
\text{span}(c, n) \to \text{distribute}(\mu, [t_1, \ldots, t_n])
$$

**rowspans.rs** resolves cells spanning multiple tracks via **tension distribution**.

---

## Prescriptive Axioms

### Axiom I: Tension Distribution Law

$$
\text{Cell spanning tracks } [t_1, \ldots, t_n]: \mu_{min}(c) \leq \sum_{k=1}^n \mu(t_k)
$$

**Tension Distribution Law**: A cell occupying $n$ channels must distribute its minimum magnitude across the aggregate metric of those channels. If the aggregate is insufficient, the channels must expand to accommodate.

$$
\sum \mu(t_k) < \mu_{min}(c) \Rightarrow \text{expand tracks proportionally}
$$

---

### Axiom II: Aggregate Height

$$
h_{span} = \sum_{k=i}^{i+n-1} h_k
$$

Spanning cell height equals the **sum** of spanned row heights.

---

### Axiom III: Unbreakable Group Formation

$$
\text{span across rows} \Rightarrow \text{UnbreakableGroup}
$$

Rows connected by a spanning cell form an **unbreakable group** — they cannot be separated across region boundaries.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    TENSION DISTRIBUTION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Cell spans rows 1-3:                                          │
│                                                                 │
│   Row 1: h₁ = 20    ┐                                           │
│   Row 2: h₂ = 15    ├── sum = 55 (cell needs 60)                │
│   Row 3: h₃ = 20    ┘                                           │
│                                                                 │
│   Tension: 60 - 55 = 5 → distribute among rows                  │
│                                                                 │
│   Adjusted:                                                     │
│   Row 1: h₁ = 22    ┐                                           │
│   Row 2: h₂ = 16    ├── sum = 60 ✓                              │
│   Row 3: h₃ = 22    ┘                                           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SPAN RESOLVER (grid/rowspans.rs)            │
├──────────────────────────────────────────────────────────┤
│  Role: Tension distribution geometry                     │
│                                                          │
│  Laws:                                                   │
│    ✓ Tension Distribution Law — aggregate accommodation  │
│    ✓ Aggregate Height — sum of spanned rows              │
│    ✓ Unbreakable Group Formation — atomic row clusters   │
└──────────────────────────────────────────────────────────┘
```
