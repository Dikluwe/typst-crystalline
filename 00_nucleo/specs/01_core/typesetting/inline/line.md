# 🧬 Crystal Facet: inline/line.rs

> **Crystal Face**: The Line Manifold — Finite-Width Linear Space.

---

## 💎 Facet DNA

$$
\text{Line} = \text{Manifold}_{1D}(w_{finite})
$$

**line.rs** assembles items into a **line manifold** — a unidimensional space of finite width.

---

## Prescriptive Axioms

### Axiom I: Linear Manifold Definition

$$
\text{Line} = \{(x, \text{item}) \mid 0 \leq x \leq w\}
$$

A line is a **unidimensional manifold** of finite width where items are positioned along the x-axis.

---

### Axiom II: Baseline Symmetry Axis

$$
\forall i \in \text{Line}: y_i = \text{baseline}
$$

All items in a line share a common **baseline** — the symmetry axis for vertical alignment.

---

### Axiom III: Vacuum Compensation Law

$$
\delta w = w_{manifold} - \sum_{i} w_i
$$
$$
\text{space}_j := \text{space}_j + \frac{\delta w}{\#\text{spaces}}
$$

**Vacuum Compensation Law**: The difference between the manifold width and the sum of item widths (the **vacuum**) is distributed proportionally among space glyphs to achieve justification.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    VACUUM COMPENSATION                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Manifold width: ══════════════════════════════ (100pt)        │
│                                                                 │
│   Items:          ████ ███ █████ ██████ ████     (80pt)         │
│                                                                 │
│   Vacuum:                                         (20pt)        │
│                                                                 │
│   Spaces (4):     ▢    ▢         ▢        ▢                     │
│                                                                 │
│   Compensation:   +5pt +5pt      +5pt     +5pt                  │
│                                                                 │
│   Result:         ████▢███▢█████▢██████▢████ (100pt justified)  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE LINE MANIFOLD (inline/line.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Finite-width linear space                         │
│                                                          │
│  Laws:                                                   │
│    ✓ Linear Manifold Definition — 1D finite width        │
│    ✓ Baseline Symmetry Axis — common vertical anchor     │
│    ✓ Vacuum Compensation Law — distribute excess width   │
└──────────────────────────────────────────────────────────┘
```
